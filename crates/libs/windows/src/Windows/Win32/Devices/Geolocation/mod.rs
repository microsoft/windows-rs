#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICivicAddressReport(::windows_core::IUnknown);
impl ICivicAddressReport {
    pub unsafe fn GetSensorID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSensorID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTimestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), pkey, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAddressLine1(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAddressLine1)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAddressLine2(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAddressLine2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCity(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStateProvince(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStateProvince)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPostalCode(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPostalCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCountryRegion(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCountryRegion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDetailLevel(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDetailLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ICivicAddressReport, ::windows_core::IUnknown, ILocationReport);
unsafe impl ::windows_core::Interface for ICivicAddressReport {
    type Vtable = ICivicAddressReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICivicAddressReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0b19f70_4adf_445d_87f2_cad8fd711792);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReport_Vtbl {
    pub base__: ILocationReport_Vtbl,
    pub GetAddressLine1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstraddress1: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetAddressLine2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstraddress2: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetCity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetStateProvince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstateprovince: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpostalcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetCountryRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcountryregion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetDetailLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICivicAddressReportFactory(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICivicAddressReportFactory {
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ListenForReports)(::windows_core::Interface::as_raw(self), requestedreportinterval).ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopListeningForReports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReportInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetReportInterval)(::windows_core::Interface::as_raw(self), millisecondsrequested).ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DesiredAccuracy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDesiredAccuracy)(::windows_core::Interface::as_raw(self), desiredaccuracy).ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RequestPermissions)(::windows_core::Interface::as_raw(self), hwnd).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CivicAddressReport(&self) -> ::windows_core::Result<IDispCivicAddressReport> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CivicAddressReport)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ICivicAddressReportFactory, ::windows_core::IUnknown, super::super::System::Com::IDispatch, ILocationReportFactory);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICivicAddressReportFactory {
    type Vtable = ICivicAddressReportFactory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ICivicAddressReportFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf773b93_c64f_4bee_beb2_67c0b8df66e0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CivicAddressReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CivicAddressReport: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDefaultLocation(::windows_core::IUnknown);
impl IDefaultLocation {
    pub unsafe fn SetReport<P0>(&self, reporttype: *const ::windows_core::GUID, plocationreport: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ILocationReport>,
    {
        (::windows_core::Interface::vtable(self).SetReport)(::windows_core::Interface::as_raw(self), reporttype, plocationreport.into_param().abi()).ok()
    }
    pub unsafe fn GetReport(&self, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<ILocationReport> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReport)(::windows_core::Interface::as_raw(self), reporttype, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDefaultLocation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDefaultLocation {
    type Vtable = IDefaultLocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDefaultLocation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa65af77e_969a_4a2e_8aca_33bb7cbb1235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultLocation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, plocationreport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pplocationreport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispCivicAddressReport(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDispCivicAddressReport {
    pub unsafe fn AddressLine1(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddressLine1)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddressLine2(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddressLine2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn City(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).City)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StateProvince(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StateProvince)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PostalCode(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PostalCode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CountryRegion(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CountryRegion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DetailLevel(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DetailLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Timestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDispCivicAddressReport, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDispCivicAddressReport {
    type Vtable = IDispCivicAddressReport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDispCivicAddressReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16ff1a34_9e30_42c3_b44d_e22513b5767a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispCivicAddressReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddressLine1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress1: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddressLine2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddress2: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcity: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StateProvince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstateprovince: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppostalcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CountryRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcountryregion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DetailLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdetaillevel: *mut u32) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDispLatLongReport(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDispLatLongReport {
    pub unsafe fn Latitude(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Latitude)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Longitude(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Longitude)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ErrorRadius(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ErrorRadius)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Altitude(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Altitude)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AltitudeError(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AltitudeError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Timestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IDispLatLongReport, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDispLatLongReport {
    type Vtable = IDispLatLongReport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IDispLatLongReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ae32723_389b_4a11_9957_5bdd48fc9617);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispLatLongReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Latitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Longitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub ErrorRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Altitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub AltitudeError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILatLongReport(::windows_core::IUnknown);
impl ILatLongReport {
    pub unsafe fn GetSensorID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSensorID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTimestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetValue)(::windows_core::Interface::as_raw(self), pkey, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLatitude(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLatitude)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLongitude(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLongitude)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorRadius(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorRadius)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAltitude(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAltitude)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAltitudeError(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAltitudeError)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ILatLongReport, ::windows_core::IUnknown, ILocationReport);
unsafe impl ::windows_core::Interface for ILatLongReport {
    type Vtable = ILatLongReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILatLongReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fed806d_0ef8_4f07_80ac_36a0beae3134);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReport_Vtbl {
    pub base__: ILocationReport_Vtbl,
    pub GetLatitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, platitude: *mut f64) -> ::windows_core::HRESULT,
    pub GetLongitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plongitude: *mut f64) -> ::windows_core::HRESULT,
    pub GetErrorRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorradius: *mut f64) -> ::windows_core::HRESULT,
    pub GetAltitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paltitude: *mut f64) -> ::windows_core::HRESULT,
    pub GetAltitudeError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paltitudeerror: *mut f64) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILatLongReportFactory(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ILatLongReportFactory {
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ListenForReports)(::windows_core::Interface::as_raw(self), requestedreportinterval).ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.StopListeningForReports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReportInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetReportInterval)(::windows_core::Interface::as_raw(self), millisecondsrequested).ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DesiredAccuracy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDesiredAccuracy)(::windows_core::Interface::as_raw(self), desiredaccuracy).ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RequestPermissions)(::windows_core::Interface::as_raw(self), hwnd).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LatLongReport(&self) -> ::windows_core::Result<IDispLatLongReport> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LatLongReport)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ILatLongReportFactory, ::windows_core::IUnknown, super::super::System::Com::IDispatch, ILocationReportFactory);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ILatLongReportFactory {
    type Vtable = ILatLongReportFactory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ILatLongReportFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f0804cb_b114_447d_83dd_390174ebb082);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LatLongReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LatLongReport: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocation(::windows_core::IUnknown);
impl ILocation {
    pub unsafe fn RegisterForReport<P0>(&self, pevents: P0, reporttype: *const ::windows_core::GUID, dwrequestedreportinterval: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ILocationEvents>,
    {
        (::windows_core::Interface::vtable(self).RegisterForReport)(::windows_core::Interface::as_raw(self), pevents.into_param().abi(), reporttype, dwrequestedreportinterval).ok()
    }
    pub unsafe fn UnregisterForReport(&self, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterForReport)(::windows_core::Interface::as_raw(self), reporttype).ok()
    }
    pub unsafe fn GetReport(&self, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<ILocationReport> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReport)(::windows_core::Interface::as_raw(self), reporttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReportStatus(&self, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<LOCATION_REPORT_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReportStatus)(::windows_core::Interface::as_raw(self), reporttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReportInterval(&self, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetReportInterval)(::windows_core::Interface::as_raw(self), reporttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReportInterval(&self, reporttype: *const ::windows_core::GUID, millisecondsrequested: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReportInterval)(::windows_core::Interface::as_raw(self), reporttype, millisecondsrequested).ok()
    }
    #[doc = "Required features: `\"Win32_Devices_Sensors\"`"]
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn GetDesiredAccuracy(&self, reporttype: *const ::windows_core::GUID) -> ::windows_core::Result<super::Sensors::LOCATION_DESIRED_ACCURACY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDesiredAccuracy)(::windows_core::Interface::as_raw(self), reporttype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Devices_Sensors\"`"]
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub unsafe fn SetDesiredAccuracy(&self, reporttype: *const ::windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDesiredAccuracy)(::windows_core::Interface::as_raw(self), reporttype, desiredaccuracy).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestPermissions<P0, P1>(&self, hparent: P0, preporttypes: &[::windows_core::GUID], fmodal: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).RequestPermissions)(::windows_core::Interface::as_raw(self), hparent.into_param().abi(), ::core::mem::transmute(preporttypes.as_ptr()), preporttypes.len().try_into().unwrap(), fmodal.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ILocation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocation {
    type Vtable = ILocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILocation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2ece69_56d9_4f28_b525_de1b0ee44237);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterForReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, dwrequestedreportinterval: u32) -> ::windows_core::HRESULT,
    pub UnregisterForReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pplocationreport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetReportStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> ::windows_core::HRESULT,
    pub GetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pmilliseconds: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, millisecondsrequested: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub GetDesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, pdesiredaccuracy: *mut super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))]
    GetDesiredAccuracy: usize,
    #[cfg(feature = "Win32_Devices_Sensors")]
    pub SetDesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, desiredaccuracy: super::Sensors::LOCATION_DESIRED_ACCURACY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Sensors"))]
    SetDesiredAccuracy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, preporttypes: *const ::windows_core::GUID, count: u32, fmodal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestPermissions: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocationEvents(::windows_core::IUnknown);
impl ILocationEvents {
    pub unsafe fn OnLocationChanged<P0>(&self, reporttype: *const ::windows_core::GUID, plocationreport: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ILocationReport>,
    {
        (::windows_core::Interface::vtable(self).OnLocationChanged)(::windows_core::Interface::as_raw(self), reporttype, plocationreport.into_param().abi()).ok()
    }
    pub unsafe fn OnStatusChanged(&self, reporttype: *const ::windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnStatusChanged)(::windows_core::Interface::as_raw(self), reporttype, newstatus).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ILocationEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocationEvents {
    type Vtable = ILocationEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILocationEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcae02bbf_798b_4508_a207_35a7906dc73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnLocationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, plocationreport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *const ::windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocationPower(::windows_core::IUnknown);
impl ILocationPower {
    pub unsafe fn Connect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Connect)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Disconnect)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ILocationPower, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocationPower {
    type Vtable = ILocationPower_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILocationPower {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x193e7729_ab6b_4b12_8617_7596e1bb191c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPower_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocationReport(::windows_core::IUnknown);
impl ILocationReport {
    pub unsafe fn GetSensorID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSensorID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTimestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), pkey, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ILocationReport, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocationReport {
    type Vtable = ILocationReport_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILocationReport {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8b7f7ee_75d0_4db9_b62d_7a0f369ca456);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReport_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSensorID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psensorid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcreationtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimestamp: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocationReportFactory(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ILocationReportFactory {
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ListenForReports)(::windows_core::Interface::as_raw(self), requestedreportinterval).ok()
    }
    pub unsafe fn StopListeningForReports(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).StopListeningForReports)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Status(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReportInterval)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReportInterval)(::windows_core::Interface::as_raw(self), millisecondsrequested).ok()
    }
    pub unsafe fn DesiredAccuracy(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DesiredAccuracy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDesiredAccuracy)(::windows_core::Interface::as_raw(self), desiredaccuracy).ok()
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestPermissions)(::windows_core::Interface::as_raw(self), hwnd).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ILocationReportFactory, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ILocationReportFactory {
    type Vtable = ILocationReportFactory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ILocationReportFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2daec322_90b2_47e4_bb08_0da841935a6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReportFactory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ListenForReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedreportinterval: u32) -> ::windows_core::HRESULT,
    pub StopListeningForReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmilliseconds: *mut u32) -> ::windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, millisecondsrequested: u32) -> ::windows_core::HRESULT,
    pub DesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdesiredaccuracy: *mut u32) -> ::windows_core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredaccuracy: u32) -> ::windows_core::HRESULT,
    pub RequestPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *const u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct _ICivicAddressReportFactoryEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _ICivicAddressReportFactoryEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(_ICivicAddressReportFactoryEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for _ICivicAddressReportFactoryEvents {
    type Vtable = _ICivicAddressReportFactoryEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for _ICivicAddressReportFactoryEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc96039ff_72ec_4617_89bd_84d88bedc722);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ICivicAddressReportFactoryEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct _ILatLongReportFactoryEvents(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _ILatLongReportFactoryEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(_ILatLongReportFactoryEvents, ::windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for _ILatLongReportFactoryEvents {
    type Vtable = _ILatLongReportFactoryEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for _ILatLongReportFactoryEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16ee6cb7_ab3c_424b_849f_269be551fcbc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ILatLongReportFactoryEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
pub const BREADCRUMBING_UNSUPPORTED: u32 = 0u32;
pub const BREADCRUMBING_VERSION_1: u32 = 1u32;
pub const CivicAddressReport: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd39e7bdd_7d05_46b8_8721_80cf035f57d7);
pub const CivicAddressReportFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a11f42c_3e81_4ad4_9cbe_45579d89671a);
pub const DefaultLocation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b7fbfe0_5cd7_494a_af8c_283a65707506);
pub const DispCivicAddressReport: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c596aec_8544_4082_ba9f_eb0a7d8e65c6);
pub const DispLatLongReport: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a7c3277_8f84_4636_95b2_ebb5507ff77e);
pub const GNSS_AGNSSFORMAT_LTO: u32 = 4u32;
pub const GNSS_AGNSSFORMAT_XTRA1: u32 = 1u32;
pub const GNSS_AGNSSFORMAT_XTRA2: u32 = 2u32;
pub const GNSS_AGNSSFORMAT_XTRA3: u32 = 8u32;
pub const GNSS_AGNSSFORMAT_XTRA3_1: u32 = 16u32;
pub const GNSS_AGNSSFORMAT_XTRA3_2: u32 = 32u32;
pub const GNSS_AGNSSFORMAT_XTRA_INT: u32 = 64u32;
pub const GNSS_AGNSS_BlobInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(3i32);
pub const GNSS_AGNSS_PositionInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(2i32);
pub const GNSS_AGNSS_TimeInjection: GNSS_AGNSS_REQUEST_TYPE = GNSS_AGNSS_REQUEST_TYPE(1i32);
pub const GNSS_ClearAgnssData: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(10i32);
pub const GNSS_CustomCommand: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(256i32);
pub const GNSS_DRIVER_VERSION_1: u32 = 1u32;
pub const GNSS_DRIVER_VERSION_2: u32 = 2u32;
pub const GNSS_DRIVER_VERSION_3: u32 = 3u32;
pub const GNSS_DRIVER_VERSION_4: u32 = 4u32;
pub const GNSS_DRIVER_VERSION_5: u32 = 5u32;
pub const GNSS_DRIVER_VERSION_6: u32 = 6u32;
pub const GNSS_Event_BreadcrumbAlertEvent: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(17i32);
pub const GNSS_Event_Custom: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(32768i32);
pub const GNSS_Event_DriverRequest: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(16i32);
pub const GNSS_Event_Error: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(3i32);
pub const GNSS_Event_FixAvailable: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(1i32);
pub const GNSS_Event_FixAvailable_2: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(18i32);
pub const GNSS_Event_GeofenceAlertData: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(14i32);
pub const GNSS_Event_GeofencesTrackingStatus: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(15i32);
pub const GNSS_Event_NiRequest: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(12i32);
pub const GNSS_Event_NmeaData: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(13i32);
pub const GNSS_Event_RequireAgnss: GNSS_EVENT_TYPE = GNSS_EVENT_TYPE(2i32);
pub const GNSS_FIXDETAIL_ACCURACY: u32 = 2u32;
pub const GNSS_FIXDETAIL_BASIC: u32 = 1u32;
pub const GNSS_FIXDETAIL_SATELLITE: u32 = 4u32;
pub const GNSS_FixSession_ContinuousTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(3i32);
pub const GNSS_FixSession_DistanceTracking: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(2i32);
pub const GNSS_FixSession_LKG: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(4i32);
pub const GNSS_FixSession_SingleShot: GNSS_FIXSESSIONTYPE = GNSS_FIXSESSIONTYPE(1i32);
pub const GNSS_ForceOperationMode: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(4i32);
pub const GNSS_ForceSatelliteSystem: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(3i32);
pub const GNSS_GEOFENCESUPPORT_CIRCLE: u32 = 2u32;
pub const GNSS_GEOFENCESUPPORT_SUPPORTED: u32 = 1u32;
pub const GNSS_GeoRegion_Circle: GNSS_GEOREGIONTYPE = GNSS_GEOREGIONTYPE(1i32);
pub const GNSS_GeofenceState_Entered: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(1i32);
pub const GNSS_GeofenceState_Exited: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(2i32);
pub const GNSS_GeofenceState_Unknown: GNSS_GEOFENCE_STATE = GNSS_GEOFENCE_STATE(0i32);
pub const GNSS_MAXSATELLITE: u32 = 64u32;
pub const GNSS_NI_CP: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(2i32);
pub const GNSS_NI_NoNotifyNoVerify: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(1i32);
pub const GNSS_NI_NotifyOnly: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(2i32);
pub const GNSS_NI_NotifyVerifyDefaultAllow: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(3i32);
pub const GNSS_NI_NotifyVerifyDefaultNotAllow: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(4i32);
pub const GNSS_NI_PrivacyOverride: GNSS_NI_NOTIFICATION_TYPE = GNSS_NI_NOTIFICATION_TYPE(5i32);
pub const GNSS_NI_Request_AreaTrigger: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(2i32);
pub const GNSS_NI_Request_SingleShot: GNSS_NI_REQUEST_TYPE = GNSS_NI_REQUEST_TYPE(1i32);
pub const GNSS_NI_SUPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(1i32);
pub const GNSS_NI_V2UPL: GNSS_NI_PLANE_TYPE = GNSS_NI_PLANE_TYPE(3i32);
pub const GNSS_NMEALOGGING_ALL: u32 = 255u32;
pub const GNSS_NMEALOGGING_NONE: u32 = 0u32;
pub const GNSS_Ni_UserResponseAccept: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(1i32);
pub const GNSS_Ni_UserResponseDeny: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(2i32);
pub const GNSS_Ni_UserResponseTimeout: GNSS_NI_USER_RESPONSE = GNSS_NI_USER_RESPONSE(3i32);
pub const GNSS_OPERMODE_AFLT: u32 = 16u32;
pub const GNSS_OPERMODE_ANY: u32 = 0u32;
pub const GNSS_OPERMODE_CELLID: u32 = 8u32;
pub const GNSS_OPERMODE_MSA: u32 = 1u32;
pub const GNSS_OPERMODE_MSB: u32 = 2u32;
pub const GNSS_OPERMODE_MSS: u32 = 4u32;
pub const GNSS_OPERMODE_OTDOA: u32 = 32u32;
pub const GNSS_ResetEngine: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(9i32);
pub const GNSS_ResetGeofencesTracking: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(16i32);
pub const GNSS_SATELLITE_ANY: u32 = 0u32;
pub const GNSS_SATELLITE_BEIDOU: u32 = 4u32;
pub const GNSS_SATELLITE_GALILEO: u32 = 8u32;
pub const GNSS_SATELLITE_GLONASS: u32 = 2u32;
pub const GNSS_SATELLITE_GPS: u32 = 1u32;
pub const GNSS_SetLocationNIRequestAllowed: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(2i32);
pub const GNSS_SetLocationServiceEnabled: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(1i32);
pub const GNSS_SetNMEALogging: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(13i32);
pub const GNSS_SetNiTimeoutInterval: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(15i32);
pub const GNSS_SetSuplVersion: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(12i32);
pub const GNSS_SetSuplVersion2: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(17i32);
pub const GNSS_SetUplServerAccessInterval: GNSS_DRIVERCOMMAND_TYPE = GNSS_DRIVERCOMMAND_TYPE(14i32);
pub const GNSS_Supl_Cert_Delete: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(2i32);
pub const GNSS_Supl_Cert_Inject: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(1i32);
pub const GNSS_Supl_Cert_Purge: GNSS_SUPL_CERT_ACTION = GNSS_SUPL_CERT_ACTION(3i32);
pub const GUID_DEVINTERFACE_GNSS: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3336e5e4_018a_4669_84c5_bd05f3bd368b);
pub const IOCTL_GNSS_CONFIG_SUPL_CERT: u32 = 2228488u32;
pub const IOCTL_GNSS_CREATE_GEOFENCE: u32 = 2228544u32;
pub const IOCTL_GNSS_DELETE_GEOFENCE: u32 = 2228548u32;
pub const IOCTL_GNSS_EXECUTE_CWTEST: u32 = 2228496u32;
pub const IOCTL_GNSS_EXECUTE_SELFTEST: u32 = 2228500u32;
pub const IOCTL_GNSS_GET_CHIPSETINFO: u32 = 2228504u32;
pub const IOCTL_GNSS_GET_DEVICE_CAPABILITY: u32 = 2228232u32;
pub const IOCTL_GNSS_GET_FIXDATA: u32 = 2228300u32;
pub const IOCTL_GNSS_INJECT_AGNSS: u32 = 2228352u32;
pub const IOCTL_GNSS_LISTEN_AGNSS: u32 = 2228416u32;
pub const IOCTL_GNSS_LISTEN_BREADCRUMBING_ALERT: u32 = 2228680u32;
pub const IOCTL_GNSS_LISTEN_DRIVER_REQUEST: u32 = 2228608u32;
pub const IOCTL_GNSS_LISTEN_ERROR: u32 = 2228420u32;
pub const IOCTL_GNSS_LISTEN_GEOFENCES_TRACKINGSTATUS: u32 = 2228556u32;
pub const IOCTL_GNSS_LISTEN_GEOFENCE_ALERT: u32 = 2228552u32;
pub const IOCTL_GNSS_LISTEN_NI: u32 = 2228480u32;
pub const IOCTL_GNSS_LISTEN_NMEA: u32 = 2228508u32;
pub const IOCTL_GNSS_MODIFY_FIXSESSION: u32 = 2228292u32;
pub const IOCTL_GNSS_POP_BREADCRUMBS: u32 = 2228684u32;
pub const IOCTL_GNSS_RESPOND_NI: u32 = 2228492u32;
pub const IOCTL_GNSS_SEND_DRIVERCOMMAND: u32 = 2228236u32;
pub const IOCTL_GNSS_SEND_PLATFORM_CAPABILITY: u32 = 2228228u32;
pub const IOCTL_GNSS_SET_SUPL_HSLP: u32 = 2228484u32;
pub const IOCTL_GNSS_SET_V2UPL_CONFIG: u32 = 2228512u32;
pub const IOCTL_GNSS_START_BREADCRUMBING: u32 = 2228672u32;
pub const IOCTL_GNSS_START_FIXSESSION: u32 = 2228288u32;
pub const IOCTL_GNSS_STOP_BREADCRUMBING: u32 = 2228676u32;
pub const IOCTL_GNSS_STOP_FIXSESSION: u32 = 2228296u32;
pub const LOCATION_API_VERSION: u32 = 1u32;
pub const LatLongReport: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed81c073_1f84_4ca8_a161_183c776bc651);
pub const LatLongReportFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dcc3cc8_8609_4863_bad4_03601f4c65e8);
pub const Location: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5b8e079_ee6d_4e33_a438_c87f2e959254);
pub const MAX_SERVER_URL_NAME: u32 = 260u32;
pub const MIN_BREADCRUMBS_SUPPORTED: u32 = 120u32;
pub const MIN_GEOFENCES_REQUIRED: u32 = 100u32;
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(2i32);
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(1i32);
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(3i32);
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(0i32);
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = LOCATION_REPORT_STATUS(4i32);
pub const SUPL_CONFIG_DATA: GNSS_DRIVER_REQUEST = GNSS_DRIVER_REQUEST(1i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_AGNSS_REQUEST_TYPE(pub i32);
impl ::core::marker::Copy for GNSS_AGNSS_REQUEST_TYPE {}
impl ::core::clone::Clone for GNSS_AGNSS_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_AGNSS_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_AGNSS_REQUEST_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_AGNSS_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_AGNSS_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_DRIVERCOMMAND_TYPE(pub i32);
impl ::core::marker::Copy for GNSS_DRIVERCOMMAND_TYPE {}
impl ::core::clone::Clone for GNSS_DRIVERCOMMAND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_DRIVERCOMMAND_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_DRIVERCOMMAND_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_DRIVERCOMMAND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_DRIVERCOMMAND_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_DRIVER_REQUEST(pub i32);
impl ::core::marker::Copy for GNSS_DRIVER_REQUEST {}
impl ::core::clone::Clone for GNSS_DRIVER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_DRIVER_REQUEST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_DRIVER_REQUEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_DRIVER_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_DRIVER_REQUEST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_EVENT_TYPE(pub i32);
impl ::core::marker::Copy for GNSS_EVENT_TYPE {}
impl ::core::clone::Clone for GNSS_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_EVENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_FIXSESSIONTYPE(pub i32);
impl ::core::marker::Copy for GNSS_FIXSESSIONTYPE {}
impl ::core::clone::Clone for GNSS_FIXSESSIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_FIXSESSIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_FIXSESSIONTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_FIXSESSIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_FIXSESSIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_GEOFENCE_STATE(pub i32);
impl ::core::marker::Copy for GNSS_GEOFENCE_STATE {}
impl ::core::clone::Clone for GNSS_GEOFENCE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_GEOFENCE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_GEOFENCE_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_GEOFENCE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_GEOFENCE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_GEOREGIONTYPE(pub i32);
impl ::core::marker::Copy for GNSS_GEOREGIONTYPE {}
impl ::core::clone::Clone for GNSS_GEOREGIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_GEOREGIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_GEOREGIONTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_GEOREGIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_GEOREGIONTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_NI_NOTIFICATION_TYPE(pub i32);
impl ::core::marker::Copy for GNSS_NI_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for GNSS_NI_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_NI_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_NI_NOTIFICATION_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_NI_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_NI_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_NI_PLANE_TYPE(pub i32);
impl ::core::marker::Copy for GNSS_NI_PLANE_TYPE {}
impl ::core::clone::Clone for GNSS_NI_PLANE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_NI_PLANE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_NI_PLANE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_NI_PLANE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_NI_PLANE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_NI_REQUEST_TYPE(pub i32);
impl ::core::marker::Copy for GNSS_NI_REQUEST_TYPE {}
impl ::core::clone::Clone for GNSS_NI_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_NI_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_NI_REQUEST_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_NI_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_NI_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_NI_USER_RESPONSE(pub i32);
impl ::core::marker::Copy for GNSS_NI_USER_RESPONSE {}
impl ::core::clone::Clone for GNSS_NI_USER_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_NI_USER_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_NI_USER_RESPONSE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_NI_USER_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_NI_USER_RESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GNSS_SUPL_CERT_ACTION(pub i32);
impl ::core::marker::Copy for GNSS_SUPL_CERT_ACTION {}
impl ::core::clone::Clone for GNSS_SUPL_CERT_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GNSS_SUPL_CERT_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GNSS_SUPL_CERT_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GNSS_SUPL_CERT_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GNSS_SUPL_CERT_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOCATION_REPORT_STATUS(pub i32);
impl ::core::marker::Copy for LOCATION_REPORT_STATUS {}
impl ::core::clone::Clone for LOCATION_REPORT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCATION_REPORT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LOCATION_REPORT_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LOCATION_REPORT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCATION_REPORT_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECT {
    pub Size: u32,
    pub Version: u32,
    pub InjectionType: GNSS_AGNSS_REQUEST_TYPE,
    pub InjectionStatus: super::super::Foundation::NTSTATUS,
    pub InjectionDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_AGNSS_INJECT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_AGNSS_INJECT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_AGNSS_INJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_AGNSS_INJECT_0 {
    pub Time: GNSS_AGNSS_INJECTTIME,
    pub Position: GNSS_AGNSS_INJECTPOSITION,
    pub BlobData: GNSS_AGNSS_INJECTBLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_AGNSS_INJECT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_AGNSS_INJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_AGNSS_INJECTBLOB {
    pub Size: u32,
    pub Version: u32,
    pub BlobOui: u32,
    pub BlobVersion: u32,
    pub AgnssFormat: u32,
    pub BlobSize: u32,
    pub BlobData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_AGNSS_INJECTBLOB {}
impl ::core::clone::Clone for GNSS_AGNSS_INJECTBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_AGNSS_INJECTBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_INJECTBLOB").field("Size", &self.Size).field("Version", &self.Version).field("BlobOui", &self.BlobOui).field("BlobVersion", &self.BlobVersion).field("AgnssFormat", &self.AgnssFormat).field("BlobSize", &self.BlobSize).field("BlobData", &self.BlobData).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_AGNSS_INJECTBLOB {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECTBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.BlobOui == other.BlobOui && self.BlobVersion == other.BlobVersion && self.AgnssFormat == other.AgnssFormat && self.BlobSize == other.BlobSize && self.BlobData == other.BlobData
    }
}
impl ::core::cmp::Eq for GNSS_AGNSS_INJECTBLOB {}
impl ::core::default::Default for GNSS_AGNSS_INJECTBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_AGNSS_INJECTPOSITION {
    pub Size: u32,
    pub Version: u32,
    pub Age: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
}
impl ::core::marker::Copy for GNSS_AGNSS_INJECTPOSITION {}
impl ::core::clone::Clone for GNSS_AGNSS_INJECTPOSITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_AGNSS_INJECTPOSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_INJECTPOSITION").field("Size", &self.Size).field("Version", &self.Version).field("Age", &self.Age).field("BasicData", &self.BasicData).field("AccuracyData", &self.AccuracyData).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_AGNSS_INJECTPOSITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECTPOSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Age == other.Age && self.BasicData == other.BasicData && self.AccuracyData == other.AccuracyData
    }
}
impl ::core::cmp::Eq for GNSS_AGNSS_INJECTPOSITION {}
impl ::core::default::Default for GNSS_AGNSS_INJECTPOSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_AGNSS_INJECTTIME {
    pub Size: u32,
    pub Version: u32,
    pub UtcTime: super::super::Foundation::FILETIME,
    pub TimeUncertainty: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_AGNSS_INJECTTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_AGNSS_INJECTTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_INJECTTIME").field("Size", &self.Size).field("Version", &self.Version).field("UtcTime", &self.UtcTime).field("TimeUncertainty", &self.TimeUncertainty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_AGNSS_INJECTTIME {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_AGNSS_INJECTTIME {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.UtcTime == other.UtcTime && self.TimeUncertainty == other.TimeUncertainty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_AGNSS_INJECTTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_AGNSS_INJECTTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_AGNSS_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestType: GNSS_AGNSS_REQUEST_TYPE,
    pub BlobFormat: u32,
}
impl ::core::marker::Copy for GNSS_AGNSS_REQUEST_PARAM {}
impl ::core::clone::Clone for GNSS_AGNSS_REQUEST_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_AGNSS_REQUEST_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_AGNSS_REQUEST_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("RequestType", &self.RequestType).field("BlobFormat", &self.BlobFormat).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_AGNSS_REQUEST_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_AGNSS_REQUEST_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.RequestType == other.RequestType && self.BlobFormat == other.BlobFormat
    }
}
impl ::core::cmp::Eq for GNSS_AGNSS_REQUEST_PARAM {}
impl ::core::default::Default for GNSS_AGNSS_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_BREADCRUMBING_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_BREADCRUMBING_ALERT_DATA {}
impl ::core::clone::Clone for GNSS_BREADCRUMBING_ALERT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_BREADCRUMBING_ALERT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_BREADCRUMBING_ALERT_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_BREADCRUMBING_ALERT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_BREADCRUMBING_ALERT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_BREADCRUMBING_ALERT_DATA {}
impl ::core::default::Default for GNSS_BREADCRUMBING_ALERT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_BREADCRUMBING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MaximumHorizontalUncertainty: u32,
    pub MinDistanceBetweenFixes: u32,
    pub MaximumErrorTimeoutMs: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_BREADCRUMBING_PARAM {}
