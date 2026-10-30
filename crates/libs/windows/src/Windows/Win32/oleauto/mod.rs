#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BstrFromVector(psa: *const super::SAFEARRAY, pbstr: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn BstrFromVector(psa : *const super::SAFEARRAY, pbstr : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { BstrFromVector(psa, core::mem::transmute(pbstr)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ClearCustData(pcustdata: super::LPCUSTDATA) {
    windows_core::link!("oleaut32.dll" "system" fn ClearCustData(pcustdata : super::LPCUSTDATA));
    unsafe { ClearCustData(pcustdata as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: super::LCID, pptinfo: *mut Option<super::ITypeInfo>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn CreateDispTypeInfo(pidata : *mut INTERFACEDATA, lcid : super::LCID, pptinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateDispTypeInfo(pidata as _, lcid, pptinfo as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn CreateErrorInfo() -> windows_core::Result<super::ICreateErrorInfo> {
    windows_core::link!("oleaut32.dll" "system" fn CreateErrorInfo(pperrinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateErrorInfo(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn CreateStdDispatch<P0, P2>(punkouter: P0, pvthis: *mut core::ffi::c_void, ptinfo: P2, ppunkstddisp: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P2: windows_core::Param<super::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn CreateStdDispatch(punkouter : *mut core::ffi::c_void, pvthis : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, ppunkstddisp : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateStdDispatch(punkouter.param().abi(), pvthis as _, ptinfo.param().abi(), ppunkstddisp as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateTypeLib(syskind: super::SYSKIND, szfile: super::LPCOLESTR, ppctlib: *mut Option<super::ICreateTypeLib>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn CreateTypeLib(syskind : super::SYSKIND, szfile : super::LPCOLESTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateTypeLib(syskind, szfile, ppctlib as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CreateTypeLib2(syskind: super::SYSKIND, szfile: super::LPCOLESTR, ppctlib: *mut Option<super::ICreateTypeLib2>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn CreateTypeLib2(syskind : super::SYSKIND, szfile : super::LPCOLESTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateTypeLib2(syskind, szfile, ppctlib as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispCallFunc(pvinstance: Option<*const core::ffi::c_void>, ovft: super::ULONG_PTR, cc: super::CALLCONV, vtreturn: super::VARTYPE, cactuals: super::UINT, prgvt: *const super::VARTYPE, prgpvarg: *const *const super::VARIANTARG, pvargresult: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn DispCallFunc(pvinstance : *const core::ffi::c_void, ovft : super::ULONG_PTR, cc : super::CALLCONV, vtreturn : super::VARTYPE, cactuals : super::UINT, prgvt : *const super::VARTYPE, prgpvarg : *const *const super::VARIANTARG, pvargresult : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { DispCallFunc(pvinstance.unwrap_or(core::mem::zeroed()) as _, ovft, cc, vtreturn, cactuals, prgvt, prgpvarg, pvargresult) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispGetIDsOfNames<P0>(ptinfo: P0, rgsznames: *const super::LPOLESTR, cnames: super::UINT, rgdispid: *mut super::DISPID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn DispGetIDsOfNames(ptinfo : *mut core::ffi::c_void, rgsznames : *const super::LPOLESTR, cnames : super::UINT, rgdispid : *mut super::DISPID) -> windows_core::HRESULT);
    unsafe { DispGetIDsOfNames(ptinfo.param().abi(), rgsznames, cnames, rgdispid as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispGetParam(pdispparams: *const super::DISPPARAMS, position: super::UINT, vttarg: super::VARTYPE, pvarresult: *mut super::VARIANT, puargerr: Option<*mut super::UINT>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn DispGetParam(pdispparams : *const super::DISPPARAMS, position : super::UINT, vttarg : super::VARTYPE, pvarresult : *mut super::VARIANT, puargerr : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DispGetParam(pdispparams, position, vttarg, pvarresult, puargerr.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DispInvoke<P1>(_this: *mut core::ffi::c_void, ptinfo: P1, dispidmember: super::DISPID, wflags: super::WORD, pparams: *mut super::DISPPARAMS, pvarresult: *mut super::VARIANT, pexcepinfo: *mut super::EXCEPINFO, puargerr: *mut super::UINT) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn DispInvoke(_this : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, dispidmember : super::DISPID, wflags : super::WORD, pparams : *mut super::DISPPARAMS, pvarresult : *mut super::VARIANT, pexcepinfo : *mut super::EXCEPINFO, puargerr : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DispInvoke(_this as _, ptinfo.param().abi(), dispidmember, wflags, pparams as _, pvarresult as _, pexcepinfo as _, puargerr as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn DosDateTimeToVariantTime(wdosdate: super::USHORT, wdostime: super::USHORT, pvtime: *mut super::DOUBLE) -> super::INT {
    windows_core::link!("oleaut32.dll" "system" fn DosDateTimeToVariantTime(wdosdate : super::USHORT, wdostime : super::USHORT, pvtime : *mut super::DOUBLE) -> super::INT);
    unsafe { DosDateTimeToVariantTime(wdosdate, wdostime, pvtime as _) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn GetActiveObject(rclsid: *const super::IID, pvreserved: *mut core::ffi::c_void, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn GetActiveObject(rclsid : *const super::IID, pvreserved : *mut core::ffi::c_void, ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetActiveObject(rclsid, pvreserved as _, ppunk as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn GetAltMonthNames(lcid: super::LCID, prgp: *mut *mut super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn GetAltMonthNames(lcid : super::LCID, prgp : *mut *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { GetAltMonthNames(lcid, prgp as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: super::ULONG) -> windows_core::Result<super::IErrorInfo> {
    windows_core::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : super::ULONG, pperrinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetErrorInfo(dwreserved, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn GetRecordInfoFromGuids(rguidtypelib: *const windows_core::GUID, uvermajor: super::ULONG, uverminor: super::ULONG, lcid: super::LCID, rguidtypeinfo: *const windows_core::GUID, pprecinfo: *mut Option<super::IRecordInfo>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn GetRecordInfoFromGuids(rguidtypelib : *const windows_core::GUID, uvermajor : super::ULONG, uverminor : super::ULONG, lcid : super::LCID, rguidtypeinfo : *const windows_core::GUID, pprecinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetRecordInfoFromGuids(rguidtypelib, uvermajor, uverminor, lcid, rguidtypeinfo, pprecinfo as _) }
}
#[cfg(feature = "oaidl")]
#[inline]
pub unsafe fn GetRecordInfoFromTypeInfo<P0>(ptypeinfo: P0, pprecinfo: *mut Option<super::IRecordInfo>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ITypeInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn GetRecordInfoFromTypeInfo(ptypeinfo : *mut core::ffi::c_void, pprecinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetRecordInfoFromTypeInfo(ptypeinfo.param().abi(), pprecinfo as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn LHashValOfNameSys(syskind: super::SYSKIND, lcid: super::LCID, szname: *const super::OLECHAR) -> super::ULONG {
    windows_core::link!("oleaut32.dll" "system" fn LHashValOfNameSys(syskind : super::SYSKIND, lcid : super::LCID, szname : *const super::OLECHAR) -> super::ULONG);
    unsafe { LHashValOfNameSys(syskind, lcid, szname) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn LHashValOfNameSysA<P2>(syskind: super::SYSKIND, lcid: super::LCID, szname: P2) -> super::ULONG
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn LHashValOfNameSysA(syskind : super::SYSKIND, lcid : super::LCID, szname : windows_core::PCSTR) -> super::ULONG);
    unsafe { LHashValOfNameSysA(syskind, lcid, szname.param().abi()) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn LoadRegTypeLib(rguid: *const windows_core::GUID, wvermajor: super::WORD, wverminor: super::WORD, lcid: super::LCID, pptlib: *mut Option<super::ITypeLib>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn LoadRegTypeLib(rguid : *const windows_core::GUID, wvermajor : super::WORD, wverminor : super::WORD, lcid : super::LCID, pptlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LoadRegTypeLib(rguid, wvermajor, wverminor, lcid, pptlib as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn LoadTypeLib(szfile: super::LPCOLESTR, pptlib: *mut Option<super::ITypeLib>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn LoadTypeLib(szfile : super::LPCOLESTR, pptlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LoadTypeLib(szfile, pptlib as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn LoadTypeLibEx(szfile: super::LPCOLESTR, regkind: REGKIND, pptlib: *mut Option<super::ITypeLib>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn LoadTypeLibEx(szfile : super::LPCOLESTR, regkind : REGKIND, pptlib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LoadTypeLibEx(szfile, regkind, pptlib as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn OaBuildVersion() -> super::ULONG {
    windows_core::link!("oleaut32.dll" "system" fn OaBuildVersion() -> super::ULONG);
    unsafe { OaBuildVersion() }
}
#[inline]
pub unsafe fn OaEnablePerUserTLibRegistration() {
    windows_core::link!("oleaut32.dll" "system" fn OaEnablePerUserTLibRegistration());
    unsafe { OaEnablePerUserTLibRegistration() }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn QueryPathOfRegTypeLib(guid: *const windows_core::GUID, wmaj: super::USHORT, wmin: super::USHORT, lcid: super::LCID, lpbstrpathname: super::LPBSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn QueryPathOfRegTypeLib(guid : *const windows_core::GUID, wmaj : super::USHORT, wmin : super::USHORT, lcid : super::LCID, lpbstrpathname : super::LPBSTR) -> windows_core::HRESULT);
    unsafe { QueryPathOfRegTypeLib(guid, wmaj, wmin, lcid, lpbstrpathname as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn RegisterActiveObject<P0>(punk: P0, rclsid: *const super::IID, dwflags: super::DWORD, pdwregister: *mut super::DWORD) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("oleaut32.dll" "system" fn RegisterActiveObject(punk : *mut core::ffi::c_void, rclsid : *const super::IID, dwflags : super::DWORD, pdwregister : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { RegisterActiveObject(punk.param().abi(), rclsid, dwflags, pdwregister as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn RegisterTypeLib<P0>(ptlib: P0, szfullpath: super::LPCOLESTR, szhelpdir: Option<super::LPCOLESTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ITypeLib>,
{
    windows_core::link!("oleaut32.dll" "system" fn RegisterTypeLib(ptlib : *mut core::ffi::c_void, szfullpath : super::LPCOLESTR, szhelpdir : super::LPCOLESTR) -> windows_core::HRESULT);
    unsafe { RegisterTypeLib(ptlib.param().abi(), szfullpath, szhelpdir.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn RegisterTypeLibForUser<P0>(ptlib: P0, szfullpath: *const super::OLECHAR, szhelpdir: Option<*const super::OLECHAR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::ITypeLib>,
{
    windows_core::link!("oleaut32.dll" "system" fn RegisterTypeLibForUser(ptlib : *mut core::ffi::c_void, szfullpath : *const super::OLECHAR, szhelpdir : *const super::OLECHAR) -> windows_core::HRESULT);
    unsafe { RegisterTypeLibForUser(ptlib.param().abi(), szfullpath, szhelpdir.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RevokeActiveObject(dwregister: super::DWORD, pvreserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn RevokeActiveObject(dwregister : super::DWORD, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RevokeActiveObject(dwregister, pvreserved as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayAccessData(psa: *const super::SAFEARRAY, ppvdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAccessData(psa : *const super::SAFEARRAY, ppvdata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayAccessData(psa, ppvdata as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayAddRef(psa: *const super::SAFEARRAY, ppdatatorelease: *mut super::PVOID) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAddRef(psa : *const super::SAFEARRAY, ppdatatorelease : *mut super::PVOID) -> windows_core::HRESULT);
    unsafe { SafeArrayAddRef(psa, ppdatatorelease as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayAllocData(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAllocData(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayAllocData(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayAllocDescriptor(cdims: super::UINT, ppsaout: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptor(cdims : super::UINT, ppsaout : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayAllocDescriptor(cdims, ppsaout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayAllocDescriptorEx(vt: super::VARTYPE, cdims: super::UINT, ppsaout: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptorEx(vt : super::VARTYPE, cdims : super::UINT, ppsaout : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayAllocDescriptorEx(vt, cdims, ppsaout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayCopy(psa: *const super::SAFEARRAY, ppsaout: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCopy(psa : *const super::SAFEARRAY, ppsaout : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayCopy(psa, ppsaout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayCopyData(psasource: *const super::SAFEARRAY, psatarget: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCopyData(psasource : *const super::SAFEARRAY, psatarget : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayCopyData(psasource, psatarget) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreate(vt: super::VARTYPE, cdims: super::UINT, rgsabound: *const super::SAFEARRAYBOUND) -> *mut super::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreate(vt : super::VARTYPE, cdims : super::UINT, rgsabound : *const super::SAFEARRAYBOUND) -> *mut super::SAFEARRAY);
    unsafe { SafeArrayCreate(vt, cdims, rgsabound) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreateEx(vt: super::VARTYPE, cdims: super::UINT, rgsabound: *const super::SAFEARRAYBOUND, pvextra: super::PVOID) -> *mut super::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreateEx(vt : super::VARTYPE, cdims : super::UINT, rgsabound : *const super::SAFEARRAYBOUND, pvextra : super::PVOID) -> *mut super::SAFEARRAY);
    unsafe { SafeArrayCreateEx(vt, cdims, rgsabound, pvextra) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreateVector(vt: super::VARTYPE, llbound: super::LONG, celements: super::ULONG) -> *mut super::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreateVector(vt : super::VARTYPE, llbound : super::LONG, celements : super::ULONG) -> *mut super::SAFEARRAY);
    unsafe { SafeArrayCreateVector(vt, llbound, celements) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayCreateVectorEx(vt: super::VARTYPE, llbound: super::LONG, celements: super::ULONG, pvextra: super::PVOID) -> *mut super::SAFEARRAY {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayCreateVectorEx(vt : super::VARTYPE, llbound : super::LONG, celements : super::ULONG, pvextra : super::PVOID) -> *mut super::SAFEARRAY);
    unsafe { SafeArrayCreateVectorEx(vt, llbound, celements, pvextra) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayDestroy(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayDestroy(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayDestroy(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayDestroyData(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayDestroyData(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayDestroyData(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayDestroyDescriptor(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayDestroyDescriptor(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayDestroyDescriptor(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetDim(psa: *const super::SAFEARRAY) -> super::UINT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetDim(psa : *const super::SAFEARRAY) -> super::UINT);
    unsafe { SafeArrayGetDim(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetElement(psa: *const super::SAFEARRAY, rgindices: *const super::LONG, pv: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetElement(psa : *const super::SAFEARRAY, rgindices : *const super::LONG, pv : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayGetElement(psa, rgindices, pv as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetElemsize(psa: *const super::SAFEARRAY) -> super::UINT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetElemsize(psa : *const super::SAFEARRAY) -> super::UINT);
    unsafe { SafeArrayGetElemsize(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetIID(psa: *const super::SAFEARRAY, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetIID(psa : *const super::SAFEARRAY, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SafeArrayGetIID(psa, pguid as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetLBound(psa: *const super::SAFEARRAY, ndim: super::UINT, pllbound: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetLBound(psa : *const super::SAFEARRAY, ndim : super::UINT, pllbound : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { SafeArrayGetLBound(psa, ndim, pllbound as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetRecordInfo(psa: *const super::SAFEARRAY) -> windows_core::Result<super::IRecordInfo> {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetRecordInfo(psa : *const super::SAFEARRAY, prinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SafeArrayGetRecordInfo(psa, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayGetUBound(psa: *const super::SAFEARRAY, ndim: super::UINT, plubound: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetUBound(psa : *const super::SAFEARRAY, ndim : super::UINT, plubound : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { SafeArrayGetUBound(psa, ndim, plubound as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn SafeArrayGetVartype(psa: *const super::SAFEARRAY, pvt: *mut super::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayGetVartype(psa : *const super::SAFEARRAY, pvt : *mut super::VARTYPE) -> windows_core::HRESULT);
    unsafe { SafeArrayGetVartype(psa, pvt as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayLock(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayLock(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayLock(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayPtrOfIndex(psa: *const super::SAFEARRAY, rgindices: *const super::LONG, ppvdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayPtrOfIndex(psa : *const super::SAFEARRAY, rgindices : *const super::LONG, ppvdata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayPtrOfIndex(psa, rgindices, ppvdata as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayPutElement(psa: *const super::SAFEARRAY, rgindices: *const super::LONG, pv: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayPutElement(psa : *const super::SAFEARRAY, rgindices : *const super::LONG, pv : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArrayPutElement(psa, rgindices, pv) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayRedim(psa: *mut super::SAFEARRAY, psaboundnew: *const super::SAFEARRAYBOUND) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayRedim(psa : *mut super::SAFEARRAY, psaboundnew : *const super::SAFEARRAYBOUND) -> windows_core::HRESULT);
    unsafe { SafeArrayRedim(psa as _, psaboundnew) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SafeArrayReleaseData(pdata: super::PVOID) {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayReleaseData(pdata : super::PVOID));
    unsafe { SafeArrayReleaseData(pdata) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayReleaseDescriptor(psa: *const super::SAFEARRAY) {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayReleaseDescriptor(psa : *const super::SAFEARRAY));
    unsafe { SafeArrayReleaseDescriptor(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArraySetIID(psa: *const super::SAFEARRAY, guid: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArraySetIID(psa : *const super::SAFEARRAY, guid : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { SafeArraySetIID(psa, guid) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArraySetRecordInfo<P1>(psa: *const super::SAFEARRAY, prinfo: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IRecordInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn SafeArraySetRecordInfo(psa : *const super::SAFEARRAY, prinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SafeArraySetRecordInfo(psa, prinfo.param().abi()) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayUnaccessData(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayUnaccessData(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayUnaccessData(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SafeArrayUnlock(psa: *const super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SafeArrayUnlock(psa : *const super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { SafeArrayUnlock(psa) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn SetErrorInfo<P1>(dwreserved: super::ULONG, perrinfo: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IErrorInfo>,
{
    windows_core::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : super::ULONG, perrinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetErrorInfo(dwreserved, perrinfo.param().abi()) }
}
#[inline]
pub unsafe fn SysAddRefString(bstrstring: &windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn SysAddRefString(bstrstring : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SysAddRefString(core::mem::transmute_copy(bstrstring)) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn SysAllocString(psz: Option<*const super::OLECHAR>) -> windows_core::BSTR {
    windows_core::link!("oleaut32.dll" "system" fn SysAllocString(psz : *const super::OLECHAR) -> windows_core::BSTR);
    unsafe { SysAllocString(psz.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SysAllocStringByteLen<P0>(psz: P0, len: super::UINT) -> windows_core::BSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("oleaut32.dll" "system" fn SysAllocStringByteLen(psz : windows_core::PCSTR, len : super::UINT) -> windows_core::BSTR);
    unsafe { SysAllocStringByteLen(psz.param().abi(), len) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SysAllocStringLen(strin: Option<&[super::OLECHAR]>) -> windows_core::BSTR {
    windows_core::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : *const super::OLECHAR, ui : super::UINT) -> windows_core::BSTR);
    unsafe { SysAllocStringLen(strin.map_or(core::ptr::null(), |slice| slice.as_ptr()), strin.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn SysFreeString(bstrstring: windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : *mut core::ffi::c_void));
    unsafe { SysFreeString(core::mem::transmute(bstrstring)) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SysReAllocString(pbstr: *mut windows_core::BSTR, psz: Option<*const super::OLECHAR>) -> super::INT {
    windows_core::link!("oleaut32.dll" "system" fn SysReAllocString(pbstr : *mut *mut core::ffi::c_void, psz : *const super::OLECHAR) -> super::INT);
    unsafe { SysReAllocString(core::mem::transmute(pbstr), psz.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SysReAllocStringLen(pbstr: *mut windows_core::BSTR, psz: Option<*const super::OLECHAR>, len: u32) -> super::INT {
    windows_core::link!("oleaut32.dll" "system" fn SysReAllocStringLen(pbstr : *mut *mut core::ffi::c_void, psz : *const super::OLECHAR, len : u32) -> super::INT);
    unsafe { SysReAllocStringLen(core::mem::transmute(pbstr), psz.unwrap_or(core::mem::zeroed()) as _, len) }
}
#[inline]
pub unsafe fn SysReleaseString(bstrstring: &windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn SysReleaseString(bstrstring : *mut core::ffi::c_void));
    unsafe { SysReleaseString(core::mem::transmute_copy(bstrstring)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SysStringByteLen(bstr: &windows_core::BSTR) -> super::UINT {
    windows_core::link!("oleaut32.dll" "system" fn SysStringByteLen(bstr : *mut core::ffi::c_void) -> super::UINT);
    unsafe { SysStringByteLen(core::mem::transmute_copy(bstr)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SysStringLen(pbstr: &windows_core::BSTR) -> super::UINT {
    windows_core::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : *mut core::ffi::c_void) -> super::UINT);
    unsafe { SysStringLen(core::mem::transmute_copy(pbstr)) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SystemTimeToVariantTime(lpsystemtime: super::LPSYSTEMTIME, pvtime: *mut super::DOUBLE) -> super::INT {
    windows_core::link!("oleaut32.dll" "system" fn SystemTimeToVariantTime(lpsystemtime : super::LPSYSTEMTIME, pvtime : *mut super::DOUBLE) -> super::INT);
    unsafe { SystemTimeToVariantTime(lpsystemtime, pvtime as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn UnRegisterTypeLib(libid: *const windows_core::GUID, wvermajor: super::WORD, wverminor: super::WORD, lcid: super::LCID, syskind: super::SYSKIND) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn UnRegisterTypeLib(libid : *const windows_core::GUID, wvermajor : super::WORD, wverminor : super::WORD, lcid : super::LCID, syskind : super::SYSKIND) -> windows_core::HRESULT);
    unsafe { UnRegisterTypeLib(libid, wvermajor, wverminor, lcid, syskind) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn UnRegisterTypeLibForUser(libid: *const windows_core::GUID, wmajorvernum: super::WORD, wminorvernum: super::WORD, lcid: super::LCID, syskind: super::SYSKIND) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn UnRegisterTypeLibForUser(libid : *const windows_core::GUID, wmajorvernum : super::WORD, wminorvernum : super::WORD, lcid : super::LCID, syskind : super::SYSKIND) -> windows_core::HRESULT);
    unsafe { UnRegisterTypeLibForUser(libid, wmajorvernum, wminorvernum, lcid, syskind) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarAbs(pvarin: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarAbs(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarAbs(pvarin, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarAdd(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarAdd(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarAdd(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarAnd(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarAnd(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarAnd(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromCy(cyin: super::CY, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromCy(cyin : super::CY, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromCy(cyin, pboolout as _) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBoolFromDate(datein: super::DATE, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromDate(datein : super::DATE, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromDate(datein, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromDec(pdecin: *const super::DECIMAL, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromDec(pdecin : *const super::DECIMAL, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromDec(pdecin, pboolout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromDisp<P0>(pdispin: P0, lcid: super::LCID, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromDisp(pdispin.param().abi(), lcid, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromI1(cin: super::CHAR, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI1(cin : super::CHAR, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromI1(cin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromI2(sin: super::SHORT, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI2(sin : super::SHORT, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromI2(sin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromI4(lin: super::LONG, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI4(lin : super::LONG, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromI4(lin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromI8(i64in: super::LONG64, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromI8(i64in : super::LONG64, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromI8(i64in, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromInt(lin: super::LONG, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarBoolFromI4" fn VarBoolFromInt(lin : super::LONG, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromInt(lin, pboolout as _) }
}
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromR4(fltin: super::FLOAT, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromR4(fltin : super::FLOAT, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromR4(fltin, pboolout as _) }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarBoolFromR8(dblin: super::DOUBLE, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromR8(dblin : super::DOUBLE, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromR8(dblin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarBoolFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromStr(strin, lcid, dwflags, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromUI1(bin: super::BYTE, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI1(bin : super::BYTE, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromUI1(bin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromUI2(uiin: super::USHORT, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI2(uiin : super::USHORT, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromUI2(uiin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromUI4(ulin: super::ULONG, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI4(ulin : super::ULONG, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromUI4(ulin, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromUI8(i64in: super::ULONG64, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBoolFromUI8(i64in : super::ULONG64, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromUI8(i64in, pboolout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBoolFromUint(ulin: super::ULONG, pboolout: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarBoolFromUI4" fn VarBoolFromUint(ulin : super::ULONG, pboolout : *mut super::VARIANT_BOOL) -> windows_core::HRESULT);
    unsafe { VarBoolFromUint(ulin, pboolout as _) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarBstrCat(bstrleft: &windows_core::BSTR, bstrright: &windows_core::BSTR, pbstrresult: super::LPBSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrCat(bstrleft : *mut core::ffi::c_void, bstrright : *mut core::ffi::c_void, pbstrresult : super::LPBSTR) -> windows_core::HRESULT);
    unsafe { VarBstrCat(core::mem::transmute_copy(bstrleft), core::mem::transmute_copy(bstrright), pbstrresult as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrCmp(bstrleft: &windows_core::BSTR, bstrright: &windows_core::BSTR, lcid: super::LCID, dwflags: super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrCmp(bstrleft : *mut core::ffi::c_void, bstrright : *mut core::ffi::c_void, lcid : super::LCID, dwflags : super::ULONG) -> windows_core::HRESULT);
    unsafe { VarBstrCmp(core::mem::transmute_copy(bstrleft), core::mem::transmute_copy(bstrright), lcid, dwflags) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromBool(boolin: super::VARIANT_BOOL, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromBool(boolin : super::VARIANT_BOOL, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromBool(boolin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromCy(cyin: super::CY, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromCy(cyin : super::CY, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromCy(cyin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromDate(datein: super::DATE, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromDate(datein : super::DATE, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromDate(datein, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarBstrFromDec(pdecin: *const super::DECIMAL, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromDec(pdecin : *const super::DECIMAL, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromDec(pdecin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarBstrFromDisp<P0>(pdispin: P0, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromDisp(pdispin.param().abi(), lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromI1(cin: super::CHAR, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI1(cin : super::CHAR, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromI1(cin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromI2(ival: super::SHORT, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI2(ival : super::SHORT, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromI2(ival, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromI4(lin: super::LONG, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI4(lin : super::LONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromI4(lin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromI8(i64in: super::LONG64, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromI8(i64in : super::LONG64, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromI8(i64in, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromInt(lin: super::LONG, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarBstrFromI4" fn VarBstrFromInt(lin : super::LONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromInt(lin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarBstrFromR4(fltin: super::FLOAT, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromR4(fltin : super::FLOAT, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromR4(fltin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarBstrFromR8(dblin: super::DOUBLE, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromR8(dblin : super::DOUBLE, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromR8(dblin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromUI1(bval: super::BYTE, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI1(bval : super::BYTE, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromUI1(bval, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromUI2(uiin: super::USHORT, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI2(uiin : super::USHORT, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromUI2(uiin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromUI4(ulin: super::ULONG, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI4(ulin : super::ULONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromUI4(ulin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromUI8(ui64in: super::ULONG64, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarBstrFromUI8(ui64in : super::ULONG64, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromUI8(ui64in, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarBstrFromUint(ulin: super::ULONG, lcid: super::LCID, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarBstrFromUI4" fn VarBstrFromUint(ulin : super::ULONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarBstrFromUint(ulin, lcid, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarCat(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCat(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarCat(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarCmp(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, lcid: super::LCID, dwflags: super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCmp(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, lcid : super::LCID, dwflags : super::ULONG) -> windows_core::HRESULT);
    unsafe { VarCmp(pvarleft, pvarright, lcid, dwflags) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyAbs(cyin: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyAbs(cyin : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyAbs(cyin, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyAdd(cyleft: super::CY, cyright: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyAdd(cyleft : super::CY, cyright : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyAdd(cyleft, cyright, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyCmp(cyleft: super::CY, cyright: super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyCmp(cyleft : super::CY, cyright : super::CY) -> windows_core::HRESULT);
    unsafe { VarCyCmp(cyleft, cyright) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyCmpR8(cyleft: super::CY, dblright: f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyCmpR8(cyleft : super::CY, dblright : f64) -> windows_core::HRESULT);
    unsafe { VarCyCmpR8(cyleft, dblright) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFix(cyin: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFix(cyin : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyFix(cyin, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromBool(boolin: super::VARIANT_BOOL, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromBool(boolin : super::VARIANT_BOOL, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromBool(boolin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromDate(datein: super::DATE, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromDate(datein : super::DATE, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromDate(datein, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromDec(pdecin: *const super::DECIMAL, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromDec(pdecin : *const super::DECIMAL, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromDec(pdecin, pcyout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromDisp<P0>(pdispin: P0, lcid: super::LCID, pcyout: *mut super::CY) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromDisp(pdispin.param().abi(), lcid, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromI1(cin: super::CHAR, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI1(cin : super::CHAR, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromI1(cin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromI2(sin: super::SHORT, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI2(sin : super::SHORT, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromI2(sin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromI4(lin: super::LONG, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI4(lin : super::LONG, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromI4(lin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromI8(i64in: super::LONG64, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromI8(i64in : super::LONG64, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromI8(i64in, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromInt(lin: super::LONG, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarCyFromI4" fn VarCyFromInt(lin : super::LONG, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromInt(lin, pcyout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromR4(fltin: super::FLOAT, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromR4(fltin : super::FLOAT, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromR4(fltin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarCyFromR8(dblin: super::DOUBLE, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromR8(dblin : super::DOUBLE, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromR8(dblin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarCyFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromStr(strin, lcid, dwflags, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromUI1(bin: super::BYTE, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI1(bin : super::BYTE, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromUI1(bin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromUI2(uiin: super::USHORT, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI2(uiin : super::USHORT, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromUI2(uiin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromUI4(ulin: super::ULONG, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI4(ulin : super::ULONG, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromUI4(ulin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromUI8(ui64in: super::ULONG64, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyFromUI8(ui64in : super::ULONG64, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromUI8(ui64in, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyFromUint(ulin: super::ULONG, pcyout: *mut super::CY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarCyFromUI4" fn VarCyFromUint(ulin : super::ULONG, pcyout : *mut super::CY) -> windows_core::HRESULT);
    unsafe { VarCyFromUint(ulin, pcyout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyInt(cyin: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyInt(cyin : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyInt(cyin, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyMul(cyleft: super::CY, cyright: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyMul(cyleft : super::CY, cyright : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyMul(cyleft, cyright, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyMulI4(cyleft: super::CY, lright: super::LONG, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyMulI4(cyleft : super::CY, lright : super::LONG, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyMulI4(cyleft, lright, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyMulI8(cyleft: super::CY, lright: super::LONG64, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyMulI8(cyleft : super::CY, lright : super::LONG64, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyMulI8(cyleft, lright, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyNeg(cyin: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyNeg(cyin : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyNeg(cyin, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCyRound(cyin: super::CY, cdecimals: i32, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCyRound(cyin : super::CY, cdecimals : i32, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCyRound(cyin, cdecimals, pcyresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarCySub(cyleft: super::CY, cyright: super::CY, pcyresult: super::LPCY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarCySub(cyleft : super::CY, cyright : super::CY, pcyresult : super::LPCY) -> windows_core::HRESULT);
    unsafe { VarCySub(cyleft, cyright, pcyresult as _) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn VarDateFromBool(boolin: super::VARIANT_BOOL, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromBool(boolin : super::VARIANT_BOOL, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromBool(boolin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromCy(cyin: super::CY, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromCy(cyin : super::CY, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromCy(cyin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromDec(pdecin: *const super::DECIMAL, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromDec(pdecin : *const super::DECIMAL, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromDec(pdecin, pdateout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromDisp<P0>(pdispin: P0, lcid: super::LCID, pdateout: *mut super::DATE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromDisp(pdispin.param().abi(), lcid, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromI1(cin: super::CHAR, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI1(cin : super::CHAR, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromI1(cin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromI2(sin: super::SHORT, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI2(sin : super::SHORT, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromI2(sin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromI4(lin: super::LONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI4(lin : super::LONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromI4(lin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromI8(i64in: super::LONG64, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromI8(i64in : super::LONG64, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromI8(i64in, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromInt(lin: super::LONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarDateFromI4" fn VarDateFromInt(lin : super::LONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromInt(lin, pdateout as _) }
}
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromR4(fltin: super::FLOAT, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromR4(fltin : super::FLOAT, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromR4(fltin, pdateout as _) }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarDateFromR8(dblin: super::DOUBLE, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromR8(dblin : super::DOUBLE, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromR8(dblin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarDateFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromStr(strin, lcid, dwflags, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUI1(bin: super::BYTE, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI1(bin : super::BYTE, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUI1(bin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUI2(uiin: super::USHORT, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI2(uiin : super::USHORT, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUI2(uiin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUI4(ulin: super::ULONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI4(ulin : super::ULONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUI4(ulin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUI8(ui64in: super::ULONG64, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUI8(ui64in : super::ULONG64, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUI8(ui64in, pdateout as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUdate(pudatein: *const UDATE, dwflags: super::ULONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUdate(pudatein : *const UDATE, dwflags : super::ULONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUdate(pudatein, dwflags, pdateout as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: super::LCID, dwflags: super::ULONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDateFromUdateEx(pudatein : *const UDATE, lcid : super::LCID, dwflags : super::ULONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUdateEx(pudatein, lcid, dwflags, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDateFromUint(ulin: super::ULONG, pdateout: *mut super::DATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarDateFromUI4" fn VarDateFromUint(ulin : super::ULONG, pdateout : *mut super::DATE) -> windows_core::HRESULT);
    unsafe { VarDateFromUint(ulin, pdateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecAbs(pdecin: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecAbs(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecAbs(pdecin, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecAdd(pdecleft: super::LPDECIMAL, pdecright: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecAdd(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecAdd(pdecleft, pdecright, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecCmp(pdecleft: super::LPDECIMAL, pdecright: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecCmp(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecCmp(pdecleft, pdecright) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecCmpR8(pdecleft: super::LPDECIMAL, dblright: f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecCmpR8(pdecleft : super::LPDECIMAL, dblright : f64) -> windows_core::HRESULT);
    unsafe { VarDecCmpR8(pdecleft, dblright) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecDiv(pdecleft: super::LPDECIMAL, pdecright: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecDiv(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecDiv(pdecleft, pdecright, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFix(pdecin: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFix(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFix(pdecin, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromBool(boolin: super::VARIANT_BOOL, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromBool(boolin : super::VARIANT_BOOL, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromBool(boolin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromCy(cyin: super::CY, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromCy(cyin : super::CY, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromCy(cyin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromDate(datein: super::DATE, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromDate(datein : super::DATE, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromDate(datein, pdecout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromDisp<P0>(pdispin: P0, lcid: super::LCID, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromDisp(pdispin.param().abi(), lcid, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromI1(cin: super::CHAR, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI1(cin : super::CHAR, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromI1(cin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromI2(uiin: super::SHORT, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI2(uiin : super::SHORT, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromI2(uiin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromI4(lin: super::LONG, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI4(lin : super::LONG, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromI4(lin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromI8(i64in: super::LONG64, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromI8(i64in : super::LONG64, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromI8(i64in, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromInt(lin: super::LONG, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarDecFromI4" fn VarDecFromInt(lin : super::LONG, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromInt(lin, pdecout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromR4(fltin: super::FLOAT, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromR4(fltin : super::FLOAT, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromR4(fltin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarDecFromR8(dblin: super::DOUBLE, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromR8(dblin : super::DOUBLE, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromR8(dblin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarDecFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromStr(strin, lcid, dwflags, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromUI1(bin: super::BYTE, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI1(bin : super::BYTE, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromUI1(bin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromUI2(uiin: super::USHORT, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI2(uiin : super::USHORT, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromUI2(uiin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromUI4(ulin: super::ULONG, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI4(ulin : super::ULONG, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromUI4(ulin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromUI8(ui64in: super::ULONG64, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecFromUI8(ui64in : super::ULONG64, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromUI8(ui64in, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecFromUint(ulin: super::ULONG, pdecout: *mut super::DECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarDecFromUI4" fn VarDecFromUint(ulin : super::ULONG, pdecout : *mut super::DECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecFromUint(ulin, pdecout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecInt(pdecin: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecInt(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecInt(pdecin, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecMul(pdecleft: super::LPDECIMAL, pdecright: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecMul(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecMul(pdecleft, pdecright, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecNeg(pdecin: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecNeg(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecNeg(pdecin, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecRound(pdecin: super::LPDECIMAL, cdecimals: i32, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecRound(pdecin : super::LPDECIMAL, cdecimals : i32, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecRound(pdecin, cdecimals, pdecresult as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarDecSub(pdecleft: super::LPDECIMAL, pdecright: super::LPDECIMAL, pdecresult: super::LPDECIMAL) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDecSub(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_core::HRESULT);
    unsafe { VarDecSub(pdecleft, pdecright, pdecresult as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarDiv(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarDiv(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarDiv(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarEqv(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarEqv(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarEqv(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFix(pvarin: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFix(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarFix(pvarin, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormat(pvarin: super::LPVARIANT, pstrformat: Option<super::LPOLESTR>, ifirstday: i32, ifirstweek: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFormat(pvarin : super::LPVARIANT, pstrformat : super::LPOLESTR, ifirstday : i32, ifirstweek : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarFormat(pvarin, pstrformat.unwrap_or(core::mem::zeroed()) as _, ifirstday, ifirstweek, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatCurrency(pvarin: super::LPVARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatCurrency(pvarin : super::LPVARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarFormatCurrency(pvarin, inumdig, iinclead, iuseparens, igroup, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatDateTime(pvarin: super::LPVARIANT, inamedformat: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatDateTime(pvarin : super::LPVARIANT, inamedformat : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarFormatDateTime(pvarin, inamedformat, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatFromTokens(pvarin: super::LPVARIANT, pstrformat: Option<super::LPOLESTR>, pbtokcur: super::LPBYTE, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR, lcid: super::LCID) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatFromTokens(pvarin : super::LPVARIANT, pstrformat : super::LPOLESTR, pbtokcur : super::LPBYTE, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void, lcid : super::LCID) -> windows_core::HRESULT);
    unsafe { VarFormatFromTokens(pvarin, pstrformat.unwrap_or(core::mem::zeroed()) as _, pbtokcur, dwflags, core::mem::transmute(pbstrout), lcid) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatNumber(pvarin: super::LPVARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatNumber(pvarin : super::LPVARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarFormatNumber(pvarin, inumdig, iinclead, iuseparens, igroup, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarFormatPercent(pvarin: super::LPVARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarFormatPercent(pvarin : super::LPVARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarFormatPercent(pvarin, inumdig, iinclead, iuseparens, igroup, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI1FromBool(boolin: super::VARIANT_BOOL, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromBool(boolin : super::VARIANT_BOOL, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromBool(boolin, pcout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI1FromCy(cyin: super::CY, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromCy(cyin : super::CY, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromCy(cyin, pcout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI1FromDate(datein: super::DATE, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : super::DATE, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromDate(datein, pcout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI1FromDec(pdecin: *const super::DECIMAL, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromDec(pdecin : *const super::DECIMAL, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromDec(pdecin, pcout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarI1FromDisp<P0>(pdispin: P0, lcid: super::LCID, pcout: *mut super::CHAR) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromDisp(pdispin.param().abi(), lcid, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromI2(uiin: super::SHORT, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromI2(uiin : super::SHORT, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromI2(uiin, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromI4(lin: super::LONG, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromI4(lin : super::LONG, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromI4(lin, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromI8(i64in: super::LONG64, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromI8(i64in : super::LONG64, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromI8(i64in, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromInt(lin: super::LONG, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI1FromI4" fn VarI1FromInt(lin : super::LONG, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromInt(lin, pcout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarI1FromR4(fltin: super::FLOAT, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromR4(fltin : super::FLOAT, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromR4(fltin, pcout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI1FromR8(dblin: super::DOUBLE, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromR8(dblin : super::DOUBLE, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromR8(dblin, pcout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI1FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromStr(strin, lcid, dwflags, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromUI1(bin: super::BYTE, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI1(bin : super::BYTE, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromUI1(bin, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromUI2(uiin: super::USHORT, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI2(uiin : super::USHORT, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromUI2(uiin, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromUI4(ulin: super::ULONG, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI4(ulin : super::ULONG, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromUI4(ulin, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromUI8(i64in: super::ULONG64, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI1FromUI8(i64in : super::ULONG64, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromUI8(i64in, pcout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI1FromUint(ulin: super::ULONG, pcout: *mut super::CHAR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI1FromUI4" fn VarI1FromUint(ulin : super::ULONG, pcout : *mut super::CHAR) -> windows_core::HRESULT);
    unsafe { VarI1FromUint(ulin, pcout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI2FromBool(boolin: super::VARIANT_BOOL, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromBool(boolin : super::VARIANT_BOOL, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromBool(boolin, psout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI2FromCy(cyin: super::CY, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromCy(cyin : super::CY, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromCy(cyin, psout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI2FromDate(datein: super::DATE, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromDate(datein : super::DATE, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromDate(datein, psout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI2FromDec(pdecin: *const super::DECIMAL, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromDec(pdecin : *const super::DECIMAL, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromDec(pdecin, psout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarI2FromDisp<P0>(pdispin: P0, lcid: super::LCID, psout: *mut super::SHORT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromDisp(pdispin.param().abi(), lcid, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromI1(cin: super::CHAR, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromI1(cin : super::CHAR, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromI1(cin, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromI4(lin: super::LONG, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromI4(lin : super::LONG, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromI4(lin, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromI8(i64in: super::LONG64, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromI8(i64in : super::LONG64, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromI8(i64in, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromInt(lin: super::LONG, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI2FromI4" fn VarI2FromInt(lin : super::LONG, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromInt(lin, psout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarI2FromR4(fltin: super::FLOAT, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromR4(fltin : super::FLOAT, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromR4(fltin, psout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI2FromR8(dblin: super::DOUBLE, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromR8(dblin : super::DOUBLE, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromR8(dblin, psout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI2FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromStr(strin, lcid, dwflags, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromUI1(bin: super::BYTE, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI1(bin : super::BYTE, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromUI1(bin, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromUI2(uiin: super::USHORT, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI2(uiin : super::USHORT, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromUI2(uiin, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromUI4(ulin: super::ULONG, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI4(ulin : super::ULONG, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromUI4(ulin, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromUI8(ui64in: super::ULONG64, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI2FromUI8(ui64in : super::ULONG64, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromUI8(ui64in, psout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI2FromUint(ulin: super::ULONG, psout: *mut super::SHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI2FromUI4" fn VarI2FromUint(ulin : super::ULONG, psout : *mut super::SHORT) -> windows_core::HRESULT);
    unsafe { VarI2FromUint(ulin, psout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI4FromBool(boolin: super::VARIANT_BOOL, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromBool(boolin : super::VARIANT_BOOL, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromBool(boolin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI4FromCy(cyin: super::CY, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromCy(cyin : super::CY, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromCy(cyin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI4FromDate(datein: super::DATE, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromDate(datein : super::DATE, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromDate(datein, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI4FromDec(pdecin: *const super::DECIMAL, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromDec(pdecin : *const super::DECIMAL, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromDec(pdecin, plout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarI4FromDisp<P0>(pdispin: P0, lcid: super::LCID, plout: *mut super::LONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromDisp(pdispin.param().abi(), lcid, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromI1(cin: super::CHAR, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromI1(cin : super::CHAR, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromI1(cin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromI2(sin: super::SHORT, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromI2(sin : super::SHORT, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromI2(sin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromI8(i64in: super::LONG64, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromI8(i64in : super::LONG64, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromI8(i64in, plout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarI4FromR4(fltin: super::FLOAT, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromR4(fltin : super::FLOAT, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromR4(fltin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI4FromR8(dblin: super::DOUBLE, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromR8(dblin : super::DOUBLE, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromR8(dblin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI4FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromStr(strin, lcid, dwflags, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromUI1(bin: super::BYTE, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI1(bin : super::BYTE, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromUI1(bin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromUI2(uiin: super::USHORT, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI2(uiin : super::USHORT, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromUI2(uiin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromUI4(ulin: super::ULONG, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI4(ulin : super::ULONG, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromUI4(ulin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromUI8(ui64in: super::ULONG64, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI4FromUI8(ui64in : super::ULONG64, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromUI8(ui64in, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI4FromUint(ulin: super::ULONG, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromUI4" fn VarI4FromUint(ulin : super::ULONG, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarI4FromUint(ulin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI8FromBool(boolin: super::VARIANT_BOOL, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromBool(boolin : super::VARIANT_BOOL, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromBool(boolin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI8FromCy(cyin: super::CY, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromCy(cyin : super::CY, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromCy(cyin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI8FromDate(datein: super::DATE, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromDate(datein : super::DATE, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromDate(datein, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarI8FromDec(pdecin: *const super::DECIMAL, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromDec(pdecin : *const super::DECIMAL, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromDec(pdecin, pi64out as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarI8FromDisp<P0>(pdispin: P0, lcid: super::LCID, pi64out: *mut super::LONG64) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromDisp(pdispin.param().abi(), lcid, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromI1(cin: super::CHAR, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromI1(cin : super::CHAR, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromI1(cin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromI2(sin: super::SHORT, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromI2(sin : super::SHORT, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromI2(sin, pi64out as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarI8FromR4(fltin: super::FLOAT, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromR4(fltin : super::FLOAT, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromR4(fltin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI8FromR8(dblin: super::DOUBLE, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromR8(dblin : super::DOUBLE, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromR8(dblin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarI8FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromStr(strin, lcid, dwflags, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromUI1(bin: super::BYTE, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI1(bin : super::BYTE, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromUI1(bin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromUI2(uiin: super::USHORT, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI2(uiin : super::USHORT, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromUI2(uiin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromUI4(ulin: super::ULONG, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI4(ulin : super::ULONG, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromUI4(ulin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromUI8(ui64in: super::ULONG64, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarI8FromUI8(ui64in : super::ULONG64, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromUI8(ui64in, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarI8FromUint(ulin: super::ULONG, pi64out: *mut super::LONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI8FromUI4" fn VarI8FromUint(ulin : super::ULONG, pi64out : *mut super::LONG64) -> windows_core::HRESULT);
    unsafe { VarI8FromUint(ulin, pi64out as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarIdiv(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarIdiv(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarIdiv(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarImp(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarImp(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarImp(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarInt(pvarin: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarInt(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarInt(pvarin, pvarresult) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarIntFromBool(boolin: super::VARIANT_BOOL, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromBool" fn VarIntFromBool(boolin : super::VARIANT_BOOL, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromBool(boolin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarIntFromCy(cyin: super::CY, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromCy" fn VarIntFromCy(cyin : super::CY, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromCy(cyin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarIntFromDate(datein: super::DATE, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromDate" fn VarIntFromDate(datein : super::DATE, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromDate(datein, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarIntFromDec(pdecin: *const super::DECIMAL, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromDec" fn VarIntFromDec(pdecin : *const super::DECIMAL, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromDec(pdecin, plout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarIntFromDisp<P0>(pdispin: P0, lcid: super::LCID, plout: *mut super::LONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" "VarI4FromDisp" fn VarIntFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromDisp(pdispin.param().abi(), lcid, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromI1(cin: super::CHAR, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromI1" fn VarIntFromI1(cin : super::CHAR, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromI1(cin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromI2(sin: super::SHORT, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromI2" fn VarIntFromI2(sin : super::SHORT, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromI2(sin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromI8(i64in: super::LONG64, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromI8" fn VarIntFromI8(i64in : super::LONG64, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromI8(i64in, plout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarIntFromR4(fltin: super::FLOAT, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromR4" fn VarIntFromR4(fltin : super::FLOAT, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromR4(fltin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarIntFromR8(dblin: super::DOUBLE, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromR8" fn VarIntFromR8(dblin : super::DOUBLE, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromR8(dblin, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarIntFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromStr" fn VarIntFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromStr(strin, lcid, dwflags, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromUI1(bin: super::BYTE, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromUI1" fn VarIntFromUI1(bin : super::BYTE, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromUI1(bin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromUI2(uiin: super::USHORT, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromUI2" fn VarIntFromUI2(uiin : super::USHORT, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromUI2(uiin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromUI4(ulin: super::ULONG, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromUI4" fn VarIntFromUI4(ulin : super::ULONG, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromUI4(ulin, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromUI8(ui64in: super::ULONG64, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromUI8" fn VarIntFromUI8(ui64in : super::ULONG64, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromUI8(ui64in, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarIntFromUint(ulin: super::ULONG, plout: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarI4FromUI4" fn VarIntFromUint(ulin : super::ULONG, plout : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { VarIntFromUint(ulin, plout as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarMod(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarMod(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarMod(pvarleft, pvarright, pvarresult) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarMonthName(imonth : i32, fabbrev : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarMonthName(imonth, fabbrev, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarMul(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarMul(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarMul(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarNeg(pvarin: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarNeg(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarNeg(pvarin, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarNot(pvarin: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarNot(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarNot(pvarin, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const super::BYTE, dwvtbits: super::ULONG, pvar: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarNumFromParseNum(pnumprs : *const NUMPARSE, rgbdig : *const super::BYTE, dwvtbits : super::ULONG, pvar : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { VarNumFromParseNum(pnumprs, rgbdig, dwvtbits, pvar) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarOr(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarOr(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarOr(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarParseNumFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pnumprs: *mut NUMPARSE, rgbdig: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarParseNumFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pnumprs : *mut NUMPARSE, rgbdig : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarParseNumFromStr(strin, lcid, dwflags, pnumprs as _, rgbdig as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarPow(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarPow(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarPow(pvarleft, pvarright, pvarresult) }
}
#[inline]
pub unsafe fn VarR4CmpR8(fltleft: f32, dblright: f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4CmpR8(fltleft : f32, dblright : f64) -> windows_core::HRESULT);
    unsafe { VarR4CmpR8(fltleft, dblright) }
}
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
#[inline]
pub unsafe fn VarR4FromBool(boolin: super::VARIANT_BOOL, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromBool(boolin : super::VARIANT_BOOL, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromBool(boolin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarR4FromCy(cyin: super::CY, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromCy(cyin : super::CY, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromCy(cyin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
#[inline]
pub unsafe fn VarR4FromDate(datein: super::DATE, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromDate(datein : super::DATE, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromDate(datein, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarR4FromDec(pdecin: *const super::DECIMAL, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromDec(pdecin : *const super::DECIMAL, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromDec(pdecin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromDisp<P0>(pdispin: P0, lcid: super::LCID, pfltout: *mut super::FLOAT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromDisp(pdispin.param().abi(), lcid, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromI1(cin: super::CHAR, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI1(cin : super::CHAR, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromI1(cin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromI2(sin: super::SHORT, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI2(sin : super::SHORT, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromI2(sin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromI4(lin: super::LONG, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI4(lin : super::LONG, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromI4(lin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromI8(i64in: super::LONG64, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromI8(i64in : super::LONG64, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromI8(i64in, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromInt(lin: super::LONG, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarR4FromI4" fn VarR4FromInt(lin : super::LONG, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromInt(lin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR4FromR8(dblin: super::DOUBLE, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromR8(dblin : super::DOUBLE, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromR8(dblin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR4FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromStr(strin, lcid, dwflags, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromUI1(bin: super::BYTE, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI1(bin : super::BYTE, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromUI1(bin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromUI2(uiin: super::USHORT, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI2(uiin : super::USHORT, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromUI2(uiin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromUI4(ulin: super::ULONG, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI4(ulin : super::ULONG, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromUI4(ulin, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromUI8(ui64in: super::ULONG64, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR4FromUI8(ui64in : super::ULONG64, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromUI8(ui64in, pfltout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarR4FromUint(ulin: super::ULONG, pfltout: *mut super::FLOAT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarR4FromUI4" fn VarR4FromUint(ulin : super::ULONG, pfltout : *mut super::FLOAT) -> windows_core::HRESULT);
    unsafe { VarR4FromUint(ulin, pfltout as _) }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromBool(boolin: super::VARIANT_BOOL, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromBool(boolin : super::VARIANT_BOOL, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromBool(boolin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromCy(cyin: super::CY, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromCy(cyin : super::CY, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromCy(cyin, pdblout as _) }
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromDate(datein: super::DATE, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromDate(datein : super::DATE, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromDate(datein, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromDec(pdecin: *const super::DECIMAL, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromDec(pdecin : *const super::DECIMAL, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromDec(pdecin, pdblout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromDisp<P0>(pdispin: P0, lcid: super::LCID, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromDisp(pdispin.param().abi(), lcid, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromI1(cin: super::CHAR, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI1(cin : super::CHAR, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromI1(cin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromI2(sin: super::SHORT, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI2(sin : super::SHORT, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromI2(sin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromI4(lin: super::LONG, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI4(lin : super::LONG, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromI4(lin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromI8(i64in: super::LONG64, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromI8(i64in : super::LONG64, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromI8(i64in, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromInt(lin: super::LONG, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarR8FromI4" fn VarR8FromInt(lin : super::LONG, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromInt(lin, pdblout as _) }
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromR4(fltin: super::FLOAT, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromR4(fltin : super::FLOAT, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromR4(fltin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromStr(strin, lcid, dwflags, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromUI1(bin: super::BYTE, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI1(bin : super::BYTE, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromUI1(bin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromUI2(uiin: super::USHORT, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI2(uiin : super::USHORT, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromUI2(uiin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromUI4(ulin: super::ULONG, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI4(ulin : super::ULONG, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromUI4(ulin, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromUI8(ui64in: super::ULONG64, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8FromUI8(ui64in : super::ULONG64, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromUI8(ui64in, pdblout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarR8FromUint(ulin: super::ULONG, pdblout: *mut super::DOUBLE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarR8FromUI4" fn VarR8FromUint(ulin : super::ULONG, pdblout : *mut super::DOUBLE) -> windows_core::HRESULT);
    unsafe { VarR8FromUint(ulin, pdblout as _) }
}
#[inline]
pub unsafe fn VarR8Pow(dblleft: f64, dblright: f64, pdblresult: *mut f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8Pow(dblleft : f64, dblright : f64, pdblresult : *mut f64) -> windows_core::HRESULT);
    unsafe { VarR8Pow(dblleft, dblright, pdblresult as _) }
}
#[inline]
pub unsafe fn VarR8Round(dblin: f64, cdecimals: i32, pdblresult: *mut f64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarR8Round(dblin : f64, cdecimals : i32, pdblresult : *mut f64) -> windows_core::HRESULT);
    unsafe { VarR8Round(dblin, cdecimals, pdblresult as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarRound(pvarin: super::LPVARIANT, cdecimals: i32, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarRound(pvarin : super::LPVARIANT, cdecimals : i32, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarRound(pvarin, cdecimals, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarSub(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarSub(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarSub(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarTokenizeFormatString(pstrformat: Option<super::LPOLESTR>, rgbtok: &mut [u8], ifirstday: i32, ifirstweek: i32, lcid: super::LCID, pcbactual: Option<*const i32>) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarTokenizeFormatString(pstrformat : super::LPOLESTR, rgbtok : super::LPBYTE, cbtok : i32, ifirstday : i32, ifirstweek : i32, lcid : super::LCID, pcbactual : *const i32) -> windows_core::HRESULT);
    unsafe { VarTokenizeFormatString(pstrformat.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(rgbtok.as_mut_ptr()), rgbtok.len().try_into().unwrap(), ifirstday, ifirstweek, lcid, pcbactual.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI1FromBool(boolin: super::VARIANT_BOOL, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromBool(boolin : super::VARIANT_BOOL, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromBool(boolin, pbout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI1FromCy(cyin: super::CY, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromCy(cyin : super::CY, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromCy(cyin, pbout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI1FromDate(datein: super::DATE, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromDate(datein : super::DATE, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromDate(datein, pbout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI1FromDec(pdecin: *const super::DECIMAL, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromDec(pdecin : *const super::DECIMAL, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromDec(pdecin, pbout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarUI1FromDisp<P0>(pdispin: P0, lcid: super::LCID, pbout: *mut super::BYTE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromDisp(pdispin.param().abi(), lcid, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromI1(cin: super::CHAR, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI1(cin : super::CHAR, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromI1(cin, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromI2(sin: super::SHORT, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI2(sin : super::SHORT, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromI2(sin, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromI4(lin: super::LONG, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI4(lin : super::LONG, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromI4(lin, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromI8(i64in: super::LONG64, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromI8(i64in : super::LONG64, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromI8(i64in, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromInt(lin: super::LONG, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI1FromI4" fn VarUI1FromInt(lin : super::LONG, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromInt(lin, pbout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarUI1FromR4(fltin: super::FLOAT, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromR4(fltin : super::FLOAT, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromR4(fltin, pbout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI1FromR8(dblin: super::DOUBLE, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromR8(dblin : super::DOUBLE, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromR8(dblin, pbout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI1FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromStr(strin, lcid, dwflags, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromUI2(uiin: super::USHORT, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromUI2(uiin : super::USHORT, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromUI2(uiin, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromUI4(ulin: super::ULONG, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromUI4(ulin : super::ULONG, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromUI4(ulin, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromUI8(ui64in: super::ULONG64, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI1FromUI8(ui64in : super::ULONG64, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromUI8(ui64in, pbout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI1FromUint(ulin: super::ULONG, pbout: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI1FromUI4" fn VarUI1FromUint(ulin : super::ULONG, pbout : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { VarUI1FromUint(ulin, pbout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI2FromBool(boolin: super::VARIANT_BOOL, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromBool(boolin : super::VARIANT_BOOL, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromBool(boolin, puiout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI2FromCy(cyin: super::CY, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromCy(cyin : super::CY, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromCy(cyin, puiout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI2FromDate(datein: super::DATE, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromDate(datein : super::DATE, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromDate(datein, puiout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI2FromDec(pdecin: *const super::DECIMAL, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromDec(pdecin : *const super::DECIMAL, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromDec(pdecin, puiout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarUI2FromDisp<P0>(pdispin: P0, lcid: super::LCID, puiout: *mut super::USHORT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromDisp(pdispin.param().abi(), lcid, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromI1(cin: super::CHAR, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI1(cin : super::CHAR, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromI1(cin, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromI2(uiin: super::SHORT, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI2(uiin : super::SHORT, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromI2(uiin, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromI4(lin: super::LONG, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI4(lin : super::LONG, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromI4(lin, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromI8(i64in: super::LONG64, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromI8(i64in : super::LONG64, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromI8(i64in, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromInt(lin: super::LONG, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI2FromI4" fn VarUI2FromInt(lin : super::LONG, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromInt(lin, puiout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarUI2FromR4(fltin: super::FLOAT, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromR4(fltin : super::FLOAT, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromR4(fltin, puiout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI2FromR8(dblin: super::DOUBLE, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromR8(dblin : super::DOUBLE, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromR8(dblin, puiout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI2FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromStr(strin, lcid, dwflags, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromUI1(bin: super::BYTE, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromUI1(bin : super::BYTE, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromUI1(bin, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromUI4(ulin: super::ULONG, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromUI4(ulin : super::ULONG, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromUI4(ulin, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromUI8(i64in: super::ULONG64, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI2FromUI8(i64in : super::ULONG64, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromUI8(i64in, puiout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI2FromUint(ulin: super::ULONG, puiout: *mut super::USHORT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI2FromUI4" fn VarUI2FromUint(ulin : super::ULONG, puiout : *mut super::USHORT) -> windows_core::HRESULT);
    unsafe { VarUI2FromUint(ulin, puiout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI4FromBool(boolin: super::VARIANT_BOOL, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromBool(boolin : super::VARIANT_BOOL, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromBool(boolin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI4FromCy(cyin: super::CY, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromCy(cyin : super::CY, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromCy(cyin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI4FromDate(datein: super::DATE, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromDate(datein : super::DATE, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromDate(datein, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI4FromDec(pdecin: *const super::DECIMAL, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromDec(pdecin : *const super::DECIMAL, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromDec(pdecin, pulout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarUI4FromDisp<P0>(pdispin: P0, lcid: super::LCID, pulout: *mut super::ULONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromDisp(pdispin.param().abi(), lcid, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromI1(cin: super::CHAR, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI1(cin : super::CHAR, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromI1(cin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromI2(uiin: super::SHORT, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI2(uiin : super::SHORT, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromI2(uiin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromI4(lin: super::LONG, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI4(lin : super::LONG, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromI4(lin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromI8(i64in: super::LONG64, plout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromI8(i64in : super::LONG64, plout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromI8(i64in, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromInt(lin: super::LONG, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromI4" fn VarUI4FromInt(lin : super::LONG, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromInt(lin, pulout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarUI4FromR4(fltin: super::FLOAT, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromR4(fltin : super::FLOAT, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromR4(fltin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI4FromR8(dblin: super::DOUBLE, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromR8(dblin : super::DOUBLE, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromR8(dblin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI4FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromStr(strin, lcid, dwflags, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromUI1(bin: super::BYTE, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromUI1(bin : super::BYTE, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromUI1(bin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromUI2(uiin: super::USHORT, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromUI2(uiin : super::USHORT, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromUI2(uiin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI4FromUI8(ui64in: super::ULONG64, plout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI4FromUI8(ui64in : super::ULONG64, plout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUI4FromUI8(ui64in, plout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI8FromBool(boolin: super::VARIANT_BOOL, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromBool(boolin : super::VARIANT_BOOL, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromBool(boolin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI8FromCy(cyin: super::CY, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromCy(cyin : super::CY, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromCy(cyin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI8FromDate(datein: super::DATE, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromDate(datein : super::DATE, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromDate(datein, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUI8FromDec(pdecin: *const super::DECIMAL, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromDec(pdecin : *const super::DECIMAL, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromDec(pdecin, pi64out as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarUI8FromDisp<P0>(pdispin: P0, lcid: super::LCID, pi64out: *mut super::ULONG64) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromDisp(pdispin.param().abi(), lcid, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI8FromI1(cin: super::CHAR, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromI1(cin : super::CHAR, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromI1(cin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI8FromI2(sin: super::SHORT, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromI2(sin : super::SHORT, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromI2(sin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI8FromI8(ui64in: super::LONG64, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromI8(ui64in : super::LONG64, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromI8(ui64in, pi64out as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarUI8FromR4(fltin: super::FLOAT, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromR4(fltin : super::FLOAT, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromR4(fltin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI8FromR8(dblin: super::DOUBLE, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromR8(dblin : super::DOUBLE, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromR8(dblin, pi64out as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUI8FromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromStr(strin, lcid, dwflags, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI8FromUI1(bin: super::BYTE, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromUI1(bin : super::BYTE, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromUI1(bin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI8FromUI2(uiin: super::USHORT, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromUI2(uiin : super::USHORT, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromUI2(uiin, pi64out as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUI8FromUI4(ulin: super::ULONG, pi64out: *mut super::ULONG64) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUI8FromUI4(ulin : super::ULONG, pi64out : *mut super::ULONG64) -> windows_core::HRESULT);
    unsafe { VarUI8FromUI4(ulin, pi64out as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUdateFromDate(datein: super::DATE, dwflags: super::ULONG, pudateout: *mut UDATE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarUdateFromDate(datein : super::DATE, dwflags : super::ULONG, pudateout : *mut UDATE) -> windows_core::HRESULT);
    unsafe { VarUdateFromDate(datein, dwflags, pudateout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUintFromBool(boolin: super::VARIANT_BOOL, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromBool" fn VarUintFromBool(boolin : super::VARIANT_BOOL, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromBool(boolin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUintFromCy(cyin: super::CY, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromCy" fn VarUintFromCy(cyin : super::CY, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromCy(cyin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUintFromDate(datein: super::DATE, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromDate" fn VarUintFromDate(datein : super::DATE, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromDate(datein, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn VarUintFromDec(pdecin: *const super::DECIMAL, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromDec" fn VarUintFromDec(pdecin : *const super::DECIMAL, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromDec(pdecin, pulout as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
#[inline]
pub unsafe fn VarUintFromDisp<P0>(pdispin: P0, lcid: super::LCID, pulout: *mut super::ULONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDispatch>,
{
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromDisp" fn VarUintFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromDisp(pdispin.param().abi(), lcid, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromI1(cin: super::CHAR, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromI1" fn VarUintFromI1(cin : super::CHAR, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromI1(cin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromI2(uiin: super::SHORT, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromI2" fn VarUintFromI2(uiin : super::SHORT, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromI2(uiin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromI4(lin: super::LONG, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromI4" fn VarUintFromI4(lin : super::LONG, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromI4(lin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromI8(i64in: super::LONG64, plout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromI8" fn VarUintFromI8(i64in : super::LONG64, plout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromI8(i64in, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromInt(lin: super::LONG, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromI4" fn VarUintFromInt(lin : super::LONG, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromInt(lin, pulout as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VarUintFromR4(fltin: super::FLOAT, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromR4" fn VarUintFromR4(fltin : super::FLOAT, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromR4(fltin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUintFromR8(dblin: super::DOUBLE, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromR8" fn VarUintFromR8(dblin : super::DOUBLE, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromR8(dblin, pulout as _) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarUintFromStr(strin: super::LPCOLESTR, lcid: super::LCID, dwflags: super::ULONG, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromStr" fn VarUintFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromStr(strin, lcid, dwflags, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromUI1(bin: super::BYTE, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromUI1" fn VarUintFromUI1(bin : super::BYTE, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromUI1(bin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromUI2(uiin: super::USHORT, pulout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromUI2" fn VarUintFromUI2(uiin : super::USHORT, pulout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromUI2(uiin, pulout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarUintFromUI8(ui64in: super::ULONG64, plout: *mut super::ULONG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" "VarUI4FromUI8" fn VarUintFromUI8(ui64in : super::ULONG64, plout : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { VarUintFromUI8(ui64in, plout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: super::ULONG, pbstrout: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarWeekdayName(iweekday : i32, fabbrev : i32, ifirstday : i32, dwflags : super::ULONG, pbstrout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { VarWeekdayName(iweekday, fabbrev, ifirstday, dwflags, core::mem::transmute(pbstrout)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VarXor(pvarleft: super::LPVARIANT, pvarright: super::LPVARIANT, pvarresult: super::LPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VarXor(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_core::HRESULT);
    unsafe { VarXor(pvarleft, pvarright, pvarresult) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantChangeType(pvargdest: *mut super::VARIANTARG, pvarsrc: *const super::VARIANTARG, wflags: super::USHORT, vt: super::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantChangeType(pvargdest : *mut super::VARIANTARG, pvarsrc : *const super::VARIANTARG, wflags : super::USHORT, vt : super::VARTYPE) -> windows_core::HRESULT);
    unsafe { VariantChangeType(pvargdest, pvarsrc, wflags, vt) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantChangeTypeEx(pvargdest: *mut super::VARIANTARG, pvarsrc: *const super::VARIANTARG, lcid: super::LCID, wflags: super::USHORT, vt: super::VARTYPE) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantChangeTypeEx(pvargdest : *mut super::VARIANTARG, pvarsrc : *const super::VARIANTARG, lcid : super::LCID, wflags : super::USHORT, vt : super::VARTYPE) -> windows_core::HRESULT);
    unsafe { VariantChangeTypeEx(pvargdest, pvarsrc, lcid, wflags, vt) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantClear(pvarg: *mut super::VARIANTARG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut super::VARIANTARG) -> windows_core::HRESULT);
    unsafe { VariantClear(pvarg) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantCopy(pvargdest: *mut super::VARIANTARG, pvargsrc: *const super::VARIANTARG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut super::VARIANTARG, pvargsrc : *const super::VARIANTARG) -> windows_core::HRESULT);
    unsafe { VariantCopy(pvargdest, pvargsrc) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantCopyInd(pvardest: *mut super::VARIANT, pvargsrc: *const super::VARIANTARG) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VariantCopyInd(pvardest : *mut super::VARIANT, pvargsrc : *const super::VARIANTARG) -> windows_core::HRESULT);
    unsafe { VariantCopyInd(pvardest, pvargsrc) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantInit(pvarg: *mut super::VARIANTARG) {
    windows_core::link!("oleaut32.dll" "system" fn VariantInit(pvarg : *mut super::VARIANTARG));
    unsafe { VariantInit(pvarg) }
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantTimeToDosDateTime(vtime: super::DOUBLE, pwdosdate: *mut super::USHORT, pwdostime: *mut super::USHORT) -> super::INT {
    windows_core::link!("oleaut32.dll" "system" fn VariantTimeToDosDateTime(vtime : super::DOUBLE, pwdosdate : *mut super::USHORT, pwdostime : *mut super::USHORT) -> super::INT);
    unsafe { VariantTimeToDosDateTime(vtime, pwdosdate as _, pwdostime as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VariantTimeToSystemTime(vtime: super::DOUBLE, lpsystemtime: super::LPSYSTEMTIME) -> super::INT {
    windows_core::link!("oleaut32.dll" "system" fn VariantTimeToSystemTime(vtime : super::DOUBLE, lpsystemtime : super::LPSYSTEMTIME) -> super::INT);
    unsafe { VariantTimeToSystemTime(vtime, lpsystemtime as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VectorFromBstr(bstr: &windows_core::BSTR, ppsa: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("oleaut32.dll" "system" fn VectorFromBstr(bstr : *mut core::ffi::c_void, ppsa : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { VectorFromBstr(core::mem::transmute_copy(bstr), ppsa as _) }
}
pub const ACTIVEOBJECT_STRONG: i32 = 0;
pub const ACTIVEOBJECT_WEAK: i32 = 1;
pub const DISPATCH_METHOD: i32 = 1;
pub const DISPATCH_PROPERTYGET: i32 = 2;
pub const DISPATCH_PROPERTYPUT: i32 = 4;
pub const DISPATCH_PROPERTYPUTREF: i32 = 8;
pub const ID_DEFAULTINST: i32 = -2;
#[repr(C, packed(8))]
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: super::UINT,
}
pub const LOAD_TLB_AS_32BIT: i32 = 32;
pub const LOAD_TLB_AS_64BIT: i32 = 64;
pub const LOCALE_USE_NLS: i32 = 268435456;
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type LPINTERFACEDATA = *mut INTERFACEDATA;
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
pub type LPMETHODDATA = *mut METHODDATA;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPPARAMDATA = *mut PARAMDATA;
pub const MASK_TO_RESET_TLB_BITS: i32 = -97;
pub const MEMBERID_NIL: i32 = -1;
#[repr(C, packed(8))]
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct METHODDATA {
    pub szName: *mut super::OLECHAR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: super::DISPID,
    pub iMeth: super::UINT,
    pub cc: super::CALLCONV,
    pub cArgs: super::UINT,
    pub wFlags: super::WORD,
    pub vtReturn: super::VARTYPE,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NUMPARSE {
    pub cDig: super::INT,
    pub dwInFlags: super::ULONG,
    pub dwOutFlags: super::ULONG,
    pub cchUsed: super::INT,
    pub nBaseShift: super::INT,
    pub nPwr10: super::INT,
}
pub const NUMPRS_CURRENCY: i32 = 1024;
pub const NUMPRS_DECIMAL: i32 = 256;
pub const NUMPRS_EXPONENT: i32 = 2048;
pub const NUMPRS_HEX_OCT: i32 = 64;
pub const NUMPRS_INEXACT: i32 = 131072;
pub const NUMPRS_LEADING_MINUS: i32 = 16;
pub const NUMPRS_LEADING_PLUS: i32 = 4;
pub const NUMPRS_LEADING_WHITE: i32 = 1;
pub const NUMPRS_NEG: i32 = 65536;
pub const NUMPRS_PARENS: i32 = 128;
pub const NUMPRS_STD: i32 = 8191;
pub const NUMPRS_THOUSANDS: i32 = 512;
pub const NUMPRS_TRAILING_MINUS: i32 = 32;
pub const NUMPRS_TRAILING_PLUS: i32 = 8;
pub const NUMPRS_TRAILING_WHITE: i32 = 2;
pub const NUMPRS_USE_ALL: i32 = 4096;
#[repr(C, packed(8))]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct PARAMDATA {
    pub szName: *mut super::OLECHAR,
    pub vt: super::VARTYPE,
}
pub type REGKIND = i32;
pub const REGKIND_DEFAULT: REGKIND = 0;
pub const REGKIND_NONE: REGKIND = 2;
pub const REGKIND_REGISTER: REGKIND = 1;
pub const STDOLE2_LCID: i32 = 0;
pub const STDOLE2_MAJORVERNUM: i32 = 2;
pub const STDOLE2_MINORVERNUM: i32 = 0;
pub const STDOLE_LCID: i32 = 0;
pub const STDOLE_MAJORVERNUM: i32 = 1;
pub const STDOLE_MINORVERNUM: i32 = 0;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct UDATE {
    pub st: super::SYSTEMTIME,
    pub wDayOfYear: super::USHORT,
}
pub const VARCMP_EQ: i32 = 1;
pub const VARCMP_GT: i32 = 2;
pub const VARCMP_LT: i32 = 0;
pub const VARCMP_NULL: i32 = 3;
pub const VARIANT_ALPHABOOL: i32 = 2;
pub const VARIANT_CALENDAR_GREGORIAN: i32 = 64;
pub const VARIANT_CALENDAR_HIJRI: i32 = 8;
pub const VARIANT_CALENDAR_THAI: i32 = 32;
pub const VARIANT_LOCALBOOL: i32 = 16;
pub const VARIANT_NOUSEROVERRIDE: i32 = 4;
pub const VARIANT_NOVALUEPROP: i32 = 1;
pub const VARIANT_USE_NLS: i32 = 128;
#[cfg(feature = "shared")]
pub const VAR_CALENDAR_GREGORIAN: super::DWORD = 256;
#[cfg(feature = "shared")]
pub const VAR_CALENDAR_HIJRI: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const VAR_CALENDAR_THAI: super::DWORD = 128;
#[cfg(feature = "shared")]
pub const VAR_DATEVALUEONLY: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const VAR_FORMAT_NOSUBSTITUTE: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const VAR_FOURDIGITYEARS: super::DWORD = 64;
#[cfg(feature = "shared")]
pub const VAR_LOCALBOOL: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const VAR_TIMEVALUEONLY: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const VAR_VALIDDATE: super::DWORD = 4;
pub const VTBIT_CY: i32 = 64;
pub const VTBIT_DECIMAL: i32 = 16384;
pub const VTBIT_I1: i32 = 65536;
pub const VTBIT_I2: i32 = 4;
pub const VTBIT_I4: i32 = 8;
pub const VTBIT_I8: i32 = 1048576;
pub const VTBIT_R4: i32 = 16;
pub const VTBIT_R8: i32 = 32;
pub const VTBIT_UI1: i32 = 131072;
pub const VTBIT_UI2: i32 = 262144;
pub const VTBIT_UI4: i32 = 524288;
pub const VTBIT_UI8: i32 = 2097152;
pub const VTDATEGRE_MAX: i32 = 2958465;
pub const VTDATEGRE_MIN: i32 = -657434;
#[cfg(feature = "wtypes")]
pub const VT_HARDTYPE: super::VARENUM = 32768;
