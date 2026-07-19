pub const CivicAddressReport: windows_core::GUID = windows_core::GUID::from_u128(0xd39e7bdd_7d05_46b8_8721_80cf035f57d7);
pub const CivicAddressReportFactory: windows_core::GUID = windows_core::GUID::from_u128(0x2a11f42c_3e81_4ad4_9cbe_45579d89671a);
pub const DefaultLocation: windows_core::GUID = windows_core::GUID::from_u128(0x8b7fbfe0_5cd7_494a_af8c_283a65707506);
pub const DispCivicAddressReport: windows_core::GUID = windows_core::GUID::from_u128(0x4c596aec_8544_4082_ba9f_eb0a7d8e65c6);
pub const DispLatLongReport: windows_core::GUID = windows_core::GUID::from_u128(0x7a7c3277_8f84_4636_95b2_ebb5507ff77e);
windows_core::imp::define_interface!(ICivicAddressReport, ICivicAddressReport_Vtbl, 0xc0b19f70_4adf_445d_87f2_cad8fd711792);
impl core::ops::Deref for ICivicAddressReport {
    type Target = ILocationReport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICivicAddressReport, windows_core::IUnknown, ILocationReport);
impl ICivicAddressReport {
    pub unsafe fn GetAddressLine1(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAddressLine1)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAddressLine2(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAddressLine2)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCity(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCity)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetStateProvince(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStateProvince)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPostalCode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostalCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCountryRegion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCountryRegion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDetailLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDetailLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReport_Vtbl {
    pub base__: ILocationReport_Vtbl,
    pub GetAddressLine1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAddressLine2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStateProvince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCountryRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDetailLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICivicAddressReport_Impl: ILocationReport_Impl {
    fn GetAddressLine1(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAddressLine2(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStateProvince(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPostalCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCountryRegion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDetailLevel(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
impl ICivicAddressReport_Vtbl {
    pub const fn new<Identity: ICivicAddressReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAddressLine1<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstraddress1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetAddressLine1(this) {
                    Ok(ok__) => {
                        pbstraddress1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAddressLine2<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstraddress2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetAddressLine2(this) {
                    Ok(ok__) => {
                        pbstraddress2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCity<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetCity(this) {
                    Ok(ok__) => {
                        pbstrcity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStateProvince<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstateprovince: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetStateProvince(this) {
                    Ok(ok__) => {
                        pbstrstateprovince.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPostalCode<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpostalcode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetPostalCode(this) {
                    Ok(ok__) => {
                        pbstrpostalcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCountryRegion<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcountryregion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetCountryRegion(this) {
                    Ok(ok__) => {
                        pbstrcountryregion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDetailLevel<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetaillevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReport_Impl::GetDetailLevel(this) {
                    Ok(ok__) => {
                        pdetaillevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ILocationReport_Vtbl::new::<Identity, OFFSET>(),
            GetAddressLine1: GetAddressLine1::<Identity, OFFSET>,
            GetAddressLine2: GetAddressLine2::<Identity, OFFSET>,
            GetCity: GetCity::<Identity, OFFSET>,
            GetStateProvince: GetStateProvince::<Identity, OFFSET>,
            GetPostalCode: GetPostalCode::<Identity, OFFSET>,
            GetCountryRegion: GetCountryRegion::<Identity, OFFSET>,
            GetDetailLevel: GetDetailLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICivicAddressReport as windows_core::Interface>::IID || iid == &<ILocationReport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICivicAddressReport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ICivicAddressReportFactory, ICivicAddressReportFactory_Vtbl, 0xbf773b93_c64f_4bee_beb2_67c0b8df66e0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ICivicAddressReportFactory {
    type Target = ILocationReportFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ICivicAddressReportFactory, windows_core::IUnknown, super::IDispatch, ILocationReportFactory);
#[cfg(feature = "oaidl")]
impl ICivicAddressReportFactory {
    pub unsafe fn CivicAddressReport(&self) -> windows_core::Result<IDispCivicAddressReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CivicAddressReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    pub CivicAddressReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICivicAddressReportFactory_Impl: ILocationReportFactory_Impl {
    fn CivicAddressReport(&self) -> windows_core::Result<IDispCivicAddressReport>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICivicAddressReportFactory_Vtbl {
    pub const fn new<Identity: ICivicAddressReportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CivicAddressReport<Identity: ICivicAddressReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReportFactory_Impl::CivicAddressReport(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, OFFSET>(), CivicAddressReport: CivicAddressReport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICivicAddressReportFactory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ILocationReportFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICivicAddressReportFactory {}
windows_core::imp::define_interface!(IDefaultLocation, IDefaultLocation_Vtbl, 0xa65af77e_969a_4a2e_8aca_33bb7cbb1235);
windows_core::imp::interface_hierarchy!(IDefaultLocation, windows_core::IUnknown);
impl IDefaultLocation {
    pub unsafe fn SetReport<P1>(&self, reporttype: *const windows_core::GUID, plocationreport: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ILocationReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReport)(windows_core::Interface::as_raw(self), reporttype, plocationreport.param().abi()) }
    }
    pub unsafe fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultLocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDefaultLocation_Impl: windows_core::IUnknownImpl {
    fn SetReport(&self, reporttype: *const windows_core::GUID, plocationreport: windows_core::Ref<ILocationReport>) -> windows_core::Result<()>;
    fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport>;
}
impl IDefaultLocation_Vtbl {
    pub const fn new<Identity: IDefaultLocation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetReport<Identity: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, plocationreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDefaultLocation_Impl::SetReport(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&plocationreport)).into()
            }
        }
        unsafe extern "system" fn GetReport<Identity: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pplocationreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDefaultLocation_Impl::GetReport(this, core::mem::transmute_copy(&reporttype)) {
                    Ok(ok__) => {
                        pplocationreport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetReport: SetReport::<Identity, OFFSET>,
            GetReport: GetReport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDefaultLocation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDefaultLocation {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDispCivicAddressReport, IDispCivicAddressReport_Vtbl, 0x16ff1a34_9e30_42c3_b44d_e22513b5767a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDispCivicAddressReport {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDispCivicAddressReport, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDispCivicAddressReport {
    pub unsafe fn AddressLine1(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddressLine1)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddressLine2(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddressLine2)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn City(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).City)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn StateProvince(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StateProvince)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PostalCode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostalCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CountryRegion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountryRegion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DetailLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetailLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispCivicAddressReport_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub AddressLine1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddressLine2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateProvince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CountryRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetailLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispCivicAddressReport_Impl: super::IDispatch_Impl {
    fn AddressLine1(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddressLine2(&self) -> windows_core::Result<windows_core::BSTR>;
    fn City(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StateProvince(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PostalCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CountryRegion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DetailLevel(&self) -> windows_core::Result<u32>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispCivicAddressReport_Vtbl {
    pub const fn new<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddressLine1<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::AddressLine1(this) {
                    Ok(ok__) => {
                        paddress1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddressLine2<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::AddressLine2(this) {
                    Ok(ok__) => {
                        paddress2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn City<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::City(this) {
                    Ok(ok__) => {
                        pcity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StateProvince<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstateprovince: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::StateProvince(this) {
                    Ok(ok__) => {
                        pstateprovince.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PostalCode<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppostalcode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::PostalCode(this) {
                    Ok(ok__) => {
                        ppostalcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CountryRegion<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcountryregion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::CountryRegion(this) {
                    Ok(ok__) => {
                        pcountryregion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DetailLevel<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetaillevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::DetailLevel(this) {
                    Ok(ok__) => {
                        pdetaillevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AddressLine1: AddressLine1::<Identity, OFFSET>,
            AddressLine2: AddressLine2::<Identity, OFFSET>,
            City: City::<Identity, OFFSET>,
            StateProvince: StateProvince::<Identity, OFFSET>,
            PostalCode: PostalCode::<Identity, OFFSET>,
            CountryRegion: CountryRegion::<Identity, OFFSET>,
            DetailLevel: DetailLevel::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispCivicAddressReport as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDispCivicAddressReport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDispLatLongReport, IDispLatLongReport_Vtbl, 0x8ae32723_389b_4a11_9957_5bdd48fc9617);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDispLatLongReport {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDispLatLongReport, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDispLatLongReport {
    pub unsafe fn Latitude(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Latitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Longitude(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Longitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ErrorRadius(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Altitude(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Altitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AltitudeError(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AltitudeError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Timestamp(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispLatLongReport_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Latitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Longitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ErrorRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Altitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub AltitudeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispLatLongReport_Impl: super::IDispatch_Impl {
    fn Latitude(&self) -> windows_core::Result<f64>;
    fn Longitude(&self) -> windows_core::Result<f64>;
    fn ErrorRadius(&self) -> windows_core::Result<f64>;
    fn Altitude(&self) -> windows_core::Result<f64>;
    fn AltitudeError(&self) -> windows_core::Result<f64>;
    fn Timestamp(&self) -> windows_core::Result<f64>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispLatLongReport_Vtbl {
    pub const fn new<Identity: IDispLatLongReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Latitude<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::Latitude(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Longitude<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::Longitude(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorRadius<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::ErrorRadius(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Altitude<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::Altitude(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AltitudeError<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::AltitudeError(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Latitude: Latitude::<Identity, OFFSET>,
            Longitude: Longitude::<Identity, OFFSET>,
            ErrorRadius: ErrorRadius::<Identity, OFFSET>,
            Altitude: Altitude::<Identity, OFFSET>,
            AltitudeError: AltitudeError::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispLatLongReport as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDispLatLongReport {}
windows_core::imp::define_interface!(ILatLongReport, ILatLongReport_Vtbl, 0x7fed806d_0ef8_4f07_80ac_36a0beae3134);
impl core::ops::Deref for ILatLongReport {
    type Target = ILocationReport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILatLongReport, windows_core::IUnknown, ILocationReport);
impl ILatLongReport {
    pub unsafe fn GetLatitude(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLongitude(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLongitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetErrorRadius(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAltitude(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAltitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAltitudeError(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAltitudeError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReport_Vtbl {
    pub base__: ILocationReport_Vtbl,
    pub GetLatitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetLongitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetErrorRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetAltitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetAltitudeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILatLongReport_Impl: ILocationReport_Impl {
    fn GetLatitude(&self) -> windows_core::Result<f64>;
    fn GetLongitude(&self) -> windows_core::Result<f64>;
    fn GetErrorRadius(&self) -> windows_core::Result<f64>;
    fn GetAltitude(&self) -> windows_core::Result<f64>;
    fn GetAltitudeError(&self) -> windows_core::Result<f64>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
impl ILatLongReport_Vtbl {
    pub const fn new<Identity: ILatLongReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLatitude<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platitude: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReport_Impl::GetLatitude(this) {
                    Ok(ok__) => {
                        platitude.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLongitude<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plongitude: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReport_Impl::GetLongitude(this) {
                    Ok(ok__) => {
                        plongitude.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetErrorRadius<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorradius: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReport_Impl::GetErrorRadius(this) {
                    Ok(ok__) => {
                        perrorradius.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAltitude<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltitude: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReport_Impl::GetAltitude(this) {
                    Ok(ok__) => {
                        paltitude.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAltitudeError<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltitudeerror: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReport_Impl::GetAltitudeError(this) {
                    Ok(ok__) => {
                        paltitudeerror.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ILocationReport_Vtbl::new::<Identity, OFFSET>(),
            GetLatitude: GetLatitude::<Identity, OFFSET>,
            GetLongitude: GetLongitude::<Identity, OFFSET>,
            GetErrorRadius: GetErrorRadius::<Identity, OFFSET>,
            GetAltitude: GetAltitude::<Identity, OFFSET>,
            GetAltitudeError: GetAltitudeError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILatLongReport as windows_core::Interface>::IID || iid == &<ILocationReport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILatLongReport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ILatLongReportFactory, ILatLongReportFactory_Vtbl, 0x3f0804cb_b114_447d_83dd_390174ebb082);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ILatLongReportFactory {
    type Target = ILocationReportFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ILatLongReportFactory, windows_core::IUnknown, super::IDispatch, ILocationReportFactory);
#[cfg(feature = "oaidl")]
impl ILatLongReportFactory {
    pub unsafe fn LatLongReport(&self) -> windows_core::Result<IDispLatLongReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LatLongReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    pub LatLongReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILatLongReportFactory_Impl: ILocationReportFactory_Impl {
    fn LatLongReport(&self) -> windows_core::Result<IDispLatLongReport>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ILatLongReportFactory_Vtbl {
    pub const fn new<Identity: ILatLongReportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LatLongReport<Identity: ILatLongReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReportFactory_Impl::LatLongReport(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, OFFSET>(), LatLongReport: LatLongReport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILatLongReportFactory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ILocationReportFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILatLongReportFactory {}
windows_core::imp::define_interface!(ILocation, ILocation_Vtbl, 0xab2ece69_56d9_4f28_b525_de1b0ee44237);
windows_core::imp::interface_hierarchy!(ILocation, windows_core::IUnknown);
impl ILocation {
    pub unsafe fn RegisterForReport<P0>(&self, pevents: P0, reporttype: *const windows_core::GUID, dwrequestedreportinterval: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ILocationEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterForReport)(windows_core::Interface::as_raw(self), pevents.param().abi(), reporttype, dwrequestedreportinterval) }
    }
    pub unsafe fn UnregisterForReport(&self, reporttype: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterForReport)(windows_core::Interface::as_raw(self), reporttype) }
    }
    pub unsafe fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetReportStatus(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<LOCATION_REPORT_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportStatus)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetReportInterval(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportInterval)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportInterval(&self, reporttype: *const windows_core::GUID, millisecondsrequested: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReportInterval)(windows_core::Interface::as_raw(self), reporttype, millisecondsrequested) }
    }
    #[cfg(feature = "sensorsapi")]
    pub unsafe fn GetDesiredAccuracy(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<super::LOCATION_DESIRED_ACCURACY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesiredAccuracy)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "sensorsapi")]
    pub unsafe fn SetDesiredAccuracy(&self, reporttype: *const windows_core::GUID, desiredaccuracy: super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredAccuracy)(windows_core::Interface::as_raw(self), reporttype, desiredaccuracy) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn RequestPermissions(&self, hparent: super::HWND, preporttypes: *const windows_core::GUID, count: u32, fmodal: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hparent, preporttypes, count, fmodal.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterForReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub UnregisterForReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut LOCATION_REPORT_STATUS) -> windows_core::HRESULT,
    pub GetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    #[cfg(feature = "sensorsapi")]
    pub GetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT,
    #[cfg(not(feature = "sensorsapi"))]
    GetDesiredAccuracy: usize,
    #[cfg(feature = "sensorsapi")]
    pub SetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT,
    #[cfg(not(feature = "sensorsapi"))]
    SetDesiredAccuracy: usize,
    #[cfg(feature = "windef")]
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const windows_core::GUID, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RequestPermissions: usize,
}
#[cfg(all(feature = "sensorsapi", feature = "windef"))]
pub trait ILocation_Impl: windows_core::IUnknownImpl {
    fn RegisterForReport(&self, pevents: windows_core::Ref<ILocationEvents>, reporttype: *const windows_core::GUID, dwrequestedreportinterval: u32) -> windows_core::Result<()>;
    fn UnregisterForReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetReport(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<ILocationReport>;
    fn GetReportStatus(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<LOCATION_REPORT_STATUS>;
    fn GetReportInterval(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn SetReportInterval(&self, reporttype: *const windows_core::GUID, millisecondsrequested: u32) -> windows_core::Result<()>;
    fn GetDesiredAccuracy(&self, reporttype: *const windows_core::GUID) -> windows_core::Result<super::LOCATION_DESIRED_ACCURACY>;
    fn SetDesiredAccuracy(&self, reporttype: *const windows_core::GUID, desiredaccuracy: super::LOCATION_DESIRED_ACCURACY) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hparent: super::HWND, preporttypes: *const windows_core::GUID, count: u32, fmodal: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "sensorsapi", feature = "windef"))]
impl ILocation_Vtbl {
    pub const fn new<Identity: ILocation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterForReport<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, dwrequestedreportinterval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::RegisterForReport(this, core::mem::transmute_copy(&pevents), core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&dwrequestedreportinterval)).into()
            }
        }
        unsafe extern "system" fn UnregisterForReport<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::UnregisterForReport(this, core::mem::transmute_copy(&reporttype)).into()
            }
        }
        unsafe extern "system" fn GetReport<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pplocationreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocation_Impl::GetReport(this, core::mem::transmute_copy(&reporttype)) {
                    Ok(ok__) => {
                        pplocationreport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReportStatus<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pstatus: *mut LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocation_Impl::GetReportStatus(this, core::mem::transmute_copy(&reporttype)) {
                    Ok(ok__) => {
                        pstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReportInterval<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pmilliseconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocation_Impl::GetReportInterval(this, core::mem::transmute_copy(&reporttype)) {
                    Ok(ok__) => {
                        pmilliseconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportInterval<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, millisecondsrequested: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::SetReportInterval(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&millisecondsrequested)).into()
            }
        }
        unsafe extern "system" fn GetDesiredAccuracy<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, pdesiredaccuracy: *mut super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocation_Impl::GetDesiredAccuracy(this, core::mem::transmute_copy(&reporttype)) {
                    Ok(ok__) => {
                        pdesiredaccuracy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, desiredaccuracy: super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::SetDesiredAccuracy(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&desiredaccuracy)).into()
            }
        }
        unsafe extern "system" fn RequestPermissions<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::HWND, preporttypes: *const windows_core::GUID, count: u32, fmodal: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::RequestPermissions(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&preporttypes), core::mem::transmute_copy(&count), core::mem::transmute_copy(&fmodal)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForReport: RegisterForReport::<Identity, OFFSET>,
            UnregisterForReport: UnregisterForReport::<Identity, OFFSET>,
            GetReport: GetReport::<Identity, OFFSET>,
            GetReportStatus: GetReportStatus::<Identity, OFFSET>,
            GetReportInterval: GetReportInterval::<Identity, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, OFFSET>,
            GetDesiredAccuracy: GetDesiredAccuracy::<Identity, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "sensorsapi", feature = "windef"))]
impl windows_core::RuntimeName for ILocation {}
windows_core::imp::define_interface!(ILocationEvents, ILocationEvents_Vtbl, 0xcae02bbf_798b_4508_a207_35a7906dc73d);
windows_core::imp::interface_hierarchy!(ILocationEvents, windows_core::IUnknown);
impl ILocationEvents {
    pub unsafe fn OnLocationChanged<P1>(&self, reporttype: *const windows_core::GUID, plocationreport: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ILocationReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnLocationChanged)(windows_core::Interface::as_raw(self), reporttype, plocationreport.param().abi()) }
    }
    pub unsafe fn OnStatusChanged(&self, reporttype: *const windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChanged)(windows_core::Interface::as_raw(self), reporttype, newstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnLocationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, LOCATION_REPORT_STATUS) -> windows_core::HRESULT,
}
pub trait ILocationEvents_Impl: windows_core::IUnknownImpl {
    fn OnLocationChanged(&self, reporttype: *const windows_core::GUID, plocationreport: windows_core::Ref<ILocationReport>) -> windows_core::Result<()>;
    fn OnStatusChanged(&self, reporttype: *const windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::Result<()>;
}
impl ILocationEvents_Vtbl {
    pub const fn new<Identity: ILocationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLocationChanged<Identity: ILocationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, plocationreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationEvents_Impl::OnLocationChanged(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&plocationreport)).into()
            }
        }
        unsafe extern "system" fn OnStatusChanged<Identity: ILocationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const windows_core::GUID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationEvents_Impl::OnStatusChanged(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&newstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLocationChanged: OnLocationChanged::<Identity, OFFSET>,
            OnStatusChanged: OnStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILocationEvents {}
windows_core::imp::define_interface!(ILocationPower, ILocationPower_Vtbl, 0x193e7729_ab6b_4b12_8617_7596e1bb191c);
windows_core::imp::interface_hierarchy!(ILocationPower, windows_core::IUnknown);
impl ILocationPower {
    pub unsafe fn Connect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPower_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILocationPower_Impl: windows_core::IUnknownImpl {
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
}
impl ILocationPower_Vtbl {
    pub const fn new<Identity: ILocationPower_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: ILocationPower_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationPower_Impl::Connect(this).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: ILocationPower_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationPower_Impl::Disconnect(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Connect: Connect::<Identity, OFFSET>, Disconnect: Disconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationPower as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILocationPower {}
windows_core::imp::define_interface!(ILocationReport, ILocationReport_Vtbl, 0xc8b7f7ee_75d0_4db9_b62d_7a0f369ca456);
windows_core::imp::interface_hierarchy!(ILocationReport, windows_core::IUnknown);
impl ILocationReport {
    #[cfg(feature = "sensorsapi")]
    pub unsafe fn GetSensorID(&self) -> windows_core::Result<super::SENSOR_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwinbase")]
    pub unsafe fn GetTimestamp(&self) -> windows_core::Result<super::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self, pkey: *const super::PROPERTYKEY) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pkey, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "sensorsapi")]
    pub GetSensorID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SENSOR_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "sensorsapi"))]
    GetSensorID: usize,
    #[cfg(feature = "minwinbase")]
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwinbase"))]
    GetTimestamp: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILocationReport_Impl: windows_core::IUnknownImpl {
    fn GetSensorID(&self) -> windows_core::Result<super::SENSOR_ID>;
    fn GetTimestamp(&self) -> windows_core::Result<super::SYSTEMTIME>;
    fn GetValue(&self, pkey: *const super::PROPERTYKEY) -> windows_core::Result<super::PROPVARIANT>;
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
impl ILocationReport_Vtbl {
    pub const fn new<Identity: ILocationReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSensorID<Identity: ILocationReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensorid: *mut super::SENSOR_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReport_Impl::GetSensorID(this) {
                    Ok(ok__) => {
                        psensorid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimestamp<Identity: ILocationReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcreationtime: *mut super::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReport_Impl::GetTimestamp(this) {
                    Ok(ok__) => {
                        pcreationtime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ILocationReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const super::PROPERTYKEY, pvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReport_Impl::GetValue(this, core::mem::transmute_copy(&pkey)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSensorID: GetSensorID::<Identity, OFFSET>,
            GetTimestamp: GetTimestamp::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationReport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILocationReport {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ILocationReportFactory, ILocationReportFactory_Vtbl, 0x2daec322_90b2_47e4_bb08_0da841935a6b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ILocationReportFactory {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ILocationReportFactory, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ILocationReportFactory {
    pub unsafe fn ListenForReports(&self, requestedreportinterval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ListenForReports)(windows_core::Interface::as_raw(self), requestedreportinterval) }
    }
    pub unsafe fn StopListeningForReports(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopListeningForReports)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReportInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReportInterval(&self, millisecondsrequested: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReportInterval)(windows_core::Interface::as_raw(self), millisecondsrequested) }
    }
    pub unsafe fn DesiredAccuracy(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredAccuracy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredAccuracy)(windows_core::Interface::as_raw(self), desiredaccuracy) }
    }
    pub unsafe fn RequestPermissions(&self, hwnd: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hwnd) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReportFactory_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub ListenForReports: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StopListeningForReports: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILocationReportFactory_Impl: super::IDispatch_Impl {
    fn ListenForReports(&self, requestedreportinterval: u32) -> windows_core::Result<()>;
    fn StopListeningForReports(&self) -> windows_core::Result<()>;
    fn Status(&self) -> windows_core::Result<u32>;
    fn ReportInterval(&self) -> windows_core::Result<u32>;
    fn SetReportInterval(&self, millisecondsrequested: u32) -> windows_core::Result<()>;
    fn DesiredAccuracy(&self) -> windows_core::Result<u32>;
    fn SetDesiredAccuracy(&self, desiredaccuracy: u32) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hwnd: *const u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ILocationReportFactory_Vtbl {
    pub const fn new<Identity: ILocationReportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ListenForReports<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedreportinterval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::ListenForReports(this, core::mem::transmute_copy(&requestedreportinterval)).into()
            }
        }
        unsafe extern "system" fn StopListeningForReports<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::StopListeningForReports(this).into()
            }
        }
        unsafe extern "system" fn Status<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReportFactory_Impl::Status(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportInterval<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmilliseconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReportFactory_Impl::ReportInterval(this) {
                    Ok(ok__) => {
                        pmilliseconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportInterval<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, millisecondsrequested: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::SetReportInterval(this, core::mem::transmute_copy(&millisecondsrequested)).into()
            }
        }
        unsafe extern "system" fn DesiredAccuracy<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesiredaccuracy: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReportFactory_Impl::DesiredAccuracy(this) {
                    Ok(ok__) => {
                        pdesiredaccuracy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredaccuracy: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::SetDesiredAccuracy(this, core::mem::transmute_copy(&desiredaccuracy)).into()
            }
        }
        unsafe extern "system" fn RequestPermissions<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::RequestPermissions(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ListenForReports: ListenForReports::<Identity, OFFSET>,
            StopListeningForReports: StopListeningForReports::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            ReportInterval: ReportInterval::<Identity, OFFSET>,
            SetReportInterval: SetReportInterval::<Identity, OFFSET>,
            DesiredAccuracy: DesiredAccuracy::<Identity, OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Identity, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationReportFactory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILocationReportFactory {}
pub const LOCATION_API_VERSION: u32 = 1;
pub type LOCATION_REPORT_STATUS = i32;
pub const LatLongReport: windows_core::GUID = windows_core::GUID::from_u128(0xed81c073_1f84_4ca8_a161_183c776bc651);
pub const LatLongReportFactory: windows_core::GUID = windows_core::GUID::from_u128(0x9dcc3cc8_8609_4863_bad4_03601f4c65e8);
pub const Location: windows_core::GUID = windows_core::GUID::from_u128(0xe5b8e079_ee6d_4e33_a438_c87f2e959254);
pub const REPORT_ACCESS_DENIED: LOCATION_REPORT_STATUS = 2;
pub const REPORT_ERROR: LOCATION_REPORT_STATUS = 1;
pub const REPORT_INITIALIZING: LOCATION_REPORT_STATUS = 3;
pub const REPORT_NOT_SUPPORTED: LOCATION_REPORT_STATUS = 0;
pub const REPORT_RUNNING: LOCATION_REPORT_STATUS = 4;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(_ICivicAddressReportFactoryEvents, _ICivicAddressReportFactoryEvents_Vtbl, 0xc96039ff_72ec_4617_89bd_84d88bedc722);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for _ICivicAddressReportFactoryEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(_ICivicAddressReportFactoryEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct _ICivicAddressReportFactoryEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait _ICivicAddressReportFactoryEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl _ICivicAddressReportFactoryEvents_Vtbl {
    pub const fn new<Identity: _ICivicAddressReportFactoryEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ICivicAddressReportFactoryEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for _ICivicAddressReportFactoryEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(_ILatLongReportFactoryEvents, _ILatLongReportFactoryEvents_Vtbl, 0x16ee6cb7_ab3c_424b_849f_269be551fcbc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for _ILatLongReportFactoryEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(_ILatLongReportFactoryEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct _ILatLongReportFactoryEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait _ILatLongReportFactoryEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl _ILatLongReportFactoryEvents_Vtbl {
    pub const fn new<Identity: _ILatLongReportFactoryEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ILatLongReportFactoryEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for _ILatLongReportFactoryEvents {}