impl ::core::clone::Clone for GNSS_BREADCRUMBING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_BREADCRUMBING_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_BREADCRUMBING_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("MaximumHorizontalUncertainty", &self.MaximumHorizontalUncertainty).field("MinDistanceBetweenFixes", &self.MinDistanceBetweenFixes).field("MaximumErrorTimeoutMs", &self.MaximumErrorTimeoutMs).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_BREADCRUMBING_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_BREADCRUMBING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.MaximumHorizontalUncertainty == other.MaximumHorizontalUncertainty && self.MinDistanceBetweenFixes == other.MinDistanceBetweenFixes && self.MaximumErrorTimeoutMs == other.MaximumErrorTimeoutMs && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_BREADCRUMBING_PARAM {}
impl ::core::default::Default for GNSS_BREADCRUMBING_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_LIST {
    pub Size: u32,
    pub Version: u32,
    pub NumCrumbs: u32,
    pub Anonymous: GNSS_BREADCRUMB_LIST_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_BREADCRUMB_LIST {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_BREADCRUMB_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_BREADCRUMB_LIST_0 {
    pub v1: [GNSS_BREADCRUMB_V1; 50],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_LIST_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_BREADCRUMB_LIST_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_BREADCRUMB_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_BREADCRUMB_V1 {
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub Latitude: f64,
    pub Longitude: f64,
    pub HorizontalAccuracy: u32,
    pub Speed: u16,
    pub SpeedAccuracy: u16,
    pub Altitude: i16,
    pub AltitudeAccuracy: u16,
    pub Heading: i16,
    pub HeadingAccuracy: u8,
    pub FixSuccess: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_BREADCRUMB_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_BREADCRUMB_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_BREADCRUMB_V1")
            .field("FixTimeStamp", &self.FixTimeStamp)
            .field("Latitude", &self.Latitude)
            .field("Longitude", &self.Longitude)
            .field("HorizontalAccuracy", &self.HorizontalAccuracy)
            .field("Speed", &self.Speed)
            .field("SpeedAccuracy", &self.SpeedAccuracy)
            .field("Altitude", &self.Altitude)
            .field("AltitudeAccuracy", &self.AltitudeAccuracy)
            .field("Heading", &self.Heading)
            .field("HeadingAccuracy", &self.HeadingAccuracy)
            .field("FixSuccess", &self.FixSuccess)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_BREADCRUMB_V1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_BREADCRUMB_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.FixTimeStamp == other.FixTimeStamp && self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.HorizontalAccuracy == other.HorizontalAccuracy && self.Speed == other.Speed && self.SpeedAccuracy == other.SpeedAccuracy && self.Altitude == other.Altitude && self.AltitudeAccuracy == other.AltitudeAccuracy && self.Heading == other.Heading && self.HeadingAccuracy == other.HeadingAccuracy && self.FixSuccess == other.FixSuccess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_BREADCRUMB_V1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_BREADCRUMB_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_CHIPSETINFO {
    pub Size: u32,
    pub Version: u32,
    pub ManufacturerID: [u16; 25],
    pub HardwareID: [u16; 25],
    pub FirmwareVersion: [u16; 20],
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_CHIPSETINFO {}
impl ::core::clone::Clone for GNSS_CHIPSETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_CHIPSETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CHIPSETINFO").field("Size", &self.Size).field("Version", &self.Version).field("ManufacturerID", &self.ManufacturerID).field("HardwareID", &self.HardwareID).field("FirmwareVersion", &self.FirmwareVersion).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_CHIPSETINFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_CHIPSETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.ManufacturerID == other.ManufacturerID && self.HardwareID == other.HardwareID && self.FirmwareVersion == other.FirmwareVersion && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_CHIPSETINFO {}
impl ::core::default::Default for GNSS_CHIPSETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_CONTINUOUSTRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub PreferredInterval: u32,
}
impl ::core::marker::Copy for GNSS_CONTINUOUSTRACKING_PARAM {}
impl ::core::clone::Clone for GNSS_CONTINUOUSTRACKING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_CONTINUOUSTRACKING_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CONTINUOUSTRACKING_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("PreferredInterval", &self.PreferredInterval).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_CONTINUOUSTRACKING_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_CONTINUOUSTRACKING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.PreferredInterval == other.PreferredInterval
    }
}
impl ::core::cmp::Eq for GNSS_CONTINUOUSTRACKING_PARAM {}
impl ::core::default::Default for GNSS_CONTINUOUSTRACKING_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_CP_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub NotificationText: [u16; 260],
}
impl ::core::marker::Copy for GNSS_CP_NI_INFO {}
impl ::core::clone::Clone for GNSS_CP_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_CP_NI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CP_NI_INFO").field("Size", &self.Size).field("Version", &self.Version).field("RequestorId", &self.RequestorId).field("NotificationText", &self.NotificationText).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_CP_NI_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_CP_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.RequestorId == other.RequestorId && self.NotificationText == other.NotificationText
    }
}
impl ::core::cmp::Eq for GNSS_CP_NI_INFO {}
impl ::core::default::Default for GNSS_CP_NI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_CWTESTDATA {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub SignalToNoiseRatio: f64,
    pub Frequency: f64,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_CWTESTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_CWTESTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_CWTESTDATA").field("Size", &self.Size).field("Version", &self.Version).field("TestResultStatus", &self.TestResultStatus).field("SignalToNoiseRatio", &self.SignalToNoiseRatio).field("Frequency", &self.Frequency).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_CWTESTDATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_CWTESTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.TestResultStatus == other.TestResultStatus && self.SignalToNoiseRatio == other.SignalToNoiseRatio && self.Frequency == other.Frequency && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_CWTESTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_CWTESTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_DEVICE_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportMultipleFixSessions: super::super::Foundation::BOOL,
    pub SupportMultipleAppSessions: super::super::Foundation::BOOL,
    pub RequireAGnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub AgnssFormatPreferred: u32,
    pub SupportDistanceTracking: super::super::Foundation::BOOL,
    pub SupportContinuousTracking: super::super::Foundation::BOOL,
    pub Reserved1: u32,
    pub Reserved2: super::super::Foundation::BOOL,
    pub Reserved3: super::super::Foundation::BOOL,
    pub Reserved4: super::super::Foundation::BOOL,
    pub Reserved5: super::super::Foundation::BOOL,
    pub GeofencingSupport: u32,
    pub Reserved6: super::super::Foundation::BOOL,
    pub Reserved7: super::super::Foundation::BOOL,
    pub SupportCpLocation: super::super::Foundation::BOOL,
    pub SupportUplV2: super::super::Foundation::BOOL,
    pub SupportSuplV1: super::super::Foundation::BOOL,
    pub SupportSuplV2: super::super::Foundation::BOOL,
    pub SupportedSuplVersion: GNSS_SUPL_VERSION,
    pub MaxGeofencesSupported: u32,
    pub SupportMultipleSuplRootCert: super::super::Foundation::BOOL,
    pub GnssBreadCrumbPayloadVersion: u32,
    pub MaxGnssBreadCrumbFixes: u32,
    pub Unused: [u8; 496],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_DEVICE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_DEVICE_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DEVICE_CAPABILITY")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("SupportMultipleFixSessions", &self.SupportMultipleFixSessions)
            .field("SupportMultipleAppSessions", &self.SupportMultipleAppSessions)
            .field("RequireAGnssInjection", &self.RequireAGnssInjection)
            .field("AgnssFormatSupported", &self.AgnssFormatSupported)
            .field("AgnssFormatPreferred", &self.AgnssFormatPreferred)
            .field("SupportDistanceTracking", &self.SupportDistanceTracking)
            .field("SupportContinuousTracking", &self.SupportContinuousTracking)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Reserved4", &self.Reserved4)
            .field("Reserved5", &self.Reserved5)
            .field("GeofencingSupport", &self.GeofencingSupport)
            .field("Reserved6", &self.Reserved6)
            .field("Reserved7", &self.Reserved7)
            .field("SupportCpLocation", &self.SupportCpLocation)
            .field("SupportUplV2", &self.SupportUplV2)
            .field("SupportSuplV1", &self.SupportSuplV1)
            .field("SupportSuplV2", &self.SupportSuplV2)
            .field("SupportedSuplVersion", &self.SupportedSuplVersion)
            .field("MaxGeofencesSupported", &self.MaxGeofencesSupported)
            .field("SupportMultipleSuplRootCert", &self.SupportMultipleSuplRootCert)
            .field("GnssBreadCrumbPayloadVersion", &self.GnssBreadCrumbPayloadVersion)
            .field("MaxGnssBreadCrumbFixes", &self.MaxGnssBreadCrumbFixes)
            .field("Unused", &self.Unused)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_DEVICE_CAPABILITY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_DEVICE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.SupportMultipleFixSessions == other.SupportMultipleFixSessions
            && self.SupportMultipleAppSessions == other.SupportMultipleAppSessions
            && self.RequireAGnssInjection == other.RequireAGnssInjection
            && self.AgnssFormatSupported == other.AgnssFormatSupported
            && self.AgnssFormatPreferred == other.AgnssFormatPreferred
            && self.SupportDistanceTracking == other.SupportDistanceTracking
            && self.SupportContinuousTracking == other.SupportContinuousTracking
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
            && self.Reserved3 == other.Reserved3
            && self.Reserved4 == other.Reserved4
            && self.Reserved5 == other.Reserved5
            && self.GeofencingSupport == other.GeofencingSupport
            && self.Reserved6 == other.Reserved6
            && self.Reserved7 == other.Reserved7
            && self.SupportCpLocation == other.SupportCpLocation
            && self.SupportUplV2 == other.SupportUplV2
            && self.SupportSuplV1 == other.SupportSuplV1
            && self.SupportSuplV2 == other.SupportSuplV2
            && self.SupportedSuplVersion == other.SupportedSuplVersion
            && self.MaxGeofencesSupported == other.MaxGeofencesSupported
            && self.SupportMultipleSuplRootCert == other.SupportMultipleSuplRootCert
            && self.GnssBreadCrumbPayloadVersion == other.GnssBreadCrumbPayloadVersion
            && self.MaxGnssBreadCrumbFixes == other.MaxGnssBreadCrumbFixes
            && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_DEVICE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_DEVICE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_DISTANCETRACKING_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub MovementThreshold: u32,
}
impl ::core::marker::Copy for GNSS_DISTANCETRACKING_PARAM {}
impl ::core::clone::Clone for GNSS_DISTANCETRACKING_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_DISTANCETRACKING_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DISTANCETRACKING_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("MovementThreshold", &self.MovementThreshold).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_DISTANCETRACKING_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_DISTANCETRACKING_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.MovementThreshold == other.MovementThreshold
    }
}
impl ::core::cmp::Eq for GNSS_DISTANCETRACKING_PARAM {}
impl ::core::default::Default for GNSS_DISTANCETRACKING_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_DRIVERCOMMAND_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub CommandType: GNSS_DRIVERCOMMAND_TYPE,
    pub Reserved: u32,
    pub CommandDataSize: u32,
    pub Unused: [u8; 512],
    pub CommandData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_DRIVERCOMMAND_PARAM {}
