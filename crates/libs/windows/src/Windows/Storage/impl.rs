#[cfg(feature = "implement_exclusive")]
pub trait IAppDataPathsImpl: Sized {
    fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDataPaths {
    const NAME: &'static str = "Windows.Storage.IAppDataPaths";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDataPathsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDataPathsImpl, const OFFSET: isize>() -> IAppDataPathsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppDataPaths>,
            ::windows::core::GetTrustLevel,
            Cookies::<Impl, OFFSET>,
            Desktop::<Impl, OFFSET>,
            Documents::<Impl, OFFSET>,
            Favorites::<Impl, OFFSET>,
            History::<Impl, OFFSET>,
            InternetCache::<Impl, OFFSET>,
            LocalAppData::<Impl, OFFSET>,
            ProgramData::<Impl, OFFSET>,
            RoamingAppData::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDataPathsStaticsImpl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<AppDataPaths>;
    fn GetDefault(&self) -> ::windows::core::Result<AppDataPaths>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDataPathsStatics {
    const NAME: &'static str = "Windows.Storage.IAppDataPathsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDataPathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDataPathsStaticsImpl, const OFFSET: isize>() -> IAppDataPathsStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDataPathsStatics>, ::windows::core::GetTrustLevel, GetForUser::<Impl, OFFSET>, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataImpl: Sized {
    fn Version(&self) -> ::windows::core::Result<u32>;
    fn SetVersionAsync(&self, desiredversion: u32, handler: &::core::option::Option<ApplicationDataSetVersionHandler>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ClearAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ClearAsync(&self, locality: ApplicationDataLocality) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LocalSettings(&self) -> ::windows::core::Result<ApplicationDataContainer>;
    fn RoamingSettings(&self) -> ::windows::core::Result<ApplicationDataContainer>;
    fn LocalFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn RoamingFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn TemporaryFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn DataChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<ApplicationData, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveDataChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SignalDataChanged(&self) -> ::windows::core::Result<()>;
    fn RoamingStorageQuota(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationData {
    const NAME: &'static str = "Windows.Storage.IApplicationData";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataImpl, const OFFSET: isize>() -> IApplicationDataVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IApplicationData>,
            ::windows::core::GetTrustLevel,
            Version::<Impl, OFFSET>,
            SetVersionAsync::<Impl, OFFSET>,
            ClearAllAsync::<Impl, OFFSET>,
            ClearAsync::<Impl, OFFSET>,
            LocalSettings::<Impl, OFFSET>,
            RoamingSettings::<Impl, OFFSET>,
            LocalFolder::<Impl, OFFSET>,
            RoamingFolder::<Impl, OFFSET>,
            TemporaryFolder::<Impl, OFFSET>,
            DataChanged::<Impl, OFFSET>,
            RemoveDataChanged::<Impl, OFFSET>,
            SignalDataChanged::<Impl, OFFSET>,
            RoamingStorageQuota::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationData2Impl: Sized {
    fn LocalCacheFolder(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationData2 {
    const NAME: &'static str = "Windows.Storage.IApplicationData2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationData2Impl, const OFFSET: isize>() -> IApplicationData2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationData2>, ::windows::core::GetTrustLevel, LocalCacheFolder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationData3Impl: Sized {
    fn GetPublisherCacheFolder(&self, foldername: &::windows::core::HSTRING) -> ::windows::core::Result<StorageFolder>;
    fn ClearPublisherCacheFolderAsync(&self, foldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn SharedLocalFolder(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationData3 {
    const NAME: &'static str = "Windows.Storage.IApplicationData3";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationData3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationData3Impl, const OFFSET: isize>() -> IApplicationData3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationData3>, ::windows::core::GetTrustLevel, GetPublisherCacheFolder::<Impl, OFFSET>, ClearPublisherCacheFolderAsync::<Impl, OFFSET>, SharedLocalFolder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataContainerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Locality(&self) -> ::windows::core::Result<ApplicationDataLocality>;
    fn Values(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet>;
    fn Containers(&self) -> ::windows::core::Result<super::Foundation::Collections::IMapView<::windows::core::HSTRING, ApplicationDataContainer>>;
    fn CreateContainer(&self, name: &::windows::core::HSTRING, disposition: ApplicationDataCreateDisposition) -> ::windows::core::Result<ApplicationDataContainer>;
    fn DeleteContainer(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataContainer {
    const NAME: &'static str = "Windows.Storage.IApplicationDataContainer";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataContainerImpl, const OFFSET: isize>() -> IApplicationDataContainerVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationDataContainer>, ::windows::core::GetTrustLevel, Name::<Impl, OFFSET>, Locality::<Impl, OFFSET>, Values::<Impl, OFFSET>, Containers::<Impl, OFFSET>, CreateContainer::<Impl, OFFSET>, DeleteContainer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<ApplicationData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataStatics {
    const NAME: &'static str = "Windows.Storage.IApplicationDataStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataStaticsImpl, const OFFSET: isize>() -> IApplicationDataStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationDataStatics>, ::windows::core::GetTrustLevel, Current::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataStatics2Impl: Sized {
    fn GetForUserAsync(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ApplicationData>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationDataStatics2 {
    const NAME: &'static str = "Windows.Storage.IApplicationDataStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationDataStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationDataStatics2Impl, const OFFSET: isize>() -> IApplicationDataStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApplicationDataStatics2>, ::windows::core::GetTrustLevel, GetForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileManagerStaticsImpl: Sized {
    fn DeferUpdates(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<()>;
    fn CompleteUpdatesAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Provider::FileUpdateStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICachedFileManagerStatics {
    const NAME: &'static str = "Windows.Storage.ICachedFileManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICachedFileManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICachedFileManagerStaticsImpl, const OFFSET: isize>() -> ICachedFileManagerStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICachedFileManagerStatics>, ::windows::core::GetTrustLevel, DeferUpdates::<Impl, OFFSET>, CompleteUpdatesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadsFolderStaticsImpl: Sized {
    fn CreateFileAsync(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsync(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFileWithCollisionOptionAsync(&self, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderWithCollisionOptionAsync(&self, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadsFolderStatics {
    const NAME: &'static str = "Windows.Storage.IDownloadsFolderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadsFolderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadsFolderStaticsImpl, const OFFSET: isize>() -> IDownloadsFolderStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadsFolderStatics>, ::windows::core::GetTrustLevel, CreateFileAsync::<Impl, OFFSET>, CreateFolderAsync::<Impl, OFFSET>, CreateFileWithCollisionOptionAsync::<Impl, OFFSET>, CreateFolderWithCollisionOptionAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadsFolderStatics2Impl: Sized {
    fn CreateFileForUserAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderForUserAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFileForUserWithCollisionOptionAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderForUserWithCollisionOptionAsync(&self, user: &::core::option::Option<super::System::User>, desiredname: &::windows::core::HSTRING, option: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDownloadsFolderStatics2 {
    const NAME: &'static str = "Windows.Storage.IDownloadsFolderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDownloadsFolderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDownloadsFolderStatics2Impl, const OFFSET: isize>() -> IDownloadsFolderStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDownloadsFolderStatics2>, ::windows::core::GetTrustLevel, CreateFileForUserAsync::<Impl, OFFSET>, CreateFolderForUserAsync::<Impl, OFFSET>, CreateFileForUserWithCollisionOptionAsync::<Impl, OFFSET>, CreateFolderForUserWithCollisionOptionAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileIOStaticsImpl: Sized {
    fn ReadTextAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReadTextWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn WriteTextAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteTextWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadLinesAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn ReadLinesWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn WriteLinesAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteLinesWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesWithEncodingAsync(&self, file: &::core::option::Option<IStorageFile>, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadBufferAsync(&self, file: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>>;
    fn WriteBufferAsync(&self, file: &::core::option::Option<IStorageFile>, buffer: &::core::option::Option<Streams::IBuffer>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteBytesAsync(&self, file: &::core::option::Option<IStorageFile>, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFileIOStatics {
    const NAME: &'static str = "Windows.Storage.IFileIOStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFileIOStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileIOStaticsImpl, const OFFSET: isize>() -> IFileIOStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFileIOStatics>,
            ::windows::core::GetTrustLevel,
            ReadTextAsync::<Impl, OFFSET>,
            ReadTextWithEncodingAsync::<Impl, OFFSET>,
            WriteTextAsync::<Impl, OFFSET>,
            WriteTextWithEncodingAsync::<Impl, OFFSET>,
            AppendTextAsync::<Impl, OFFSET>,
            AppendTextWithEncodingAsync::<Impl, OFFSET>,
            ReadLinesAsync::<Impl, OFFSET>,
            ReadLinesWithEncodingAsync::<Impl, OFFSET>,
            WriteLinesAsync::<Impl, OFFSET>,
            WriteLinesWithEncodingAsync::<Impl, OFFSET>,
            AppendLinesAsync::<Impl, OFFSET>,
            AppendLinesWithEncodingAsync::<Impl, OFFSET>,
            ReadBufferAsync::<Impl, OFFSET>,
            WriteBufferAsync::<Impl, OFFSET>,
            WriteBytesAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersCameraRollStaticsImpl: Sized {
    fn CameraRoll(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersCameraRollStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersCameraRollStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersCameraRollStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersCameraRollStaticsImpl, const OFFSET: isize>() -> IKnownFoldersCameraRollStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownFoldersCameraRollStatics>, ::windows::core::GetTrustLevel, CameraRoll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersPlaylistsStaticsImpl: Sized {
    fn Playlists(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersPlaylistsStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersPlaylistsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersPlaylistsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersPlaylistsStaticsImpl, const OFFSET: isize>() -> IKnownFoldersPlaylistsStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownFoldersPlaylistsStatics>, ::windows::core::GetTrustLevel, Playlists::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersSavedPicturesStaticsImpl: Sized {
    fn SavedPictures(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersSavedPicturesStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersSavedPicturesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersSavedPicturesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersSavedPicturesStaticsImpl, const OFFSET: isize>() -> IKnownFoldersSavedPicturesStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownFoldersSavedPicturesStatics>, ::windows::core::GetTrustLevel, SavedPictures::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStaticsImpl: Sized {
    fn MusicLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn PicturesLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn VideosLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn DocumentsLibrary(&self) -> ::windows::core::Result<StorageFolder>;
    fn HomeGroup(&self) -> ::windows::core::Result<StorageFolder>;
    fn RemovableDevices(&self) -> ::windows::core::Result<StorageFolder>;
    fn MediaServerDevices(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersStatics {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStaticsImpl, const OFFSET: isize>() -> IKnownFoldersStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IKnownFoldersStatics>,
            ::windows::core::GetTrustLevel,
            MusicLibrary::<Impl, OFFSET>,
            PicturesLibrary::<Impl, OFFSET>,
            VideosLibrary::<Impl, OFFSET>,
            DocumentsLibrary::<Impl, OFFSET>,
            HomeGroup::<Impl, OFFSET>,
            RemovableDevices::<Impl, OFFSET>,
            MediaServerDevices::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics2Impl: Sized {
    fn Objects3D(&self) -> ::windows::core::Result<StorageFolder>;
    fn AppCaptures(&self) -> ::windows::core::Result<StorageFolder>;
    fn RecordedCalls(&self) -> ::windows::core::Result<StorageFolder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersStatics2 {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStatics2Impl, const OFFSET: isize>() -> IKnownFoldersStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownFoldersStatics2>, ::windows::core::GetTrustLevel, Objects3D::<Impl, OFFSET>, AppCaptures::<Impl, OFFSET>, RecordedCalls::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics3Impl: Sized {
    fn GetFolderForUserAsync(&self, user: &::core::option::Option<super::System::User>, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersStatics3 {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStatics3Impl, const OFFSET: isize>() -> IKnownFoldersStatics3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownFoldersStatics3>, ::windows::core::GetTrustLevel, GetFolderForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownFoldersStatics4Impl: Sized {
    fn RequestAccessAsync(&self, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>;
    fn RequestAccessForUserAsync(&self, user: &::core::option::Option<super::System::User>, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<KnownFoldersAccessStatus>>;
    fn GetFolderAsync(&self, folderid: KnownFolderId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownFoldersStatics4 {
    const NAME: &'static str = "Windows.Storage.IKnownFoldersStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownFoldersStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownFoldersStatics4Impl, const OFFSET: isize>() -> IKnownFoldersStatics4Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownFoldersStatics4>, ::windows::core::GetTrustLevel, RequestAccessAsync::<Impl, OFFSET>, RequestAccessForUserAsync::<Impl, OFFSET>, GetFolderAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPathIOStaticsImpl: Sized {
    fn ReadTextAsync(&self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ReadTextWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn WriteTextAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteTextWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendTextWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, contents: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadLinesAsync(&self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn ReadLinesWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn WriteLinesAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteLinesWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn AppendLinesWithEncodingAsync(&self, absolutepath: &::windows::core::HSTRING, lines: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, encoding: Streams::UnicodeEncoding) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn ReadBufferAsync(&self, absolutepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IBuffer>>;
    fn WriteBufferAsync(&self, absolutepath: &::windows::core::HSTRING, buffer: &::core::option::Option<Streams::IBuffer>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn WriteBytesAsync(&self, absolutepath: &::windows::core::HSTRING, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPathIOStatics {
    const NAME: &'static str = "Windows.Storage.IPathIOStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPathIOStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPathIOStaticsImpl, const OFFSET: isize>() -> IPathIOStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPathIOStatics>,
            ::windows::core::GetTrustLevel,
            ReadTextAsync::<Impl, OFFSET>,
            ReadTextWithEncodingAsync::<Impl, OFFSET>,
            WriteTextAsync::<Impl, OFFSET>,
            WriteTextWithEncodingAsync::<Impl, OFFSET>,
            AppendTextAsync::<Impl, OFFSET>,
            AppendTextWithEncodingAsync::<Impl, OFFSET>,
            ReadLinesAsync::<Impl, OFFSET>,
            ReadLinesWithEncodingAsync::<Impl, OFFSET>,
            WriteLinesAsync::<Impl, OFFSET>,
            WriteLinesWithEncodingAsync::<Impl, OFFSET>,
            AppendLinesAsync::<Impl, OFFSET>,
            AppendLinesWithEncodingAsync::<Impl, OFFSET>,
            ReadBufferAsync::<Impl, OFFSET>,
            WriteBufferAsync::<Impl, OFFSET>,
            WriteBytesAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetVersionDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetVersionDeferral {
    const NAME: &'static str = "Windows.Storage.ISetVersionDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl ISetVersionDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetVersionDeferralImpl, const OFFSET: isize>() -> ISetVersionDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ISetVersionDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISetVersionDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetVersionRequestImpl: Sized {
    fn CurrentVersion(&self) -> ::windows::core::Result<u32>;
    fn DesiredVersion(&self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&self) -> ::windows::core::Result<SetVersionDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetVersionRequest {
    const NAME: &'static str = "Windows.Storage.ISetVersionRequest";
}
#[cfg(feature = "implement_exclusive")]
impl ISetVersionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetVersionRequestImpl, const OFFSET: isize>() -> ISetVersionRequestVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISetVersionRequest>, ::windows::core::GetTrustLevel, CurrentVersion::<Impl, OFFSET>, DesiredVersion::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait IStorageFileImpl: Sized + IInputStreamReferenceImpl + IRandomAccessStreamReferenceImpl + IStorageItemImpl {
    fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OpenAsync(&self, accessmode: FileAccessMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
    fn CopyOverloadDefaultNameAndOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverloadDefaultOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyOverload(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CopyAndReplaceAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultNameAndOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverloadDefaultOptions(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveOverload(&self, destinationfolder: &::core::option::Option<IStorageFolder>, desirednewname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn MoveAndReplaceAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for IStorageFile {
    const NAME: &'static str = "Windows.Storage.IStorageFile";
}
#[cfg(feature = "Storage_Streams")]
impl IStorageFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileImpl, const OFFSET: isize>() -> IStorageFileVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageFile>,
            ::windows::core::GetTrustLevel,
            FileType::<Impl, OFFSET>,
            ContentType::<Impl, OFFSET>,
            OpenAsync::<Impl, OFFSET>,
            OpenTransactedWriteAsync::<Impl, OFFSET>,
            CopyOverloadDefaultNameAndOptions::<Impl, OFFSET>,
            CopyOverloadDefaultOptions::<Impl, OFFSET>,
            CopyOverload::<Impl, OFFSET>,
            CopyAndReplaceAsync::<Impl, OFFSET>,
            MoveOverloadDefaultNameAndOptions::<Impl, OFFSET>,
            MoveOverloadDefaultOptions::<Impl, OFFSET>,
            MoveOverload::<Impl, OFFSET>,
            MoveAndReplaceAsync::<Impl, OFFSET>,
        )
    }
}
pub trait IStorageFile2Impl: Sized {
    fn OpenWithOptionsAsync(&self, accessmode: FileAccessMode, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<Streams::IRandomAccessStream>>;
    fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageStreamTransaction>>;
}
impl ::windows::core::RuntimeName for IStorageFile2 {
    const NAME: &'static str = "Windows.Storage.IStorageFile2";
}
impl IStorageFile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFile2Impl, const OFFSET: isize>() -> IStorageFile2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFile2>, ::windows::core::GetTrustLevel, OpenWithOptionsAsync::<Impl, OFFSET>, OpenTransactedWriteWithOptionsAsync::<Impl, OFFSET>)
    }
}
pub trait IStorageFilePropertiesWithAvailabilityImpl: Sized {
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageFilePropertiesWithAvailability {
    const NAME: &'static str = "Windows.Storage.IStorageFilePropertiesWithAvailability";
}
impl IStorageFilePropertiesWithAvailabilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFilePropertiesWithAvailabilityImpl, const OFFSET: isize>() -> IStorageFilePropertiesWithAvailabilityVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFilePropertiesWithAvailability>, ::windows::core::GetTrustLevel, IsAvailable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileStaticsImpl: Sized {
    fn GetFileFromPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFileFromApplicationUriAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateStreamedFileAsync(&self, displaynamewithextension: &::windows::core::HSTRING, datarequested: &::core::option::Option<StreamedFileDataRequestedHandler>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn ReplaceWithStreamedFileAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>, datarequested: &::core::option::Option<StreamedFileDataRequestedHandler>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateStreamedFileFromUriAsync(&self, displaynamewithextension: &::windows::core::HSTRING, uri: &::core::option::Option<super::Foundation::Uri>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn ReplaceWithStreamedFileFromUriAsync(&self, filetoreplace: &::core::option::Option<IStorageFile>, uri: &::core::option::Option<super::Foundation::Uri>, thumbnail: &::core::option::Option<Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFileStatics {
    const NAME: &'static str = "Windows.Storage.IStorageFileStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFileStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileStaticsImpl, const OFFSET: isize>() -> IStorageFileStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageFileStatics>,
            ::windows::core::GetTrustLevel,
            GetFileFromPathAsync::<Impl, OFFSET>,
            GetFileFromApplicationUriAsync::<Impl, OFFSET>,
            CreateStreamedFileAsync::<Impl, OFFSET>,
            ReplaceWithStreamedFileAsync::<Impl, OFFSET>,
            CreateStreamedFileFromUriAsync::<Impl, OFFSET>,
            ReplaceWithStreamedFileFromUriAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFileStatics2Impl: Sized {
    fn GetFileFromPathForUserAsync(&self, user: &::core::option::Option<super::System::User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFileStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageFileStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFileStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFileStatics2Impl, const OFFSET: isize>() -> IStorageFileStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFileStatics2>, ::windows::core::GetTrustLevel, GetFileFromPathForUserAsync::<Impl, OFFSET>)
    }
}
pub trait IStorageFolderImpl: Sized + IStorageItemImpl {
    fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFileAsync(&self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn CreateFolderAsync(&self, desiredname: &::windows::core::HSTRING, options: CreationCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetFileAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFile>>;
    fn GetFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn GetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFile>>>;
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageFolder>>>;
    fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<IStorageItem>>>;
}
impl ::windows::core::RuntimeName for IStorageFolder {
    const NAME: &'static str = "Windows.Storage.IStorageFolder";
}
impl IStorageFolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderImpl, const OFFSET: isize>() -> IStorageFolderVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageFolder>,
            ::windows::core::GetTrustLevel,
            CreateFileAsyncOverloadDefaultOptions::<Impl, OFFSET>,
            CreateFileAsync::<Impl, OFFSET>,
            CreateFolderAsyncOverloadDefaultOptions::<Impl, OFFSET>,
            CreateFolderAsync::<Impl, OFFSET>,
            GetFileAsync::<Impl, OFFSET>,
            GetFolderAsync::<Impl, OFFSET>,
            GetItemAsync::<Impl, OFFSET>,
            GetFilesAsyncOverloadDefaultOptionsStartAndCount::<Impl, OFFSET>,
            GetFoldersAsyncOverloadDefaultOptionsStartAndCount::<Impl, OFFSET>,
            GetItemsAsyncOverloadDefaultStartAndCount::<Impl, OFFSET>,
        )
    }
}
pub trait IStorageFolder2Impl: Sized {
    fn TryGetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
impl ::windows::core::RuntimeName for IStorageFolder2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder2";
}
impl IStorageFolder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder2Impl, const OFFSET: isize>() -> IStorageFolder2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFolder2>, ::windows::core::GetTrustLevel, TryGetItemAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolder3Impl: Sized {
    fn TryGetChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFolder3 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder3";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFolder3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolder3Impl, const OFFSET: isize>() -> IStorageFolder3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFolder3>, ::windows::core::GetTrustLevel, TryGetChangeTracker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolderStaticsImpl: Sized {
    fn GetFolderFromPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFolderStatics {
    const NAME: &'static str = "Windows.Storage.IStorageFolderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFolderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderStaticsImpl, const OFFSET: isize>() -> IStorageFolderStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFolderStatics>, ::windows::core::GetTrustLevel, GetFolderFromPathAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageFolderStatics2Impl: Sized {
    fn GetFolderFromPathForUserAsync(&self, user: &::core::option::Option<super::System::User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageFolderStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageFolderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageFolderStatics2Impl, const OFFSET: isize>() -> IStorageFolderStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageFolderStatics2>, ::windows::core::GetTrustLevel, GetFolderFromPathForUserAsync::<Impl, OFFSET>)
    }
}
pub trait IStorageItemImpl: Sized {
    fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn RenameAsync(&self, desiredname: &::windows::core::HSTRING, option: NameCollisionOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self, option: StorageDeleteOption) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::BasicProperties>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Attributes(&self) -> ::windows::core::Result<FileAttributes>;
    fn DateCreated(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageItem {
    const NAME: &'static str = "Windows.Storage.IStorageItem";
}
impl IStorageItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemImpl, const OFFSET: isize>() -> IStorageItemVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageItem>,
            ::windows::core::GetTrustLevel,
            RenameAsyncOverloadDefaultOptions::<Impl, OFFSET>,
            RenameAsync::<Impl, OFFSET>,
            DeleteAsyncOverloadDefaultOptions::<Impl, OFFSET>,
            DeleteAsync::<Impl, OFFSET>,
            GetBasicPropertiesAsync::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Path::<Impl, OFFSET>,
            Attributes::<Impl, OFFSET>,
            DateCreated::<Impl, OFFSET>,
            IsOfType::<Impl, OFFSET>,
        )
    }
}
pub trait IStorageItem2Impl: Sized + IStorageItemImpl {
    fn GetParentAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn IsEqual(&self, item: &::core::option::Option<IStorageItem>) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IStorageItem2 {
    const NAME: &'static str = "Windows.Storage.IStorageItem2";
}
impl IStorageItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItem2Impl, const OFFSET: isize>() -> IStorageItem2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageItem2>, ::windows::core::GetTrustLevel, GetParentAsync::<Impl, OFFSET>, IsEqual::<Impl, OFFSET>)
    }
}
pub trait IStorageItemPropertiesImpl: Sized {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<FileProperties::StorageItemContentProperties>;
}
impl ::windows::core::RuntimeName for IStorageItemProperties {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties";
}
impl IStorageItemPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemPropertiesImpl, const OFFSET: isize>() -> IStorageItemPropertiesVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStorageItemProperties>,
            ::windows::core::GetTrustLevel,
            GetThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Impl, OFFSET>,
            GetThumbnailAsyncOverloadDefaultOptions::<Impl, OFFSET>,
            GetThumbnailAsync::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            DisplayType::<Impl, OFFSET>,
            FolderRelativeId::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
        )
    }
}
pub trait IStorageItemProperties2Impl: Sized + IStorageItemPropertiesImpl {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: FileProperties::ThumbnailMode) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
    fn GetScaledImageAsThumbnailAsync(&self, mode: FileProperties::ThumbnailMode, requestedsize: u32, options: FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FileProperties::StorageItemThumbnail>>;
}
impl ::windows::core::RuntimeName for IStorageItemProperties2 {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties2";
}
impl IStorageItemProperties2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemProperties2Impl, const OFFSET: isize>() -> IStorageItemProperties2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageItemProperties2>, ::windows::core::GetTrustLevel, GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions::<Impl, OFFSET>, GetScaledImageAsThumbnailAsyncOverloadDefaultOptions::<Impl, OFFSET>, GetScaledImageAsThumbnailAsync::<Impl, OFFSET>)
    }
}
pub trait IStorageItemPropertiesWithProviderImpl: Sized + IStorageItemPropertiesImpl {
    fn Provider(&self) -> ::windows::core::Result<StorageProvider>;
}
impl ::windows::core::RuntimeName for IStorageItemPropertiesWithProvider {
    const NAME: &'static str = "Windows.Storage.IStorageItemPropertiesWithProvider";
}
impl IStorageItemPropertiesWithProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageItemPropertiesWithProviderImpl, const OFFSET: isize>() -> IStorageItemPropertiesWithProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageItemPropertiesWithProvider>, ::windows::core::GetTrustLevel, Provider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryImpl: Sized {
    fn RequestAddFolderAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageFolder>>;
    fn RequestRemoveFolderAsync(&self, folder: &::core::option::Option<StorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn Folders(&self) -> ::windows::core::Result<super::Foundation::Collections::IObservableVector<StorageFolder>>;
    fn SaveFolder(&self) -> ::windows::core::Result<StorageFolder>;
    fn DefinitionChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<StorageLibrary, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveDefinitionChanged(&self, eventcookie: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibrary {
    const NAME: &'static str = "Windows.Storage.IStorageLibrary";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryImpl, const OFFSET: isize>() -> IStorageLibraryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibrary>, ::windows::core::GetTrustLevel, RequestAddFolderAsync::<Impl, OFFSET>, RequestRemoveFolderAsync::<Impl, OFFSET>, Folders::<Impl, OFFSET>, SaveFolder::<Impl, OFFSET>, DefinitionChanged::<Impl, OFFSET>, RemoveDefinitionChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibrary2Impl: Sized {
    fn ChangeTracker(&self) -> ::windows::core::Result<StorageLibraryChangeTracker>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibrary2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibrary2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibrary2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibrary2Impl, const OFFSET: isize>() -> IStorageLibrary2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibrary2>, ::windows::core::GetTrustLevel, ChangeTracker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibrary3Impl: Sized {
    fn AreFolderSuggestionsAvailableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibrary3 {
    const NAME: &'static str = "Windows.Storage.IStorageLibrary3";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibrary3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibrary3Impl, const OFFSET: isize>() -> IStorageLibrary3Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibrary3>, ::windows::core::GetTrustLevel, AreFolderSuggestionsAvailableAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeImpl: Sized {
    fn ChangeType(&self) -> ::windows::core::Result<StorageLibraryChangeType>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PreviousPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOfType(&self, r#type: StorageItemTypes) -> ::windows::core::Result<bool>;
    fn GetStorageItemAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<IStorageItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChange {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChange";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeImpl, const OFFSET: isize>() -> IStorageLibraryChangeVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChange>, ::windows::core::GetTrustLevel, ChangeType::<Impl, OFFSET>, Path::<Impl, OFFSET>, PreviousPath::<Impl, OFFSET>, IsOfType::<Impl, OFFSET>, GetStorageItemAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StorageLibraryChange>>>;
    fn AcceptChangesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeReader {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeReader";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeReaderImpl, const OFFSET: isize>() -> IStorageLibraryChangeReaderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeReader>, ::windows::core::GetTrustLevel, ReadBatchAsync::<Impl, OFFSET>, AcceptChangesAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeReader2Impl: Sized {
    fn GetLastChangeId(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeReader2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeReader2Impl, const OFFSET: isize>() -> IStorageLibraryChangeReader2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeReader2>, ::windows::core::GetTrustLevel, GetLastChangeId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerImpl: Sized {
    fn GetChangeReader(&self) -> ::windows::core::Result<StorageLibraryChangeReader>;
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTracker {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeTracker";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerImpl, const OFFSET: isize>() -> IStorageLibraryChangeTrackerVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeTracker>, ::windows::core::GetTrustLevel, GetChangeReader::<Impl, OFFSET>, Enable::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTracker2Impl: Sized {
    fn EnableWithOptions(&self, options: &::core::option::Option<StorageLibraryChangeTrackerOptions>) -> ::windows::core::Result<()>;
    fn Disable(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTracker2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeTracker2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTracker2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTracker2Impl, const OFFSET: isize>() -> IStorageLibraryChangeTracker2Vtbl {
        unsafe extern "system" fn EnableWithOptions<Impl: IStorageLibraryChangeTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableWithOptions(&*(&options as *const <StorageLibraryChangeTrackerOptions as ::windows::core::Abi>::Abi as *const <StorageLibraryChangeTrackerOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disable<Impl: IStorageLibraryChangeTracker2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeTracker2>, ::windows::core::GetTrustLevel, EnableWithOptions::<Impl, OFFSET>, Disable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerOptionsImpl: Sized {
    fn TrackChangeDetails(&self) -> ::windows::core::Result<bool>;
    fn SetTrackChangeDetails(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryChangeTrackerOptions {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryChangeTrackerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryChangeTrackerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryChangeTrackerOptionsImpl, const OFFSET: isize>() -> IStorageLibraryChangeTrackerOptionsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryChangeTrackerOptions>, ::windows::core::GetTrustLevel, TrackChangeDetails::<Impl, OFFSET>, SetTrackChangeDetails::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryLastChangeIdImpl, const OFFSET: isize>() -> IStorageLibraryLastChangeIdVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryLastChangeId>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryLastChangeIdStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryLastChangeIdStatics {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryLastChangeIdStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryLastChangeIdStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryLastChangeIdStaticsImpl, const OFFSET: isize>() -> IStorageLibraryLastChangeIdStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryLastChangeIdStatics>, ::windows::core::GetTrustLevel, Unknown::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryStaticsImpl: Sized {
    fn GetLibraryAsync(&self, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryStatics {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryStaticsImpl, const OFFSET: isize>() -> IStorageLibraryStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryStatics>, ::windows::core::GetTrustLevel, GetLibraryAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryStatics2Impl: Sized {
    fn GetLibraryForUserAsync(&self, user: &::core::option::Option<super::System::User>, libraryid: KnownLibraryId) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StorageLibrary>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageLibraryStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageLibraryStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageLibraryStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageLibraryStatics2Impl, const OFFSET: isize>() -> IStorageLibraryStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageLibraryStatics2>, ::windows::core::GetTrustLevel, GetLibraryForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProviderImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProvider {
    const NAME: &'static str = "Windows.Storage.IStorageProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProviderImpl, const OFFSET: isize>() -> IStorageProviderVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageProvider>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageProvider2Impl: Sized + IStorageProviderImpl {
    fn IsPropertySupportedForPartialFileAsync(&self, propertycanonicalname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStorageProvider2 {
    const NAME: &'static str = "Windows.Storage.IStorageProvider2";
}
#[cfg(feature = "implement_exclusive")]
impl IStorageProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageProvider2Impl, const OFFSET: isize>() -> IStorageProvider2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageProvider2>, ::windows::core::GetTrustLevel, IsPropertySupportedForPartialFileAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStorageStreamTransactionImpl: Sized + IClosableImpl {
    fn Stream(&self) -> ::windows::core::Result<Streams::IRandomAccessStream>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.IStorageStreamTransaction";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStorageStreamTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStorageStreamTransactionImpl, const OFFSET: isize>() -> IStorageStreamTransactionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStorageStreamTransaction>, ::windows::core::GetTrustLevel, Stream::<Impl, OFFSET>, CommitAsync::<Impl, OFFSET>)
    }
}
pub trait IStreamedFileDataRequestImpl: Sized {
    fn FailAndClose(&self, failuremode: StreamedFileFailureMode) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStreamedFileDataRequest {
    const NAME: &'static str = "Windows.Storage.IStreamedFileDataRequest";
}
impl IStreamedFileDataRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamedFileDataRequestImpl, const OFFSET: isize>() -> IStreamedFileDataRequestVtbl {
        unsafe extern "system" fn FailAndClose<Impl: IStreamedFileDataRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, failuremode: StreamedFileFailureMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FailAndClose(failuremode).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamedFileDataRequest>, ::windows::core::GetTrustLevel, FailAndClose::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemAudioPropertiesImpl: Sized {
    fn EncodingBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemAudioProperties {
    const NAME: &'static str = "Windows.Storage.ISystemAudioProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemAudioPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemAudioPropertiesImpl, const OFFSET: isize>() -> ISystemAudioPropertiesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemAudioProperties>, ::windows::core::GetTrustLevel, EncodingBitrate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDataPathsImpl: Sized {
    fn Fonts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProgramData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Public(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDesktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDocuments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicDownloads(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicMusic(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicVideos(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn System(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemHost(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemX86(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemX64(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemArm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserProfiles(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Windows(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDataPaths {
    const NAME: &'static str = "Windows.Storage.ISystemDataPaths";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDataPathsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDataPathsImpl, const OFFSET: isize>() -> ISystemDataPathsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemDataPaths>,
            ::windows::core::GetTrustLevel,
            Fonts::<Impl, OFFSET>,
            ProgramData::<Impl, OFFSET>,
            Public::<Impl, OFFSET>,
            PublicDesktop::<Impl, OFFSET>,
            PublicDocuments::<Impl, OFFSET>,
            PublicDownloads::<Impl, OFFSET>,
            PublicMusic::<Impl, OFFSET>,
            PublicPictures::<Impl, OFFSET>,
            PublicVideos::<Impl, OFFSET>,
            System::<Impl, OFFSET>,
            SystemHost::<Impl, OFFSET>,
            SystemX86::<Impl, OFFSET>,
            SystemX64::<Impl, OFFSET>,
            SystemArm::<Impl, OFFSET>,
            UserProfiles::<Impl, OFFSET>,
            Windows::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDataPathsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SystemDataPaths>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemDataPathsStatics {
    const NAME: &'static str = "Windows.Storage.ISystemDataPathsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemDataPathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemDataPathsStaticsImpl, const OFFSET: isize>() -> ISystemDataPathsStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemDataPathsStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemGPSPropertiesImpl: Sized {
    fn LatitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LongitudeDecimal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemGPSProperties {
    const NAME: &'static str = "Windows.Storage.ISystemGPSProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemGPSPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemGPSPropertiesImpl, const OFFSET: isize>() -> ISystemGPSPropertiesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemGPSProperties>, ::windows::core::GetTrustLevel, LatitudeDecimal::<Impl, OFFSET>, LongitudeDecimal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemImagePropertiesImpl: Sized {
    fn HorizontalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VerticalSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemImageProperties {
    const NAME: &'static str = "Windows.Storage.ISystemImageProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemImagePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemImagePropertiesImpl, const OFFSET: isize>() -> ISystemImagePropertiesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemImageProperties>, ::windows::core::GetTrustLevel, HorizontalSize::<Impl, OFFSET>, VerticalSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMediaPropertiesImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Producer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Writer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Year(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMediaProperties {
    const NAME: &'static str = "Windows.Storage.ISystemMediaProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMediaPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaPropertiesImpl, const OFFSET: isize>() -> ISystemMediaPropertiesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemMediaProperties>, ::windows::core::GetTrustLevel, Duration::<Impl, OFFSET>, Producer::<Impl, OFFSET>, Publisher::<Impl, OFFSET>, SubTitle::<Impl, OFFSET>, Writer::<Impl, OFFSET>, Year::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMusicPropertiesImpl: Sized {
    fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Composer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Conductor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemMusicProperties {
    const NAME: &'static str = "Windows.Storage.ISystemMusicProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemMusicPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMusicPropertiesImpl, const OFFSET: isize>() -> ISystemMusicPropertiesVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemMusicProperties>,
            ::windows::core::GetTrustLevel,
            AlbumArtist::<Impl, OFFSET>,
            AlbumTitle::<Impl, OFFSET>,
            Artist::<Impl, OFFSET>,
            Composer::<Impl, OFFSET>,
            Conductor::<Impl, OFFSET>,
            DisplayArtist::<Impl, OFFSET>,
            Genre::<Impl, OFFSET>,
            TrackNumber::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemPhotoPropertiesImpl: Sized {
    fn CameraManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CameraModel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateTaken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PeopleNames(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemPhotoProperties {
    const NAME: &'static str = "Windows.Storage.ISystemPhotoProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemPhotoPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemPhotoPropertiesImpl, const OFFSET: isize>() -> ISystemPhotoPropertiesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemPhotoProperties>, ::windows::core::GetTrustLevel, CameraManufacturer::<Impl, OFFSET>, CameraModel::<Impl, OFFSET>, DateTaken::<Impl, OFFSET>, Orientation::<Impl, OFFSET>, PeopleNames::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemPropertiesImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ItemNameDisplay(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Keywords(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rating(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Audio(&self) -> ::windows::core::Result<SystemAudioProperties>;
    fn GPS(&self) -> ::windows::core::Result<SystemGPSProperties>;
    fn Media(&self) -> ::windows::core::Result<SystemMediaProperties>;
    fn Music(&self) -> ::windows::core::Result<SystemMusicProperties>;
    fn Photo(&self) -> ::windows::core::Result<SystemPhotoProperties>;
    fn Video(&self) -> ::windows::core::Result<SystemVideoProperties>;
    fn Image(&self) -> ::windows::core::Result<SystemImageProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemProperties {
    const NAME: &'static str = "Windows.Storage.ISystemProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemPropertiesImpl, const OFFSET: isize>() -> ISystemPropertiesVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISystemProperties>,
            ::windows::core::GetTrustLevel,
            Author::<Impl, OFFSET>,
            Comment::<Impl, OFFSET>,
            ItemNameDisplay::<Impl, OFFSET>,
            Keywords::<Impl, OFFSET>,
            Rating::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            Audio::<Impl, OFFSET>,
            GPS::<Impl, OFFSET>,
            Media::<Impl, OFFSET>,
            Music::<Impl, OFFSET>,
            Photo::<Impl, OFFSET>,
            Video::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemVideoPropertiesImpl: Sized {
    fn Director(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameHeight(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameWidth(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TotalBitrate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemVideoProperties {
    const NAME: &'static str = "Windows.Storage.ISystemVideoProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemVideoPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemVideoPropertiesImpl, const OFFSET: isize>() -> ISystemVideoPropertiesVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemVideoProperties>, ::windows::core::GetTrustLevel, Director::<Impl, OFFSET>, FrameHeight::<Impl, OFFSET>, FrameWidth::<Impl, OFFSET>, Orientation::<Impl, OFFSET>, TotalBitrate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataPathsImpl: Sized {
    fn CameraRoll(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cookies(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Documents(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Downloads(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Favorites(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn History(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InternetCache(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAppDataLow(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Music(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pictures(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Profile(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recent(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoamingAppData(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SavedPictures(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Screenshots(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Templates(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Videos(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataPaths {
    const NAME: &'static str = "Windows.Storage.IUserDataPaths";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataPathsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataPathsImpl, const OFFSET: isize>() -> IUserDataPathsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserDataPaths>,
            ::windows::core::GetTrustLevel,
            CameraRoll::<Impl, OFFSET>,
            Cookies::<Impl, OFFSET>,
            Desktop::<Impl, OFFSET>,
            Documents::<Impl, OFFSET>,
            Downloads::<Impl, OFFSET>,
            Favorites::<Impl, OFFSET>,
            History::<Impl, OFFSET>,
            InternetCache::<Impl, OFFSET>,
            LocalAppData::<Impl, OFFSET>,
            LocalAppDataLow::<Impl, OFFSET>,
            Music::<Impl, OFFSET>,
            Pictures::<Impl, OFFSET>,
            Profile::<Impl, OFFSET>,
            Recent::<Impl, OFFSET>,
            RoamingAppData::<Impl, OFFSET>,
            SavedPictures::<Impl, OFFSET>,
            Screenshots::<Impl, OFFSET>,
            Templates::<Impl, OFFSET>,
            Videos::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataPathsStaticsImpl: Sized {
    fn GetForUser(&self, user: &::core::option::Option<super::System::User>) -> ::windows::core::Result<UserDataPaths>;
    fn GetDefault(&self) -> ::windows::core::Result<UserDataPaths>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataPathsStatics {
    const NAME: &'static str = "Windows.Storage.IUserDataPathsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataPathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataPathsStaticsImpl, const OFFSET: isize>() -> IUserDataPathsStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDataPathsStatics>, ::windows::core::GetTrustLevel, GetForUser::<Impl, OFFSET>, GetDefault::<Impl, OFFSET>)
    }
}
