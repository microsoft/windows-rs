pub trait ICivicAddressReportImpl: Sized + ILocationReportImpl {
    fn GetAddressLine1();
    fn GetAddressLine2();
    fn GetCity();
    fn GetStateProvince();
    fn GetPostalCode();
    fn GetCountryRegion();
    fn GetDetailLevel();
}
impl ::windows::core::RuntimeName for ICivicAddressReport {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ICivicAddressReport";
}
impl ICivicAddressReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICivicAddressReportImpl, const OFFSET: isize>() -> ICivicAddressReportVtbl {
        unsafe extern "system" fn GetAddressLine1<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAddressLine1(::core::mem::transmute_copy(&pbstraddress1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAddressLine2<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAddressLine2(::core::mem::transmute_copy(&pbstraddress2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCity<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCity(::core::mem::transmute_copy(&pbstrcity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateProvince<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStateProvince(::core::mem::transmute_copy(&pbstrstateprovince)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostalCode<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostalCode(::core::mem::transmute_copy(&pbstrpostalcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCountryRegion<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCountryRegion(::core::mem::transmute_copy(&pbstrcountryregion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDetailLevel<Impl: ICivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDetailLevel(::core::mem::transmute_copy(&pdetaillevel)) {
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
            ::windows::core::GetRuntimeClassName::<ICivicAddressReport>,
            ::windows::core::GetTrustLevel,
            GetAddressLine1::<Impl, OFFSET>,
            GetAddressLine2::<Impl, OFFSET>,
            GetCity::<Impl, OFFSET>,
            GetStateProvince::<Impl, OFFSET>,
            GetPostalCode::<Impl, OFFSET>,
            GetCountryRegion::<Impl, OFFSET>,
            GetDetailLevel::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICivicAddressReportFactoryImpl: Sized + ILocationReportFactoryImpl + IDispatchImpl {
    fn CivicAddressReport();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICivicAddressReportFactory {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ICivicAddressReportFactory";
}
#[cfg(feature = "Win32_System_Com")]
impl ICivicAddressReportFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICivicAddressReportFactoryImpl, const OFFSET: isize>() -> ICivicAddressReportFactoryVtbl {
        unsafe extern "system" fn CivicAddressReport<Impl: ICivicAddressReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CivicAddressReport(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICivicAddressReportFactory>, ::windows::core::GetTrustLevel, CivicAddressReport::<Impl, OFFSET>)
    }
}
pub trait IDefaultLocationImpl: Sized {
    fn SetReport();
    fn GetReport();
}
impl ::windows::core::RuntimeName for IDefaultLocation {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.IDefaultLocation";
}
impl IDefaultLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultLocationImpl, const OFFSET: isize>() -> IDefaultLocationVtbl {
        unsafe extern "system" fn SetReport<Impl: IDefaultLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReport(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&plocationreport as *const <ILocationReport as ::windows::core::Abi>::Abi as *const <ILocationReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReport<Impl: IDefaultLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplocationreport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDefaultLocation>, ::windows::core::GetTrustLevel, SetReport::<Impl, OFFSET>, GetReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispCivicAddressReportImpl: Sized + IDispatchImpl {
    fn AddressLine1();
    fn AddressLine2();
    fn City();
    fn StateProvince();
    fn PostalCode();
    fn CountryRegion();
    fn DetailLevel();
    fn Timestamp();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDispCivicAddressReport {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.IDispCivicAddressReport";
}
#[cfg(feature = "Win32_System_Com")]
impl IDispCivicAddressReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispCivicAddressReportImpl, const OFFSET: isize>() -> IDispCivicAddressReportVtbl {
        unsafe extern "system" fn AddressLine1<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressLine1(::core::mem::transmute_copy(&paddress1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressLine2<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressLine2(::core::mem::transmute_copy(&paddress2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City(::core::mem::transmute_copy(&pcity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateProvince<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateProvince(::core::mem::transmute_copy(&pstateprovince)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode(::core::mem::transmute_copy(&ppostalcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegion<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryRegion(::core::mem::transmute_copy(&pcountryregion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailLevel<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailLevel(::core::mem::transmute_copy(&pdetaillevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IDispCivicAddressReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp(::core::mem::transmute_copy(&pval)) {
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
            ::windows::core::GetRuntimeClassName::<IDispCivicAddressReport>,
            ::windows::core::GetTrustLevel,
            AddressLine1::<Impl, OFFSET>,
            AddressLine2::<Impl, OFFSET>,
            City::<Impl, OFFSET>,
            StateProvince::<Impl, OFFSET>,
            PostalCode::<Impl, OFFSET>,
            CountryRegion::<Impl, OFFSET>,
            DetailLevel::<Impl, OFFSET>,
            Timestamp::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispLatLongReportImpl: Sized + IDispatchImpl {
    fn Latitude();
    fn Longitude();
    fn ErrorRadius();
    fn Altitude();
    fn AltitudeError();
    fn Timestamp();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDispLatLongReport {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.IDispLatLongReport";
}
#[cfg(feature = "Win32_System_Com")]
impl IDispLatLongReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispLatLongReportImpl, const OFFSET: isize>() -> IDispLatLongReportVtbl {
        unsafe extern "system" fn Latitude<Impl: IDispLatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Latitude(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Longitude<Impl: IDispLatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Longitude(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorRadius<Impl: IDispLatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorRadius(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Altitude<Impl: IDispLatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Altitude(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeError<Impl: IDispLatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltitudeError(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IDispLatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispLatLongReport>, ::windows::core::GetTrustLevel, Latitude::<Impl, OFFSET>, Longitude::<Impl, OFFSET>, ErrorRadius::<Impl, OFFSET>, Altitude::<Impl, OFFSET>, AltitudeError::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
pub trait ILatLongReportImpl: Sized + ILocationReportImpl {
    fn GetLatitude();
    fn GetLongitude();
    fn GetErrorRadius();
    fn GetAltitude();
    fn GetAltitudeError();
}
impl ::windows::core::RuntimeName for ILatLongReport {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILatLongReport";
}
impl ILatLongReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILatLongReportImpl, const OFFSET: isize>() -> ILatLongReportVtbl {
        unsafe extern "system" fn GetLatitude<Impl: ILatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatitude(::core::mem::transmute_copy(&platitude)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongitude<Impl: ILatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plongitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongitude(::core::mem::transmute_copy(&plongitude)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorRadius<Impl: ILatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorradius: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorRadius(::core::mem::transmute_copy(&perrorradius)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitude<Impl: ILatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paltitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAltitude(::core::mem::transmute_copy(&paltitude)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitudeError<Impl: ILatLongReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paltitudeerror: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAltitudeError(::core::mem::transmute_copy(&paltitudeerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILatLongReport>, ::windows::core::GetTrustLevel, GetLatitude::<Impl, OFFSET>, GetLongitude::<Impl, OFFSET>, GetErrorRadius::<Impl, OFFSET>, GetAltitude::<Impl, OFFSET>, GetAltitudeError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILatLongReportFactoryImpl: Sized + ILocationReportFactoryImpl + IDispatchImpl {
    fn LatLongReport();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ILatLongReportFactory {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILatLongReportFactory";
}
#[cfg(feature = "Win32_System_Com")]
impl ILatLongReportFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILatLongReportFactoryImpl, const OFFSET: isize>() -> ILatLongReportFactoryVtbl {
        unsafe extern "system" fn LatLongReport<Impl: ILatLongReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LatLongReport(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILatLongReportFactory>, ::windows::core::GetTrustLevel, LatLongReport::<Impl, OFFSET>)
    }
}
pub trait ILocationImpl: Sized {
    fn RegisterForReport();
    fn UnregisterForReport();
    fn GetReport();
    fn GetReportStatus();
    fn GetReportInterval();
    fn SetReportInterval();
    fn GetDesiredAccuracy();
    fn SetDesiredAccuracy();
    fn RequestPermissions();
}
impl ::windows::core::RuntimeName for ILocation {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILocation";
}
impl ILocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationImpl, const OFFSET: isize>() -> ILocationVtbl {
        unsafe extern "system" fn RegisterForReport<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterForReport(&*(&pevents as *const <ILocationEvents as ::windows::core::Abi>::Abi as *const <ILocationEvents as ::windows::core::DefaultType>::DefaultType), &*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwrequestedreportinterval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterForReport<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterForReport(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReport<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pplocationreport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportStatus<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReportStatus(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportInterval<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pmilliseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReportInterval(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pmilliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReportInterval(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), millisecondsrequested) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesiredAccuracy<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesiredAccuracy(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdesiredaccuracy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredAccuracy(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), desiredaccuracy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPermissions<Impl: ILocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPermissions(
                &*(&hparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&preporttypes as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                count,
                &*(&fmodal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
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
            ::windows::core::GetRuntimeClassName::<ILocation>,
            ::windows::core::GetTrustLevel,
            RegisterForReport::<Impl, OFFSET>,
            UnregisterForReport::<Impl, OFFSET>,
            GetReport::<Impl, OFFSET>,
            GetReportStatus::<Impl, OFFSET>,
            GetReportInterval::<Impl, OFFSET>,
            SetReportInterval::<Impl, OFFSET>,
            GetDesiredAccuracy::<Impl, OFFSET>,
            SetDesiredAccuracy::<Impl, OFFSET>,
            RequestPermissions::<Impl, OFFSET>,
        )
    }
}
pub trait ILocationEventsImpl: Sized {
    fn OnLocationChanged();
    fn OnStatusChanged();
}
impl ::windows::core::RuntimeName for ILocationEvents {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILocationEvents";
}
impl ILocationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationEventsImpl, const OFFSET: isize>() -> ILocationEventsVtbl {
        unsafe extern "system" fn OnLocationChanged<Impl: ILocationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLocationChanged(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&plocationreport as *const <ILocationReport as ::windows::core::Abi>::Abi as *const <ILocationReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStatusChanged<Impl: ILocationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStatusChanged(&*(&reporttype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), newstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocationEvents>, ::windows::core::GetTrustLevel, OnLocationChanged::<Impl, OFFSET>, OnStatusChanged::<Impl, OFFSET>)
    }
}
pub trait ILocationPowerImpl: Sized {
    fn Connect();
    fn Disconnect();
}
impl ::windows::core::RuntimeName for ILocationPower {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILocationPower";
}
impl ILocationPowerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationPowerImpl, const OFFSET: isize>() -> ILocationPowerVtbl {
        unsafe extern "system" fn Connect<Impl: ILocationPowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: ILocationPowerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocationPower>, ::windows::core::GetTrustLevel, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>)
    }
}
pub trait ILocationReportImpl: Sized {
    fn GetSensorID();
    fn GetTimestamp();
    fn GetValue();
}
impl ::windows::core::RuntimeName for ILocationReport {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILocationReport";
}
impl ILocationReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationReportImpl, const OFFSET: isize>() -> ILocationReportVtbl {
        unsafe extern "system" fn GetSensorID<Impl: ILocationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorID(::core::mem::transmute_copy(&psensorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimestamp<Impl: ILocationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimestamp(::core::mem::transmute_copy(&pcreationtime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ILocationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&pkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocationReport>, ::windows::core::GetTrustLevel, GetSensorID::<Impl, OFFSET>, GetTimestamp::<Impl, OFFSET>, GetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILocationReportFactoryImpl: Sized + IDispatchImpl {
    fn ListenForReports();
    fn StopListeningForReports();
    fn Status();
    fn ReportInterval();
    fn SetReportInterval();
    fn DesiredAccuracy();
    fn SetDesiredAccuracy();
    fn RequestPermissions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ILocationReportFactory {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation.ILocationReportFactory";
}
#[cfg(feature = "Win32_System_Com")]
impl ILocationReportFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationReportFactoryImpl, const OFFSET: isize>() -> ILocationReportFactoryVtbl {
        unsafe extern "system" fn ListenForReports<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenForReports(requestedreportinterval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopListeningForReports<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopListeningForReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInterval<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval(::core::mem::transmute_copy(&pmilliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetReportInterval(millisecondsrequested) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredAccuracy<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAccuracy(::core::mem::transmute_copy(&pdesiredaccuracy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredAccuracy(desiredaccuracy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPermissions<Impl: ILocationReportFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPermissions(hwnd) {
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
            ::windows::core::GetRuntimeClassName::<ILocationReportFactory>,
            ::windows::core::GetTrustLevel,
            ListenForReports::<Impl, OFFSET>,
            StopListeningForReports::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            ReportInterval::<Impl, OFFSET>,
            SetReportInterval::<Impl, OFFSET>,
            DesiredAccuracy::<Impl, OFFSET>,
            SetDesiredAccuracy::<Impl, OFFSET>,
            RequestPermissions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ICivicAddressReportFactoryEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _ICivicAddressReportFactoryEvents {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation._ICivicAddressReportFactoryEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _ICivicAddressReportFactoryEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ICivicAddressReportFactoryEventsImpl, const OFFSET: isize>() -> _ICivicAddressReportFactoryEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_ICivicAddressReportFactoryEvents>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ILatLongReportFactoryEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for _ILatLongReportFactoryEvents {
    const NAME: &'static str = "Windows.Win32.Devices.Geolocation._ILatLongReportFactoryEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl _ILatLongReportFactoryEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ILatLongReportFactoryEventsImpl, const OFFSET: isize>() -> _ILatLongReportFactoryEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<_ILatLongReportFactoryEvents>, ::windows::core::GetTrustLevel)
    }
}