impl ::core::clone::Clone for GNSS_DRIVERCOMMAND_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_DRIVERCOMMAND_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DRIVERCOMMAND_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("CommandType", &self.CommandType).field("Reserved", &self.Reserved).field("CommandDataSize", &self.CommandDataSize).field("Unused", &self.Unused).field("CommandData", &self.CommandData).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_DRIVERCOMMAND_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_DRIVERCOMMAND_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.CommandType == other.CommandType && self.Reserved == other.Reserved && self.CommandDataSize == other.CommandDataSize && self.Unused == other.Unused && self.CommandData == other.CommandData
    }
}
impl ::core::cmp::Eq for GNSS_DRIVERCOMMAND_PARAM {}
impl ::core::default::Default for GNSS_DRIVERCOMMAND_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_DRIVER_REQUEST_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Request: GNSS_DRIVER_REQUEST,
    pub RequestFlag: u32,
}
impl ::core::marker::Copy for GNSS_DRIVER_REQUEST_DATA {}
impl ::core::clone::Clone for GNSS_DRIVER_REQUEST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_DRIVER_REQUEST_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_DRIVER_REQUEST_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Request", &self.Request).field("RequestFlag", &self.RequestFlag).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_DRIVER_REQUEST_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_DRIVER_REQUEST_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Request == other.Request && self.RequestFlag == other.RequestFlag
    }
}
impl ::core::cmp::Eq for GNSS_DRIVER_REQUEST_DATA {}
impl ::core::default::Default for GNSS_DRIVER_REQUEST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_ERRORINFO {
    pub Size: u32,
    pub Version: u32,
    pub ErrorCode: u32,
    pub IsRecoverable: super::super::Foundation::BOOL,
    pub ErrorDescription: [u16; 256],
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_ERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_ERRORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_ERRORINFO").field("Size", &self.Size).field("Version", &self.Version).field("ErrorCode", &self.ErrorCode).field("IsRecoverable", &self.IsRecoverable).field("ErrorDescription", &self.ErrorDescription).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_ERRORINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_ERRORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.ErrorCode == other.ErrorCode && self.IsRecoverable == other.IsRecoverable && self.ErrorDescription == other.ErrorDescription && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_ERRORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_ERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_EVENT_0 {
    pub FixData: GNSS_FIXDATA,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_EVENT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_EVENT_2 {
    pub Size: u32,
    pub Version: u32,
    pub EventType: GNSS_EVENT_TYPE,
    pub EventDataSize: u32,
    pub Unused: [u8; 512],
    pub Anonymous: GNSS_EVENT_2_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_EVENT_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_EVENT_2_0 {
    pub FixData: GNSS_FIXDATA,
    pub FixData2: GNSS_FIXDATA_2,
    pub AgnssRequest: GNSS_AGNSS_REQUEST_PARAM,
    pub NiRequest: GNSS_NI_REQUEST_PARAM,
    pub ErrorInformation: GNSS_ERRORINFO,
    pub NmeaData: GNSS_NMEA_DATA,
    pub GeofenceAlertData: GNSS_GEOFENCE_ALERT_DATA,
    pub BreadcrumbAlertData: GNSS_BREADCRUMBING_ALERT_DATA,
    pub GeofencesTrackingStatus: GNSS_GEOFENCES_TRACKINGSTATUS_DATA,
    pub DriverRequestData: GNSS_DRIVER_REQUEST_DATA,
    pub CustomData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_EVENT_2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_EVENT_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_EVENT_2_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_EVENT_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_FIXDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA").field("Size", &self.Size).field("Version", &self.Version).field("FixSessionID", &self.FixSessionID).field("FixTimeStamp", &self.FixTimeStamp).field("IsFinalFix", &self.IsFinalFix).field("FixStatus", &self.FixStatus).field("FixLevelOfDetails", &self.FixLevelOfDetails).field("BasicData", &self.BasicData).field("AccuracyData", &self.AccuracyData).field("SatelliteData", &self.SatelliteData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_FIXDATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_FIXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.FixSessionID == other.FixSessionID && self.FixTimeStamp == other.FixTimeStamp && self.IsFinalFix == other.IsFinalFix && self.FixStatus == other.FixStatus && self.FixLevelOfDetails == other.FixLevelOfDetails && self.BasicData == other.BasicData && self.AccuracyData == other.AccuracyData && self.SatelliteData == other.SatelliteData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_FIXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_FIXDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_2 {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub FixTimeStamp: super::super::Foundation::FILETIME,
    pub IsFinalFix: super::super::Foundation::BOOL,
    pub FixStatus: super::super::Foundation::NTSTATUS,
    pub FixLevelOfDetails: u32,
    pub BasicData: GNSS_FIXDATA_BASIC_2,
    pub AccuracyData: GNSS_FIXDATA_ACCURACY_2,
    pub SatelliteData: GNSS_FIXDATA_SATELLITE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_FIXDATA_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_2").field("Size", &self.Size).field("Version", &self.Version).field("FixSessionID", &self.FixSessionID).field("FixTimeStamp", &self.FixTimeStamp).field("IsFinalFix", &self.IsFinalFix).field("FixStatus", &self.FixStatus).field("FixLevelOfDetails", &self.FixLevelOfDetails).field("BasicData", &self.BasicData).field("AccuracyData", &self.AccuracyData).field("SatelliteData", &self.SatelliteData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_FIXDATA_2 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_FIXDATA_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.FixSessionID == other.FixSessionID && self.FixTimeStamp == other.FixTimeStamp && self.IsFinalFix == other.IsFinalFix && self.FixStatus == other.FixStatus && self.FixLevelOfDetails == other.FixLevelOfDetails && self.BasicData == other.BasicData && self.AccuracyData == other.AccuracyData && self.SatelliteData == other.SatelliteData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_FIXDATA_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_FIXDATA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_FIXDATA_ACCURACY {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: u32,
    pub HorizontalErrorMajorAxis: u32,
    pub HorizontalErrorMinorAxis: u32,
    pub HorizontalErrorAngle: u32,
    pub HeadingAccuracy: u32,
    pub AltitudeAccuracy: u32,
    pub SpeedAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f32,
    pub HorizontalDilutionOfPrecision: f32,
    pub VerticalDilutionOfPrecision: f32,
}
impl ::core::marker::Copy for GNSS_FIXDATA_ACCURACY {}
impl ::core::clone::Clone for GNSS_FIXDATA_ACCURACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_ACCURACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_ACCURACY")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("HorizontalAccuracy", &self.HorizontalAccuracy)
            .field("HorizontalErrorMajorAxis", &self.HorizontalErrorMajorAxis)
            .field("HorizontalErrorMinorAxis", &self.HorizontalErrorMinorAxis)
            .field("HorizontalErrorAngle", &self.HorizontalErrorAngle)
            .field("HeadingAccuracy", &self.HeadingAccuracy)
            .field("AltitudeAccuracy", &self.AltitudeAccuracy)
            .field("SpeedAccuracy", &self.SpeedAccuracy)
            .field("HorizontalConfidence", &self.HorizontalConfidence)
            .field("HeadingConfidence", &self.HeadingConfidence)
            .field("AltitudeConfidence", &self.AltitudeConfidence)
            .field("SpeedConfidence", &self.SpeedConfidence)
            .field("PositionDilutionOfPrecision", &self.PositionDilutionOfPrecision)
            .field("HorizontalDilutionOfPrecision", &self.HorizontalDilutionOfPrecision)
            .field("VerticalDilutionOfPrecision", &self.VerticalDilutionOfPrecision)
            .finish()
    }
}
impl ::windows_core::TypeKind for GNSS_FIXDATA_ACCURACY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_ACCURACY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.HorizontalAccuracy == other.HorizontalAccuracy
            && self.HorizontalErrorMajorAxis == other.HorizontalErrorMajorAxis
            && self.HorizontalErrorMinorAxis == other.HorizontalErrorMinorAxis
            && self.HorizontalErrorAngle == other.HorizontalErrorAngle
            && self.HeadingAccuracy == other.HeadingAccuracy
            && self.AltitudeAccuracy == other.AltitudeAccuracy
            && self.SpeedAccuracy == other.SpeedAccuracy
            && self.HorizontalConfidence == other.HorizontalConfidence
            && self.HeadingConfidence == other.HeadingConfidence
            && self.AltitudeConfidence == other.AltitudeConfidence
            && self.SpeedConfidence == other.SpeedConfidence
            && self.PositionDilutionOfPrecision == other.PositionDilutionOfPrecision
            && self.HorizontalDilutionOfPrecision == other.HorizontalDilutionOfPrecision
            && self.VerticalDilutionOfPrecision == other.VerticalDilutionOfPrecision
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_ACCURACY {}
impl ::core::default::Default for GNSS_FIXDATA_ACCURACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_FIXDATA_ACCURACY_2 {
    pub Size: u32,
    pub Version: u32,
    pub HorizontalAccuracy: f64,
    pub HorizontalErrorMajorAxis: f64,
    pub HorizontalErrorMinorAxis: f64,
    pub HorizontalErrorAngle: f64,
    pub HeadingAccuracy: f64,
    pub AltitudeAccuracy: f64,
    pub SpeedAccuracy: f64,
    pub HorizontalConfidence: u32,
    pub HeadingConfidence: u32,
    pub AltitudeConfidence: u32,
    pub SpeedConfidence: u32,
    pub PositionDilutionOfPrecision: f64,
    pub HorizontalDilutionOfPrecision: f64,
    pub VerticalDilutionOfPrecision: f64,
    pub GeometricDilutionOfPrecision: f64,
    pub TimeDilutionOfPrecision: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_ACCURACY_2 {}
impl ::core::clone::Clone for GNSS_FIXDATA_ACCURACY_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_ACCURACY_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_ACCURACY_2")
            .field("Size", &self.Size)
            .field("Version", &self.Version)
            .field("HorizontalAccuracy", &self.HorizontalAccuracy)
            .field("HorizontalErrorMajorAxis", &self.HorizontalErrorMajorAxis)
            .field("HorizontalErrorMinorAxis", &self.HorizontalErrorMinorAxis)
            .field("HorizontalErrorAngle", &self.HorizontalErrorAngle)
            .field("HeadingAccuracy", &self.HeadingAccuracy)
            .field("AltitudeAccuracy", &self.AltitudeAccuracy)
            .field("SpeedAccuracy", &self.SpeedAccuracy)
            .field("HorizontalConfidence", &self.HorizontalConfidence)
            .field("HeadingConfidence", &self.HeadingConfidence)
            .field("AltitudeConfidence", &self.AltitudeConfidence)
            .field("SpeedConfidence", &self.SpeedConfidence)
            .field("PositionDilutionOfPrecision", &self.PositionDilutionOfPrecision)
            .field("HorizontalDilutionOfPrecision", &self.HorizontalDilutionOfPrecision)
            .field("VerticalDilutionOfPrecision", &self.VerticalDilutionOfPrecision)
            .field("GeometricDilutionOfPrecision", &self.GeometricDilutionOfPrecision)
            .field("TimeDilutionOfPrecision", &self.TimeDilutionOfPrecision)
            .finish()
    }
}
impl ::windows_core::TypeKind for GNSS_FIXDATA_ACCURACY_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_ACCURACY_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Version == other.Version
            && self.HorizontalAccuracy == other.HorizontalAccuracy
            && self.HorizontalErrorMajorAxis == other.HorizontalErrorMajorAxis
            && self.HorizontalErrorMinorAxis == other.HorizontalErrorMinorAxis
            && self.HorizontalErrorAngle == other.HorizontalErrorAngle
            && self.HeadingAccuracy == other.HeadingAccuracy
            && self.AltitudeAccuracy == other.AltitudeAccuracy
            && self.SpeedAccuracy == other.SpeedAccuracy
            && self.HorizontalConfidence == other.HorizontalConfidence
            && self.HeadingConfidence == other.HeadingConfidence
            && self.AltitudeConfidence == other.AltitudeConfidence
            && self.SpeedConfidence == other.SpeedConfidence
            && self.PositionDilutionOfPrecision == other.PositionDilutionOfPrecision
            && self.HorizontalDilutionOfPrecision == other.HorizontalDilutionOfPrecision
            && self.VerticalDilutionOfPrecision == other.VerticalDilutionOfPrecision
            && self.GeometricDilutionOfPrecision == other.GeometricDilutionOfPrecision
            && self.TimeDilutionOfPrecision == other.TimeDilutionOfPrecision
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_ACCURACY_2 {}
impl ::core::default::Default for GNSS_FIXDATA_ACCURACY_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_FIXDATA_BASIC {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_BASIC {}
impl ::core::clone::Clone for GNSS_FIXDATA_BASIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_BASIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_BASIC").field("Size", &self.Size).field("Version", &self.Version).field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).field("Speed", &self.Speed).field("Heading", &self.Heading).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_FIXDATA_BASIC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_BASIC {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude && self.Speed == other.Speed && self.Heading == other.Heading
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_BASIC {}
impl ::core::default::Default for GNSS_FIXDATA_BASIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_FIXDATA_BASIC_2 {
    pub Size: u32,
    pub Version: u32,
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
    pub Speed: f64,
    pub Heading: f64,
    pub AltitudeEllipsoid: f64,
}
impl ::core::marker::Copy for GNSS_FIXDATA_BASIC_2 {}
impl ::core::clone::Clone for GNSS_FIXDATA_BASIC_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_FIXDATA_BASIC_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_BASIC_2").field("Size", &self.Size).field("Version", &self.Version).field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).field("Speed", &self.Speed).field("Heading", &self.Heading).field("AltitudeEllipsoid", &self.AltitudeEllipsoid).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_FIXDATA_BASIC_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_FIXDATA_BASIC_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude && self.Speed == other.Speed && self.Heading == other.Heading && self.AltitudeEllipsoid == other.AltitudeEllipsoid
    }
}
impl ::core::cmp::Eq for GNSS_FIXDATA_BASIC_2 {}
impl ::core::default::Default for GNSS_FIXDATA_BASIC_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_FIXDATA_SATELLITE {
    pub Size: u32,
    pub Version: u32,
    pub SatelliteCount: u32,
    pub SatelliteArray: [GNSS_SATELLITEINFO; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_FIXDATA_SATELLITE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_FIXDATA_SATELLITE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_FIXDATA_SATELLITE").field("Size", &self.Size).field("Version", &self.Version).field("SatelliteCount", &self.SatelliteCount).field("SatelliteArray", &self.SatelliteArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_FIXDATA_SATELLITE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_FIXDATA_SATELLITE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.SatelliteCount == other.SatelliteCount && self.SatelliteArray == other.SatelliteArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_FIXDATA_SATELLITE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_FIXDATA_SATELLITE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_FIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub SessionType: GNSS_FIXSESSIONTYPE,
    pub HorizontalAccuracy: u32,
    pub HorizontalConfidence: u32,
    pub Reserved: [u32; 9],
    pub FixLevelOfDetails: u32,
    pub Anonymous: GNSS_FIXSESSION_PARAM_0,
    pub Unused: [u8; 256],
}
impl ::core::marker::Copy for GNSS_FIXSESSION_PARAM {}
impl ::core::clone::Clone for GNSS_FIXSESSION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for GNSS_FIXSESSION_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for GNSS_FIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union GNSS_FIXSESSION_PARAM_0 {
    pub SingleShotParam: GNSS_SINGLESHOT_PARAM,
    pub DistanceParam: GNSS_DISTANCETRACKING_PARAM,
    pub ContinuousParam: GNSS_CONTINUOUSTRACKING_PARAM,
    pub LkgFixParam: GNSS_LKGFIX_PARAM,
    pub UnusedParam: [u8; 268],
}
impl ::core::marker::Copy for GNSS_FIXSESSION_PARAM_0 {}
impl ::core::clone::Clone for GNSS_FIXSESSION_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for GNSS_FIXSESSION_PARAM_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for GNSS_FIXSESSION_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    pub Size: u32,
    pub Version: u32,
    pub Status: super::super::Foundation::NTSTATUS,
    pub StatusTimeStamp: super::super::Foundation::FILETIME,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCES_TRACKINGSTATUS_DATA").field("Size", &self.Size).field("Version", &self.Version).field("Status", &self.Status).field("StatusTimeStamp", &self.StatusTimeStamp).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.Status == other.Status && self.StatusTimeStamp == other.StatusTimeStamp && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_GEOFENCES_TRACKINGSTATUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_GEOFENCE_ALERT_DATA {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub GeofenceState: GNSS_GEOFENCE_STATE,
    pub FixBasicData: GNSS_FIXDATA_BASIC,
    pub FixAccuracyData: GNSS_FIXDATA_ACCURACY,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_ALERT_DATA {}
impl ::core::clone::Clone for GNSS_GEOFENCE_ALERT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_GEOFENCE_ALERT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCE_ALERT_DATA").field("Size", &self.Size).field("Version", &self.Version).field("GeofenceID", &self.GeofenceID).field("GeofenceState", &self.GeofenceState).field("FixBasicData", &self.FixBasicData).field("FixAccuracyData", &self.FixAccuracyData).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_GEOFENCE_ALERT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_ALERT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.GeofenceID == other.GeofenceID && self.GeofenceState == other.GeofenceState && self.FixBasicData == other.FixBasicData && self.FixAccuracyData == other.FixAccuracyData && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_GEOFENCE_ALERT_DATA {}
impl ::core::default::Default for GNSS_GEOFENCE_ALERT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_GEOFENCE_CREATE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub AlertTypes: u32,
    pub InitialState: GNSS_GEOFENCE_STATE,
    pub Boundary: GNSS_GEOREGION,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_CREATE_PARAM {}
impl ::core::clone::Clone for GNSS_GEOFENCE_CREATE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for GNSS_GEOFENCE_CREATE_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for GNSS_GEOFENCE_CREATE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_GEOFENCE_CREATE_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub CreationStatus: super::super::Foundation::NTSTATUS,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCE_CREATE_RESPONSE").field("Size", &self.Size).field("Version", &self.Version).field("CreationStatus", &self.CreationStatus).field("GeofenceID", &self.GeofenceID).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_GEOFENCE_CREATE_RESPONSE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.CreationStatus == other.CreationStatus && self.GeofenceID == other.GeofenceID && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_GEOFENCE_CREATE_RESPONSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_GEOFENCE_CREATE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_GEOFENCE_DELETE_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub GeofenceID: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOFENCE_DELETE_PARAM {}
impl ::core::clone::Clone for GNSS_GEOFENCE_DELETE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_GEOFENCE_DELETE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOFENCE_DELETE_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("GeofenceID", &self.GeofenceID).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_GEOFENCE_DELETE_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_GEOFENCE_DELETE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.GeofenceID == other.GeofenceID && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_GEOFENCE_DELETE_PARAM {}
impl ::core::default::Default for GNSS_GEOFENCE_DELETE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_GEOREGION {
    pub Size: u32,
    pub Version: u32,
    pub GeoRegionType: GNSS_GEOREGIONTYPE,
    pub Anonymous: GNSS_GEOREGION_0,
}
impl ::core::marker::Copy for GNSS_GEOREGION {}
impl ::core::clone::Clone for GNSS_GEOREGION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for GNSS_GEOREGION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for GNSS_GEOREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union GNSS_GEOREGION_0 {
    pub Circle: GNSS_GEOREGION_CIRCLE,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_GEOREGION_0 {}
impl ::core::clone::Clone for GNSS_GEOREGION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for GNSS_GEOREGION_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for GNSS_GEOREGION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_GEOREGION_CIRCLE {
    pub Latitude: f64,
    pub Longitude: f64,
    pub RadiusInMeters: f64,
}
impl ::core::marker::Copy for GNSS_GEOREGION_CIRCLE {}
impl ::core::clone::Clone for GNSS_GEOREGION_CIRCLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_GEOREGION_CIRCLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_GEOREGION_CIRCLE").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("RadiusInMeters", &self.RadiusInMeters).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_GEOREGION_CIRCLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_GEOREGION_CIRCLE {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.RadiusInMeters == other.RadiusInMeters
    }
}
impl ::core::cmp::Eq for GNSS_GEOREGION_CIRCLE {}
impl ::core::default::Default for GNSS_GEOREGION_CIRCLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_LKGFIX_PARAM {
    pub Size: u32,
    pub Version: u32,
}
impl ::core::marker::Copy for GNSS_LKGFIX_PARAM {}
impl ::core::clone::Clone for GNSS_LKGFIX_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_LKGFIX_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_LKGFIX_PARAM").field("Size", &self.Size).field("Version", &self.Version).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_LKGFIX_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_LKGFIX_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for GNSS_LKGFIX_PARAM {}
impl ::core::default::Default for GNSS_LKGFIX_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_NI_REQUEST_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub RequestType: GNSS_NI_REQUEST_TYPE,
    pub NotificationType: GNSS_NI_NOTIFICATION_TYPE,
    pub RequestPlaneType: GNSS_NI_PLANE_TYPE,
    pub Anonymous: GNSS_NI_REQUEST_PARAM_0,
    pub ResponseTimeInSec: u32,
    pub EmergencyLocation: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NI_REQUEST_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NI_REQUEST_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_NI_REQUEST_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_NI_REQUEST_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union GNSS_NI_REQUEST_PARAM_0 {
    pub SuplNiInfo: GNSS_SUPL_NI_INFO,
    pub CpNiInfo: GNSS_CP_NI_INFO,
    pub V2UplNiInfo: GNSS_V2UPL_NI_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_NI_REQUEST_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_NI_REQUEST_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_NI_REQUEST_PARAM_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_NI_REQUEST_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_NI_RESPONSE {
    pub Size: u32,
    pub Version: u32,
    pub RequestId: u32,
    pub UserResponse: GNSS_NI_USER_RESPONSE,
}
impl ::core::marker::Copy for GNSS_NI_RESPONSE {}
impl ::core::clone::Clone for GNSS_NI_RESPONSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_NI_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_NI_RESPONSE").field("Size", &self.Size).field("Version", &self.Version).field("RequestId", &self.RequestId).field("UserResponse", &self.UserResponse).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_NI_RESPONSE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_NI_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.RequestId == other.RequestId && self.UserResponse == other.UserResponse
    }
}
impl ::core::cmp::Eq for GNSS_NI_RESPONSE {}
impl ::core::default::Default for GNSS_NI_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_NMEA_DATA {
    pub Size: u32,
    pub Version: u32,
    pub NmeaSentences: [u8; 256],
}
impl ::core::marker::Copy for GNSS_NMEA_DATA {}
impl ::core::clone::Clone for GNSS_NMEA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_NMEA_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_NMEA_DATA").field("Size", &self.Size).field("Version", &self.Version).field("NmeaSentences", &self.NmeaSentences).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_NMEA_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_NMEA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.NmeaSentences == other.NmeaSentences
    }
}
impl ::core::cmp::Eq for GNSS_NMEA_DATA {}
impl ::core::default::Default for GNSS_NMEA_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_PLATFORM_CAPABILITY {
    pub Size: u32,
    pub Version: u32,
    pub SupportAgnssInjection: super::super::Foundation::BOOL,
    pub AgnssFormatSupported: u32,
    pub Unused: [u8; 516],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_PLATFORM_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_PLATFORM_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_PLATFORM_CAPABILITY").field("Size", &self.Size).field("Version", &self.Version).field("SupportAgnssInjection", &self.SupportAgnssInjection).field("AgnssFormatSupported", &self.AgnssFormatSupported).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_PLATFORM_CAPABILITY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_PLATFORM_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.SupportAgnssInjection == other.SupportAgnssInjection && self.AgnssFormatSupported == other.AgnssFormatSupported && self.Unused == other.Unused
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_PLATFORM_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_PLATFORM_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SATELLITEINFO {
    pub SatelliteId: u32,
    pub UsedInPositiong: super::super::Foundation::BOOL,
    pub Elevation: f64,
    pub Azimuth: f64,
    pub SignalToNoiseRatio: f64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SATELLITEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SATELLITEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SATELLITEINFO").field("SatelliteId", &self.SatelliteId).field("UsedInPositiong", &self.UsedInPositiong).field("Elevation", &self.Elevation).field("Azimuth", &self.Azimuth).field("SignalToNoiseRatio", &self.SignalToNoiseRatio).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_SATELLITEINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SATELLITEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.SatelliteId == other.SatelliteId && self.UsedInPositiong == other.UsedInPositiong && self.Elevation == other.Elevation && self.Azimuth == other.Azimuth && self.SignalToNoiseRatio == other.SignalToNoiseRatio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SATELLITEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SATELLITEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SELFTESTCONFIG {
    pub Size: u32,
    pub Version: u32,
    pub TestType: u32,
    pub Unused: [u8; 512],
    pub InBufLen: u32,
    pub InBuffer: [u8; 1],
}
impl ::core::marker::Copy for GNSS_SELFTESTCONFIG {}
impl ::core::clone::Clone for GNSS_SELFTESTCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SELFTESTCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SELFTESTCONFIG").field("Size", &self.Size).field("Version", &self.Version).field("TestType", &self.TestType).field("Unused", &self.Unused).field("InBufLen", &self.InBufLen).field("InBuffer", &self.InBuffer).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SELFTESTCONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SELFTESTCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.TestType == other.TestType && self.Unused == other.Unused && self.InBufLen == other.InBufLen && self.InBuffer == other.InBuffer
    }
}
impl ::core::cmp::Eq for GNSS_SELFTESTCONFIG {}
impl ::core::default::Default for GNSS_SELFTESTCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GNSS_SELFTESTRESULT {
    pub Size: u32,
    pub Version: u32,
    pub TestResultStatus: super::super::Foundation::NTSTATUS,
    pub Result: u32,
    pub PinFailedBitMask: u32,
    pub Unused: [u8; 512],
    pub OutBufLen: u32,
    pub OutBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GNSS_SELFTESTRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GNSS_SELFTESTRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SELFTESTRESULT").field("Size", &self.Size).field("Version", &self.Version).field("TestResultStatus", &self.TestResultStatus).field("Result", &self.Result).field("PinFailedBitMask", &self.PinFailedBitMask).field("Unused", &self.Unused).field("OutBufLen", &self.OutBufLen).field("OutBuffer", &self.OutBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GNSS_SELFTESTRESULT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GNSS_SELFTESTRESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.TestResultStatus == other.TestResultStatus && self.Result == other.Result && self.PinFailedBitMask == other.PinFailedBitMask && self.Unused == other.Unused && self.OutBufLen == other.OutBufLen && self.OutBuffer == other.OutBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GNSS_SELFTESTRESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GNSS_SELFTESTRESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SINGLESHOT_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub ResponseTime: u32,
}
impl ::core::marker::Copy for GNSS_SINGLESHOT_PARAM {}
impl ::core::clone::Clone for GNSS_SINGLESHOT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SINGLESHOT_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SINGLESHOT_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("ResponseTime", &self.ResponseTime).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SINGLESHOT_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SINGLESHOT_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.ResponseTime == other.ResponseTime
    }
}
impl ::core::cmp::Eq for GNSS_SINGLESHOT_PARAM {}
impl ::core::default::Default for GNSS_SINGLESHOT_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_STOPFIXSESSION_PARAM {
    pub Size: u32,
    pub Version: u32,
    pub FixSessionID: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_STOPFIXSESSION_PARAM {}
impl ::core::clone::Clone for GNSS_STOPFIXSESSION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_STOPFIXSESSION_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_STOPFIXSESSION_PARAM").field("Size", &self.Size).field("Version", &self.Version).field("FixSessionID", &self.FixSessionID).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_STOPFIXSESSION_PARAM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_STOPFIXSESSION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.FixSessionID == other.FixSessionID && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_STOPFIXSESSION_PARAM {}
impl ::core::default::Default for GNSS_STOPFIXSESSION_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SUPL_CERT_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub CertAction: GNSS_SUPL_CERT_ACTION,
    pub SuplCertName: [u8; 260],
    pub CertSize: u32,
    pub Unused: [u8; 512],
    pub CertData: [u8; 1],
}
impl ::core::marker::Copy for GNSS_SUPL_CERT_CONFIG {}
impl ::core::clone::Clone for GNSS_SUPL_CERT_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_CERT_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_CERT_CONFIG").field("Size", &self.Size).field("Version", &self.Version).field("CertAction", &self.CertAction).field("SuplCertName", &self.SuplCertName).field("CertSize", &self.CertSize).field("Unused", &self.Unused).field("CertData", &self.CertData).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SUPL_CERT_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_CERT_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.CertAction == other.CertAction && self.SuplCertName == other.SuplCertName && self.CertSize == other.CertSize && self.Unused == other.Unused && self.CertData == other.CertData
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_CERT_CONFIG {}
impl ::core::default::Default for GNSS_SUPL_CERT_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SUPL_HSLP_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub SuplHslp: [u8; 260],
    pub SuplHslpFromImsi: [u8; 260],
    pub Reserved: u32,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_SUPL_HSLP_CONFIG {}
