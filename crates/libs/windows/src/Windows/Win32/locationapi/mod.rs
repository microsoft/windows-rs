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
    #[cfg(feature = "shared")]
    pub unsafe fn GetDetailLevel(&self) -> windows_core::Result<super::DWORD> {
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
    #[cfg(feature = "shared")]
    pub GetDetailLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDetailLevel: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICivicAddressReport_Impl: ILocationReport_Impl {
    fn GetAddressLine1(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAddressLine2(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetStateProvince(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPostalCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCountryRegion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDetailLevel(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetDetailLevel<Identity: ICivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetaillevel: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_CivicAddressReport(&self) -> windows_core::Result<IDispCivicAddressReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CivicAddressReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddressReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    pub get_CivicAddressReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICivicAddressReportFactory_Impl: ILocationReportFactory_Impl {
    fn get_CivicAddressReport(&self) -> windows_core::Result<IDispCivicAddressReport>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICivicAddressReportFactory_Vtbl {
    pub const fn new<Identity: ICivicAddressReportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CivicAddressReport<Identity: ICivicAddressReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICivicAddressReportFactory_Impl::get_CivicAddressReport(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, OFFSET>(), get_CivicAddressReport: get_CivicAddressReport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICivicAddressReportFactory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ILocationReportFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICivicAddressReportFactory {}
windows_core::imp::define_interface!(IDefaultLocation, IDefaultLocation_Vtbl, 0xa65af77e_969a_4a2e_8aca_33bb7cbb1235);
windows_core::imp::interface_hierarchy!(IDefaultLocation, windows_core::IUnknown);
impl IDefaultLocation {
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetReport<P1>(&self, reporttype: *const super::IID, plocationreport: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ILocationReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReport)(windows_core::Interface::as_raw(self), reporttype, plocationreport.param().abi()) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetReport(&self, reporttype: *const super::IID) -> windows_core::Result<ILocationReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultLocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub SetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetReport: usize,
    #[cfg(feature = "guiddef")]
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetReport: usize,
}
#[cfg(feature = "guiddef")]
pub trait IDefaultLocation_Impl: windows_core::IUnknownImpl {
    fn SetReport(&self, reporttype: *const super::IID, plocationreport: windows_core::Ref<ILocationReport>) -> windows_core::Result<()>;
    fn GetReport(&self, reporttype: *const super::IID) -> windows_core::Result<ILocationReport>;
}
#[cfg(feature = "guiddef")]
impl IDefaultLocation_Vtbl {
    pub const fn new<Identity: IDefaultLocation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetReport<Identity: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, plocationreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDefaultLocation_Impl::SetReport(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&plocationreport)).into()
            }
        }
        unsafe extern "system" fn GetReport<Identity: IDefaultLocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, pplocationreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
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
    pub unsafe fn get_AddressLine1(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AddressLine1)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_AddressLine2(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AddressLine2)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_City(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_City)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_StateProvince(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StateProvince)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_PostalCode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostalCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CountryRegion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CountryRegion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DetailLevel(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DetailLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Timestamp(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispCivicAddressReport_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_AddressLine1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AddressLine2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_City: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_StateProvince: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PostalCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CountryRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_DetailLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DetailLevel: usize,
    #[cfg(feature = "wtypes")]
    pub get_Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Timestamp: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispCivicAddressReport_Impl: super::IDispatch_Impl {
    fn get_AddressLine1(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_AddressLine2(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_City(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_StateProvince(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_PostalCode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CountryRegion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DetailLevel(&self) -> windows_core::Result<super::ULONG>;
    fn get_Timestamp(&self) -> windows_core::Result<super::DATE>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispCivicAddressReport_Vtbl {
    pub const fn new<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AddressLine1<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_AddressLine1(this) {
                    Ok(ok__) => {
                        paddress1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AddressLine2<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_AddressLine2(this) {
                    Ok(ok__) => {
                        paddress2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_City<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_City(this) {
                    Ok(ok__) => {
                        pcity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StateProvince<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstateprovince: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_StateProvince(this) {
                    Ok(ok__) => {
                        pstateprovince.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PostalCode<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppostalcode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_PostalCode(this) {
                    Ok(ok__) => {
                        ppostalcode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CountryRegion<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcountryregion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_CountryRegion(this) {
                    Ok(ok__) => {
                        pcountryregion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DetailLevel<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdetaillevel: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_DetailLevel(this) {
                    Ok(ok__) => {
                        pdetaillevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Timestamp<Identity: IDispCivicAddressReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispCivicAddressReport_Impl::get_Timestamp(this) {
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
            get_AddressLine1: get_AddressLine1::<Identity, OFFSET>,
            get_AddressLine2: get_AddressLine2::<Identity, OFFSET>,
            get_City: get_City::<Identity, OFFSET>,
            get_StateProvince: get_StateProvince::<Identity, OFFSET>,
            get_PostalCode: get_PostalCode::<Identity, OFFSET>,
            get_CountryRegion: get_CountryRegion::<Identity, OFFSET>,
            get_DetailLevel: get_DetailLevel::<Identity, OFFSET>,
            get_Timestamp: get_Timestamp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispCivicAddressReport as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn get_Latitude(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Latitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn get_Longitude(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Longitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn get_ErrorRadius(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ErrorRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn get_Altitude(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Altitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn get_AltitudeError(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AltitudeError)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Timestamp(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispLatLongReport_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub get_Latitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    get_Latitude: usize,
    #[cfg(feature = "wtypesbase")]
    pub get_Longitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    get_Longitude: usize,
    #[cfg(feature = "wtypesbase")]
    pub get_ErrorRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    get_ErrorRadius: usize,
    #[cfg(feature = "wtypesbase")]
    pub get_Altitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    get_Altitude: usize,
    #[cfg(feature = "wtypesbase")]
    pub get_AltitudeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    get_AltitudeError: usize,
    #[cfg(feature = "wtypes")]
    pub get_Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Timestamp: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispLatLongReport_Impl: super::IDispatch_Impl {
    fn get_Latitude(&self) -> windows_core::Result<super::DOUBLE>;
    fn get_Longitude(&self) -> windows_core::Result<super::DOUBLE>;
    fn get_ErrorRadius(&self) -> windows_core::Result<super::DOUBLE>;
    fn get_Altitude(&self) -> windows_core::Result<super::DOUBLE>;
    fn get_AltitudeError(&self) -> windows_core::Result<super::DOUBLE>;
    fn get_Timestamp(&self) -> windows_core::Result<super::DATE>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispLatLongReport_Vtbl {
    pub const fn new<Identity: IDispLatLongReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Latitude<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DOUBLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::get_Latitude(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Longitude<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DOUBLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::get_Longitude(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ErrorRadius<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DOUBLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::get_ErrorRadius(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Altitude<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DOUBLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::get_Altitude(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AltitudeError<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DOUBLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::get_AltitudeError(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Timestamp<Identity: IDispLatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispLatLongReport_Impl::get_Timestamp(this) {
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
            get_Latitude: get_Latitude::<Identity, OFFSET>,
            get_Longitude: get_Longitude::<Identity, OFFSET>,
            get_ErrorRadius: get_ErrorRadius::<Identity, OFFSET>,
            get_Altitude: get_Altitude::<Identity, OFFSET>,
            get_AltitudeError: get_AltitudeError::<Identity, OFFSET>,
            get_Timestamp: get_Timestamp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispLatLongReport as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetLatitude(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetLongitude(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLongitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetErrorRadius(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetAltitude(&self) -> windows_core::Result<super::DOUBLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAltitude)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn GetAltitudeError(&self) -> windows_core::Result<super::DOUBLE> {
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
    #[cfg(feature = "wtypesbase")]
    pub GetLatitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetLatitude: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetLongitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetLongitude: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetErrorRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetErrorRadius: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetAltitude: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetAltitude: usize,
    #[cfg(feature = "wtypesbase")]
    pub GetAltitudeError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    GetAltitudeError: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILatLongReport_Impl: ILocationReport_Impl {
    fn GetLatitude(&self) -> windows_core::Result<super::DOUBLE>;
    fn GetLongitude(&self) -> windows_core::Result<super::DOUBLE>;
    fn GetErrorRadius(&self) -> windows_core::Result<super::DOUBLE>;
    fn GetAltitude(&self) -> windows_core::Result<super::DOUBLE>;
    fn GetAltitudeError(&self) -> windows_core::Result<super::DOUBLE>;
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ILatLongReport_Vtbl {
    pub const fn new<Identity: ILatLongReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLatitude<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platitude: *mut super::DOUBLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetLongitude<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plongitude: *mut super::DOUBLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetErrorRadius<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorradius: *mut super::DOUBLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAltitude<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltitude: *mut super::DOUBLE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAltitudeError<Identity: ILatLongReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltitudeerror: *mut super::DOUBLE) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn get_LatLongReport(&self) -> windows_core::Result<IDispLatLongReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LatLongReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ILatLongReportFactory_Vtbl {
    pub base__: ILocationReportFactory_Vtbl,
    pub get_LatLongReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILatLongReportFactory_Impl: ILocationReportFactory_Impl {
    fn get_LatLongReport(&self) -> windows_core::Result<IDispLatLongReport>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ILatLongReportFactory_Vtbl {
    pub const fn new<Identity: ILatLongReportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LatLongReport<Identity: ILatLongReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILatLongReportFactory_Impl::get_LatLongReport(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ILocationReportFactory_Vtbl::new::<Identity, OFFSET>(), get_LatLongReport: get_LatLongReport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILatLongReportFactory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ILocationReportFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILatLongReportFactory {}
windows_core::imp::define_interface!(ILocation, ILocation_Vtbl, 0xab2ece69_56d9_4f28_b525_de1b0ee44237);
windows_core::imp::interface_hierarchy!(ILocation, windows_core::IUnknown);
impl ILocation {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn RegisterForReport<P0>(&self, pevents: P0, reporttype: *const super::IID, dwrequestedreportinterval: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ILocationEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterForReport)(windows_core::Interface::as_raw(self), pevents.param().abi(), reporttype, dwrequestedreportinterval) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn UnregisterForReport(&self, reporttype: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterForReport)(windows_core::Interface::as_raw(self), reporttype) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetReport(&self, reporttype: *const super::IID) -> windows_core::Result<ILocationReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), reporttype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetReportStatus(&self, reporttype: *const super::IID) -> windows_core::Result<LOCATION_REPORT_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportStatus)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetReportInterval(&self, reporttype: *const super::IID) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportInterval)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetReportInterval(&self, reporttype: *const super::IID, millisecondsrequested: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReportInterval)(windows_core::Interface::as_raw(self), reporttype, millisecondsrequested) }
    }
    #[cfg(all(feature = "guiddef", feature = "sensorsapi"))]
    pub unsafe fn GetDesiredAccuracy(&self, reporttype: *const super::IID) -> windows_core::Result<super::LOCATION_DESIRED_ACCURACY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesiredAccuracy)(windows_core::Interface::as_raw(self), reporttype, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "sensorsapi"))]
    pub unsafe fn SetDesiredAccuracy(&self, reporttype: *const super::IID, desiredaccuracy: super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredAccuracy)(windows_core::Interface::as_raw(self), reporttype, desiredaccuracy) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
    pub unsafe fn RequestPermissions(&self, hparent: Option<super::HWND>, preporttypes: &[super::IID], fmodal: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hparent.unwrap_or(core::mem::zeroed()) as _, preporttypes.as_ptr(), preporttypes.len().try_into().unwrap(), fmodal.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub RegisterForReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    RegisterForReport: usize,
    #[cfg(feature = "guiddef")]
    pub UnregisterForReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    UnregisterForReport: usize,
    #[cfg(feature = "guiddef")]
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetReport: usize,
    #[cfg(feature = "guiddef")]
    pub GetReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut LOCATION_REPORT_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetReportStatus: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetReportInterval: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetReportInterval: usize,
    #[cfg(all(feature = "guiddef", feature = "sensorsapi"))]
    pub GetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "sensorsapi")))]
    GetDesiredAccuracy: usize,
    #[cfg(all(feature = "guiddef", feature = "sensorsapi"))]
    pub SetDesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "sensorsapi")))]
    SetDesiredAccuracy: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "windef"))]
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const super::IID, super::ULONG, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "windef")))]
    RequestPermissions: usize,
}
#[cfg(all(feature = "guiddef", feature = "sensorsapi", feature = "shared", feature = "windef"))]
pub trait ILocation_Impl: windows_core::IUnknownImpl {
    fn RegisterForReport(&self, pevents: windows_core::Ref<ILocationEvents>, reporttype: *const super::IID, dwrequestedreportinterval: super::DWORD) -> windows_core::Result<()>;
    fn UnregisterForReport(&self, reporttype: *const super::IID) -> windows_core::Result<()>;
    fn GetReport(&self, reporttype: *const super::IID) -> windows_core::Result<ILocationReport>;
    fn GetReportStatus(&self, reporttype: *const super::IID) -> windows_core::Result<LOCATION_REPORT_STATUS>;
    fn GetReportInterval(&self, reporttype: *const super::IID) -> windows_core::Result<super::DWORD>;
    fn SetReportInterval(&self, reporttype: *const super::IID, millisecondsrequested: super::DWORD) -> windows_core::Result<()>;
    fn GetDesiredAccuracy(&self, reporttype: *const super::IID) -> windows_core::Result<super::LOCATION_DESIRED_ACCURACY>;
    fn SetDesiredAccuracy(&self, reporttype: *const super::IID, desiredaccuracy: super::LOCATION_DESIRED_ACCURACY) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hparent: super::HWND, preporttypes: *const super::IID, count: super::ULONG, fmodal: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "sensorsapi", feature = "shared", feature = "windef"))]
impl ILocation_Vtbl {
    pub const fn new<Identity: ILocation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterForReport<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void, reporttype: *const super::IID, dwrequestedreportinterval: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::RegisterForReport(this, core::mem::transmute_copy(&pevents), core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&dwrequestedreportinterval)).into()
            }
        }
        unsafe extern "system" fn UnregisterForReport<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::UnregisterForReport(this, core::mem::transmute_copy(&reporttype)).into()
            }
        }
        unsafe extern "system" fn GetReport<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, pplocationreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetReportStatus<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, pstatus: *mut LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetReportInterval<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, pmilliseconds: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetReportInterval<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, millisecondsrequested: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::SetReportInterval(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&millisecondsrequested)).into()
            }
        }
        unsafe extern "system" fn GetDesiredAccuracy<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, pdesiredaccuracy: *mut super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetDesiredAccuracy<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, desiredaccuracy: super::LOCATION_DESIRED_ACCURACY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocation_Impl::SetDesiredAccuracy(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&desiredaccuracy)).into()
            }
        }
        unsafe extern "system" fn RequestPermissions<Identity: ILocation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::HWND, preporttypes: *const super::IID, count: super::ULONG, fmodal: windows_core::BOOL) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "sensorsapi", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for ILocation {}
windows_core::imp::define_interface!(ILocationEvents, ILocationEvents_Vtbl, 0xcae02bbf_798b_4508_a207_35a7906dc73d);
windows_core::imp::interface_hierarchy!(ILocationEvents, windows_core::IUnknown);
impl ILocationEvents {
    #[cfg(feature = "guiddef")]
    pub unsafe fn OnLocationChanged<P1>(&self, reporttype: *const super::IID, plocationreport: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ILocationReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnLocationChanged)(windows_core::Interface::as_raw(self), reporttype, plocationreport.param().abi()) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn OnStatusChanged(&self, reporttype: *const super::IID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChanged)(windows_core::Interface::as_raw(self), reporttype, newstatus) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub OnLocationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    OnLocationChanged: usize,
    #[cfg(feature = "guiddef")]
    pub OnStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, LOCATION_REPORT_STATUS) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    OnStatusChanged: usize,
}
#[cfg(feature = "guiddef")]
pub trait ILocationEvents_Impl: windows_core::IUnknownImpl {
    fn OnLocationChanged(&self, reporttype: *const super::IID, plocationreport: windows_core::Ref<ILocationReport>) -> windows_core::Result<()>;
    fn OnStatusChanged(&self, reporttype: *const super::IID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl ILocationEvents_Vtbl {
    pub const fn new<Identity: ILocationEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLocationChanged<Identity: ILocationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, plocationreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationEvents_Impl::OnLocationChanged(this, core::mem::transmute_copy(&reporttype), core::mem::transmute_copy(&plocationreport)).into()
            }
        }
        unsafe extern "system" fn OnStatusChanged<Identity: ILocationEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reporttype: *const super::IID, newstatus: LOCATION_REPORT_STATUS) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
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
    #[cfg(all(feature = "minwinbase", feature = "shared"))]
    pub unsafe fn GetTimestamp(&self) -> windows_core::Result<super::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwinbase", feature = "shared"))]
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared")))]
    GetTimestamp: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPERTYKEY, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILocationReport_Impl: windows_core::IUnknownImpl {
    fn GetSensorID(&self) -> windows_core::Result<super::SENSOR_ID>;
    fn GetTimestamp(&self) -> windows_core::Result<super::SYSTEMTIME>;
    fn GetValue(&self, pkey: *const super::PROPERTYKEY) -> windows_core::Result<super::PROPVARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "sensorsapi", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn ListenForReports(&self, requestedreportinterval: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ListenForReports)(windows_core::Interface::as_raw(self), requestedreportinterval) }
    }
    pub unsafe fn StopListeningForReports(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopListeningForReports)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Status(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ReportInterval(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReportInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_ReportInterval(&self, millisecondsrequested: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReportInterval)(windows_core::Interface::as_raw(self), millisecondsrequested) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DesiredAccuracy(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DesiredAccuracy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_DesiredAccuracy(&self, desiredaccuracy: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DesiredAccuracy)(windows_core::Interface::as_raw(self), desiredaccuracy) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RequestPermissions(&self, hwnd: *const super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hwnd) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ILocationReportFactory_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub ListenForReports: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ListenForReports: usize,
    pub StopListeningForReports: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Status: usize,
    #[cfg(feature = "shared")]
    pub get_ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ReportInterval: usize,
    #[cfg(feature = "shared")]
    pub put_ReportInterval: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_ReportInterval: usize,
    #[cfg(feature = "shared")]
    pub get_DesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DesiredAccuracy: usize,
    #[cfg(feature = "shared")]
    pub put_DesiredAccuracy: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_DesiredAccuracy: usize,
    #[cfg(feature = "shared")]
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RequestPermissions: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILocationReportFactory_Impl: super::IDispatch_Impl {
    fn ListenForReports(&self, requestedreportinterval: super::ULONG) -> windows_core::Result<()>;
    fn StopListeningForReports(&self) -> windows_core::Result<()>;
    fn get_Status(&self) -> windows_core::Result<super::ULONG>;
    fn get_ReportInterval(&self) -> windows_core::Result<super::ULONG>;
    fn put_ReportInterval(&self, millisecondsrequested: super::ULONG) -> windows_core::Result<()>;
    fn get_DesiredAccuracy(&self) -> windows_core::Result<super::ULONG>;
    fn put_DesiredAccuracy(&self, desiredaccuracy: super::ULONG) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hwnd: *const super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ILocationReportFactory_Vtbl {
    pub const fn new<Identity: ILocationReportFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ListenForReports<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedreportinterval: super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_Status<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReportFactory_Impl::get_Status(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReportInterval<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmilliseconds: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReportFactory_Impl::get_ReportInterval(this) {
                    Ok(ok__) => {
                        pmilliseconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReportInterval<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, millisecondsrequested: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::put_ReportInterval(this, core::mem::transmute_copy(&millisecondsrequested)).into()
            }
        }
        unsafe extern "system" fn get_DesiredAccuracy<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesiredaccuracy: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationReportFactory_Impl::get_DesiredAccuracy(this) {
                    Ok(ok__) => {
                        pdesiredaccuracy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DesiredAccuracy<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredaccuracy: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::put_DesiredAccuracy(this, core::mem::transmute_copy(&desiredaccuracy)).into()
            }
        }
        unsafe extern "system" fn RequestPermissions<Identity: ILocationReportFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *const super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationReportFactory_Impl::RequestPermissions(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ListenForReports: ListenForReports::<Identity, OFFSET>,
            StopListeningForReports: StopListeningForReports::<Identity, OFFSET>,
            get_Status: get_Status::<Identity, OFFSET>,
            get_ReportInterval: get_ReportInterval::<Identity, OFFSET>,
            put_ReportInterval: put_ReportInterval::<Identity, OFFSET>,
            get_DesiredAccuracy: get_DesiredAccuracy::<Identity, OFFSET>,
            put_DesiredAccuracy: put_DesiredAccuracy::<Identity, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationReportFactory as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILocationReportFactory {}
pub const LOCATION_API_VERSION: i32 = 1;
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait _ICivicAddressReportFactoryEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl _ICivicAddressReportFactoryEvents_Vtbl {
    pub const fn new<Identity: _ICivicAddressReportFactoryEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ICivicAddressReportFactoryEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait _ILatLongReportFactoryEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl _ILatLongReportFactoryEvents_Vtbl {
    pub const fn new<Identity: _ILatLongReportFactoryEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ILatLongReportFactoryEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for _ILatLongReportFactoryEvents {}
