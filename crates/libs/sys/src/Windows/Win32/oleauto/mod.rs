#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn BstrFromVector(psa : *const super::SAFEARRAY, pbstr : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn ClearCustData(pcustdata : super::LPCUSTDATA));
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn CreateDispTypeInfo(pidata : *mut INTERFACEDATA, lcid : super::LCID, pptinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("oleaut32.dll" "system" fn CreateErrorInfo(pperrinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("oleaut32.dll" "system" fn CreateStdDispatch(punkouter : *mut core::ffi::c_void, pvthis : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, ppunkstddisp : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn CreateTypeLib(syskind : super::SYSKIND, szfile : super::LPCOLESTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn CreateTypeLib2(syskind : super::SYSKIND, szfile : super::LPCOLESTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispCallFunc(pvinstance : *const core::ffi::c_void, ovft : super::ULONG_PTR, cc : super::CALLCONV, vtreturn : super::VARTYPE, cactuals : super::UINT, prgvt : *const super::VARTYPE, prgpvarg : *const *const super::VARIANTARG, pvargresult : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispGetIDsOfNames(ptinfo : *mut core::ffi::c_void, rgsznames : *const super::LPOLESTR, cnames : super::UINT, rgdispid : *mut super::DISPID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispGetParam(pdispparams : *const super::DISPPARAMS, position : super::UINT, vttarg : super::VARTYPE, pvarresult : *mut super::VARIANT, puargerr : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispInvoke(_this : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, dispidmember : super::DISPID, wflags : super::WORD, pparams : *mut super::DISPPARAMS, pvarresult : *mut super::VARIANT, pexcepinfo : *mut super::EXCEPINFO, puargerr : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DosDateTimeToVariantTime(wdosdate : super::USHORT, wdostime : super::USHORT, pvtime : *mut super::DOUBLE) -> super::INT);
#[cfg(feature = "guiddef")]
windows_link::link!("oleaut32.dll" "system" fn GetActiveObject(rclsid : *const super::IID, pvreserved : *mut core::ffi::c_void, ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn GetAltMonthNames(lcid : super::LCID, prgp : *mut *mut super::LPOLESTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : super::ULONG, pperrinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn GetRecordInfoFromGuids(rguidtypelib : *const windows_sys::core::GUID, uvermajor : super::ULONG, uverminor : super::ULONG, lcid : super::LCID, rguidtypeinfo : *const windows_sys::core::GUID, pprecinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("oleaut32.dll" "system" fn GetRecordInfoFromTypeInfo(ptypeinfo : *mut core::ffi::c_void, pprecinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn LHashValOfNameSys(syskind : super::SYSKIND, lcid : super::LCID, szname : *const super::OLECHAR) -> super::ULONG);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn LHashValOfNameSysA(syskind : super::SYSKIND, lcid : super::LCID, szname : windows_sys::core::PCSTR) -> super::ULONG);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn LoadRegTypeLib(rguid : *const windows_sys::core::GUID, wvermajor : super::WORD, wverminor : super::WORD, lcid : super::LCID, pptlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn LoadTypeLib(szfile : super::LPCOLESTR, pptlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn LoadTypeLibEx(szfile : super::LPCOLESTR, regkind : REGKIND, pptlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn OaBuildVersion() -> super::ULONG);
windows_link::link!("oleaut32.dll" "system" fn OaEnablePerUserTLibRegistration());
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn QueryPathOfRegTypeLib(guid : *const windows_sys::core::GUID, wmaj : super::USHORT, wmin : super::USHORT, lcid : super::LCID, lpbstrpathname : super::LPBSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn RegisterActiveObject(punk : *mut core::ffi::c_void, rclsid : *const super::IID, dwflags : super::DWORD, pdwregister : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn RegisterTypeLib(ptlib : *mut core::ffi::c_void, szfullpath : super::LPCOLESTR, szhelpdir : super::LPCOLESTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn RegisterTypeLibForUser(ptlib : *mut core::ffi::c_void, szfullpath : *const super::OLECHAR, szhelpdir : *const super::OLECHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn RevokeActiveObject(dwregister : super::DWORD, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAccessData(psa : *const super::SAFEARRAY, ppvdata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAddRef(psa : *const super::SAFEARRAY, ppdatatorelease : *mut super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAllocData(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptor(cdims : super::UINT, ppsaout : *mut *mut super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptorEx(vt : super::VARTYPE, cdims : super::UINT, ppsaout : *mut *mut super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCopy(psa : *const super::SAFEARRAY, ppsaout : *mut *mut super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCopyData(psasource : *const super::SAFEARRAY, psatarget : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreate(vt : super::VARTYPE, cdims : super::UINT, rgsabound : *const super::SAFEARRAYBOUND) -> *mut super::SAFEARRAY);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreateEx(vt : super::VARTYPE, cdims : super::UINT, rgsabound : *const super::SAFEARRAYBOUND, pvextra : super::PVOID) -> *mut super::SAFEARRAY);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreateVector(vt : super::VARTYPE, llbound : super::LONG, celements : super::ULONG) -> *mut super::SAFEARRAY);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreateVectorEx(vt : super::VARTYPE, llbound : super::LONG, celements : super::ULONG, pvextra : super::PVOID) -> *mut super::SAFEARRAY);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayDestroy(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayDestroyData(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayDestroyDescriptor(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetDim(psa : *const super::SAFEARRAY) -> super::UINT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetElement(psa : *const super::SAFEARRAY, rgindices : *const super::LONG, pv : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetElemsize(psa : *const super::SAFEARRAY) -> super::UINT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetIID(psa : *const super::SAFEARRAY, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetLBound(psa : *const super::SAFEARRAY, ndim : super::UINT, pllbound : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetRecordInfo(psa : *const super::SAFEARRAY, prinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetUBound(psa : *const super::SAFEARRAY, ndim : super::UINT, plubound : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetVartype(psa : *const super::SAFEARRAY, pvt : *mut super::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayLock(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayPtrOfIndex(psa : *const super::SAFEARRAY, rgindices : *const super::LONG, ppvdata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayPutElement(psa : *const super::SAFEARRAY, rgindices : *const super::LONG, pv : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayRedim(psa : *mut super::SAFEARRAY, psaboundnew : *const super::SAFEARRAYBOUND) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayReleaseData(pdata : super::PVOID));
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayReleaseDescriptor(psa : *const super::SAFEARRAY));
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArraySetIID(psa : *const super::SAFEARRAY, guid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArraySetRecordInfo(psa : *const super::SAFEARRAY, prinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayUnaccessData(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayUnlock(psa : *const super::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : super::ULONG, perrinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn SysAddRefString(bstrstring : windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypesbase")]
windows_link::link!("oleaut32.dll" "system" fn SysAllocString(psz : *const super::OLECHAR) -> windows_sys::core::BSTR);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn SysAllocStringByteLen(psz : windows_sys::core::PCSTR, len : super::UINT) -> windows_sys::core::BSTR);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : *const super::OLECHAR, ui : super::UINT) -> windows_sys::core::BSTR);
windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : windows_sys::core::BSTR));
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn SysReAllocString(pbstr : *mut windows_sys::core::BSTR, psz : *const super::OLECHAR) -> super::INT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn SysReAllocStringLen(pbstr : *mut windows_sys::core::BSTR, psz : *const super::OLECHAR, len : u32) -> super::INT);
windows_link::link!("oleaut32.dll" "system" fn SysReleaseString(bstrstring : windows_sys::core::BSTR));
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn SysStringByteLen(bstr : windows_sys::core::BSTR) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : windows_sys::core::BSTR) -> super::UINT);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn SystemTimeToVariantTime(lpsystemtime : super::LPSYSTEMTIME, pvtime : *mut super::DOUBLE) -> super::INT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn UnRegisterTypeLib(libid : *const windows_sys::core::GUID, wvermajor : super::WORD, wverminor : super::WORD, lcid : super::LCID, syskind : super::SYSKIND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn UnRegisterTypeLibForUser(libid : *const windows_sys::core::GUID, wmajorvernum : super::WORD, wminorvernum : super::WORD, lcid : super::LCID, syskind : super::SYSKIND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarAbs(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarAdd(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarAnd(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromCy(cyin : super::CY, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromDate(datein : super::DATE, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromDec(pdecin : *const super::DECIMAL, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI1(cin : super::CHAR, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI2(sin : super::SHORT, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI4(lin : super::LONG, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI8(i64in : super::LONG64, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarBoolFromI4" fn VarBoolFromInt(lin : super::LONG, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromR4(fltin : super::FLOAT, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromR8(dblin : super::DOUBLE, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI1(bin : super::BYTE, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI2(uiin : super::USHORT, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI4(ulin : super::ULONG, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI8(i64in : super::ULONG64, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarBoolFromUI4" fn VarBoolFromUint(ulin : super::ULONG, pboolout : *mut super::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrCat(bstrleft : windows_sys::core::BSTR, bstrright : windows_sys::core::BSTR, pbstrresult : super::LPBSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrCmp(bstrleft : windows_sys::core::BSTR, bstrright : windows_sys::core::BSTR, lcid : super::LCID, dwflags : super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromBool(boolin : super::VARIANT_BOOL, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromCy(cyin : super::CY, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromDate(datein : super::DATE, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromDec(pdecin : *const super::DECIMAL, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI1(cin : super::CHAR, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI2(ival : super::SHORT, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI4(lin : super::LONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI8(i64in : super::LONG64, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarBstrFromI4" fn VarBstrFromInt(lin : super::LONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromR4(fltin : super::FLOAT, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromR8(dblin : super::DOUBLE, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI1(bval : super::BYTE, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI2(uiin : super::USHORT, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI4(ulin : super::ULONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI8(ui64in : super::ULONG64, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarBstrFromUI4" fn VarBstrFromUint(ulin : super::ULONG, lcid : super::LCID, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarCat(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarCmp(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, lcid : super::LCID, dwflags : super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyAbs(cyin : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyAdd(cyleft : super::CY, cyright : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyCmp(cyleft : super::CY, cyright : super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyCmpR8(cyleft : super::CY, dblright : f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFix(cyin : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromBool(boolin : super::VARIANT_BOOL, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromDate(datein : super::DATE, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromDec(pdecin : *const super::DECIMAL, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI1(cin : super::CHAR, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI2(sin : super::SHORT, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI4(lin : super::LONG, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI8(i64in : super::LONG64, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarCyFromI4" fn VarCyFromInt(lin : super::LONG, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromR4(fltin : super::FLOAT, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromR8(dblin : super::DOUBLE, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI1(bin : super::BYTE, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI2(uiin : super::USHORT, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI4(ulin : super::ULONG, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI8(ui64in : super::ULONG64, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarCyFromUI4" fn VarCyFromUint(ulin : super::ULONG, pcyout : *mut super::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyInt(cyin : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyMul(cyleft : super::CY, cyright : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyMulI4(cyleft : super::CY, lright : super::LONG, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyMulI8(cyleft : super::CY, lright : super::LONG64, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyNeg(cyin : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyRound(cyin : super::CY, cdecimals : i32, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCySub(cyleft : super::CY, cyright : super::CY, pcyresult : super::LPCY) -> windows_sys::core::HRESULT);
#[cfg(feature = "wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromBool(boolin : super::VARIANT_BOOL, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromCy(cyin : super::CY, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromDec(pdecin : *const super::DECIMAL, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI1(cin : super::CHAR, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI2(sin : super::SHORT, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI4(lin : super::LONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI8(i64in : super::LONG64, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarDateFromI4" fn VarDateFromInt(lin : super::LONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromR4(fltin : super::FLOAT, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromR8(dblin : super::DOUBLE, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI1(bin : super::BYTE, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI2(uiin : super::USHORT, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI4(ulin : super::ULONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI8(ui64in : super::ULONG64, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUdate(pudatein : *const UDATE, dwflags : super::ULONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUdateEx(pudatein : *const UDATE, lcid : super::LCID, dwflags : super::ULONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarDateFromUI4" fn VarDateFromUint(ulin : super::ULONG, pdateout : *mut super::DATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecAbs(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecAdd(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecCmp(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecCmpR8(pdecleft : super::LPDECIMAL, dblright : f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecDiv(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFix(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromBool(boolin : super::VARIANT_BOOL, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromCy(cyin : super::CY, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromDate(datein : super::DATE, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI1(cin : super::CHAR, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI2(uiin : super::SHORT, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI4(lin : super::LONG, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI8(i64in : super::LONG64, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarDecFromI4" fn VarDecFromInt(lin : super::LONG, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromR4(fltin : super::FLOAT, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromR8(dblin : super::DOUBLE, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI1(bin : super::BYTE, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI2(uiin : super::USHORT, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI4(ulin : super::ULONG, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI8(ui64in : super::ULONG64, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarDecFromUI4" fn VarDecFromUint(ulin : super::ULONG, pdecout : *mut super::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecInt(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecMul(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecNeg(pdecin : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecRound(pdecin : super::LPDECIMAL, cdecimals : i32, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecSub(pdecleft : super::LPDECIMAL, pdecright : super::LPDECIMAL, pdecresult : super::LPDECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarDiv(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarEqv(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFix(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormat(pvarin : super::LPVARIANT, pstrformat : super::LPOLESTR, ifirstday : i32, ifirstweek : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatCurrency(pvarin : super::LPVARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatDateTime(pvarin : super::LPVARIANT, inamedformat : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatFromTokens(pvarin : super::LPVARIANT, pstrformat : super::LPOLESTR, pbtokcur : super::LPBYTE, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR, lcid : super::LCID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatNumber(pvarin : super::LPVARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatPercent(pvarin : super::LPVARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromBool(boolin : super::VARIANT_BOOL, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromCy(cyin : super::CY, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : super::DATE, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromDec(pdecin : *const super::DECIMAL, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromI2(uiin : super::SHORT, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromI4(lin : super::LONG, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromI8(i64in : super::LONG64, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI1FromI4" fn VarI1FromInt(lin : super::LONG, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromR4(fltin : super::FLOAT, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromR8(dblin : super::DOUBLE, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI1(bin : super::BYTE, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI2(uiin : super::USHORT, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI4(ulin : super::ULONG, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI8(i64in : super::ULONG64, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI1FromUI4" fn VarI1FromUint(ulin : super::ULONG, pcout : *mut super::CHAR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromBool(boolin : super::VARIANT_BOOL, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromCy(cyin : super::CY, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromDate(datein : super::DATE, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromDec(pdecin : *const super::DECIMAL, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromI1(cin : super::CHAR, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromI4(lin : super::LONG, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromI8(i64in : super::LONG64, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI2FromI4" fn VarI2FromInt(lin : super::LONG, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromR4(fltin : super::FLOAT, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromR8(dblin : super::DOUBLE, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI1(bin : super::BYTE, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI2(uiin : super::USHORT, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI4(ulin : super::ULONG, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI8(ui64in : super::ULONG64, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI2FromUI4" fn VarI2FromUint(ulin : super::ULONG, psout : *mut super::SHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromBool(boolin : super::VARIANT_BOOL, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromCy(cyin : super::CY, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromDate(datein : super::DATE, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromDec(pdecin : *const super::DECIMAL, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromI1(cin : super::CHAR, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromI2(sin : super::SHORT, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromI8(i64in : super::LONG64, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromR4(fltin : super::FLOAT, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromR8(dblin : super::DOUBLE, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI1(bin : super::BYTE, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI2(uiin : super::USHORT, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI4(ulin : super::ULONG, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI8(ui64in : super::ULONG64, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromUI4" fn VarI4FromUint(ulin : super::ULONG, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromBool(boolin : super::VARIANT_BOOL, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromCy(cyin : super::CY, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromDate(datein : super::DATE, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromDec(pdecin : *const super::DECIMAL, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromI1(cin : super::CHAR, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromI2(sin : super::SHORT, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromR4(fltin : super::FLOAT, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromR8(dblin : super::DOUBLE, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI1(bin : super::BYTE, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI2(uiin : super::USHORT, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI4(ulin : super::ULONG, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI8(ui64in : super::ULONG64, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI8FromUI4" fn VarI8FromUint(ulin : super::ULONG, pi64out : *mut super::LONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarIdiv(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarImp(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarInt(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromBool" fn VarIntFromBool(boolin : super::VARIANT_BOOL, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromCy" fn VarIntFromCy(cyin : super::CY, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromDate" fn VarIntFromDate(datein : super::DATE, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromDec" fn VarIntFromDec(pdecin : *const super::DECIMAL, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromDisp" fn VarIntFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromI1" fn VarIntFromI1(cin : super::CHAR, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromI2" fn VarIntFromI2(sin : super::SHORT, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromI8" fn VarIntFromI8(i64in : super::LONG64, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromR4" fn VarIntFromR4(fltin : super::FLOAT, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromR8" fn VarIntFromR8(dblin : super::DOUBLE, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" "VarI4FromStr" fn VarIntFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromUI1" fn VarIntFromUI1(bin : super::BYTE, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromUI2" fn VarIntFromUI2(uiin : super::USHORT, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromUI4" fn VarIntFromUI4(ulin : super::ULONG, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromUI8" fn VarIntFromUI8(ui64in : super::ULONG64, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarI4FromUI4" fn VarIntFromUint(ulin : super::ULONG, plout : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarMod(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarMonthName(imonth : i32, fabbrev : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarMul(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarNeg(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarNot(pvarin : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarNumFromParseNum(pnumprs : *const NUMPARSE, rgbdig : *const super::BYTE, dwvtbits : super::ULONG, pvar : *mut super::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarOr(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarParseNumFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pnumprs : *mut NUMPARSE, rgbdig : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarPow(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4CmpR8(fltleft : f32, dblright : f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromBool(boolin : super::VARIANT_BOOL, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromCy(cyin : super::CY, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromDate(datein : super::DATE, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromDec(pdecin : *const super::DECIMAL, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI1(cin : super::CHAR, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI2(sin : super::SHORT, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI4(lin : super::LONG, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI8(i64in : super::LONG64, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" "VarR4FromI4" fn VarR4FromInt(lin : super::LONG, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromR8(dblin : super::DOUBLE, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI1(bin : super::BYTE, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI2(uiin : super::USHORT, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI4(ulin : super::ULONG, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI8(ui64in : super::ULONG64, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" "VarR4FromUI4" fn VarR4FromUint(ulin : super::ULONG, pfltout : *mut super::FLOAT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromBool(boolin : super::VARIANT_BOOL, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromCy(cyin : super::CY, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromDate(datein : super::DATE, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromDec(pdecin : *const super::DECIMAL, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI1(cin : super::CHAR, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI2(sin : super::SHORT, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI4(lin : super::LONG, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI8(i64in : super::LONG64, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" "VarR8FromI4" fn VarR8FromInt(lin : super::LONG, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromR4(fltin : super::FLOAT, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI1(bin : super::BYTE, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI2(uiin : super::USHORT, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI4(ulin : super::ULONG, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI8(ui64in : super::ULONG64, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" "VarR8FromUI4" fn VarR8FromUint(ulin : super::ULONG, pdblout : *mut super::DOUBLE) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8Pow(dblleft : f64, dblright : f64, pdblresult : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8Round(dblin : f64, cdecimals : i32, pdblresult : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarRound(pvarin : super::LPVARIANT, cdecimals : i32, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarSub(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarTokenizeFormatString(pstrformat : super::LPOLESTR, rgbtok : super::LPBYTE, cbtok : i32, ifirstday : i32, ifirstweek : i32, lcid : super::LCID, pcbactual : *const i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromBool(boolin : super::VARIANT_BOOL, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromCy(cyin : super::CY, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromDate(datein : super::DATE, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromDec(pdecin : *const super::DECIMAL, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI1(cin : super::CHAR, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI2(sin : super::SHORT, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI4(lin : super::LONG, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI8(i64in : super::LONG64, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI1FromI4" fn VarUI1FromInt(lin : super::LONG, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromR4(fltin : super::FLOAT, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromR8(dblin : super::DOUBLE, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromUI2(uiin : super::USHORT, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromUI4(ulin : super::ULONG, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromUI8(ui64in : super::ULONG64, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI1FromUI4" fn VarUI1FromUint(ulin : super::ULONG, pbout : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromBool(boolin : super::VARIANT_BOOL, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromCy(cyin : super::CY, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromDate(datein : super::DATE, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromDec(pdecin : *const super::DECIMAL, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI1(cin : super::CHAR, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI2(uiin : super::SHORT, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI4(lin : super::LONG, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI8(i64in : super::LONG64, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI2FromI4" fn VarUI2FromInt(lin : super::LONG, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromR4(fltin : super::FLOAT, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromR8(dblin : super::DOUBLE, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromUI1(bin : super::BYTE, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromUI4(ulin : super::ULONG, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromUI8(i64in : super::ULONG64, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI2FromUI4" fn VarUI2FromUint(ulin : super::ULONG, puiout : *mut super::USHORT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromBool(boolin : super::VARIANT_BOOL, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromCy(cyin : super::CY, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromDate(datein : super::DATE, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromDec(pdecin : *const super::DECIMAL, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI1(cin : super::CHAR, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI2(uiin : super::SHORT, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI4(lin : super::LONG, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI8(i64in : super::LONG64, plout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromI4" fn VarUI4FromInt(lin : super::LONG, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromR4(fltin : super::FLOAT, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromR8(dblin : super::DOUBLE, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromUI1(bin : super::BYTE, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromUI2(uiin : super::USHORT, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromUI8(ui64in : super::ULONG64, plout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromBool(boolin : super::VARIANT_BOOL, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromCy(cyin : super::CY, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromDate(datein : super::DATE, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromDec(pdecin : *const super::DECIMAL, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromI1(cin : super::CHAR, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromI2(sin : super::SHORT, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromI8(ui64in : super::LONG64, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromR4(fltin : super::FLOAT, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromR8(dblin : super::DOUBLE, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromUI1(bin : super::BYTE, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromUI2(uiin : super::USHORT, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromUI4(ulin : super::ULONG, pi64out : *mut super::ULONG64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarUdateFromDate(datein : super::DATE, dwflags : super::ULONG, pudateout : *mut UDATE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromBool" fn VarUintFromBool(boolin : super::VARIANT_BOOL, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromCy" fn VarUintFromCy(cyin : super::CY, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromDate" fn VarUintFromDate(datein : super::DATE, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypes"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromDec" fn VarUintFromDec(pdecin : *const super::DECIMAL, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromDisp" fn VarUintFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::LCID, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromI1" fn VarUintFromI1(cin : super::CHAR, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromI2" fn VarUintFromI2(uiin : super::SHORT, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromI4" fn VarUintFromI4(lin : super::LONG, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromI8" fn VarUintFromI8(i64in : super::LONG64, plout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromI4" fn VarUintFromInt(lin : super::LONG, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromR4" fn VarUintFromR4(fltin : super::FLOAT, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromR8" fn VarUintFromR8(dblin : super::DOUBLE, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromStr" fn VarUintFromStr(strin : super::LPCOLESTR, lcid : super::LCID, dwflags : super::ULONG, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromUI1" fn VarUintFromUI1(bin : super::BYTE, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromUI2" fn VarUintFromUI2(uiin : super::USHORT, pulout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" "VarUI4FromUI8" fn VarUintFromUI8(ui64in : super::ULONG64, plout : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("oleaut32.dll" "system" fn VarWeekdayName(iweekday : i32, fabbrev : i32, ifirstday : i32, dwflags : super::ULONG, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarXor(pvarleft : super::LPVARIANT, pvarright : super::LPVARIANT, pvarresult : super::LPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantChangeType(pvargdest : *mut super::VARIANTARG, pvarsrc : *const super::VARIANTARG, wflags : super::USHORT, vt : super::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantChangeTypeEx(pvargdest : *mut super::VARIANTARG, pvarsrc : *const super::VARIANTARG, lcid : super::LCID, wflags : super::USHORT, vt : super::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut super::VARIANTARG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut super::VARIANTARG, pvargsrc : *const super::VARIANTARG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantCopyInd(pvardest : *mut super::VARIANT, pvargsrc : *const super::VARIANTARG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantInit(pvarg : *mut super::VARIANTARG));
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantTimeToDosDateTime(vtime : super::DOUBLE, pwdosdate : *mut super::USHORT, pwdostime : *mut super::USHORT) -> super::INT);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantTimeToSystemTime(vtime : super::DOUBLE, lpsystemtime : super::LPSYSTEMTIME) -> super::INT);
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VectorFromBstr(bstr : windows_sys::core::BSTR, ppsa : *mut *mut super::SAFEARRAY) -> windows_sys::core::HRESULT);
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
