#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICivicAddressReport_Impl: Sized + ILocationReport_Impl {
    fn GetAddressLine1(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAddressLine2(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCity(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetStateProvince(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPostalCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetCountryRegion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDetailLevel(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for ICivicAddressReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ICivicAddressReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>() -> ICivicAddressReport_Vtbl {
        unsafe extern "system" fn GetAddressLine1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAddressLine1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraddress1, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAddressLine2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstraddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAddressLine2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstraddress2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateProvince<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStateProvince() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstateprovince, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostalCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpostalcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCountryRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCountryRegion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcountryregion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDetailLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDetailLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdetaillevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ILocationReport_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAddressLine1: GetAddressLine1::<Identity, Impl, OFFSET>,
            GetAddressLine2: GetAddressLine2::<Identity, Impl, OFFSET>,
            GetCity: GetCity::<Identity, Impl, OFFSET>,
            GetStateProvince: GetStateProvince::<Identity, Impl, OFFSET>,
            GetPostalCode: GetPostalCode::<Identity, Impl, OFFSET>,
            GetCountryRegion: GetCountryRegion::<Identity, Impl, OFFSET>,
            GetDetailLevel: GetDetailLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICivicAddressReport as ::windows::core::Interface>::IID || iid == &<ILocationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICivicAddressReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl + ILocationReportFactory_Impl {
    fn CivicAddressReport(&self) -> ::windows::core::Result<IDispCivicAddressReport>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICivicAddressReportFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICivicAddressReportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReportFactory_Impl, const OFFSET: isize>() -> ICivicAddressReportFactory_Vtbl {
        unsafe extern "system" fn CivicAddressReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICivicAddressReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CivicAddressReport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, Impl, OFFSET>(), CivicAddressReport: CivicAddressReport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICivicAddressReportFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ILocationReportFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDefaultLocation_Impl: Sized {
    fn SetReport(&self, reporttype: *const ::windows::core::GUID, plocationreport: &::core::option::Option<ILocationReport>) -> ::windows::core::Result<()>;
    fn GetReport(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<ILocationReport>;
}
impl ::windows::core::RuntimeName for IDefaultLocation {}
impl IDefaultLocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: isize>() -> IDefaultLocation_Vtbl {
        unsafe extern "system" fn SetReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReport(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute(&plocationreport)).into()
        }
        unsafe extern "system" fn GetReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReport(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplocationreport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetReport: SetReport::<Identity, Impl, OFFSET>,
            GetReport: GetReport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDispCivicAddressReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressLine1(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddressLine2(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn City(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StateProvince(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PostalCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CountryRegion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DetailLevel(&self) -> ::windows::core::Result<u32>;
    fn Timestamp(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDispCivicAddressReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDispCivicAddressReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>() -> IDispCivicAddressReport_Vtbl {
        unsafe extern "system" fn AddressLine1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress1: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddressLine1() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddress1, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressLine2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress2: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddressLine2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddress2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.City() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcity, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateProvince<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstateprovince: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StateProvince() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstateprovince, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppostalcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppostalcode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountryregion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CountryRegion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcountryregion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetailLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DetailLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdetaillevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddressLine1: AddressLine1::<Identity, Impl, OFFSET>,
            AddressLine2: AddressLine2::<Identity, Impl, OFFSET>,
            City: City::<Identity, Impl, OFFSET>,
            StateProvince: StateProvince::<Identity, Impl, OFFSET>,
            PostalCode: PostalCode::<Identity, Impl, OFFSET>,
            CountryRegion: CountryRegion::<Identity, Impl, OFFSET>,
            DetailLevel: DetailLevel::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispCivicAddressReport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDispLatLongReport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Latitude(&self) -> ::windows::core::Result<f64>;
    fn Longitude(&self) -> ::windows::core::Result<f64>;
    fn ErrorRadius(&self) -> ::windows::core::Result<f64>;
    fn Altitude(&self) -> ::windows::core::Result<f64>;
    fn AltitudeError(&self) -> ::windows::core::Result<f64>;
    fn Timestamp(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDispLatLongReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDispLatLongReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>() -> IDispLatLongReport_Vtbl {
        unsafe extern "system" fn Latitude<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Latitude() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Longitude<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Longitude() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorRadius<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ErrorRadius() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Altitude<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Altitude() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AltitudeError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Latitude: Latitude::<Identity, Impl, OFFSET>,
            Longitude: Longitude::<Identity, Impl, OFFSET>,
            ErrorRadius: ErrorRadius::<Identity, Impl, OFFSET>,
            Altitude: Altitude::<Identity, Impl, OFFSET>,
            AltitudeError: AltitudeError::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispLatLongReport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ILatLongReport_Impl: Sized + ILocationReport_Impl {
    fn GetLatitude(&self) -> ::windows::core::Result<f64>;
    fn GetLongitude(&self) -> ::windows::core::Result<f64>;
    fn GetErrorRadius(&self) -> ::windows::core::Result<f64>;
    fn GetAltitude(&self) -> ::windows::core::Result<f64>;
    fn GetAltitudeError(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for ILatLongReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ILatLongReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>() -> ILatLongReport_Vtbl {
        unsafe extern "system" fn GetLatitude<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLatitude() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(platitude, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLongitude<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plongitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLongitude() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plongitude, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorRadius<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, perrorradius: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorRadius() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrorradius, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitude<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paltitude: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAltitude() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paltitude, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAltitudeError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paltitudeerror: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAltitudeError() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paltitudeerror, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ILocationReport_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLatitude: GetLatitude::<Identity, Impl, OFFSET>,
            GetLongitude: GetLongitude::<Identity, Impl, OFFSET>,
            GetErrorRadius: GetErrorRadius::<Identity, Impl, OFFSET>,
            GetAltitude: GetAltitude::<Identity, Impl, OFFSET>,
            GetAltitudeError: GetAltitudeError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILatLongReport as ::windows::core::Interface>::IID || iid == &<ILocationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILatLongReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl + ILocationReportFactory_Impl {
    fn LatLongReport(&self) -> ::windows::core::Result<IDispLatLongReport>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ILatLongReportFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILatLongReportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReportFactory_Impl, const OFFSET: isize>() -> ILatLongReportFactory_Vtbl {
        unsafe extern "system" fn LatLongReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILatLongReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LatLongReport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, Impl, OFFSET>(), LatLongReport: LatLongReport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILatLongReportFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ILocationReportFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
pub trait ILocation_Impl: Sized {
    fn RegisterForReport(&self, pevents: &::core::option::Option<ILocationEvents>, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::Result<()>;
    fn UnregisterForReport(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetReport(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<ILocationReport>;
    fn GetReportStatus(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<LOCATION_REPORT_STATUS>;
    fn GetReportInterval(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::Result<()>;
    fn GetDesiredAccuracy(&self, reporttype: *const ::windows::core::GUID) -> ::windows::core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY>;
    fn SetDesiredAccuracy(&self, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::Result<()>;
    fn RequestPermissions(&self, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
impl ::windows::core::RuntimeName for ILocation {}
#[cfg(all(feature = "Win32_Devices_Sensors", feature = "Win32_Foundation"))]
impl ILocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>() -> ILocation_Vtbl {
        unsafe extern "system" fn RegisterForReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, dwrequestedreportinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterForReport(::core::mem::transmute(&pevents), ::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&dwrequestedreportinterval)).into()
        }
        unsafe extern "system" fn UnregisterForReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterForReport(::core::mem::transmute_copy(&reporttype)).into()
        }
        unsafe extern "system" fn GetReport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pplocationreport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReport(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplocationreport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReportStatus(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReportInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pmilliseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReportInterval(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmilliseconds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, millisecondsrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReportInterval(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&millisecondsrequested)).into()
        }
        unsafe extern "system" fn GetDesiredAccuracy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDesiredAccuracy(::core::mem::transmute_copy(&reporttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesiredaccuracy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredAccuracy(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&desiredaccuracy)).into()
        }
        unsafe extern "system" fn RequestPermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows::core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestPermissions(::core::mem::transmute_copy(&hparent), ::core::mem::transmute_copy(&preporttypes), ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&fmodal)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterForReport: RegisterForReport::<Identity, Impl, OFFSET>,
            UnregisterForReport: UnregisterForReport::<Identity, Impl, OFFSET>,
            GetReport: GetReport::<Identity, Impl, OFFSET>,
            GetReportStatus: GetReportStatus::<Identity, Impl, OFFSET>,
            GetReportInterval: GetReportInterval::<Identity, Impl, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, Impl, OFFSET>,
            GetDesiredAccuracy: GetDesiredAccuracy::<Identity, Impl, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocation as ::windows::core::Interface>::IID
    }
}
pub trait ILocationEvents_Impl: Sized {
    fn OnLocationChanged(&self, reporttype: *const ::windows::core::GUID, plocationreport: &::core::option::Option<ILocationReport>) -> ::windows::core::Result<()>;
    fn OnStatusChanged(&self, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILocationEvents {}
impl ILocationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: isize>() -> ILocationEvents_Vtbl {
        unsafe extern "system" fn OnLocationChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, plocationreport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLocationChanged(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute(&plocationreport)).into()
        }
        unsafe extern "system" fn OnStatusChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reporttype: *const ::windows::core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatusChanged(::core::mem::transmute_copy(&reporttype), ::core::mem::transmute_copy(&newstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnLocationChanged: OnLocationChanged::<Identity, Impl, OFFSET>,
            OnStatusChanged: OnStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationEvents as ::windows::core::Interface>::IID
    }
}
pub trait ILocationPower_Impl: Sized {
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILocationPower {}
impl ILocationPower_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: isize>() -> ILocationPower_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect().into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationPower_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationPower as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ILocationReport_Impl: Sized {
    fn GetSensorID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for ILocationReport {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ILocationReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>() -> ILocationReport_Vtbl {
        unsafe extern "system" fn GetSensorID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSensorID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psensorid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcreationtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSensorID: GetSensorID::<Identity, Impl, OFFSET>,
            GetTimestamp: GetTimestamp::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILocationReportFactory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows::core::Result<()>;
    fn StopListeningForReports(&self) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<u32>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows::core::Result<()>;
    fn DesiredAccuracy(&self) -> ::windows::core::Result<u32>;
    fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows::core::Result<()>;
    fn RequestPermissions(&self, hwnd: *const u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ILocationReportFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILocationReportFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>() -> ILocationReportFactory_Vtbl {
        unsafe extern "system" fn ListenForReports<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ListenForReports(::core::mem::transmute_copy(&requestedreportinterval)).into()
        }
        unsafe extern "system" fn StopListeningForReports<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopListeningForReports().into()
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmilliseconds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReportInterval(::core::mem::transmute_copy(&millisecondsrequested)).into()
        }
        unsafe extern "system" fn DesiredAccuracy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DesiredAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdesiredaccuracy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDesiredAccuracy(::core::mem::transmute_copy(&desiredaccuracy)).into()
        }
        unsafe extern "system" fn RequestPermissions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestPermissions(::core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ListenForReports: ListenForReports::<Identity, Impl, OFFSET>,
            StopListeningForReports: StopListeningForReports::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ReportInterval: ReportInterval::<Identity, Impl, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, Impl, OFFSET>,
            DesiredAccuracy: DesiredAccuracy::<Identity, Impl, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationReportFactory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ICivicAddressReportFactoryEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _ICivicAddressReportFactoryEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ICivicAddressReportFactoryEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _ICivicAddressReportFactoryEvents_Impl, const OFFSET: isize>() -> _ICivicAddressReportFactoryEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ICivicAddressReportFactoryEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ILatLongReportFactoryEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for _ILatLongReportFactoryEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ILatLongReportFactoryEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: _ILatLongReportFactoryEvents_Impl, const OFFSET: isize>() -> _ILatLongReportFactoryEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ILatLongReportFactoryEvents as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