impl ::core::clone::Clone for GNSS_SUPL_HSLP_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_HSLP_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_HSLP_CONFIG").field("Size", &self.Size).field("Version", &self.Version).field("SuplHslp", &self.SuplHslp).field("SuplHslpFromImsi", &self.SuplHslpFromImsi).field("Reserved", &self.Reserved).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SUPL_HSLP_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_HSLP_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.SuplHslp == other.SuplHslp && self.SuplHslpFromImsi == other.SuplHslpFromImsi && self.Reserved == other.Reserved && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_HSLP_CONFIG {}
impl ::core::default::Default for GNSS_SUPL_HSLP_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SUPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
    pub ClientName: [u16; 260],
    pub SuplNiUrl: [u8; 260],
}
impl ::core::marker::Copy for GNSS_SUPL_NI_INFO {}
impl ::core::clone::Clone for GNSS_SUPL_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_NI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_NI_INFO").field("Size", &self.Size).field("Version", &self.Version).field("RequestorId", &self.RequestorId).field("ClientName", &self.ClientName).field("SuplNiUrl", &self.SuplNiUrl).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SUPL_NI_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.RequestorId == other.RequestorId && self.ClientName == other.ClientName && self.SuplNiUrl == other.SuplNiUrl
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_NI_INFO {}
impl ::core::default::Default for GNSS_SUPL_NI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SUPL_VERSION {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl ::core::marker::Copy for GNSS_SUPL_VERSION {}
impl ::core::clone::Clone for GNSS_SUPL_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SUPL_VERSION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_VERSION {}
impl ::core::default::Default for GNSS_SUPL_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_SUPL_VERSION_2 {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ServiceIndicator: u32,
}
impl ::core::marker::Copy for GNSS_SUPL_VERSION_2 {}
impl ::core::clone::Clone for GNSS_SUPL_VERSION_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_SUPL_VERSION_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_SUPL_VERSION_2").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("ServiceIndicator", &self.ServiceIndicator).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_SUPL_VERSION_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_SUPL_VERSION_2 {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.ServiceIndicator == other.ServiceIndicator
    }
}
impl ::core::cmp::Eq for GNSS_SUPL_VERSION_2 {}
impl ::core::default::Default for GNSS_SUPL_VERSION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_V2UPL_CONFIG {
    pub Size: u32,
    pub Version: u32,
    pub MPC: [u8; 260],
    pub PDE: [u8; 260],
    pub ApplicationTypeIndicator_MR: u8,
    pub Unused: [u8; 512],
}
impl ::core::marker::Copy for GNSS_V2UPL_CONFIG {}
impl ::core::clone::Clone for GNSS_V2UPL_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_V2UPL_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_V2UPL_CONFIG").field("Size", &self.Size).field("Version", &self.Version).field("MPC", &self.MPC).field("PDE", &self.PDE).field("ApplicationTypeIndicator_MR", &self.ApplicationTypeIndicator_MR).field("Unused", &self.Unused).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_V2UPL_CONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_V2UPL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.MPC == other.MPC && self.PDE == other.PDE && self.ApplicationTypeIndicator_MR == other.ApplicationTypeIndicator_MR && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for GNSS_V2UPL_CONFIG {}
impl ::core::default::Default for GNSS_V2UPL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GNSS_V2UPL_NI_INFO {
    pub Size: u32,
    pub Version: u32,
    pub RequestorId: [u16; 260],
}
impl ::core::marker::Copy for GNSS_V2UPL_NI_INFO {}
impl ::core::clone::Clone for GNSS_V2UPL_NI_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GNSS_V2UPL_NI_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GNSS_V2UPL_NI_INFO").field("Size", &self.Size).field("Version", &self.Version).field("RequestorId", &self.RequestorId).finish()
    }
}
impl ::windows_core::TypeKind for GNSS_V2UPL_NI_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GNSS_V2UPL_NI_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.RequestorId == other.RequestorId
    }
}
impl ::core::cmp::Eq for GNSS_V2UPL_NI_INFO {}
impl ::core::default::Default for GNSS_V2UPL_NI_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
