#[cfg(feature = "implement_exclusive")]
pub trait IAppDataPathsImpl: Sized {
    fn Cookies(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Documents(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Favorites(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn History(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InternetCache(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProgramData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingAppData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDataPaths {
    const NAME: &'static str = "Windows.Storage.IAppDataPaths";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDataPathsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDataPathsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDataPathsVtbl {
        unsafe extern "system" fn Cookies<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Desktop<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Desktop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Documents<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Documents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Favorites<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Favorites() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn History<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).History() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternetCache<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternetCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAppData<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAppData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgramData<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgramData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingAppData<Impl: IAppDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingAppData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDataPaths, BASE_OFFSET>(),
            Cookies: Cookies::<Impl, IMPL_OFFSET>,
            Desktop: Desktop::<Impl, IMPL_OFFSET>,
            Documents: Documents::<Impl, IMPL_OFFSET>,
            Favorites: Favorites::<Impl, IMPL_OFFSET>,
            History: History::<Impl, IMPL_OFFSET>,
            InternetCache: InternetCache::<Impl, IMPL_OFFSET>,
            LocalAppData: LocalAppData::<Impl, IMPL_OFFSET>,
            ProgramData: ProgramData::<Impl, IMPL_OFFSET>,
            RoamingAppData: RoamingAppData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDataPaths as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppDataPathsStaticsImpl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<AppDataPaths>;
    fn GetDefault(&mut self) -> ::windows::core::Result<AppDataPaths>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDataPathsStatics {
    const NAME: &'static str = "Windows.Storage.IAppDataPathsStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppDataPathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDataPathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDataPathsStaticsVtbl {
        unsafe extern "system" fn GetForUser<Impl: IAppDataPathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Impl: IAppDataPathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDataPathsStatics, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDataPathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationDataImpl: Sized {
    fn Version(&mut self) -> ::windows::core::Result<u32>;
    fn SetVersionAsync(&mut self, desiredversion: u32, handler: &::core::option::Option<ApplicationDataSetVersionHandler>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ClearAllAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ClearAsync(&mut self, locality: ApplicationDataLocality) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LocalSettings(&mut self) -> ::windows::core::Result<ApplicationDataContainer>;
    fn RoamingSettings(&mut self) -> ::windows::core::Result<ApplicationDataContainer>;
    fn LocalFolder(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn RoamingFolder(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn TemporaryFolder(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn DataChanged(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveDataChanged(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SignalDataChanged(&mut self) -> ::windows::core::Result<()>;
    fn RoamingStorageQuota(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationData {
    const NAME: &'static str = "Windows.Storage.IApplicationData";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationDataVtbl {
        unsafe extern "system" fn Version<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVersionAsync<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredversion: u32, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVersionAsync(desiredversion, &*(&handler as *const <ApplicationDataSetVersionHandler as ::windows::core::Abi>::Abi as *const <ApplicationDataSetVersionHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAllAsync<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAsync<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locality: ApplicationDataLocality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearAsync(locality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalSettings<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingSettings<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalFolder<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingFolder<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemporaryFolder<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemporaryFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataChanged<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataChanged<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalDataChanged<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SignalDataChanged().into()
        }
        unsafe extern "system" fn RoamingStorageQuota<Impl: IApplicationDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingStorageQuota() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationData, BASE_OFFSET>(),
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersionAsync: SetVersionAsync::<Impl, IMPL_OFFSET>,
            ClearAllAsync: ClearAllAsync::<Impl, IMPL_OFFSET>,
            ClearAsync: ClearAsync::<Impl, IMPL_OFFSET>,
            LocalSettings: LocalSettings::<Impl, IMPL_OFFSET>,
            RoamingSettings: RoamingSettings::<Impl, IMPL_OFFSET>,
            LocalFolder: LocalFolder::<Impl, IMPL_OFFSET>,
            RoamingFolder: RoamingFolder::<Impl, IMPL_OFFSET>,
            TemporaryFolder: TemporaryFolder::<Impl, IMPL_OFFSET>,
            DataChanged: DataChanged::<Impl, IMPL_OFFSET>,
            RemoveDataChanged: RemoveDataChanged::<Impl, IMPL_OFFSET>,
            SignalDataChanged: SignalDataChanged::<Impl, IMPL_OFFSET>,
            RoamingStorageQuota: RoamingStorageQuota::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationData as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationData2Impl: Sized {
    fn LocalCacheFolder(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationData2 {
    const NAME: &'static str = "Windows.Storage.IApplicationData2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationData2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationData2Vtbl {
        unsafe extern "system" fn LocalCacheFolder<Impl: IApplicationData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalCacheFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationData2, BASE_OFFSET>(),
            LocalCacheFolder: LocalCacheFolder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationData2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationData3Impl: Sized {
    fn GetPublisherCacheFolder(&mut self, foldername: &::windows::core::HSTRING) -> ::windows::core::Result<StorageFolder>;
    fn ClearPublisherCacheFolderAsync(&mut self, foldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn SharedLocalFolder(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationData3 {
    const NAME: &'static str = "Windows.Storage.IApplicationData3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationData3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationData3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationData3Vtbl {
        unsafe extern "system" fn GetPublisherCacheFolder<Impl: IApplicationData3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisherCacheFolder(&*(&foldername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPublisherCacheFolderAsync<Impl: IApplicationData3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, foldername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearPublisherCacheFolderAsync(&*(&foldername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharedLocalFolder<Impl: IApplicationData3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharedLocalFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationData3, BASE_OFFSET>(),
            GetPublisherCacheFolder: GetPublisherCacheFolder::<Impl, IMPL_OFFSET>,
            ClearPublisherCacheFolderAsync: ClearPublisherCacheFolderAsync::<Impl, IMPL_OFFSET>,
            SharedLocalFolder: SharedLocalFolder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationData3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IApplicationDataContainerImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Locality(&mut self) -> ::windows::core::Result<ApplicationDataLocality>;
    fn Values(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
    fn Containers(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>>;
    fn CreateContainer(&mut self, name: &::windows::core::HSTRING, disposition: ApplicationDataCreateDisposition) -> ::windows::core::Result<ApplicationDataContainer>;
    fn DeleteContainer(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationDataContainer {
    const NAME: &'static str = "Windows.Storage.IApplicationDataContainer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IApplicationDataContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationDataContainerVtbl {
        unsafe extern "system" fn Name<Impl: IApplicationDataContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Locality<Impl: IApplicationDataContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationDataLocality) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Impl: IApplicationDataContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Values() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Containers<Impl: IApplicationDataContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Containers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContainer<Impl: IApplicationDataContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, disposition: ApplicationDataCreateDisposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainer(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), disposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteContainer<Impl: IApplicationDataContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteContainer(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationDataContainer, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Locality: Locality::<Impl, IMPL_OFFSET>,
            Values: Values::<Impl, IMPL_OFFSET>,
            Containers: Containers::<Impl, IMPL_OFFSET>,
            CreateContainer: CreateContainer::<Impl, IMPL_OFFSET>,
            DeleteContainer: DeleteContainer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationDataContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataStaticsImpl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<ApplicationData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataStatics {
    const NAME: &'static str = "Windows.Storage.IApplicationDataStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationDataStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IApplicationDataStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationDataStatics, BASE_OFFSET>(), Current: Current::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationDataStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IApplicationDataStatics2Impl: Sized {
    fn GetForUserAsync(&mut self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ApplicationData>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationDataStatics2 {
    const NAME: &'static str = "Windows.Storage.IApplicationDataStatics2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IApplicationDataStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationDataStatics2Vtbl {
        unsafe extern "system" fn GetForUserAsync<Impl: IApplicationDataStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationDataStatics2, BASE_OFFSET>(),
            GetForUserAsync: GetForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationDataStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Provider", feature = "implement_exclusive"))]
pub trait ICachedFileManagerStaticsImpl: Sized {
    fn DeferUpdates(&mut self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<()>;
    fn CompleteUpdatesAsync(&mut self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Provider", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICachedFileManagerStatics {
    const NAME: &'static str = "Windows.Storage.ICachedFileManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Provider", feature = "implement_exclusive"))]
impl ICachedFileManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICachedFileManagerStaticsVtbl {
        unsafe extern "system" fn DeferUpdates<Impl: ICachedFileManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeferUpdates(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompleteUpdatesAsync<Impl: ICachedFileManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteUpdatesAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICachedFileManagerStatics, BASE_OFFSET>(),
            DeferUpdates: DeferUpdates::<Impl, IMPL_OFFSET>,
            CompleteUpdatesAsync: CompleteUpdatesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICachedFileManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDownloadsFolderStaticsImpl: Sized {
    fn CreateFileAsync(&mut self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsync(&mut self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFileWithCollisionOptionAsync(&mut self, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderWithCollisionOptionAsync(&mut self, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDownloadsFolderStatics {
    const NAME: &'static str = "Windows.Storage.IDownloadsFolderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDownloadsFolderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadsFolderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadsFolderStaticsVtbl {
        unsafe extern "system" fn CreateFileAsync<Impl: IDownloadsFolderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsync<Impl: IDownloadsFolderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileWithCollisionOptionAsync<Impl: IDownloadsFolderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileWithCollisionOptionAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderWithCollisionOptionAsync<Impl: IDownloadsFolderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderWithCollisionOptionAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadsFolderStatics, BASE_OFFSET>(),
            CreateFileAsync: CreateFileAsync::<Impl, IMPL_OFFSET>,
            CreateFolderAsync: CreateFolderAsync::<Impl, IMPL_OFFSET>,
            CreateFileWithCollisionOptionAsync: CreateFileWithCollisionOptionAsync::<Impl, IMPL_OFFSET>,
            CreateFolderWithCollisionOptionAsync: CreateFolderWithCollisionOptionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadsFolderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IDownloadsFolderStatics2Impl: Sized {
    fn CreateFileForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFileForUserWithCollisionOptionAsync(&mut self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderForUserWithCollisionOptionAsync(&mut self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDownloadsFolderStatics2 {
    const NAME: &'static str = "Windows.Storage.IDownloadsFolderStatics2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IDownloadsFolderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadsFolderStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDownloadsFolderStatics2Vtbl {
        unsafe extern "system" fn CreateFileForUserAsync<Impl: IDownloadsFolderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderForUserAsync<Impl: IDownloadsFolderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileForUserWithCollisionOptionAsync<Impl: IDownloadsFolderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileForUserWithCollisionOptionAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderForUserWithCollisionOptionAsync<Impl: IDownloadsFolderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderForUserWithCollisionOptionAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDownloadsFolderStatics2, BASE_OFFSET>(),
            CreateFileForUserAsync: CreateFileForUserAsync::<Impl, IMPL_OFFSET>,
            CreateFolderForUserAsync: CreateFolderForUserAsync::<Impl, IMPL_OFFSET>,
            CreateFileForUserWithCollisionOptionAsync: CreateFileForUserWithCollisionOptionAsync::<Impl, IMPL_OFFSET>,
            CreateFolderForUserWithCollisionOptionAsync: CreateFolderForUserWithCollisionOptionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDownloadsFolderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IFileIOStaticsImpl: Sized {
    fn ReadTextAsync(&mut self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReadTextWithEncodingAsync(&mut self, file: &::core::option::Option<IStorageFile>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn WriteTextAsync(&mut self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteTextWithEncodingAsync(&mut self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextAsync(&mut self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextWithEncodingAsync(&mut self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadLinesAsync(&mut self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn ReadLinesWithEncodingAsync(&mut self, file: &::core::option::Option<IStorageFile>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn WriteLinesAsync(&mut self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteLinesWithEncodingAsync(&mut self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesAsync(&mut self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesWithEncodingAsync(&mut self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadBufferAsync(&mut self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>>;
    fn WriteBufferAsync(&mut self, file: &::core::option::Option<IStorageFile>, buffer: &::core::option::Option<Streams::IBuffer>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteBytesAsync(&mut self, file: &::core::option::Option<IStorageFile>, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFileIOStatics {
    const NAME: &'static str = "Windows.Storage.IFileIOStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IFileIOStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileIOStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileIOStaticsVtbl {
        unsafe extern "system" fn ReadTextAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTextAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTextWithEncodingAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTextWithEncodingAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteTextAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteTextAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteTextWithEncodingAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteTextWithEncodingAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTextAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendTextAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTextWithEncodingAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendTextWithEncodingAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadLinesAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadLinesAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadLinesWithEncodingAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadLinesWithEncodingAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteLinesAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteLinesAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteLinesWithEncodingAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteLinesWithEncodingAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendLinesAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendLinesAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendLinesWithEncodingAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendLinesWithEncodingAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBufferAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBufferAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteBufferAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteBufferAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&buffer as *const <Streams::IBuffer as ::windows::core::Abi>::Abi as *const <Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteBytesAsync<Impl: IFileIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteBytesAsync(&*(&file as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileIOStatics, BASE_OFFSET>(),
            ReadTextAsync: ReadTextAsync::<Impl, IMPL_OFFSET>,
            ReadTextWithEncodingAsync: ReadTextWithEncodingAsync::<Impl, IMPL_OFFSET>,
            WriteTextAsync: WriteTextAsync::<Impl, IMPL_OFFSET>,
            WriteTextWithEncodingAsync: WriteTextWithEncodingAsync::<Impl, IMPL_OFFSET>,
            AppendTextAsync: AppendTextAsync::<Impl, IMPL_OFFSET>,
            AppendTextWithEncodingAsync: AppendTextWithEncodingAsync::<Impl, IMPL_OFFSET>,
            ReadLinesAsync: ReadLinesAsync::<Impl, IMPL_OFFSET>,
            ReadLinesWithEncodingAsync: ReadLinesWithEncodingAsync::<Impl, IMPL_OFFSET>,
            WriteLinesAsync: WriteLinesAsync::<Impl, IMPL_OFFSET>,
            WriteLinesWithEncodingAsync: WriteLinesWithEncodingAsync::<Impl, IMPL_OFFSET>,
            AppendLinesAsync: AppendLinesAsync::<Impl, IMPL_OFFSET>,
            AppendLinesWithEncodingAsync: AppendLinesWithEncodingAsync::<Impl, IMPL_OFFSET>,
            ReadBufferAsync: ReadBufferAsync::<Impl, IMPL_OFFSET>,
            WriteBufferAsync: WriteBufferAsync::<Impl, IMPL_OFFSET>,
            WriteBytesAsync: WriteBytesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileIOStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersCameraRollStaticsImpl: Sized {
    fn CameraRoll(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersCameraRollStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersCameraRollStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersCameraRollStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersCameraRollStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersCameraRollStaticsVtbl {
        unsafe extern "system" fn CameraRoll<Impl: IKnownFoldersCameraRollStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraRoll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersCameraRollStatics, BASE_OFFSET>(),
            CameraRoll: CameraRoll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersCameraRollStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersPlaylistsStaticsImpl: Sized {
    fn Playlists(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersPlaylistsStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersPlaylistsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersPlaylistsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersPlaylistsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersPlaylistsStaticsVtbl {
        unsafe extern "system" fn Playlists<Impl: IKnownFoldersPlaylistsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Playlists() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersPlaylistsStatics, BASE_OFFSET>(),
            Playlists: Playlists::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersPlaylistsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersSavedPicturesStaticsImpl: Sized {
    fn SavedPictures(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersSavedPicturesStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersSavedPicturesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersSavedPicturesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersSavedPicturesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersSavedPicturesStaticsVtbl {
        unsafe extern "system" fn SavedPictures<Impl: IKnownFoldersSavedPicturesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SavedPictures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersSavedPicturesStatics, BASE_OFFSET>(),
            SavedPictures: SavedPictures::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersSavedPicturesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStaticsImpl: Sized {
    fn MusicLibrary(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn PicturesLibrary(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn VideosLibrary(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn DocumentsLibrary(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn HomeGroup(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn RemovableDevices(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn MediaServerDevices(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersStaticsVtbl {
        unsafe extern "system" fn MusicLibrary<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MusicLibrary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PicturesLibrary<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PicturesLibrary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideosLibrary<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideosLibrary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentsLibrary<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentsLibrary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeGroup<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovableDevices<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovableDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaServerDevices<Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaServerDevices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersStatics, BASE_OFFSET>(),
            MusicLibrary: MusicLibrary::<Impl, IMPL_OFFSET>,
            PicturesLibrary: PicturesLibrary::<Impl, IMPL_OFFSET>,
            VideosLibrary: VideosLibrary::<Impl, IMPL_OFFSET>,
            DocumentsLibrary: DocumentsLibrary::<Impl, IMPL_OFFSET>,
            HomeGroup: HomeGroup::<Impl, IMPL_OFFSET>,
            RemovableDevices: RemovableDevices::<Impl, IMPL_OFFSET>,
            MediaServerDevices: MediaServerDevices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics2Impl: Sized {
    fn Objects3D(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn AppCaptures(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn RecordedCalls(&mut self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersStatics2 {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersStatics2Vtbl {
        unsafe extern "system" fn Objects3D<Impl: IKnownFoldersStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Objects3D() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppCaptures<Impl: IKnownFoldersStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppCaptures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordedCalls<Impl: IKnownFoldersStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordedCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersStatics2, BASE_OFFSET>(),
            Objects3D: Objects3D::<Impl, IMPL_OFFSET>,
            AppCaptures: AppCaptures::<Impl, IMPL_OFFSET>,
            RecordedCalls: RecordedCalls::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IKnownFoldersStatics3Impl: Sized {
    fn GetFolderForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownFoldersStatics3 {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics3";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IKnownFoldersStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersStatics3Vtbl {
        unsafe extern "system" fn GetFolderForUserAsync<Impl: IKnownFoldersStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), folderid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersStatics3, BASE_OFFSET>(),
            GetFolderForUserAsync: GetFolderForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IKnownFoldersStatics4Impl: Sized {
    fn RequestAccessAsync(&mut self, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>;
    fn RequestAccessForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>;
    fn GetFolderAsync(&mut self, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownFoldersStatics4 {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics4";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IKnownFoldersStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownFoldersStatics4Vtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IKnownFoldersStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync(folderid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessForUserAsync<Impl: IKnownFoldersStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), folderid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Impl: IKnownFoldersStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderid: KnownFolderId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderAsync(folderid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownFoldersStatics4, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            RequestAccessForUserAsync: RequestAccessForUserAsync::<Impl, IMPL_OFFSET>,
            GetFolderAsync: GetFolderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownFoldersStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPathIOStaticsImpl: Sized {
    fn ReadTextAsync(&mut self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReadTextWithEncodingAsync(&mut self, absolutepath: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn WriteTextAsync(&mut self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteTextWithEncodingAsync(&mut self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextAsync(&mut self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextWithEncodingAsync(&mut self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadLinesAsync(&mut self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn ReadLinesWithEncodingAsync(&mut self, absolutepath: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn WriteLinesAsync(&mut self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteLinesWithEncodingAsync(&mut self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesAsync(&mut self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesWithEncodingAsync(&mut self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadBufferAsync(&mut self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>>;
    fn WriteBufferAsync(&mut self, absolutepath: &::windows::core::HSTRING, buffer: &::core::option::Option<Streams::IBuffer>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteBytesAsync(&mut self, absolutepath: &::windows::core::HSTRING, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPathIOStatics {
    const NAME: &'static str = "Windows.Storage.IPathIOStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPathIOStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathIOStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPathIOStaticsVtbl {
        unsafe extern "system" fn ReadTextAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTextAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTextWithEncodingAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadTextWithEncodingAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteTextAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteTextAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteTextWithEncodingAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteTextWithEncodingAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTextAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendTextAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTextWithEncodingAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contents: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendTextWithEncodingAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&contents as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadLinesAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadLinesAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadLinesWithEncodingAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadLinesWithEncodingAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteLinesAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteLinesAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteLinesWithEncodingAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteLinesWithEncodingAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendLinesAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendLinesAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendLinesWithEncodingAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, lines: ::windows::core::RawPtr, encoding: Streams::UnicodeEncoding, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendLinesWithEncodingAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&lines as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), encoding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBufferAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBufferAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteBufferAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteBufferAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&buffer as *const <Streams::IBuffer as ::windows::core::Abi>::Abi as *const <Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteBytesAsync<Impl: IPathIOStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, absolutepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteBytesAsync(&*(&absolutepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPathIOStatics, BASE_OFFSET>(),
            ReadTextAsync: ReadTextAsync::<Impl, IMPL_OFFSET>,
            ReadTextWithEncodingAsync: ReadTextWithEncodingAsync::<Impl, IMPL_OFFSET>,
            WriteTextAsync: WriteTextAsync::<Impl, IMPL_OFFSET>,
            WriteTextWithEncodingAsync: WriteTextWithEncodingAsync::<Impl, IMPL_OFFSET>,
            AppendTextAsync: AppendTextAsync::<Impl, IMPL_OFFSET>,
            AppendTextWithEncodingAsync: AppendTextWithEncodingAsync::<Impl, IMPL_OFFSET>,
            ReadLinesAsync: ReadLinesAsync::<Impl, IMPL_OFFSET>,
            ReadLinesWithEncodingAsync: ReadLinesWithEncodingAsync::<Impl, IMPL_OFFSET>,
            WriteLinesAsync: WriteLinesAsync::<Impl, IMPL_OFFSET>,
            WriteLinesWithEncodingAsync: WriteLinesWithEncodingAsync::<Impl, IMPL_OFFSET>,
            AppendLinesAsync: AppendLinesAsync::<Impl, IMPL_OFFSET>,
            AppendLinesWithEncodingAsync: AppendLinesWithEncodingAsync::<Impl, IMPL_OFFSET>,
            ReadBufferAsync: ReadBufferAsync::<Impl, IMPL_OFFSET>,
            WriteBufferAsync: WriteBufferAsync::<Impl, IMPL_OFFSET>,
            WriteBytesAsync: WriteBytesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPathIOStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetVersionDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetVersionDeferral {
    const NAME: &'static str = "Windows.Storage.ISetVersionDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ISetVersionDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetVersionDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetVersionDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ISetVersionDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISetVersionDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetVersionDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetVersionRequestImpl: Sized {
    fn CurrentVersion(&mut self) -> ::windows::core::Result<u32>;
    fn DesiredVersion(&mut self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<SetVersionDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetVersionRequest {
    const NAME: &'static str = "Windows.Storage.ISetVersionRequest";
}
#[cfg(feature = "implement_exclusive")]
impl ISetVersionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetVersionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetVersionRequestVtbl {
        unsafe extern "system" fn CurrentVersion<Impl: ISetVersionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredVersion<Impl: ISetVersionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: ISetVersionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISetVersionRequest, BASE_OFFSET>(),
            CurrentVersion: CurrentVersion::<Impl, IMPL_OFFSET>,
            DesiredVersion: DesiredVersion::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetVersionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IStorageFileImpl: Sized + IInputStreamReferenceImpl + IRandomAccessStreamReferenceImpl + IStorageItemImpl {
    fn FileType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OpenAsync(&mut self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
    fn CopyOverloadDefaultNameAndOptions(&mut self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverloadDefaultOptions(&mut self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverload(&mut self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyAndReplaceAsync(&mut self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultNameAndOptions(&mut self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultOptions(&mut self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverload(&mut self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveAndReplaceAsync(&mut self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageFile {
    const NAME: &'static str = "Windows.Storage.IStorageFile";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IStorageFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFileVtbl {
        unsafe extern "system" fn FileType<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentType<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenAsync<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync(accessmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteAsync<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverloadDefaultNameAndOptions<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyOverloadDefaultNameAndOptions(&*(&destinationfolder as *const <IStorageFolder as ::windows::core::Abi>::Abi as *const <IStorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverloadDefaultOptions<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyOverloadDefaultOptions(&*(&destinationfolder as *const <IStorageFolder as ::windows::core::Abi>::Abi as *const <IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&desirednewname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyOverload<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyOverload(&*(&destinationfolder as *const <IStorageFolder as ::windows::core::Abi>::Abi as *const <IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&desirednewname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyAndReplaceAsync<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyAndReplaceAsync(&*(&filetoreplace as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverloadDefaultNameAndOptions<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveOverloadDefaultNameAndOptions(&*(&destinationfolder as *const <IStorageFolder as ::windows::core::Abi>::Abi as *const <IStorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverloadDefaultOptions<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveOverloadDefaultOptions(&*(&destinationfolder as *const <IStorageFolder as ::windows::core::Abi>::Abi as *const <IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&desirednewname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveOverload<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationfolder: ::windows::core::RawPtr, desirednewname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveOverload(&*(&destinationfolder as *const <IStorageFolder as ::windows::core::Abi>::Abi as *const <IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&desirednewname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveAndReplaceAsync<Impl: IStorageFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveAndReplaceAsync(&*(&filetoreplace as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFile, BASE_OFFSET>(),
            FileType: FileType::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            OpenTransactedWriteAsync: OpenTransactedWriteAsync::<Impl, IMPL_OFFSET>,
            CopyOverloadDefaultNameAndOptions: CopyOverloadDefaultNameAndOptions::<Impl, IMPL_OFFSET>,
            CopyOverloadDefaultOptions: CopyOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            CopyOverload: CopyOverload::<Impl, IMPL_OFFSET>,
            CopyAndReplaceAsync: CopyAndReplaceAsync::<Impl, IMPL_OFFSET>,
            MoveOverloadDefaultNameAndOptions: MoveOverloadDefaultNameAndOptions::<Impl, IMPL_OFFSET>,
            MoveOverloadDefaultOptions: MoveOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            MoveOverload: MoveOverload::<Impl, IMPL_OFFSET>,
            MoveAndReplaceAsync: MoveAndReplaceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IStorageFile2Impl: Sized {
    fn OpenWithOptionsAsync(&mut self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteWithOptionsAsync(&mut self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageFile2 {
    const NAME: &'static str = "Windows.Storage.IStorageFile2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IStorageFile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFile2Vtbl {
        unsafe extern "system" fn OpenWithOptionsAsync<Impl: IStorageFile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessmode: FileAccessMode, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenWithOptionsAsync(accessmode, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTransactedWriteWithOptionsAsync<Impl: IStorageFile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: StorageOpenOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTransactedWriteWithOptionsAsync(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFile2, BASE_OFFSET>(),
            OpenWithOptionsAsync: OpenWithOptionsAsync::<Impl, IMPL_OFFSET>,
            OpenTransactedWriteWithOptionsAsync: OpenTransactedWriteWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFile2 as ::windows::core::Interface>::IID
    }
}
pub trait IStorageFilePropertiesWithAvailabilityImpl: Sized {
    fn IsAvailable(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageFilePropertiesWithAvailability {
    const NAME: &'static str = "Windows.Storage.IStorageFilePropertiesWithAvailability";
}
impl IStorageFilePropertiesWithAvailabilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFilePropertiesWithAvailabilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFilePropertiesWithAvailabilityVtbl {
        unsafe extern "system" fn IsAvailable<Impl: IStorageFilePropertiesWithAvailabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFilePropertiesWithAvailability, BASE_OFFSET>(),
            IsAvailable: IsAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFilePropertiesWithAvailability as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStorageFileStaticsImpl: Sized {
    fn GetFileFromPathAsync(&mut self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFileFromApplicationUriAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateStreamedFileAsync(&mut self, displaynamewithextension: &::windows::core::HSTRING, datarequested: &::core::option::Option<StreamedFileDataRequestedHandler>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn ReplaceWithStreamedFileAsync(&mut self, filetoreplace: &::core::option::Option<IStorageFile>, datarequested: &::core::option::Option<StreamedFileDataRequestedHandler>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateStreamedFileFromUriAsync(&mut self, displaynamewithextension: &::windows::core::HSTRING, uri: &::core::option::Option<super::Foundation::Uri>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn ReplaceWithStreamedFileFromUriAsync(&mut self, filetoreplace: &::core::option::Option<IStorageFile>, uri: &::core::option::Option<super::Foundation::Uri>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFileStatics {
    const NAME: &'static str = "Windows.Storage.IStorageFileStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStorageFileStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFileStaticsVtbl {
        unsafe extern "system" fn GetFileFromPathAsync<Impl: IStorageFileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileFromPathAsync(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileFromApplicationUriAsync<Impl: IStorageFileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileFromApplicationUriAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamedFileAsync<Impl: IStorageFileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaynamewithextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datarequested: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStreamedFileAsync(
                &*(&displaynamewithextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&datarequested as *const <StreamedFileDataRequestedHandler as ::windows::core::Abi>::Abi as *const <StreamedFileDataRequestedHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&thumbnail as *const <Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceWithStreamedFileAsync<Impl: IStorageFileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, datarequested: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceWithStreamedFileAsync(
                &*(&filetoreplace as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&datarequested as *const <StreamedFileDataRequestedHandler as ::windows::core::Abi>::Abi as *const <StreamedFileDataRequestedHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&thumbnail as *const <Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStreamedFileFromUriAsync<Impl: IStorageFileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaynamewithextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, uri: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStreamedFileFromUriAsync(
                &*(&displaynamewithextension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&thumbnail as *const <Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceWithStreamedFileFromUriAsync<Impl: IStorageFileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filetoreplace: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, thumbnail: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReplaceWithStreamedFileFromUriAsync(
                &*(&filetoreplace as *const <IStorageFile as ::windows::core::Abi>::Abi as *const <IStorageFile as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&thumbnail as *const <Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFileStatics, BASE_OFFSET>(),
            GetFileFromPathAsync: GetFileFromPathAsync::<Impl, IMPL_OFFSET>,
            GetFileFromApplicationUriAsync: GetFileFromApplicationUriAsync::<Impl, IMPL_OFFSET>,
            CreateStreamedFileAsync: CreateStreamedFileAsync::<Impl, IMPL_OFFSET>,
            ReplaceWithStreamedFileAsync: ReplaceWithStreamedFileAsync::<Impl, IMPL_OFFSET>,
            CreateStreamedFileFromUriAsync: CreateStreamedFileFromUriAsync::<Impl, IMPL_OFFSET>,
            ReplaceWithStreamedFileFromUriAsync: ReplaceWithStreamedFileFromUriAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFileStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IStorageFileStatics2Impl: Sized {
    fn GetFileFromPathForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFileStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageFileStatics2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IStorageFileStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFileStatics2Vtbl {
        unsafe extern "system" fn GetFileFromPathForUserAsync<Impl: IStorageFileStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileFromPathForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFileStatics2, BASE_OFFSET>(),
            GetFileFromPathForUserAsync: GetFileFromPathForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFileStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageFolderImpl: Sized + IStorageItemImpl {
    fn CreateFileAsyncOverloadDefaultOptions(&mut self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFileAsync(&mut self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsyncOverloadDefaultOptions(&mut self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFolderAsync(&mut self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetFileAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFolderAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetItemAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>;
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>;
    fn GetItemsAsyncOverloadDefaultStartAndCount(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageFolder {
    const NAME: &'static str = "Windows.Storage.IStorageFolder";
}
#[cfg(feature = "Foundation")]
impl IStorageFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderVtbl {
        unsafe extern "system" fn CreateFileAsyncOverloadDefaultOptions<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileAsyncOverloadDefaultOptions(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileAsync<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsyncOverloadDefaultOptions<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderAsyncOverloadDefaultOptions(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFolderAsync<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: CreationCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileAsync<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolderAsync<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemAsync<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilesAsyncOverloadDefaultOptionsStartAndCount<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFilesAsyncOverloadDefaultOptionsStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsAsyncOverloadDefaultStartAndCount<Impl: IStorageFolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemsAsyncOverloadDefaultStartAndCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolder, BASE_OFFSET>(),
            CreateFileAsyncOverloadDefaultOptions: CreateFileAsyncOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            CreateFileAsync: CreateFileAsync::<Impl, IMPL_OFFSET>,
            CreateFolderAsyncOverloadDefaultOptions: CreateFolderAsyncOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            CreateFolderAsync: CreateFolderAsync::<Impl, IMPL_OFFSET>,
            GetFileAsync: GetFileAsync::<Impl, IMPL_OFFSET>,
            GetFolderAsync: GetFolderAsync::<Impl, IMPL_OFFSET>,
            GetItemAsync: GetItemAsync::<Impl, IMPL_OFFSET>,
            GetFilesAsyncOverloadDefaultOptionsStartAndCount: GetFilesAsyncOverloadDefaultOptionsStartAndCount::<Impl, IMPL_OFFSET>,
            GetFoldersAsyncOverloadDefaultOptionsStartAndCount: GetFoldersAsyncOverloadDefaultOptionsStartAndCount::<Impl, IMPL_OFFSET>,
            GetItemsAsyncOverloadDefaultStartAndCount: GetItemsAsyncOverloadDefaultStartAndCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageFolder2Impl: Sized {
    fn TryGetItemAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageFolder2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder2";
}
#[cfg(feature = "Foundation")]
impl IStorageFolder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolder2Vtbl {
        unsafe extern "system" fn TryGetItemAsync<Impl: IStorageFolder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetItemAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolder2, BASE_OFFSET>(), TryGetItemAsync: TryGetItemAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolder3Impl: Sized {
    fn TryGetChangeTracker(&mut self) -> ::windows::core::Result<StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFolder3 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder3";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFolder3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolder3Vtbl {
        unsafe extern "system" fn TryGetChangeTracker<Impl: IStorageFolder3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetChangeTracker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolder3, BASE_OFFSET>(),
            TryGetChangeTracker: TryGetChangeTracker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolder3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageFolderStaticsImpl: Sized {
    fn GetFolderFromPathAsync(&mut self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFolderStatics {
    const NAME: &'static str = "Windows.Storage.IStorageFolderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageFolderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderStaticsVtbl {
        unsafe extern "system" fn GetFolderFromPathAsync<Impl: IStorageFolderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderFromPathAsync(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolderStatics, BASE_OFFSET>(),
            GetFolderFromPathAsync: GetFolderFromPathAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IStorageFolderStatics2Impl: Sized {
    fn GetFolderFromPathForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageFolderStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolderStatics2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IStorageFolderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageFolderStatics2Vtbl {
        unsafe extern "system" fn GetFolderFromPathForUserAsync<Impl: IStorageFolderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFolderFromPathForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageFolderStatics2, BASE_OFFSET>(),
            GetFolderFromPathForUserAsync: GetFolderFromPathForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageFolderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
pub trait IStorageItemImpl: Sized {
    fn RenameAsyncOverloadDefaultOptions(&mut self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn RenameAsync(&mut self, desiredname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsyncOverloadDefaultOptions(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsync(&mut self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn GetBasicPropertiesAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Attributes(&mut self) -> ::windows::core::Result<FileAttributes>;
    fn DateCreated(&mut self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn IsOfType(&mut self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl ::windows::core::RuntimeName for IStorageItem {
    const NAME: &'static str = "Windows.Storage.IStorageItem";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
impl IStorageItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemVtbl {
        unsafe extern "system" fn RenameAsyncOverloadDefaultOptions<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameAsyncOverloadDefaultOptions(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenameAsync<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: NameCollisionOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenameAsync(&*(&desiredname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsyncOverloadDefaultOptions<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsyncOverloadDefaultOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: StorageDeleteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync(option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBasicPropertiesAsync<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBasicPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Path<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FileAttributes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateCreated<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateCreated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfType<Impl: IStorageItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOfType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItem, BASE_OFFSET>(),
            RenameAsyncOverloadDefaultOptions: RenameAsyncOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            RenameAsync: RenameAsync::<Impl, IMPL_OFFSET>,
            DeleteAsyncOverloadDefaultOptions: DeleteAsyncOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            GetBasicPropertiesAsync: GetBasicPropertiesAsync::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
            DateCreated: DateCreated::<Impl, IMPL_OFFSET>,
            IsOfType: IsOfType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IStorageItem2Impl: Sized + IStorageItemImpl {
    fn GetParentAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn IsEqual(&mut self, item: &::core::option::Option<IStorageItem>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IStorageItem2 {
    const NAME: &'static str = "Windows.Storage.IStorageItem2";
}
#[cfg(feature = "Foundation")]
impl IStorageItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItem2Vtbl {
        unsafe extern "system" fn GetParentAsync<Impl: IStorageItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IStorageItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&item as *const <IStorageItem as ::windows::core::Abi>::Abi as *const <IStorageItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItem2, BASE_OFFSET>(),
            GetParentAsync: GetParentAsync::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemPropertiesImpl: Sized {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&mut self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsyncOverloadDefaultOptions(&mut self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsync(&mut self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FolderRelativeId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemProperties {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemPropertiesVtbl {
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsyncOverloadDefaultOptions<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsyncOverloadDefaultOptions(mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailAsync<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsync(mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayType<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FolderRelativeId<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderRelativeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IStorageItemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemProperties, BASE_OFFSET>(),
            GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Impl, IMPL_OFFSET>,
            GetThumbnailAsyncOverloadDefaultOptions: GetThumbnailAsyncOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            GetThumbnailAsync: GetThumbnailAsync::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            DisplayType: DisplayType::<Impl, IMPL_OFFSET>,
            FolderRelativeId: FolderRelativeId::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemProperties2Impl: Sized + IStorageItemPropertiesImpl {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&mut self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&mut self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsync(&mut self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemProperties2 {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemProperties2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemProperties2Vtbl {
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions<Impl: IStorageItemProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions<Impl: IStorageItemProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(mode, requestedsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScaledImageAsThumbnailAsync<Impl: IStorageItemProperties2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScaledImageAsThumbnailAsync(mode, requestedsize, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemProperties2, BASE_OFFSET>(),
            GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Impl, IMPL_OFFSET>,
            GetScaledImageAsThumbnailAsyncOverloadDefaultOptions: GetScaledImageAsThumbnailAsyncOverloadDefaultOptions::<Impl, IMPL_OFFSET>,
            GetScaledImageAsThumbnailAsync: GetScaledImageAsThumbnailAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
pub trait IStorageItemPropertiesWithProviderImpl: Sized + IStorageItemPropertiesImpl {
    fn Provider(&mut self) -> ::windows::core::Result<StorageProvider>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IStorageItemPropertiesWithProvider {
    const NAME: &'static str = "Windows.Storage.IStorageItemPropertiesWithProvider";
}
#[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl IStorageItemPropertiesWithProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemPropertiesWithProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageItemPropertiesWithProviderVtbl {
        unsafe extern "system" fn Provider<Impl: IStorageItemPropertiesWithProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Provider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageItemPropertiesWithProvider, BASE_OFFSET>(),
            Provider: Provider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageItemPropertiesWithProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageLibraryImpl: Sized {
    fn RequestAddFolderAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn RequestRemoveFolderAsync(&mut self, folder: &::core::option::Option<StorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn Folders(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IObservableVector<StorageFolder>>;
    fn SaveFolder(&mut self) -> ::windows::core::Result<StorageFolder>;
    fn DefinitionChanged(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveDefinitionChanged(&mut self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibrary {
    const NAME: &'static str = "Windows.Storage.IStorageLibrary";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryVtbl {
        unsafe extern "system" fn RequestAddFolderAsync<Impl: IStorageLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddFolderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRemoveFolderAsync<Impl: IStorageLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRemoveFolderAsync(&*(&folder as *const <StorageFolder as ::windows::core::Abi>::Abi as *const <StorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Folders<Impl: IStorageLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Folders() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFolder<Impl: IStorageLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveFolder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefinitionChanged<Impl: IStorageLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefinitionChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefinitionChanged<Impl: IStorageLibraryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDefinitionChanged(&*(&eventcookie as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibrary, BASE_OFFSET>(),
            RequestAddFolderAsync: RequestAddFolderAsync::<Impl, IMPL_OFFSET>,
            RequestRemoveFolderAsync: RequestRemoveFolderAsync::<Impl, IMPL_OFFSET>,
            Folders: Folders::<Impl, IMPL_OFFSET>,
            SaveFolder: SaveFolder::<Impl, IMPL_OFFSET>,
            DefinitionChanged: DefinitionChanged::<Impl, IMPL_OFFSET>,
            RemoveDefinitionChanged: RemoveDefinitionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibrary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibrary2Impl: Sized {
    fn ChangeTracker(&mut self) -> ::windows::core::Result<StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibrary2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibrary2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibrary2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibrary2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibrary2Vtbl {
        unsafe extern "system" fn ChangeTracker<Impl: IStorageLibrary2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeTracker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibrary2, BASE_OFFSET>(), ChangeTracker: ChangeTracker::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibrary2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageLibrary3Impl: Sized {
    fn AreFolderSuggestionsAvailableAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibrary3 {
    const NAME: &'static str = "Windows.Storage.IStorageLibrary3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageLibrary3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibrary3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibrary3Vtbl {
        unsafe extern "system" fn AreFolderSuggestionsAvailableAsync<Impl: IStorageLibrary3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreFolderSuggestionsAvailableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibrary3, BASE_OFFSET>(),
            AreFolderSuggestionsAvailableAsync: AreFolderSuggestionsAvailableAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibrary3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageLibraryChangeImpl: Sized {
    fn ChangeType(&mut self) -> ::windows::core::Result<StorageLibraryChangeType>;
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PreviousPath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOfType(&mut self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
    fn GetStorageItemAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryChange {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChange";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageLibraryChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeVtbl {
        unsafe extern "system" fn ChangeType<Impl: IStorageLibraryChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StorageLibraryChangeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IStorageLibraryChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousPath<Impl: IStorageLibraryChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOfType<Impl: IStorageLibraryChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: StorageItemTypes, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOfType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageItemAsync<Impl: IStorageLibraryChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageItemAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChange, BASE_OFFSET>(),
            ChangeType: ChangeType::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            PreviousPath: PreviousPath::<Impl, IMPL_OFFSET>,
            IsOfType: IsOfType::<Impl, IMPL_OFFSET>,
            GetStorageItemAsync: GetStorageItemAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStorageLibraryChangeReaderImpl: Sized {
    fn ReadBatchAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>>;
    fn AcceptChangesAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryChangeReader {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeReader";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStorageLibraryChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IStorageLibraryChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptChangesAsync<Impl: IStorageLibraryChangeReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptChangesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeReader, BASE_OFFSET>(),
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
            AcceptChangesAsync: AcceptChangesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeReader2Impl: Sized {
    fn GetLastChangeId(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeReader2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeReader2Vtbl {
        unsafe extern "system" fn GetLastChangeId<Impl: IStorageLibraryChangeReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastChangeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeReader2, BASE_OFFSET>(),
            GetLastChangeId: GetLastChangeId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerImpl: Sized {
    fn GetChangeReader(&mut self) -> ::windows::core::Result<StorageLibraryChangeReader>;
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTracker {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeTrackerVtbl {
        unsafe extern "system" fn GetChangeReader<Impl: IStorageLibraryChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChangeReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: IStorageLibraryChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Reset<Impl: IStorageLibraryChangeTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeTracker, BASE_OFFSET>(),
            GetChangeReader: GetChangeReader::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeTracker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTracker2Impl: Sized {
    fn EnableWithOptions(&mut self, options: &::core::option::Option<StorageLibraryChangeTrackerOptions>) -> ::windows::core::Result<()>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTracker2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeTracker2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTracker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTracker2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeTracker2Vtbl {
        unsafe extern "system" fn EnableWithOptions<Impl: IStorageLibraryChangeTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableWithOptions(&*(&options as *const <StorageLibraryChangeTrackerOptions as ::windows::core::Abi>::Abi as *const <StorageLibraryChangeTrackerOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disable<Impl: IStorageLibraryChangeTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeTracker2, BASE_OFFSET>(),
            EnableWithOptions: EnableWithOptions::<Impl, IMPL_OFFSET>,
            Disable: Disable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeTracker2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerOptionsImpl: Sized {
    fn TrackChangeDetails(&mut self) -> ::windows::core::Result<bool>;
    fn SetTrackChangeDetails(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTrackerOptions {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeTrackerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryChangeTrackerOptionsVtbl {
        unsafe extern "system" fn TrackChangeDetails<Impl: IStorageLibraryChangeTrackerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackChangeDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrackChangeDetails<Impl: IStorageLibraryChangeTrackerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrackChangeDetails(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryChangeTrackerOptions, BASE_OFFSET>(),
            TrackChangeDetails: TrackChangeDetails::<Impl, IMPL_OFFSET>,
            SetTrackChangeDetails: SetTrackChangeDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryChangeTrackerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryLastChangeIdImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryLastChangeId {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryLastChangeId";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryLastChangeIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryLastChangeIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryLastChangeIdVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryLastChangeId, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryLastChangeId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryLastChangeIdStaticsImpl: Sized {
    fn Unknown(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryLastChangeIdStatics {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryLastChangeIdStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryLastChangeIdStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryLastChangeIdStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryLastChangeIdStaticsVtbl {
        unsafe extern "system" fn Unknown<Impl: IStorageLibraryLastChangeIdStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unknown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryLastChangeIdStatics, BASE_OFFSET>(),
            Unknown: Unknown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryLastChangeIdStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageLibraryStaticsImpl: Sized {
    fn GetLibraryAsync(&mut self, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryStatics {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageLibraryStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryStaticsVtbl {
        unsafe extern "system" fn GetLibraryAsync<Impl: IStorageLibraryStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libraryid: KnownLibraryId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLibraryAsync(libraryid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryStatics, BASE_OFFSET>(),
            GetLibraryAsync: GetLibraryAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IStorageLibraryStatics2Impl: Sized {
    fn GetLibraryForUserAsync(&mut self, user: &::core::option::Option<super::System::User>, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageLibraryStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryStatics2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IStorageLibraryStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageLibraryStatics2Vtbl {
        unsafe extern "system" fn GetLibraryForUserAsync<Impl: IStorageLibraryStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, libraryid: KnownLibraryId, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLibraryForUserAsync(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), libraryid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageLibraryStatics2, BASE_OFFSET>(),
            GetLibraryForUserAsync: GetLibraryForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageLibraryStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProvider {
    const NAME: &'static str = "Windows.Storage.IStorageProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProviderVtbl {
        unsafe extern "system" fn Id<Impl: IStorageProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IStorageProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProvider, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageProvider2Impl: Sized + IStorageProviderImpl {
    fn IsPropertySupportedForPartialFileAsync(&mut self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageProvider2 {
    const NAME: &'static str = "Windows.Storage.IStorageProvider2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageProvider2Vtbl {
        unsafe extern "system" fn IsPropertySupportedForPartialFileAsync<Impl: IStorageProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertySupportedForPartialFileAsync(&*(&propertycanonicalname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageProvider2, BASE_OFFSET>(),
            IsPropertySupportedForPartialFileAsync: IsPropertySupportedForPartialFileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStorageStreamTransactionImpl: Sized + IClosableImpl {
    fn Stream(&mut self) -> ::windows::core::Result<Streams::IRandomAccessStream>;
    fn CommitAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.IStorageStreamTransaction";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStorageStreamTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageStreamTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStorageStreamTransactionVtbl {
        unsafe extern "system" fn Stream<Impl: IStorageStreamTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitAsync<Impl: IStorageStreamTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStorageStreamTransaction, BASE_OFFSET>(),
            Stream: Stream::<Impl, IMPL_OFFSET>,
            CommitAsync: CommitAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStorageStreamTransaction as ::windows::core::Interface>::IID
    }
}
pub trait IStreamedFileDataRequestImpl: Sized {
    fn FailAndClose(&mut self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStreamedFileDataRequest {
    const NAME: &'static str = "Windows.Storage.IStreamedFileDataRequest";
}
impl IStreamedFileDataRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamedFileDataRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamedFileDataRequestVtbl {
        unsafe extern "system" fn FailAndClose<Impl: IStreamedFileDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failuremode: StreamedFileFailureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FailAndClose(failuremode).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamedFileDataRequest, BASE_OFFSET>(),
            FailAndClose: FailAndClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamedFileDataRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemAudioPropertiesImpl: Sized {
    fn EncodingBitrate(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemAudioProperties {
    const NAME: &'static str = "Windows.Storage.ISystemAudioProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemAudioPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemAudioPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemAudioPropertiesVtbl {
        unsafe extern "system" fn EncodingBitrate<Impl: ISystemAudioPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemAudioProperties, BASE_OFFSET>(),
            EncodingBitrate: EncodingBitrate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemAudioProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDataPathsImpl: Sized {
    fn Fonts(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProgramData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Public(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDesktop(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDocuments(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDownloads(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicMusic(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicPictures(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicVideos(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn System(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemHost(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemX86(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemX64(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemArm(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserProfiles(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Windows(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDataPaths {
    const NAME: &'static str = "Windows.Storage.ISystemDataPaths";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDataPathsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDataPathsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDataPathsVtbl {
        unsafe extern "system" fn Fonts<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fonts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgramData<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgramData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Public<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Public() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicDesktop<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicDesktop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicDocuments<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicDocuments() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicDownloads<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicDownloads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicMusic<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicMusic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicPictures<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicPictures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicVideos<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicVideos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn System<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).System() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemHost<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemHost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemX86<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemX86() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemX64<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemX64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemArm<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemArm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserProfiles<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Windows<Impl: ISystemDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Windows() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemDataPaths, BASE_OFFSET>(),
            Fonts: Fonts::<Impl, IMPL_OFFSET>,
            ProgramData: ProgramData::<Impl, IMPL_OFFSET>,
            Public: Public::<Impl, IMPL_OFFSET>,
            PublicDesktop: PublicDesktop::<Impl, IMPL_OFFSET>,
            PublicDocuments: PublicDocuments::<Impl, IMPL_OFFSET>,
            PublicDownloads: PublicDownloads::<Impl, IMPL_OFFSET>,
            PublicMusic: PublicMusic::<Impl, IMPL_OFFSET>,
            PublicPictures: PublicPictures::<Impl, IMPL_OFFSET>,
            PublicVideos: PublicVideos::<Impl, IMPL_OFFSET>,
            System: System::<Impl, IMPL_OFFSET>,
            SystemHost: SystemHost::<Impl, IMPL_OFFSET>,
            SystemX86: SystemX86::<Impl, IMPL_OFFSET>,
            SystemX64: SystemX64::<Impl, IMPL_OFFSET>,
            SystemArm: SystemArm::<Impl, IMPL_OFFSET>,
            UserProfiles: UserProfiles::<Impl, IMPL_OFFSET>,
            Windows: Windows::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDataPaths as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDataPathsStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<SystemDataPaths>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDataPathsStatics {
    const NAME: &'static str = "Windows.Storage.ISystemDataPathsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDataPathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDataPathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemDataPathsStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ISystemDataPathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemDataPathsStatics, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemDataPathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemGPSPropertiesImpl: Sized {
    fn LatitudeDecimal(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LongitudeDecimal(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemGPSProperties {
    const NAME: &'static str = "Windows.Storage.ISystemGPSProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemGPSPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemGPSPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemGPSPropertiesVtbl {
        unsafe extern "system" fn LatitudeDecimal<Impl: ISystemGPSPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LatitudeDecimal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongitudeDecimal<Impl: ISystemGPSPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongitudeDecimal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemGPSProperties, BASE_OFFSET>(),
            LatitudeDecimal: LatitudeDecimal::<Impl, IMPL_OFFSET>,
            LongitudeDecimal: LongitudeDecimal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemGPSProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemImagePropertiesImpl: Sized {
    fn HorizontalSize(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VerticalSize(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemImageProperties {
    const NAME: &'static str = "Windows.Storage.ISystemImageProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemImagePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemImagePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemImagePropertiesVtbl {
        unsafe extern "system" fn HorizontalSize<Impl: ISystemImagePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalSize<Impl: ISystemImagePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemImageProperties, BASE_OFFSET>(),
            HorizontalSize: HorizontalSize::<Impl, IMPL_OFFSET>,
            VerticalSize: VerticalSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemImageProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaPropertiesImpl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Producer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Writer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Year(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaProperties {
    const NAME: &'static str = "Windows.Storage.ISystemMediaProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaPropertiesVtbl {
        unsafe extern "system" fn Duration<Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Producer<Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Producer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SubTitle<Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writer<Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Year<Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Year() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaProperties, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            Producer: Producer::<Impl, IMPL_OFFSET>,
            Publisher: Publisher::<Impl, IMPL_OFFSET>,
            SubTitle: SubTitle::<Impl, IMPL_OFFSET>,
            Writer: Writer::<Impl, IMPL_OFFSET>,
            Year: Year::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMusicPropertiesImpl: Sized {
    fn AlbumArtist(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlbumTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Artist(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Composer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Conductor(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayArtist(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Genre(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMusicProperties {
    const NAME: &'static str = "Windows.Storage.ISystemMusicProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMusicPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMusicPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMusicPropertiesVtbl {
        unsafe extern "system" fn AlbumArtist<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlbumArtist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlbumTitle<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlbumTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Artist<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Artist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Composer<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Composer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conductor<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Conductor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayArtist<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayArtist() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Genre<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Genre() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackNumber<Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMusicProperties, BASE_OFFSET>(),
            AlbumArtist: AlbumArtist::<Impl, IMPL_OFFSET>,
            AlbumTitle: AlbumTitle::<Impl, IMPL_OFFSET>,
            Artist: Artist::<Impl, IMPL_OFFSET>,
            Composer: Composer::<Impl, IMPL_OFFSET>,
            Conductor: Conductor::<Impl, IMPL_OFFSET>,
            DisplayArtist: DisplayArtist::<Impl, IMPL_OFFSET>,
            Genre: Genre::<Impl, IMPL_OFFSET>,
            TrackNumber: TrackNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMusicProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemPhotoPropertiesImpl: Sized {
    fn CameraManufacturer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CameraModel(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateTaken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PeopleNames(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemPhotoProperties {
    const NAME: &'static str = "Windows.Storage.ISystemPhotoProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemPhotoPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemPhotoPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemPhotoPropertiesVtbl {
        unsafe extern "system" fn CameraManufacturer<Impl: ISystemPhotoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraManufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraModel<Impl: ISystemPhotoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTaken<Impl: ISystemPhotoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTaken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: ISystemPhotoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeopleNames<Impl: ISystemPhotoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeopleNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemPhotoProperties, BASE_OFFSET>(),
            CameraManufacturer: CameraManufacturer::<Impl, IMPL_OFFSET>,
            CameraModel: CameraModel::<Impl, IMPL_OFFSET>,
            DateTaken: DateTaken::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            PeopleNames: PeopleNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemPhotoProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemPropertiesImpl: Sized {
    fn Author(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ItemNameDisplay(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Keywords(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rating(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Audio(&mut self) -> ::windows::core::Result<SystemAudioProperties>;
    fn GPS(&mut self) -> ::windows::core::Result<SystemGPSProperties>;
    fn Media(&mut self) -> ::windows::core::Result<SystemMediaProperties>;
    fn Music(&mut self) -> ::windows::core::Result<SystemMusicProperties>;
    fn Photo(&mut self) -> ::windows::core::Result<SystemPhotoProperties>;
    fn Video(&mut self) -> ::windows::core::Result<SystemVideoProperties>;
    fn Image(&mut self) -> ::windows::core::Result<SystemImageProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemProperties {
    const NAME: &'static str = "Windows.Storage.ISystemProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemPropertiesVtbl {
        unsafe extern "system" fn Author<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Comment<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemNameDisplay<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemNameDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keywords<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keywords() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rating<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GPS<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GPS() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Media<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Media() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Music<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Music() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Photo<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Photo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Video<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Video() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: ISystemPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemProperties, BASE_OFFSET>(),
            Author: Author::<Impl, IMPL_OFFSET>,
            Comment: Comment::<Impl, IMPL_OFFSET>,
            ItemNameDisplay: ItemNameDisplay::<Impl, IMPL_OFFSET>,
            Keywords: Keywords::<Impl, IMPL_OFFSET>,
            Rating: Rating::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Audio: Audio::<Impl, IMPL_OFFSET>,
            GPS: GPS::<Impl, IMPL_OFFSET>,
            Media: Media::<Impl, IMPL_OFFSET>,
            Music: Music::<Impl, IMPL_OFFSET>,
            Photo: Photo::<Impl, IMPL_OFFSET>,
            Video: Video::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemVideoPropertiesImpl: Sized {
    fn Director(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameHeight(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameWidth(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TotalBitrate(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemVideoProperties {
    const NAME: &'static str = "Windows.Storage.ISystemVideoProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemVideoPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemVideoPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemVideoPropertiesVtbl {
        unsafe extern "system" fn Director<Impl: ISystemVideoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Director() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameHeight<Impl: ISystemVideoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameWidth<Impl: ISystemVideoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orientation<Impl: ISystemVideoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBitrate<Impl: ISystemVideoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemVideoProperties, BASE_OFFSET>(),
            Director: Director::<Impl, IMPL_OFFSET>,
            FrameHeight: FrameHeight::<Impl, IMPL_OFFSET>,
            FrameWidth: FrameWidth::<Impl, IMPL_OFFSET>,
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            TotalBitrate: TotalBitrate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemVideoProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataPathsImpl: Sized {
    fn CameraRoll(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cookies(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Documents(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Downloads(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Favorites(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn History(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InternetCache(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppDataLow(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Music(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pictures(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Profile(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recent(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingAppData(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SavedPictures(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Screenshots(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Templates(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Videos(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataPaths {
    const NAME: &'static str = "Windows.Storage.IUserDataPaths";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataPathsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataPathsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataPathsVtbl {
        unsafe extern "system" fn CameraRoll<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraRoll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cookies<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cookies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Desktop<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Desktop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Documents<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Documents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Downloads<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Downloads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Favorites<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Favorites() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn History<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).History() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternetCache<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternetCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAppData<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAppData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAppDataLow<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAppDataLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Music<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Music() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pictures<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pictures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Profile<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Profile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recent<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoamingAppData<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoamingAppData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SavedPictures<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SavedPictures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Screenshots<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Screenshots() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Templates<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Templates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Videos<Impl: IUserDataPathsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Videos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataPaths, BASE_OFFSET>(),
            CameraRoll: CameraRoll::<Impl, IMPL_OFFSET>,
            Cookies: Cookies::<Impl, IMPL_OFFSET>,
            Desktop: Desktop::<Impl, IMPL_OFFSET>,
            Documents: Documents::<Impl, IMPL_OFFSET>,
            Downloads: Downloads::<Impl, IMPL_OFFSET>,
            Favorites: Favorites::<Impl, IMPL_OFFSET>,
            History: History::<Impl, IMPL_OFFSET>,
            InternetCache: InternetCache::<Impl, IMPL_OFFSET>,
            LocalAppData: LocalAppData::<Impl, IMPL_OFFSET>,
            LocalAppDataLow: LocalAppDataLow::<Impl, IMPL_OFFSET>,
            Music: Music::<Impl, IMPL_OFFSET>,
            Pictures: Pictures::<Impl, IMPL_OFFSET>,
            Profile: Profile::<Impl, IMPL_OFFSET>,
            Recent: Recent::<Impl, IMPL_OFFSET>,
            RoamingAppData: RoamingAppData::<Impl, IMPL_OFFSET>,
            SavedPictures: SavedPictures::<Impl, IMPL_OFFSET>,
            Screenshots: Screenshots::<Impl, IMPL_OFFSET>,
            Templates: Templates::<Impl, IMPL_OFFSET>,
            Videos: Videos::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataPaths as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IUserDataPathsStaticsImpl: Sized {
    fn GetForUser(&mut self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<UserDataPaths>;
    fn GetDefault(&mut self) -> ::windows::core::Result<UserDataPaths>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataPathsStatics {
    const NAME: &'static str = "Windows.Storage.IUserDataPathsStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IUserDataPathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataPathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataPathsStaticsVtbl {
        unsafe extern "system" fn GetForUser<Impl: IUserDataPathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Impl: IUserDataPathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataPathsStatics, BASE_OFFSET>(),
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataPathsStatics as ::windows::core::Interface>::IID
    }
}
