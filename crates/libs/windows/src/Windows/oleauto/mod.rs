#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn BstrFromVector(psa: *const super::oaidl::SAFEARRAY) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn BstrFromVector(psa : *const super::oaidl::SAFEARRAY, pbstr : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        BstrFromVector(psa, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ClearCustData() -> super::oaidl::CUSTDATA {
    windows_core::link!("oleaut32.dll" "system" fn ClearCustData(pcustdata : *mut super::oaidl::CUSTDATA));
    unsafe {
        let mut result__ = core::mem::zeroed();
        ClearCustData(&mut result__);
        result__
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: super::winnt::LCID, pptinfo: *mut Option<super::oaidl::ITypeInfo>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn CreateDispTypeInfo(pidata : *mut INTERFACEDATA, lcid : super::winnt::LCID, pptinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateDispTypeInfo(pidata as _, lcid, core::mem::transmute(pptinfo)) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn CreateErrorInfo() -> windows_core::Result<super::oaidl::ICreateErrorInfo> {
    windows_core::link!("oleaut32.dll" "system" fn CreateErrorInfo(pperrinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateErrorInfo(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn CreateStdDispatch<P0, P2>(punkouter: P0, pvthis: *mut core::ffi::c_void, ptinfo: P2, ppunkstddisp: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<super::oaidl::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn CreateStdDispatch(punkouter : *mut core::ffi::c_void, pvthis : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, ppunkstddisp : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateStdDispatch(punkouter.param().abi(), pvthis as _, ptinfo.param().abi(), core::mem::transmute(ppunkstddisp)) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn CreateTypeLib<P1>(syskind: super::oaidl::SYSKIND, szfile: P1) -> windows_core::Result<super::oaidl::ICreateTypeLib>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn CreateTypeLib(syskind : super::oaidl::SYSKIND, szfile : windows_core::PCWSTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateTypeLib(syskind, szfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn CreateTypeLib2<P1>(syskind: super::oaidl::SYSKIND, szfile: P1) -> windows_core::Result<super::oaidl::ICreateTypeLib2>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn CreateTypeLib2(syskind : super::oaidl::SYSKIND, szfile : windows_core::PCWSTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateTypeLib2(syskind, szfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispCallFunc(pvinstance: Option<*const core::ffi::c_void>, ovft: usize, cc: super::oaidl::CALLCONV, vtreturn: super::wtypes::VARTYPE, cactuals: u32, prgvt: *const super::wtypes::VARTYPE, prgpvarg: *const *const super::oaidl::VARIANTARG) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn DispCallFunc(pvinstance : *const core::ffi::c_void, ovft : usize, cc : super::oaidl::CALLCONV, vtreturn : super::wtypes::VARTYPE, cactuals : u32, prgvt : *const super::wtypes::VARTYPE, prgpvarg : *const *const super::oaidl::VARIANTARG, pvargresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DispCallFunc(pvinstance.unwrap_or(core::mem::zeroed()) as _, ovft, cc, vtreturn, cactuals, prgvt, prgpvarg, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn DispGetIDsOfNames<P0>(ptinfo: P0, rgsznames: *const windows_core::PCWSTR, cnames: u32, rgdispid: *mut super::oaidl::DISPID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn DispGetIDsOfNames(ptinfo : *mut core::ffi::c_void, rgsznames : *const windows_core::PCWSTR, cnames : u32, rgdispid : *mut super::oaidl::DISPID) -> windows_core::HRESULT);
    unsafe { DispGetIDsOfNames(ptinfo.param().abi(), rgsznames, cnames, rgdispid as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispGetParam(pdispparams: *const super::oaidl::DISPPARAMS, position: u32, vttarg: super::wtypes::VARTYPE, pvarresult: *mut super::oaidl::VARIANT, puargerr: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn DispGetParam(pdispparams : *const super::oaidl::DISPPARAMS, position : u32, vttarg : super::wtypes::VARTYPE, pvarresult : *mut super::oaidl::VARIANT, puargerr : *mut u32) -> windows_core::HRESULT);
    unsafe { DispGetParam(pdispparams, position, vttarg, core::mem::transmute(pvarresult), puargerr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispInvoke<P1>(_this: *mut core::ffi::c_void, ptinfo: P1, dispidmember: super::oaidl::DISPID, wflags: u16, pparams: *mut super::oaidl::DISPPARAMS, pvarresult: *mut super::oaidl::VARIANT, pexcepinfo: *mut super::oaidl::EXCEPINFO, puargerr: *mut u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::oaidl::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn DispInvoke(_this : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, dispidmember : super::oaidl::DISPID, wflags : u16, pparams : *mut super::oaidl::DISPPARAMS, pvarresult : *mut super::oaidl::VARIANT, pexcepinfo : *mut super::oaidl::EXCEPINFO, puargerr : *mut u32) -> windows_core::HRESULT);
    unsafe { DispInvoke(_this as _, ptinfo.param().abi(), dispidmember, wflags, pparams as _, core::mem::transmute(pvarresult), core::mem::transmute(pexcepinfo), puargerr as _) }
}
#[inline]
pub unsafe fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn DosDateTimeToVariantTime(wdosdate : u16, wdostime : u16, pvtime : *mut f64) -> i32);
    unsafe { DosDateTimeToVariantTime(wdosdate, wdostime, pvtime as _) }
}
#[inline]
pub unsafe fn GetActiveObject(rclsid: *const windows_core::GUID, pvreserved: *mut core::ffi::c_void, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn GetActiveObject(rclsid : *const windows_core::GUID, pvreserved : *mut core::ffi::c_void, ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetActiveObject(rclsid, pvreserved as _, core::mem::transmute(ppunk)) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAltMonthNames(lcid: super::winnt::LCID) -> windows_core::Result<*mut windows_core::PWSTR> {
    windows_core::link!("oleaut32.dll" "system" fn GetAltMonthNames(lcid : super::winnt::LCID, prgp : *mut *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetAltMonthNames(lcid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> windows_core::Result<super::oaidl::IErrorInfo> {
    windows_core::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetErrorInfo(dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn GetRecordInfoFromGuids(rguidtypelib: *const windows_core::GUID, uvermajor: u32, uverminor: u32, lcid: super::winnt::LCID, rguidtypeinfo: *const windows_core::GUID) -> windows_core::Result<super::oaidl::IRecordInfo> {
    windows_core::link!("oleaut32.dll" "system" fn GetRecordInfoFromGuids(rguidtypelib : *const windows_core::GUID, uvermajor : u32, uverminor : u32, lcid : super::winnt::LCID, rguidtypeinfo : *const windows_core::GUID, pprecinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRecordInfoFromGuids(rguidtypelib, uvermajor, uverminor, lcid, rguidtypeinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn GetRecordInfoFromTypeInfo<P0>(ptypeinfo: P0) -> windows_core::Result<super::oaidl::IRecordInfo>
where
    P0: windows_core::Param<super::oaidl::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn GetRecordInfoFromTypeInfo(ptypeinfo : *mut core::ffi::c_void, pprecinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRecordInfoFromTypeInfo(ptypeinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypesbase"))]
#[inline]
pub unsafe fn LHashValOfNameSys(syskind: super::oaidl::SYSKIND, lcid: super::winnt::LCID, szname: *const super::wtypesbase::OLECHAR) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn LHashValOfNameSys(syskind : super::oaidl::SYSKIND, lcid : super::winnt::LCID, szname : *const super::wtypesbase::OLECHAR) -> u32);
    unsafe { LHashValOfNameSys(syskind, lcid, szname) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LHashValOfNameSysA<P2>(syskind: super::oaidl::SYSKIND, lcid: super::winnt::LCID, szname: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn LHashValOfNameSysA(syskind : super::oaidl::SYSKIND, lcid : super::winnt::LCID, szname : windows_core::PCSTR) -> u32);
    unsafe { LHashValOfNameSysA(syskind, lcid, szname.param().abi()) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LoadRegTypeLib(rguid: *const windows_core::GUID, wvermajor: u16, wverminor: u16, lcid: super::winnt::LCID) -> windows_core::Result<super::oaidl::ITypeLib> {
    windows_core::link!("oleaut32.dll" "system" fn LoadRegTypeLib(rguid : *const windows_core::GUID, wvermajor : u16, wverminor : u16, lcid : super::winnt::LCID, pptlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadRegTypeLib(rguid, wvermajor, wverminor, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn LoadTypeLib<P0>(szfile: P0) -> windows_core::Result<super::oaidl::ITypeLib>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn LoadTypeLib(szfile : windows_core::PCWSTR, pptlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadTypeLib(szfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn LoadTypeLibEx<P0>(szfile: P0, regkind: REGKIND) -> windows_core::Result<super::oaidl::ITypeLib>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn LoadTypeLibEx(szfile : windows_core::PCWSTR, regkind : REGKIND, pptlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        LoadTypeLibEx(szfile.param().abi(), regkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn OaBuildVersion() -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn OaBuildVersion() -> u32);
    unsafe { OaBuildVersion() }
}
#[inline]
pub unsafe fn OaEnablePerUserTLibRegistration() {
    windows_core::link!("oleaut32.dll" "system" fn OaEnablePerUserTLibRegistration());
    unsafe { OaEnablePerUserTLibRegistration() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryPathOfRegTypeLib(guid: *const windows_core::GUID, wmaj: u16, wmin: u16, lcid: super::winnt::LCID) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn QueryPathOfRegTypeLib(guid : *const windows_core::GUID, wmaj : u16, wmin : u16, lcid : super::winnt::LCID, lpbstrpathname : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        QueryPathOfRegTypeLib(guid, wmaj, wmin, lcid, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn RegisterActiveObject<P0>(punk: P0, rclsid: *const windows_core::GUID, dwflags: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("oleaut32.dll" "system" fn RegisterActiveObject(punk : *mut core::ffi::c_void, rclsid : *const windows_core::GUID, dwflags : u32, pdwregister : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RegisterActiveObject(punk.param().abi(), rclsid, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn RegisterTypeLib<P0, P1, P2>(ptlib: P0, szfullpath: P1, szhelpdir: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::ITypeLib>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn RegisterTypeLib(ptlib : *mut core::ffi::c_void, szfullpath : windows_core::PCWSTR, szhelpdir : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { RegisterTypeLib(ptlib.param().abi(), szfullpath.param().abi(), szhelpdir.param().abi()) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn RegisterTypeLibForUser<P0>(ptlib: P0, szfullpath: *const super::wtypesbase::OLECHAR, szhelpdir: Option<*const super::wtypesbase::OLECHAR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::oaidl::ITypeLib>,
{
    windows_core::link!("oleaut32.dll" "system" fn RegisterTypeLibForUser(ptlib : *mut core::ffi::c_void, szfullpath : *const super::wtypesbase::OLECHAR, szhelpdir : *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT);
    unsafe { RegisterTypeLibForUser(ptlib.param().abi(), szfullpath, szhelpdir.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RevokeActiveObject(dwregister: u32, pvreserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn RevokeActiveObject(dwregister : u32, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RevokeActiveObject(dwregister, pvreserved as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayAccessData(psa: *const super::oaidl::SAFEARRAY, ppvdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAccessData(psa : *const super::oaidl::SAFEARRAY, ppvdata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayAccessData(psa, ppvdata as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayAddRef(psa: *const super::oaidl::SAFEARRAY, ppdatatorelease: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAddRef(psa : *const super::oaidl::SAFEARRAY, ppdatatorelease : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayAddRef(psa, ppdatatorelease as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayAllocData(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAllocData(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayAllocData(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayAllocDescriptor(cdims: u32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptor(cdims : u32, ppsaout : *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayAllocDescriptor(cdims, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayAllocDescriptorEx(vt: super::wtypes::VARTYPE, cdims: u32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptorEx(vt : super::wtypes::VARTYPE, cdims : u32, ppsaout : *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayAllocDescriptorEx(vt, cdims, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayCopy(psa: *const super::oaidl::SAFEARRAY) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCopy(psa : *const super::oaidl::SAFEARRAY, ppsaout : *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayCopy(psa, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayCopyData(psasource: *const super::oaidl::SAFEARRAY, psatarget: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCopyData(psasource : *const super::oaidl::SAFEARRAY, psatarget : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayCopyData(psasource, psatarget) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreate(vt: super::wtypes::VARTYPE, cdims: u32, rgsabound: *const super::oaidl::SAFEARRAYBOUND) -> *mut super::oaidl::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreate(vt : super::wtypes::VARTYPE, cdims : u32, rgsabound : *const super::oaidl::SAFEARRAYBOUND) -> *mut super::oaidl::SAFEARRAY);
    unsafe { SafeArrayCreate(vt, cdims, rgsabound) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreateEx(vt: super::wtypes::VARTYPE, cdims: u32, rgsabound: *const super::oaidl::SAFEARRAYBOUND, pvextra: *const core::ffi::c_void) -> *mut super::oaidl::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreateEx(vt : super::wtypes::VARTYPE, cdims : u32, rgsabound : *const super::oaidl::SAFEARRAYBOUND, pvextra : *const core::ffi::c_void) -> *mut super::oaidl::SAFEARRAY);
    unsafe { SafeArrayCreateEx(vt, cdims, rgsabound, pvextra) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreateVector(vt: super::wtypes::VARTYPE, llbound: i32, celements: u32) -> *mut super::oaidl::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreateVector(vt : super::wtypes::VARTYPE, llbound : i32, celements : u32) -> *mut super::oaidl::SAFEARRAY);
    unsafe { SafeArrayCreateVector(vt, llbound, celements) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreateVectorEx(vt: super::wtypes::VARTYPE, llbound: i32, celements: u32, pvextra: *const core::ffi::c_void) -> *mut super::oaidl::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreateVectorEx(vt : super::wtypes::VARTYPE, llbound : i32, celements : u32, pvextra : *const core::ffi::c_void) -> *mut super::oaidl::SAFEARRAY);
    unsafe { SafeArrayCreateVectorEx(vt, llbound, celements, pvextra) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayDestroy(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayDestroy(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayDestroy(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayDestroyData(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayDestroyData(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayDestroyData(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayDestroyDescriptor(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayDestroyDescriptor(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayDestroyDescriptor(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetDim(psa: *const super::oaidl::SAFEARRAY) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetDim(psa : *const super::oaidl::SAFEARRAY) -> u32);
    unsafe { SafeArrayGetDim(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetElement(psa: *const super::oaidl::SAFEARRAY, rgindices: *const i32, pv: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetElement(psa : *const super::oaidl::SAFEARRAY, rgindices : *const i32, pv : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayGetElement(psa, rgindices, pv as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetElemsize(psa: *const super::oaidl::SAFEARRAY) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetElemsize(psa : *const super::oaidl::SAFEARRAY) -> u32);
    unsafe { SafeArrayGetElemsize(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetIID(psa: *const super::oaidl::SAFEARRAY) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetIID(psa : *const super::oaidl::SAFEARRAY, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayGetIID(psa, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetLBound(psa: *const super::oaidl::SAFEARRAY, ndim: u32) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetLBound(psa : *const super::oaidl::SAFEARRAY, ndim : u32, pllbound : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayGetLBound(psa, ndim, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetRecordInfo(psa: *const super::oaidl::SAFEARRAY) -> windows_core::Result<super::oaidl::IRecordInfo> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetRecordInfo(psa : *const super::oaidl::SAFEARRAY, prinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayGetRecordInfo(psa, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayGetUBound(psa: *const super::oaidl::SAFEARRAY, ndim: u32) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetUBound(psa : *const super::oaidl::SAFEARRAY, ndim : u32, plubound : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayGetUBound(psa, ndim, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayGetVartype(psa: *const super::oaidl::SAFEARRAY) -> windows_core::Result<super::wtypes::VARTYPE> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetVartype(psa : *const super::oaidl::SAFEARRAY, pvt : *mut super::wtypes::VARTYPE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayGetVartype(psa, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayLock(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayLock(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayLock(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayPtrOfIndex(psa: *const super::oaidl::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayPtrOfIndex(psa : *const super::oaidl::SAFEARRAY, rgindices : *const i32, ppvdata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayPtrOfIndex(psa, rgindices, ppvdata as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayPutElement(psa: *const super::oaidl::SAFEARRAY, rgindices: *const i32, pv: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayPutElement(psa : *const super::oaidl::SAFEARRAY, rgindices : *const i32, pv : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayPutElement(psa, rgindices, pv) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayRedim(psa: *mut super::oaidl::SAFEARRAY, psaboundnew: *const super::oaidl::SAFEARRAYBOUND) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayRedim(psa : *mut super::oaidl::SAFEARRAY, psaboundnew : *const super::oaidl::SAFEARRAYBOUND) -> windows_core::HRESULT);
    unsafe { SafeArrayRedim(psa as _, psaboundnew) }
}
#[inline]
pub unsafe fn SafeArrayReleaseData(pdata: *const core::ffi::c_void) {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayReleaseData(pdata : *const core::ffi::c_void));
    unsafe { SafeArrayReleaseData(pdata) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayReleaseDescriptor(psa: *const super::oaidl::SAFEARRAY) {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayReleaseDescriptor(psa : *const super::oaidl::SAFEARRAY));
    unsafe { SafeArrayReleaseDescriptor(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArraySetIID(psa: *const super::oaidl::SAFEARRAY, guid: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArraySetIID(psa : *const super::oaidl::SAFEARRAY, guid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SafeArraySetIID(psa, guid) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArraySetRecordInfo<P1>(psa: *const super::oaidl::SAFEARRAY, prinfo: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::oaidl::IRecordInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn SafeArraySetRecordInfo(psa : *const super::oaidl::SAFEARRAY, prinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArraySetRecordInfo(psa, prinfo.param().abi()) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayUnaccessData(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayUnaccessData(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayUnaccessData(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SafeArrayUnlock(psa: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayUnlock(psa : *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayUnlock(psa) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn SetErrorInfo<P1>(dwreserved: u32, perrinfo: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::oaidl::IErrorInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetErrorInfo(dwreserved, perrinfo.param().abi()) }
}
#[inline]
pub unsafe fn SysAddRefString(bstrstring: &windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SysAddRefString(bstrstring : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SysAddRefString(core::mem::transmute_copy(bstrstring)) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn SysAllocString(psz: Option<*const super::wtypesbase::OLECHAR>) -> windows_core::BSTR {
    windows_core::link!("oleaut32.dll" "system" fn SysAllocString(psz : *const super::wtypesbase::OLECHAR) -> windows_core::BSTR);
    unsafe { SysAllocString(psz.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SysAllocStringByteLen<P0>(psz: P0, len: u32) -> windows_core::BSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn SysAllocStringByteLen(psz : windows_core::PCSTR, len : u32) -> windows_core::BSTR);
    unsafe { SysAllocStringByteLen(psz.param().abi(), len) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn SysAllocStringLen(strin: Option<&[super::wtypesbase::OLECHAR]>) -> windows_core::BSTR {
    windows_core::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : *const super::wtypesbase::OLECHAR, ui : u32) -> windows_core::BSTR);
    unsafe { SysAllocStringLen(core::mem::transmute(strin.map_or(core::ptr::null(), |slice| slice.as_ptr())), strin.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn SysFreeString(bstrstring: &windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : *mut core::ffi::c_void));
    unsafe { SysFreeString(core::mem::transmute_copy(bstrstring)) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn SysReAllocString(pbstr: *mut windows_core::BSTR, psz: Option<*const super::wtypesbase::OLECHAR>) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn SysReAllocString(pbstr : *mut *mut core::ffi::c_void, psz : *const super::wtypesbase::OLECHAR) -> i32);
    unsafe { SysReAllocString(core::mem::transmute(pbstr), psz.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn SysReAllocStringLen(pbstr: *mut windows_core::BSTR, psz: Option<*const super::wtypesbase::OLECHAR>, len: u32) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn SysReAllocStringLen(pbstr : *mut *mut core::ffi::c_void, psz : *const super::wtypesbase::OLECHAR, len : u32) -> i32);
    unsafe { SysReAllocStringLen(core::mem::transmute(pbstr), psz.unwrap_or(core::mem::zeroed()) as _, len) }
}
#[inline]
pub unsafe fn SysReleaseString(bstrstring: &windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn SysReleaseString(bstrstring : *mut core::ffi::c_void));
    unsafe { SysReleaseString(core::mem::transmute_copy(bstrstring)) }
}
#[inline]
pub unsafe fn SysStringByteLen(bstr: &windows_core::BSTR) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn SysStringByteLen(bstr : *mut core::ffi::c_void) -> u32);
    unsafe { SysStringByteLen(core::mem::transmute_copy(bstr)) }
}
#[inline]
pub unsafe fn SysStringLen(pbstr: &windows_core::BSTR) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : *mut core::ffi::c_void) -> u32);
    unsafe { SysStringLen(core::mem::transmute_copy(pbstr)) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SystemTimeToVariantTime(lpsystemtime: *const super::minwinbase::SYSTEMTIME, pvtime: *mut f64) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn SystemTimeToVariantTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME, pvtime : *mut f64) -> i32);
    unsafe { SystemTimeToVariantTime(lpsystemtime, pvtime as _) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn UnRegisterTypeLib(libid: *const windows_core::GUID, wvermajor: u16, wverminor: u16, lcid: super::winnt::LCID, syskind: super::oaidl::SYSKIND) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn UnRegisterTypeLib(libid : *const windows_core::GUID, wvermajor : u16, wverminor : u16, lcid : super::winnt::LCID, syskind : super::oaidl::SYSKIND) -> windows_core::HRESULT);
    unsafe { UnRegisterTypeLib(libid, wvermajor, wverminor, lcid, syskind) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn UnRegisterTypeLibForUser(libid: *const windows_core::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: super::winnt::LCID, syskind: super::oaidl::SYSKIND) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn UnRegisterTypeLibForUser(libid : *const windows_core::GUID, wmajorvernum : u16, wminorvernum : u16, lcid : super::winnt::LCID, syskind : super::oaidl::SYSKIND) -> windows_core::HRESULT);
    unsafe { UnRegisterTypeLibForUser(libid, wmajorvernum, wminorvernum, lcid, syskind) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarAbs(pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarAbs(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarAbs(core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarAdd(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarAdd(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarAdd(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarAnd(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarAnd(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarAnd(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromCy(cyin: super::wtypes::CY) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromCy(cyin : super::wtypes::CY, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromDate(datein: f64) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromDate(datein : f64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromDec(pdecin : *const super::wtypes::DECIMAL, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<super::wtypes::VARIANT_BOOL>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromI1(cin: i8) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI1(cin : i8, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromI1(cin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromI2(sin: i16) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI2(sin : i16, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromI2(sin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromI4(lin: i32) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI4(lin : i32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromI4(lin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromI8(i64in: i64) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI8(i64in : i64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromI8(i64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromR4(fltin: f32) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromR4(fltin : f32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromR4(fltin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromR8(dblin: f64) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromR8(dblin : f64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<super::wtypes::VARIANT_BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromUI1(bin: u8) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI1(bin : u8, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromUI1(bin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromUI2(uiin: u16) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI2(uiin : u16, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromUI4(ulin: u32) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI4(ulin : u32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromUI8(i64in: u64) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI8(i64in : u64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBoolFromUI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarBstrCat(bstrleft: &windows_core::BSTR, bstrright: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrCat(bstrleft : *mut core::ffi::c_void, bstrright : *mut core::ffi::c_void, pbstrresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrCat(core::mem::transmute_copy(bstrleft), core::mem::transmute_copy(bstrright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrCmp(bstrleft: &windows_core::BSTR, bstrright: &windows_core::BSTR, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrCmp(bstrleft : *mut core::ffi::c_void, bstrright : *mut core::ffi::c_void, lcid : super::winnt::LCID, dwflags : u32) -> windows_core::HRESULT);
    unsafe { VarBstrCmp(core::mem::transmute_copy(bstrleft), core::mem::transmute_copy(bstrright), lcid, dwflags) }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromBool(boolin: super::wtypes::VARIANT_BOOL, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromBool(boolin : super::wtypes::VARIANT_BOOL, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromBool(boolin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromCy(cyin: super::wtypes::CY, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromCy(cyin : super::wtypes::CY, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromCy(core::mem::transmute(cyin), lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromDate(datein: f64, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromDate(datein : f64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromDate(datein, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromDec(pdecin: *const super::wtypes::DECIMAL, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromDec(pdecin : *const super::wtypes::DECIMAL, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromDec(pdecin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarBstrFromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromDisp(pdispin.param().abi(), lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromI1(cin: i8, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI1(cin : i8, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromI1(cin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromI2(ival: i16, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI2(ival : i16, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromI2(ival, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromI4(lin: i32, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI4(lin : i32, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromI4(lin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromI8(i64in: i64, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI8(i64in : i64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromI8(i64in, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromR4(fltin: f32, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromR4(fltin : f32, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromR4(fltin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromR8(dblin: f64, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromR8(dblin : f64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromR8(dblin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromUI1(bval: u8, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI1(bval : u8, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromUI1(bval, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromUI2(uiin: u16, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI2(uiin : u16, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromUI2(uiin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromUI4(ulin: u32, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI4(ulin : u32, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromUI4(ulin, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarBstrFromUI8(ui64in: u64, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI8(ui64in : u64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarBstrFromUI8(ui64in, lcid, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarCat(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarCat(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCat(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarCmp(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCmp(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, lcid : super::winnt::LCID, dwflags : u32) -> windows_core::HRESULT);
    unsafe { VarCmp(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), lcid, dwflags) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyAbs(cyin: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyAbs(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyAbs(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyAdd(cyleft: super::wtypes::CY, cyright: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyAdd(cyleft : super::wtypes::CY, cyright : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyAdd(core::mem::transmute(cyleft), core::mem::transmute(cyright), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyCmp(cyleft: super::wtypes::CY, cyright: super::wtypes::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyCmp(cyleft : super::wtypes::CY, cyright : super::wtypes::CY) -> windows_core::HRESULT);
    unsafe { VarCyCmp(core::mem::transmute(cyleft), core::mem::transmute(cyright)) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyCmpR8(cyleft: super::wtypes::CY, dblright: f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyCmpR8(cyleft : super::wtypes::CY, dblright : f64) -> windows_core::HRESULT);
    unsafe { VarCyCmpR8(core::mem::transmute(cyleft), dblright) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFix(cyin: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFix(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFix(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromBool(boolin : super::wtypes::VARIANT_BOOL, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromDate(datein: f64) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromDate(datein : f64, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromDec(pdecin : *const super::wtypes::DECIMAL, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<super::wtypes::CY>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromI1(cin: i8) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI1(cin : i8, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromI1(cin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromI2(sin: i16) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI2(sin : i16, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromI2(sin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromI4(lin: i32) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI4(lin : i32, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromI4(lin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromI8(i64in: i64) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI8(i64in : i64, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromI8(i64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromR4(fltin: f32) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromR4(fltin : f32, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromR4(fltin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromR8(dblin: f64) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromR8(dblin : f64, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<super::wtypes::CY>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromUI1(bin: u8) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI1(bin : u8, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromUI1(bin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromUI2(uiin: u16) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI2(uiin : u16, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromUI4(ulin: u32) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI4(ulin : u32, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyFromUI8(ui64in: u64) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI8(ui64in : u64, pcyout : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyFromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyInt(cyin: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyInt(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyInt(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyMul(cyleft: super::wtypes::CY, cyright: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyMul(cyleft : super::wtypes::CY, cyright : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyMul(core::mem::transmute(cyleft), core::mem::transmute(cyright), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyMulI4(cyleft: super::wtypes::CY, lright: i32) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyMulI4(cyleft : super::wtypes::CY, lright : i32, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyMulI4(core::mem::transmute(cyleft), lright, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyMulI8(cyleft: super::wtypes::CY, lright: i64) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyMulI8(cyleft : super::wtypes::CY, lright : i64, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyMulI8(core::mem::transmute(cyleft), lright, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyNeg(cyin: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyNeg(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyNeg(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCyRound(cyin: super::wtypes::CY, cdecimals: i32) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCyRound(cyin : super::wtypes::CY, cdecimals : i32, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCyRound(core::mem::transmute(cyin), cdecimals, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarCySub(cyleft: super::wtypes::CY, cyright: super::wtypes::CY) -> windows_core::Result<super::wtypes::CY> {
    windows_core::link!("oleaut32.dll" "system" fn VarCySub(cyleft : super::wtypes::CY, cyright : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarCySub(core::mem::transmute(cyleft), core::mem::transmute(cyright), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDateFromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromBool(boolin : super::wtypes::VARIANT_BOOL, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDateFromCy(cyin: super::wtypes::CY) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromCy(cyin : super::wtypes::CY, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDateFromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromDec(pdecin : *const super::wtypes::DECIMAL, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarDateFromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<f64>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromI1(cin: i8) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI1(cin : i8, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromI2(sin: i16) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI2(sin : i16, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromI4(lin: i32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI4(lin : i32, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromI8(i64in: i64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI8(i64in : i64, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromR4(fltin: f32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromR4(fltin : f32, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromR8(dblin: f64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromR8(dblin : f64, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarDateFromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<f64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromUI1(bin: u8) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI1(bin : u8, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromUI2(uiin: u16) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI2(uiin : u16, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromUI4(ulin: u32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI4(ulin : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarDateFromUI8(ui64in: u64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI8(ui64in : u64, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUdate(pudatein : *const UDATE, dwflags : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromUdate(pudatein, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUdateEx(pudatein : *const UDATE, lcid : super::winnt::LCID, dwflags : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDateFromUdateEx(pudatein, lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecAbs(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecAbs(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecAbs(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecAdd(pdecleft: *const super::wtypes::DECIMAL, pdecright: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecAdd(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecAdd(pdecleft, pdecright, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecCmp(pdecleft: *const super::wtypes::DECIMAL, pdecright: *const super::wtypes::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecCmp(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecCmp(pdecleft, pdecright) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecCmpR8(pdecleft: *const super::wtypes::DECIMAL, dblright: f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecCmpR8(pdecleft : *const super::wtypes::DECIMAL, dblright : f64) -> windows_core::HRESULT);
    unsafe { VarDecCmpR8(pdecleft, dblright) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecDiv(pdecleft: *const super::wtypes::DECIMAL, pdecright: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecDiv(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecDiv(pdecleft, pdecright, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFix(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFix(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFix(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromBool(boolin : super::wtypes::VARIANT_BOOL, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromCy(cyin: super::wtypes::CY) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromCy(cyin : super::wtypes::CY, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromDate(datein: f64) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromDate(datein : f64, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<super::wtypes::DECIMAL>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromI1(cin: i8) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI1(cin : i8, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromI1(cin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromI2(uiin: i16) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI2(uiin : i16, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromI2(uiin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromI4(lin: i32) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI4(lin : i32, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromI4(lin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromI8(i64in: i64) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI8(i64in : i64, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromI8(i64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromR4(fltin: f32) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromR4(fltin : f32, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromR4(fltin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromR8(dblin: f64) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromR8(dblin : f64, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<super::wtypes::DECIMAL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromUI1(bin: u8) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI1(bin : u8, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromUI1(bin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromUI2(uiin: u16) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI2(uiin : u16, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromUI4(ulin: u32) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI4(ulin : u32, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecFromUI8(ui64in: u64) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI8(ui64in : u64, pdecout : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecFromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecInt(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecInt(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecInt(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecMul(pdecleft: *const super::wtypes::DECIMAL, pdecright: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecMul(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecMul(pdecleft, pdecright, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecNeg(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecNeg(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecNeg(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecRound(pdecin: *const super::wtypes::DECIMAL, cdecimals: i32) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecRound(pdecin : *const super::wtypes::DECIMAL, cdecimals : i32, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecRound(pdecin, cdecimals, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDecSub(pdecleft: *const super::wtypes::DECIMAL, pdecright: *const super::wtypes::DECIMAL) -> windows_core::Result<super::wtypes::DECIMAL> {
    windows_core::link!("oleaut32.dll" "system" fn VarDecSub(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDecSub(pdecleft, pdecright, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarDiv(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarDiv(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarDiv(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarEqv(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarEqv(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarEqv(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFix(pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarFix(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarFix(core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormat<P1>(pvarin: *const super::oaidl::VARIANT, pstrformat: P1, ifirstday: i32, ifirstweek: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarFormat(pvarin : *const super::oaidl::VARIANT, pstrformat : windows_core::PCWSTR, ifirstday : i32, ifirstweek : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarFormat(core::mem::transmute(pvarin), pstrformat.param().abi(), ifirstday, ifirstweek, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatCurrency(pvarin: *const super::oaidl::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatCurrency(pvarin : *const super::oaidl::VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarFormatCurrency(core::mem::transmute(pvarin), inumdig, iinclead, iuseparens, igroup, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatDateTime(pvarin: *const super::oaidl::VARIANT, inamedformat: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatDateTime(pvarin : *const super::oaidl::VARIANT, inamedformat : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarFormatDateTime(core::mem::transmute(pvarin), inamedformat, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatFromTokens<P1>(pvarin: *const super::oaidl::VARIANT, pstrformat: P1, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut windows_core::BSTR, lcid: super::winnt::LCID) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarFormatFromTokens(pvarin : *const super::oaidl::VARIANT, pstrformat : windows_core::PCWSTR, pbtokcur : *const u8, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void, lcid : super::winnt::LCID) -> windows_core::HRESULT);
    unsafe { VarFormatFromTokens(core::mem::transmute(pvarin), pstrformat.param().abi(), pbtokcur, dwflags, core::mem::transmute(pbstrout), lcid) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatNumber(pvarin: *const super::oaidl::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatNumber(pvarin : *const super::oaidl::VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarFormatNumber(core::mem::transmute(pvarin), inumdig, iinclead, iuseparens, igroup, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatPercent(pvarin: *const super::oaidl::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatPercent(pvarin : *const super::oaidl::VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarFormatPercent(core::mem::transmute(pvarin), inumdig, iinclead, iuseparens, igroup, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI1FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromBool(boolin : super::wtypes::VARIANT_BOOL, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI1FromCy(cyin: super::wtypes::CY) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromCy(cyin : super::wtypes::CY, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromDate(datein: f64) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : f64, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI1FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromDec(pdecin : *const super::wtypes::DECIMAL, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarI1FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<i8>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromI2(uiin: i16) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromI2(uiin : i16, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromI4(lin: i32) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromI4(lin : i32, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromI8(i64in: i64) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromI8(i64in : i64, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromR4(fltin: f32) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromR4(fltin : f32, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromR8(dblin: f64) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromR8(dblin : f64, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarI1FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<i8>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromUI1(bin: u8) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI1(bin : u8, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromUI2(uiin: u16) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI2(uiin : u16, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromUI4(ulin: u32) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI4(ulin : u32, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI1FromUI8(i64in: u64) -> windows_core::Result<i8> {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI8(i64in : u64, pcout : *mut i8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI1FromUI8(i64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI2FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromBool(boolin : super::wtypes::VARIANT_BOOL, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI2FromCy(cyin: super::wtypes::CY) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromCy(cyin : super::wtypes::CY, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromDate(datein: f64) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromDate(datein : f64, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI2FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromDec(pdecin : *const super::wtypes::DECIMAL, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarI2FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<i16>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromI1(cin: i8) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromI1(cin : i8, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromI4(lin: i32) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromI4(lin : i32, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromI8(i64in: i64) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromI8(i64in : i64, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromR4(fltin: f32) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromR4(fltin : f32, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromR8(dblin: f64) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromR8(dblin : f64, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarI2FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<i16>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromUI1(bin: u8) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI1(bin : u8, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromUI2(uiin: u16) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI2(uiin : u16, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromUI4(ulin: u32) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI4(ulin : u32, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI2FromUI8(ui64in: u64) -> windows_core::Result<i16> {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI8(ui64in : u64, psout : *mut i16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI2FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI4FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromBool(boolin : super::wtypes::VARIANT_BOOL, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI4FromCy(cyin: super::wtypes::CY) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromCy(cyin : super::wtypes::CY, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromDate(datein: f64) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromDate(datein : f64, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI4FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromDec(pdecin : *const super::wtypes::DECIMAL, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarI4FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromI1(cin: i8) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromI1(cin : i8, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromI2(sin: i16) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromI2(sin : i16, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromI8(i64in: i64) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromI8(i64in : i64, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromR4(fltin: f32) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromR4(fltin : f32, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromR8(dblin: f64) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromR8(dblin : f64, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarI4FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<i32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromUI1(bin: u8) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI1(bin : u8, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromUI2(uiin: u16) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI2(uiin : u16, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromUI4(ulin: u32) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI4(ulin : u32, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI4FromUI8(ui64in: u64) -> windows_core::Result<i32> {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI8(ui64in : u64, plout : *mut i32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI4FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI8FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromBool(boolin : super::wtypes::VARIANT_BOOL, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI8FromCy(cyin: super::wtypes::CY) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromCy(cyin : super::wtypes::CY, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromDate(datein: f64) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromDate(datein : f64, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarI8FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromDec(pdecin : *const super::wtypes::DECIMAL, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarI8FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<i64>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromI1(cin: i8) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromI1(cin : i8, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromI2(sin: i16) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromI2(sin : i16, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromR4(fltin: f32) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromR4(fltin : f32, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromR8(dblin: f64) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromR8(dblin : f64, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarI8FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<i64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromUI1(bin: u8) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI1(bin : u8, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromUI2(uiin: u16) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI2(uiin : u16, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromUI4(ulin: u32) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI4(ulin : u32, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarI8FromUI8(ui64in: u64) -> windows_core::Result<i64> {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI8(ui64in : u64, pi64out : *mut i64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarI8FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarIdiv(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarIdiv(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarIdiv(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarImp(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarImp(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarImp(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarInt(pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarInt(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarInt(core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarMod(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarMod(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarMod(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarMonthName(imonth : i32, fabbrev : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarMonthName(imonth, fabbrev, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarMul(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarMul(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarMul(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarNeg(pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarNeg(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarNeg(core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarNot(pvarin: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarNot(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarNot(core::mem::transmute(pvarin), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarNumFromParseNum(pnumprs : *const NUMPARSE, rgbdig : *const u8, dwvtbits : u32, pvar : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarNumFromParseNum(pnumprs, rgbdig, dwvtbits, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarOr(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarOr(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarOr(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarParseNumFromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarParseNumFromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pnumprs : *mut NUMPARSE, rgbdig : *mut u8) -> windows_core::HRESULT);
    unsafe { VarParseNumFromStr(strin.param().abi(), lcid, dwflags, pnumprs as _, rgbdig as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarPow(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarPow(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarPow(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn VarR4CmpR8(fltleft: f32, dblright: f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4CmpR8(fltleft : f32, dblright : f64) -> windows_core::HRESULT);
    unsafe { VarR4CmpR8(fltleft, dblright) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarR4FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromBool(boolin : super::wtypes::VARIANT_BOOL, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarR4FromCy(cyin: super::wtypes::CY) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromCy(cyin : super::wtypes::CY, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromDate(datein: f64) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromDate(datein : f64, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarR4FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromDec(pdecin : *const super::wtypes::DECIMAL, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarR4FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<f32>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromI1(cin: i8) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI1(cin : i8, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromI2(sin: i16) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI2(sin : i16, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromI4(lin: i32) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI4(lin : i32, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromI8(i64in: i64) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI8(i64in : i64, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromR8(dblin: f64) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromR8(dblin : f64, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarR4FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<f32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromUI1(bin: u8) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI1(bin : u8, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromUI2(uiin: u16) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI2(uiin : u16, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromUI4(ulin: u32) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI4(ulin : u32, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR4FromUI8(ui64in: u64) -> windows_core::Result<f32> {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI8(ui64in : u64, pfltout : *mut f32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR4FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarR8FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromBool(boolin : super::wtypes::VARIANT_BOOL, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarR8FromCy(cyin: super::wtypes::CY) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromCy(cyin : super::wtypes::CY, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromDate(datein: f64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromDate(datein : f64, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarR8FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromDec(pdecin : *const super::wtypes::DECIMAL, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarR8FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<f64>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromI1(cin: i8) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI1(cin : i8, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromI2(sin: i16) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI2(sin : i16, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromI4(lin: i32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI4(lin : i32, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromI8(i64in: i64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI8(i64in : i64, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromR4(fltin: f32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromR4(fltin : f32, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarR8FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<f64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromUI1(bin: u8) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI1(bin : u8, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromUI2(uiin: u16) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI2(uiin : u16, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromUI4(ulin: u32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI4(ulin : u32, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8FromUI8(ui64in: u64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI8(ui64in : u64, pdblout : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8Pow(dblleft: f64, dblright: f64) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8Pow(dblleft : f64, dblright : f64, pdblresult : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8Pow(dblleft, dblright, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarR8Round(dblin: f64, cdecimals: i32) -> windows_core::Result<f64> {
    windows_core::link!("oleaut32.dll" "system" fn VarR8Round(dblin : f64, cdecimals : i32, pdblresult : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarR8Round(dblin, cdecimals, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarRound(pvarin: *const super::oaidl::VARIANT, cdecimals: i32) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarRound(pvarin : *const super::oaidl::VARIANT, cdecimals : i32, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarRound(core::mem::transmute(pvarin), cdecimals, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarSub(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarSub(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarSub(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarTokenizeFormatString<P0>(pstrformat: P0, rgbtok: &mut [u8], ifirstday: i32, ifirstweek: i32, lcid: super::winnt::LCID, pcbactual: Option<*const i32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarTokenizeFormatString(pstrformat : windows_core::PCWSTR, rgbtok : *mut u8, cbtok : i32, ifirstday : i32, ifirstweek : i32, lcid : super::winnt::LCID, pcbactual : *const i32) -> windows_core::HRESULT);
    unsafe { VarTokenizeFormatString(pstrformat.param().abi(), core::mem::transmute(rgbtok.as_ptr()), rgbtok.len().try_into().unwrap(), ifirstday, ifirstweek, lcid, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI1FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromBool(boolin : super::wtypes::VARIANT_BOOL, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI1FromCy(cyin: super::wtypes::CY) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromCy(cyin : super::wtypes::CY, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromDate(datein: f64) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromDate(datein : f64, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI1FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromDec(pdecin : *const super::wtypes::DECIMAL, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarUI1FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<u8>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromI1(cin: i8) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI1(cin : i8, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromI2(sin: i16) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI2(sin : i16, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromI4(lin: i32) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI4(lin : i32, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromI8(i64in: i64) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI8(i64in : i64, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromR4(fltin: f32) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromR4(fltin : f32, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromR8(dblin: f64) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromR8(dblin : f64, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarUI1FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<u8>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromUI2(uiin: u16) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromUI2(uiin : u16, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromUI4(ulin: u32) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromUI4(ulin : u32, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI1FromUI8(ui64in: u64) -> windows_core::Result<u8> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromUI8(ui64in : u64, pbout : *mut u8) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI1FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI2FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromBool(boolin : super::wtypes::VARIANT_BOOL, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI2FromCy(cyin: super::wtypes::CY) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromCy(cyin : super::wtypes::CY, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromDate(datein: f64) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromDate(datein : f64, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI2FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromDec(pdecin : *const super::wtypes::DECIMAL, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarUI2FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<u16>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromI1(cin: i8) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI1(cin : i8, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromI2(uiin: i16) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI2(uiin : i16, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromI4(lin: i32) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI4(lin : i32, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromI8(i64in: i64) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI8(i64in : i64, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromR4(fltin: f32) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromR4(fltin : f32, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromR8(dblin: f64) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromR8(dblin : f64, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarUI2FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<u16>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromUI1(bin: u8) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromUI1(bin : u8, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromUI4(ulin: u32) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromUI4(ulin : u32, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI2FromUI8(i64in: u64) -> windows_core::Result<u16> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromUI8(i64in : u64, puiout : *mut u16) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI2FromUI8(i64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI4FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromBool(boolin : super::wtypes::VARIANT_BOOL, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI4FromCy(cyin: super::wtypes::CY) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromCy(cyin : super::wtypes::CY, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromDate(datein: f64) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromDate(datein : f64, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI4FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromDec(pdecin : *const super::wtypes::DECIMAL, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarUI4FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromI1(cin: i8) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI1(cin : i8, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromI2(uiin: i16) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI2(uiin : i16, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromI4(lin: i32) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI4(lin : i32, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromI4(lin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromI8(i64in: i64) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI8(i64in : i64, plout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromI8(i64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromR4(fltin: f32) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromR4(fltin : f32, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromR8(dblin: f64) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromR8(dblin : f64, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarUI4FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromUI1(bin: u8) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromUI1(bin : u8, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromUI2(uiin: u16) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromUI2(uiin : u16, pulout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI4FromUI8(ui64in: u64) -> windows_core::Result<u32> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromUI8(ui64in : u64, plout : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI4FromUI8(ui64in, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI8FromBool(boolin: super::wtypes::VARIANT_BOOL) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromBool(boolin : super::wtypes::VARIANT_BOOL, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromBool(boolin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI8FromCy(cyin: super::wtypes::CY) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromCy(cyin : super::wtypes::CY, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromDate(datein: f64) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromDate(datein : f64, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromDate(datein, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarUI8FromDec(pdecin: *const super::wtypes::DECIMAL) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromDec(pdecin : *const super::wtypes::DECIMAL, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromDec(pdecin, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn VarUI8FromDisp<P0>(pdispin: P0, lcid: super::winnt::LCID) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::oaidl::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromI1(cin: i8) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromI1(cin : i8, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromI1(cin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromI2(sin: i16) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromI2(sin : i16, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromI2(sin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromI8(ui64in: i64) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromI8(ui64in : i64, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromI8(ui64in, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromR4(fltin: f32) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromR4(fltin : f32, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromR4(fltin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromR8(dblin: f64) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromR8(dblin : f64, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromR8(dblin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VarUI8FromStr<P0>(strin: P0, lcid: super::winnt::LCID, dwflags: u32) -> windows_core::Result<u64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromStr(strin : windows_core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromUI1(bin: u8) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromUI1(bin : u8, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromUI1(bin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromUI2(uiin: u16) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromUI2(uiin : u16, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromUI2(uiin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn VarUI8FromUI4(ulin: u32) -> windows_core::Result<u64> {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromUI4(ulin : u32, pi64out : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarUI8FromUI4(ulin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn VarUdateFromDate(datein: f64, dwflags: u32, pudateout: *mut UDATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUdateFromDate(datein : f64, dwflags : u32, pudateout : *mut UDATE) -> windows_core::HRESULT);
    unsafe { VarUdateFromDate(datein, dwflags, pudateout as _) }
}
#[inline]
pub unsafe fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_core::link!("oleaut32.dll" "system" fn VarWeekdayName(iweekday : i32, fabbrev : i32, ifirstday : i32, dwflags : u32, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarWeekdayName(iweekday, fabbrev, ifirstday, dwflags, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarXor(pvarleft: *const super::oaidl::VARIANT, pvarright: *const super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
    windows_core::link!("oleaut32.dll" "system" fn VarXor(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VarXor(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantChangeType(pvargdest: *mut super::oaidl::VARIANTARG, pvarsrc: *const super::oaidl::VARIANTARG, wflags: u16, vt: super::wtypes::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantChangeType(pvargdest : *mut super::oaidl::VARIANTARG, pvarsrc : *const super::oaidl::VARIANTARG, wflags : u16, vt : super::wtypes::VARTYPE) -> windows_core::HRESULT);
    unsafe { VariantChangeType(core::mem::transmute(pvargdest), core::mem::transmute(pvarsrc), wflags, vt) }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantChangeTypeEx(pvargdest: *mut super::oaidl::VARIANTARG, pvarsrc: *const super::oaidl::VARIANTARG, lcid: super::winnt::LCID, wflags: u16, vt: super::wtypes::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantChangeTypeEx(pvargdest : *mut super::oaidl::VARIANTARG, pvarsrc : *const super::oaidl::VARIANTARG, lcid : super::winnt::LCID, wflags : u16, vt : super::wtypes::VARTYPE) -> windows_core::HRESULT);
    unsafe { VariantChangeTypeEx(core::mem::transmute(pvargdest), core::mem::transmute(pvarsrc), lcid, wflags, vt) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantClear(pvarg: *mut super::oaidl::VARIANTARG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut super::oaidl::VARIANTARG) -> windows_core::HRESULT);
    unsafe { VariantClear(core::mem::transmute(pvarg)) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantCopy(pvargdest: *mut super::oaidl::VARIANTARG, pvargsrc: *const super::oaidl::VARIANTARG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut super::oaidl::VARIANTARG, pvargsrc : *const super::oaidl::VARIANTARG) -> windows_core::HRESULT);
    unsafe { VariantCopy(core::mem::transmute(pvargdest), core::mem::transmute(pvargsrc)) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantCopyInd(pvardest: *mut super::oaidl::VARIANT, pvargsrc: *const super::oaidl::VARIANTARG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantCopyInd(pvardest : *mut super::oaidl::VARIANT, pvargsrc : *const super::oaidl::VARIANTARG) -> windows_core::HRESULT);
    unsafe { VariantCopyInd(core::mem::transmute(pvardest), core::mem::transmute(pvargsrc)) }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantInit() -> super::oaidl::VARIANTARG {
    windows_core::link!("oleaut32.dll" "system" fn VariantInit(pvarg : *mut super::oaidl::VARIANTARG));
    unsafe {
        let mut result__ = core::mem::zeroed();
        VariantInit(&mut result__);
        core::mem::transmute(result__)
    }
}
#[inline]
pub unsafe fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn VariantTimeToDosDateTime(vtime : f64, pwdosdate : *mut u16, pwdostime : *mut u16) -> i32);
    unsafe { VariantTimeToDosDateTime(vtime, pwdosdate as _, pwdostime as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::minwinbase::SYSTEMTIME) -> i32 {
    windows_core::link!("oleaut32.dll" "system" fn VariantTimeToSystemTime(vtime : f64, lpsystemtime : *mut super::minwinbase::SYSTEMTIME) -> i32);
    unsafe { VariantTimeToSystemTime(vtime, lpsystemtime as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn VectorFromBstr(bstr: &windows_core::BSTR) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
    windows_core::link!("oleaut32.dll" "system" fn VectorFromBstr(bstr : *mut core::ffi::c_void, ppsa : *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        VectorFromBstr(core::mem::transmute_copy(bstr), &mut result__).map(|| result__)
    }
}
pub const ACTIVEOBJECT_STRONG: u32 = 0;
pub const ACTIVEOBJECT_WEAK: u32 = 1;
pub const DISPATCH_METHOD: u32 = 1;
pub const DISPATCH_PROPERTYGET: u32 = 2;
pub const DISPATCH_PROPERTYPUT: u32 = 4;
pub const DISPATCH_PROPERTYPUTREF: u32 = 8;
pub const ID_DEFAULTINST: i32 = -2;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOAD_TLB_AS_32BIT: u32 = 32;
pub const LOAD_TLB_AS_64BIT: u32 = 64;
pub const LOCALE_USE_NLS: u32 = 268435456;
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub type LPINTERFACEDATA = *mut INTERFACEDATA;
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub type LPMETHODDATA = *mut METHODDATA;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDATA = *mut PARAMDATA;
pub const MASK_TO_RESET_TLB_BITS: i32 = -97;
pub const MEMBERID_NIL: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct METHODDATA {
    pub szName: *mut super::wtypesbase::OLECHAR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: super::oaidl::DISPID,
    pub iMeth: u32,
    pub cc: super::oaidl::CALLCONV,
    pub cArgs: u32,
    pub wFlags: u16,
    pub vtReturn: super::wtypes::VARTYPE,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for METHODDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMPARSE {
    pub cDig: i32,
    pub dwInFlags: u32,
    pub dwOutFlags: u32,
    pub cchUsed: i32,
    pub nBaseShift: i32,
    pub nPwr10: i32,
}
pub const NUMPRS_CURRENCY: u32 = 1024;
pub const NUMPRS_DECIMAL: u32 = 256;
pub const NUMPRS_EXPONENT: u32 = 2048;
pub const NUMPRS_HEX_OCT: u32 = 64;
pub const NUMPRS_INEXACT: u32 = 131072;
pub const NUMPRS_LEADING_MINUS: u32 = 16;
pub const NUMPRS_LEADING_PLUS: u32 = 4;
pub const NUMPRS_LEADING_WHITE: u32 = 1;
pub const NUMPRS_NEG: u32 = 65536;
pub const NUMPRS_PARENS: u32 = 128;
pub const NUMPRS_STD: u32 = 8191;
pub const NUMPRS_THOUSANDS: u32 = 512;
pub const NUMPRS_TRAILING_MINUS: u32 = 32;
pub const NUMPRS_TRAILING_PLUS: u32 = 8;
pub const NUMPRS_TRAILING_WHITE: u32 = 2;
pub const NUMPRS_USE_ALL: u32 = 4096;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PARAMDATA {
    pub szName: *mut super::wtypesbase::OLECHAR,
    pub vt: super::wtypes::VARTYPE,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for PARAMDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type REGKIND = i32;
pub const REGKIND_DEFAULT: REGKIND = 0;
pub const REGKIND_NONE: REGKIND = 2;
pub const REGKIND_REGISTER: REGKIND = 1;
pub const STDOLE2_LCID: u32 = 0;
pub const STDOLE2_MAJORVERNUM: u32 = 2;
pub const STDOLE2_MINORVERNUM: u32 = 0;
pub const STDOLE_LCID: u32 = 0;
pub const STDOLE_MAJORVERNUM: u32 = 1;
pub const STDOLE_MINORVERNUM: u32 = 0;
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UDATE {
    pub st: super::minwinbase::SYSTEMTIME,
    pub wDayOfYear: u16,
}
pub const VARCMP_EQ: u32 = 1;
pub const VARCMP_GT: u32 = 2;
pub const VARCMP_LT: u32 = 0;
pub const VARCMP_NULL: u32 = 3;
pub const VARIANT_ALPHABOOL: u32 = 2;
pub const VARIANT_CALENDAR_GREGORIAN: u32 = 64;
pub const VARIANT_CALENDAR_HIJRI: u32 = 8;
pub const VARIANT_CALENDAR_THAI: u32 = 32;
pub const VARIANT_LOCALBOOL: u32 = 16;
pub const VARIANT_NOUSEROVERRIDE: u32 = 4;
pub const VARIANT_NOVALUEPROP: u32 = 1;
pub const VARIANT_USE_NLS: u32 = 128;
pub const VAR_CALENDAR_GREGORIAN: u32 = 256;
pub const VAR_CALENDAR_HIJRI: u32 = 8;
pub const VAR_CALENDAR_THAI: u32 = 128;
pub const VAR_DATEVALUEONLY: u32 = 2;
pub const VAR_FORMAT_NOSUBSTITUTE: u32 = 32;
pub const VAR_FOURDIGITYEARS: u32 = 64;
pub const VAR_LOCALBOOL: u32 = 16;
pub const VAR_TIMEVALUEONLY: u32 = 1;
pub const VAR_VALIDDATE: u32 = 4;
pub const VTBIT_CY: u32 = 64;
pub const VTBIT_DECIMAL: u32 = 16384;
pub const VTBIT_I1: u32 = 65536;
pub const VTBIT_I2: u32 = 4;
pub const VTBIT_I4: u32 = 8;
pub const VTBIT_I8: u32 = 1048576;
pub const VTBIT_R4: u32 = 16;
pub const VTBIT_R8: u32 = 32;
pub const VTBIT_UI1: u32 = 131072;
pub const VTBIT_UI2: u32 = 262144;
pub const VTBIT_UI4: u32 = 524288;
pub const VTBIT_UI8: u32 = 2097152;
pub const VTDATEGRE_MAX: u32 = 2958465;
pub const VTDATEGRE_MIN: i32 = -657434;
pub const VT_HARDTYPE: u32 = 32768;
