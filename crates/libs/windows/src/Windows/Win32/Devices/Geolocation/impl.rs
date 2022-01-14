#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICivicAddressReport_Impl: Sized + ILocationReport_Impl {
    fn GetAddressLine1(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAddressLine2(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCity(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetStateProvince(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPostalCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCountryRegion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDetailLevel(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ICivicAddressReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICivicAddressReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICivicAddressReport_Vtbl {
        unsafe extern "system" fn GetAddressLine1<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAddressLine1() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstraddress1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAddressLine2<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAddressLine2() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstraddress2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCity<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCity() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateProvince<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStateProvince() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrstateprovince = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostalCode<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpostalcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCountryRegion<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCountryRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcountryregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDetailLevel<Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDetailLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pdetaillevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ILocationReport_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAddressLine1: GetAddressLine1::<Impl, IMPL_OFFSET>,
            GetAddressLine2: GetAddressLine2::<Impl, IMPL_OFFSET>,
            GetCity: GetCity::<Impl, IMPL_OFFSET>,
            GetStateProvince: GetStateProvince::<Impl, IMPL_OFFSET>,
            GetPostalCode: GetPostalCode::<Impl, IMPL_OFFSET>,
            GetCountryRegion: GetCountryRegion::<Impl, IMPL_OFFSET>,
            GetDetailLevel: GetDetailLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICivicAddressReport as ::windows::core::Interface>::IID || iid == &<ILocationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICivicAddressReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl + ILocationReportFactory_Impl {
    fn CivicAddressReport(&mut self) -> ::windows::core::Result<IDispCivicAddressReport>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICivicAddressReportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICivicAddressReportFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICivicAddressReportFactory_Vtbl {
        unsafe extern "system" fn CivicAddressReport<Impl: ICivicAddressReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CivicAddressReport() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ILocationReportFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CivicAddressReport: CivicAddressReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICivicAddressReportFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ILocationReportFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDefaultLocation_Impl: Sized {
    fn SetReport(&mut self, reporttype: *const ::windows::core::GUID, plocationreport: &::core::option::Option<ILocationReport>) -> ::windows::core::Result<()>;
    fn GetReport(&mut self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<ILocationReport>;
}
impl IDefaultLocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultLocation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDefaultLocation_Vtbl {
        unsafe extern "system" fn SetReport<Impl: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReport(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute(&plocationreport)).into()
        }
        unsafe extern "system" fn GetReport<Impl: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplocationreport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetReport: SetReport::<Impl, IMPL_OFFSET>,
            GetReport: GetReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDispCivicAddressReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressLine1(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddressLine2(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn City(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StateProvince(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PostalCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CountryRegion(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DetailLevel(&mut self) -> ::windows::core::Result<u32>;
    fn Timestamp(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDispCivicAddressReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispCivicAddressReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispCivicAddressReport_Vtbl {
        unsafe extern "system" fn AddressLine1<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressLine1() {
                ::core::result::Result::Ok(ok__) => {
                    *paddress1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressLine2<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressLine2() {
                ::core::result::Result::Ok(ok__) => {
                    *paddress2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *pcity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateProvince<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateProvince() {
                ::core::result::Result::Ok(ok__) => {
                    *pstateprovince = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppostalcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegion<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *pcountryregion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailLevel<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pdetaillevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddressLine1: AddressLine1::<Impl, IMPL_OFFSET>,
            AddressLine2: AddressLine2::<Impl, IMPL_OFFSET>,
            City: City::<Impl, IMPL_OFFSET>,
            StateProvince: StateProvince::<Impl, IMPL_OFFSET>,
            PostalCode: PostalCode::<Impl, IMPL_OFFSET>,
            CountryRegion: CountryRegion::<Impl, IMPL_OFFSET>,
            DetailLevel: DetailLevel::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispCivicAddressReport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDispLatLongReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Latitude(&mut self) -> ::windows::core::Result<f64>;
    fn Longitude(&mut self) -> ::windows::core::Result<f64>;
    fn ErrorRadius(&mut self) -> ::windows::core::Result<f64>;
    fn Altitude(&mut self) -> ::windows::core::Result<f64>;
    fn AltitudeError(&mut self) -> ::windows::core::Result<f64>;
    fn Timestamp(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDispLatLongReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispLatLongReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispLatLongReport_Vtbl {
        unsafe extern "system" fn Latitude<Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Latitude() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Longitude<Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Longitude() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorRadius<Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Altitude<Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Altitude() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeError<Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltitudeError() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Latitude: Latitude::<Impl, IMPL_OFFSET>,
            Longitude: Longitude::<Impl, IMPL_OFFSET>,
            ErrorRadius: ErrorRadius::<Impl, IMPL_OFFSET>,
            Altitude: Altitude::<Impl, IMPL_OFFSET>,
            AltitudeError: AltitudeError::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispLatLongReport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ILatLongReport_Impl: Sized + ILocationReport_Impl {
    fn GetLatitude(&mut self) -> ::windows::core::Result<f64>;
    fn GetLongitude(&mut self) -> ::windows::core::Result<f64>;
    fn GetErrorRadius(&mut self) -> ::windows::core::Result<f64>;
    fn GetAltitude(&mut self) -> ::windows::core::Result<f64>;
    fn GetAltitudeError(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ILatLongReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILatLongReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILatLongReport_Vtbl {
        unsafe extern "system" fn GetLatitude<Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatitude() {
                ::core::result::Result::Ok(ok__) => {
                    *platitude = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongitude<Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plongitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLongitude() {
                ::core::result::Result::Ok(ok__) => {
                    *plongitude = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorRadius<Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorradius: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *perrorradius = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitude<Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paltitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAltitude() {
                ::core::result::Result::Ok(ok__) => {
                    *paltitude = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitudeError<Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paltitudeerror: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAltitudeError() {
                ::core::result::Result::Ok(ok__) => {
                    *paltitudeerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ILocationReport_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLatitude: GetLatitude::<Impl, IMPL_OFFSET>,
            GetLongitude: GetLongitude::<Impl, IMPL_OFFSET>,
            GetErrorRadius: GetErrorRadius::<Impl, IMPL_OFFSET>,
            GetAltitude: GetAltitude::<Impl, IMPL_OFFSET>,
            GetAltitudeError: GetAltitudeError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILatLongReport as ::windows::core::Interface>::IID || iid == &<ILocationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILatLongReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl + ILocationReportFactory_Impl {
    fn LatLongReport(&mut self) -> ::windows::core::Result<IDispLatLongReport>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILatLongReportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILatLongReportFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILatLongReportFactory_Vtbl {
        unsafe extern "system" fn LatLongReport<Impl: ILatLongReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LatLongReport() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ILocationReportFactory_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), LatLongReport: LatLongReport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILatLongReportFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ILocationReportFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
pub trait ILocation_Impl: Sized {
    fn RegisterForReport(&mut self, pevents: &::core::option::Option<ILocationEvents>, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::Result<()>;
    fn UnregisterForReport(&mut self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetReport(&mut self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<ILocationReport>;
    fn GetReportStatus(&mut self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<LOCATION_REPORT_STATUS>;
    fn GetReportInterval(&mut self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::Result<()>;
    fn GetDesiredAccuracy(&mut self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY>;
    fn SetDesiredAccuracy(&mut self, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::Result<()>;
    fn RequestPermissions(&mut self, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
impl ILocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocation_Vtbl {
        unsafe extern "system" fn RegisterForReport<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterForReport(::core::mem::transmute(&pevents), ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&dwrequestedreportinterval)).into()
        }
        unsafe extern "system" fn UnregisterForReport<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterForReport(::core::mem::transmute_copy(&reporttype)).into()
        }
        unsafe extern "system" fn GetReport<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReport(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pplocationreport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportStatus<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReportStatus(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportInterval<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pmilliseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReportInterval(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmilliseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&millisecondsrequested)).into()
        }
        unsafe extern "system" fn GetDesiredAccuracy<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesiredAccuracy(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdesiredaccuracy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredAccuracy(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&desiredaccuracy)).into()
        }
        unsafe extern "system" fn RequestPermissions<Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestPermissions(::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&preporttypes), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&fmodal)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterForReport: RegisterForReport::<Impl, IMPL_OFFSET>,
            UnregisterForReport: UnregisterForReport::<Impl, IMPL_OFFSET>,
            GetReport: GetReport::<Impl, IMPL_OFFSET>,
            GetReportStatus: GetReportStatus::<Impl, IMPL_OFFSET>,
            GetReportInterval: GetReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            GetDesiredAccuracy: GetDesiredAccuracy::<Impl, IMPL_OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Impl, IMPL_OFFSET>,
            RequestPermissions: RequestPermissions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocation as ::windows::core::Interface>::IID
    }
}
pub trait ILocationEvents_Impl: Sized {
    fn OnLocationChanged(&mut self, reporttype: *const ::windows::core::GUID, plocationreport: &::core::option::Option<ILocationReport>) -> ::windows::core::Result<()>;
    fn OnStatusChanged(&mut self, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::Result<()>;
}
impl ILocationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationEvents_Vtbl {
        unsafe extern "system" fn OnLocationChanged<Impl: ILocationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLocationChanged(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute(&plocationreport)).into()
        }
        unsafe extern "system" fn OnStatusChanged<Impl: ILocationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatusChanged(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&newstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnLocationChanged: OnLocationChanged::<Impl, IMPL_OFFSET>,
            OnStatusChanged: OnStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationEvents as ::windows::core::Interface>::IID
    }
}
pub trait ILocationPower_Impl: Sized {
    fn Connect(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
}
impl ILocationPower_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationPower_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationPower_Vtbl {
        unsafe extern "system" fn Connect<Impl: ILocationPower_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: ILocationPower_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationPower as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ILocationReport_Impl: Sized {
    fn GetSensorID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetValue(&mut self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ILocationReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationReport_Vtbl {
        unsafe extern "system" fn GetSensorID<Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorID() {
                ::core::result::Result::Ok(ok__) => {
                    *psensorid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimestamp<Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *pcreationtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSensorID: GetSensorID::<Impl, IMPL_OFFSET>,
            GetTimestamp: GetTimestamp::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILocationReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ListenForReports(&mut self, requestedreportinterval: u32) -> ::windows::core::Result<()>;
    fn StopListeningForReports(&mut self) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<u32>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, millisecondsrequested: u32) -> ::windows::core::Result<()>;
    fn DesiredAccuracy(&mut self) -> ::windows::core::Result<u32>;
    fn SetDesiredAccuracy(&mut self, desiredaccuracy: u32) -> ::windows::core::Result<()>;
    fn RequestPermissions(&mut self, hwnd: *const u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILocationReportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationReportFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationReportFactory_Vtbl {
        unsafe extern "system" fn ListenForReports<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ListenForReports(::core::mem::transmute_copy(&requestedreportinterval)).into()
        }
        unsafe extern "system" fn StopListeningForReports<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopListeningForReports().into()
        }
        unsafe extern "system" fn Status<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInterval<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pmilliseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(::core::mem::transmute_copy(&millisecondsrequested)).into()
        }
        unsafe extern "system" fn DesiredAccuracy<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesiredaccuracy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredAccuracy(::core::mem::transmute_copy(&desiredaccuracy)).into()
        }
        unsafe extern "system" fn RequestPermissions<Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestPermissions(::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ListenForReports: ListenForReports::<Impl, IMPL_OFFSET>,
            StopListeningForReports: StopListeningForReports::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            DesiredAccuracy: DesiredAccuracy::<Impl, IMPL_OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Impl, IMPL_OFFSET>,
            RequestPermissions: RequestPermissions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationReportFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ICivicAddressReportFactoryEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ICivicAddressReportFactoryEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ICivicAddressReportFactoryEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ICivicAddressReportFactoryEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ICivicAddressReportFactoryEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ILatLongReportFactoryEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ILatLongReportFactoryEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ILatLongReportFactoryEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ILatLongReportFactoryEvents_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ILatLongReportFactoryEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
