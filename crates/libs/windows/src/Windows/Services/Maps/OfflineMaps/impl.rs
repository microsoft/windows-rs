#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStatus>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnclosingRegionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EstimatedSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RequestStartDownloadAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackage";
}
#[cfg(feature = "implement_exclusive")]
impl IOfflineMapPackageVtbl {
    pub const fn new<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOfflineMapPackageVtbl {
        unsafe extern "system" fn Status<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnclosingRegionName<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnclosingRegionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimatedSizeInBytes<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EstimatedSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&value as *const <super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStartDownloadAsync<Impl: IOfflineMapPackageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestStartDownloadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOfflineMapPackage>, base.5, Status::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, EnclosingRegionName::<Impl, OFFSET>, EstimatedSizeInBytes::<Impl, OFFSET>, RemoveStatusChanged::<Impl, OFFSET>, StatusChanged::<Impl, OFFSET>, RequestStartDownloadAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageQueryResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<OfflineMapPackageQueryStatus>;
    fn Packages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult";
}
#[cfg(feature = "implement_exclusive")]
impl IOfflineMapPackageQueryResultVtbl {
    pub const fn new<Impl: IOfflineMapPackageQueryResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOfflineMapPackageQueryResultVtbl {
        unsafe extern "system" fn Status<Impl: IOfflineMapPackageQueryResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Packages<Impl: IOfflineMapPackageQueryResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Packages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOfflineMapPackageQueryResult>, base.5, Status::<Impl, OFFSET>, Packages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageStartDownloadResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult";
}
#[cfg(feature = "implement_exclusive")]
impl IOfflineMapPackageStartDownloadResultVtbl {
    pub const fn new<Impl: IOfflineMapPackageStartDownloadResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOfflineMapPackageStartDownloadResultVtbl {
        unsafe extern "system" fn Status<Impl: IOfflineMapPackageStartDownloadResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOfflineMapPackageStartDownloadResult>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageStaticsImpl: Sized {
    fn FindPackagesAsync(&self, querypoint: &::core::option::Option<super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
    fn FindPackagesInBoundingBoxAsync(&self, queryboundingbox: &::core::option::Option<super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
    fn FindPackagesInGeocircleAsync(&self, querycircle: &::core::option::Option<super::super::super::Devices::Geolocation::Geocircle>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOfflineMapPackageStatics {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IOfflineMapPackageStaticsVtbl {
    pub const fn new<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOfflineMapPackageStaticsVtbl {
        unsafe extern "system" fn FindPackagesAsync<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querypoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindPackagesAsync(&*(&querypoint as *const <super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesInBoundingBoxAsync<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, queryboundingbox: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindPackagesInBoundingBoxAsync(&*(&queryboundingbox as *const <super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesInGeocircleAsync<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querycircle: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindPackagesInGeocircleAsync(&*(&querycircle as *const <super::super::super::Devices::Geolocation::Geocircle as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geocircle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOfflineMapPackageStatics>, base.5, FindPackagesAsync::<Impl, OFFSET>, FindPackagesInBoundingBoxAsync::<Impl, OFFSET>, FindPackagesInGeocircleAsync::<Impl, OFFSET>)
    }
}
