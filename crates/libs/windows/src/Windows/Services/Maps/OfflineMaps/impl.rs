#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOfflineMapPackageImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<OfflineMapPackageStatus>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnclosingRegionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EstimatedSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn RemoveStatusChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&mut self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RequestStartDownloadAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackage";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOfflineMapPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineMapPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineMapPackageVtbl {
        unsafe extern "system" fn Status<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnclosingRegionName<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnclosingRegionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimatedSizeInBytes<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstimatedSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&value as *const <super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStartDownloadAsync<Impl: IOfflineMapPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStartDownloadAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOfflineMapPackage, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            EnclosingRegionName: EnclosingRegionName::<Impl, IMPL_OFFSET>,
            EstimatedSizeInBytes: EstimatedSizeInBytes::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RequestStartDownloadAsync: RequestStartDownloadAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineMapPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IOfflineMapPackageQueryResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<OfflineMapPackageQueryStatus>;
    fn Packages(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackageQueryResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IOfflineMapPackageQueryResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineMapPackageQueryResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineMapPackageQueryResultVtbl {
        unsafe extern "system" fn Status<Impl: IOfflineMapPackageQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Packages<Impl: IOfflineMapPackageQueryResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Packages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOfflineMapPackageQueryResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Packages: Packages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineMapPackageQueryResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageStartDownloadResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStartDownloadResult";
}
#[cfg(feature = "implement_exclusive")]
impl IOfflineMapPackageStartDownloadResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineMapPackageStartDownloadResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineMapPackageStartDownloadResultVtbl {
        unsafe extern "system" fn Status<Impl: IOfflineMapPackageStartDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOfflineMapPackageStartDownloadResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineMapPackageStartDownloadResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IOfflineMapPackageStaticsImpl: Sized {
    fn FindPackagesAsync(&mut self, querypoint: &::core::option::Option<super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
    fn FindPackagesInBoundingBoxAsync(&mut self, queryboundingbox: &::core::option::Option<super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
    fn FindPackagesInGeocircleAsync(&mut self, querycircle: &::core::option::Option<super::super::super::Devices::Geolocation::Geocircle>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOfflineMapPackageStatics {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.IOfflineMapPackageStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl IOfflineMapPackageStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOfflineMapPackageStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOfflineMapPackageStaticsVtbl {
        unsafe extern "system" fn FindPackagesAsync<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querypoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesAsync(&*(&querypoint as *const <super::super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesInBoundingBoxAsync<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queryboundingbox: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesInBoundingBoxAsync(&*(&queryboundingbox as *const <super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::GeoboundingBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesInGeocircleAsync<Impl: IOfflineMapPackageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querycircle: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesInGeocircleAsync(&*(&querycircle as *const <super::super::super::Devices::Geolocation::Geocircle as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geocircle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOfflineMapPackageStatics, BASE_OFFSET>(),
            FindPackagesAsync: FindPackagesAsync::<Impl, IMPL_OFFSET>,
            FindPackagesInBoundingBoxAsync: FindPackagesInBoundingBoxAsync::<Impl, IMPL_OFFSET>,
            FindPackagesInGeocircleAsync: FindPackagesInGeocircleAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOfflineMapPackageStatics as ::windows::core::Interface>::IID
    }
}
