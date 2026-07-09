#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn BstrFromVector(psa : *const super::oaidl::SAFEARRAY, pbstr : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn ClearCustData(pcustdata : *mut super::oaidl::CUSTDATA));
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn CreateDispTypeInfo(pidata : *mut INTERFACEDATA, lcid : super::winnt::LCID, pptinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn CreateErrorInfo(pperrinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn CreateStdDispatch(punkouter : *mut core::ffi::c_void, pvthis : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, ppunkstddisp : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn CreateTypeLib(syskind : super::oaidl::SYSKIND, szfile : windows_sys::core::PCWSTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn CreateTypeLib2(syskind : super::oaidl::SYSKIND, szfile : windows_sys::core::PCWSTR, ppctlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispCallFunc(pvinstance : *const core::ffi::c_void, ovft : usize, cc : super::oaidl::CALLCONV, vtreturn : super::wtypes::VARTYPE, cactuals : u32, prgvt : *const super::wtypes::VARTYPE, prgpvarg : *const *const super::oaidl::VARIANTARG, pvargresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn DispGetIDsOfNames(ptinfo : *mut core::ffi::c_void, rgsznames : *const windows_sys::core::PCWSTR, cnames : u32, rgdispid : *mut super::oaidl::DISPID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispGetParam(pdispparams : *const super::oaidl::DISPPARAMS, position : u32, vttarg : super::wtypes::VARTYPE, pvarresult : *mut super::oaidl::VARIANT, puargerr : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn DispInvoke(_this : *mut core::ffi::c_void, ptinfo : *mut core::ffi::c_void, dispidmember : super::oaidl::DISPID, wflags : u16, pparams : *mut super::oaidl::DISPPARAMS, pvarresult : *mut super::oaidl::VARIANT, pexcepinfo : *mut super::oaidl::EXCEPINFO, puargerr : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn DosDateTimeToVariantTime(wdosdate : u16, wdostime : u16, pvtime : *mut f64) -> i32);
windows_link::link!("oleaut32.dll" "system" fn GetActiveObject(rclsid : *const windows_sys::core::GUID, pvreserved : *mut core::ffi::c_void, ppunk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn GetAltMonthNames(lcid : super::winnt::LCID, prgp : *mut *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn GetRecordInfoFromGuids(rguidtypelib : *const windows_sys::core::GUID, uvermajor : u32, uverminor : u32, lcid : super::winnt::LCID, rguidtypeinfo : *const windows_sys::core::GUID, pprecinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn GetRecordInfoFromTypeInfo(ptypeinfo : *mut core::ffi::c_void, pprecinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn LHashValOfNameSys(syskind : super::oaidl::SYSKIND, lcid : super::winnt::LCID, szname : *const super::wtypesbase::OLECHAR) -> u32);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn LHashValOfNameSysA(syskind : super::oaidl::SYSKIND, lcid : super::winnt::LCID, szname : windows_sys::core::PCSTR) -> u32);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn LoadRegTypeLib(rguid : *const windows_sys::core::GUID, wvermajor : u16, wverminor : u16, lcid : super::winnt::LCID, pptlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn LoadTypeLib(szfile : windows_sys::core::PCWSTR, pptlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn LoadTypeLibEx(szfile : windows_sys::core::PCWSTR, regkind : REGKIND, pptlib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn OaBuildVersion() -> u32);
windows_link::link!("oleaut32.dll" "system" fn OaEnablePerUserTLibRegistration());
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn QueryPathOfRegTypeLib(guid : *const windows_sys::core::GUID, wmaj : u16, wmin : u16, lcid : super::winnt::LCID, lpbstrpathname : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn RegisterActiveObject(punk : *mut core::ffi::c_void, rclsid : *const windows_sys::core::GUID, dwflags : u32, pdwregister : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn RegisterTypeLib(ptlib : *mut core::ffi::c_void, szfullpath : windows_sys::core::PCWSTR, szhelpdir : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn RegisterTypeLibForUser(ptlib : *mut core::ffi::c_void, szfullpath : *const super::wtypesbase::OLECHAR, szhelpdir : *const super::wtypesbase::OLECHAR) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn RevokeActiveObject(dwregister : u32, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAccessData(psa : *const super::oaidl::SAFEARRAY, ppvdata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAddRef(psa : *const super::oaidl::SAFEARRAY, ppdatatorelease : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAllocData(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptor(cdims : u32, ppsaout : *mut *mut super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptorEx(vt : super::wtypes::VARTYPE, cdims : u32, ppsaout : *mut *mut super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCopy(psa : *const super::oaidl::SAFEARRAY, ppsaout : *mut *mut super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCopyData(psasource : *const super::oaidl::SAFEARRAY, psatarget : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreate(vt : super::wtypes::VARTYPE, cdims : u32, rgsabound : *const super::oaidl::SAFEARRAYBOUND) -> *mut super::oaidl::SAFEARRAY);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreateEx(vt : super::wtypes::VARTYPE, cdims : u32, rgsabound : *const super::oaidl::SAFEARRAYBOUND, pvextra : *const core::ffi::c_void) -> *mut super::oaidl::SAFEARRAY);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreateVector(vt : super::wtypes::VARTYPE, llbound : i32, celements : u32) -> *mut super::oaidl::SAFEARRAY);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayCreateVectorEx(vt : super::wtypes::VARTYPE, llbound : i32, celements : u32, pvextra : *const core::ffi::c_void) -> *mut super::oaidl::SAFEARRAY);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayDestroy(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayDestroyData(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayDestroyDescriptor(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetDim(psa : *const super::oaidl::SAFEARRAY) -> u32);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetElement(psa : *const super::oaidl::SAFEARRAY, rgindices : *const i32, pv : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetElemsize(psa : *const super::oaidl::SAFEARRAY) -> u32);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetIID(psa : *const super::oaidl::SAFEARRAY, pguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetLBound(psa : *const super::oaidl::SAFEARRAY, ndim : u32, pllbound : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetRecordInfo(psa : *const super::oaidl::SAFEARRAY, prinfo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetUBound(psa : *const super::oaidl::SAFEARRAY, ndim : u32, plubound : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayGetVartype(psa : *const super::oaidl::SAFEARRAY, pvt : *mut super::wtypes::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayLock(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayPtrOfIndex(psa : *const super::oaidl::SAFEARRAY, rgindices : *const i32, ppvdata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayPutElement(psa : *const super::oaidl::SAFEARRAY, rgindices : *const i32, pv : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayRedim(psa : *mut super::oaidl::SAFEARRAY, psaboundnew : *const super::oaidl::SAFEARRAYBOUND) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn SafeArrayReleaseData(pdata : *const core::ffi::c_void));
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayReleaseDescriptor(psa : *const super::oaidl::SAFEARRAY));
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArraySetIID(psa : *const super::oaidl::SAFEARRAY, guid : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArraySetRecordInfo(psa : *const super::oaidl::SAFEARRAY, prinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayUnaccessData(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SafeArrayUnlock(psa : *const super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn SysAddRefString(bstrstring : windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypesbase")]
windows_link::link!("oleaut32.dll" "system" fn SysAllocString(psz : *const super::wtypesbase::OLECHAR) -> windows_sys::core::BSTR);
windows_link::link!("oleaut32.dll" "system" fn SysAllocStringByteLen(psz : windows_sys::core::PCSTR, len : u32) -> windows_sys::core::BSTR);
#[cfg(feature = "Win32_wtypesbase")]
windows_link::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : *const super::wtypesbase::OLECHAR, ui : u32) -> windows_sys::core::BSTR);
windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : windows_sys::core::BSTR));
#[cfg(feature = "Win32_wtypesbase")]
windows_link::link!("oleaut32.dll" "system" fn SysReAllocString(pbstr : *mut windows_sys::core::BSTR, psz : *const super::wtypesbase::OLECHAR) -> i32);
#[cfg(feature = "Win32_wtypesbase")]
windows_link::link!("oleaut32.dll" "system" fn SysReAllocStringLen(pbstr : *mut windows_sys::core::BSTR, psz : *const super::wtypesbase::OLECHAR, len : u32) -> i32);
windows_link::link!("oleaut32.dll" "system" fn SysReleaseString(bstrstring : windows_sys::core::BSTR));
windows_link::link!("oleaut32.dll" "system" fn SysStringByteLen(bstr : windows_sys::core::BSTR) -> u32);
windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : windows_sys::core::BSTR) -> u32);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("oleaut32.dll" "system" fn SystemTimeToVariantTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME, pvtime : *mut f64) -> i32);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn UnRegisterTypeLib(libid : *const windows_sys::core::GUID, wvermajor : u16, wverminor : u16, lcid : super::winnt::LCID, syskind : super::oaidl::SYSKIND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn UnRegisterTypeLibForUser(libid : *const windows_sys::core::GUID, wmajorvernum : u16, wminorvernum : u16, lcid : super::winnt::LCID, syskind : super::oaidl::SYSKIND) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarAbs(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarAdd(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarAnd(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromCy(cyin : super::wtypes::CY, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromDate(datein : f64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromDec(pdecin : *const super::wtypes::DECIMAL, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI1(cin : i8, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI2(sin : i16, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI4(lin : i32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromI8(i64in : i64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromR4(fltin : f32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromR8(dblin : f64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI1(bin : u8, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI2(uiin : u16, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI4(ulin : u32, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarBoolFromUI8(i64in : u64, pboolout : *mut super::wtypes::VARIANT_BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarBstrCat(bstrleft : windows_sys::core::BSTR, bstrright : windows_sys::core::BSTR, pbstrresult : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrCmp(bstrleft : windows_sys::core::BSTR, bstrright : windows_sys::core::BSTR, lcid : super::winnt::LCID, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromBool(boolin : super::wtypes::VARIANT_BOOL, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromCy(cyin : super::wtypes::CY, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromDate(datein : f64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromDec(pdecin : *const super::wtypes::DECIMAL, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI1(cin : i8, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI2(ival : i16, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI4(lin : i32, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromI8(i64in : i64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromR4(fltin : f32, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromR8(dblin : f64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI1(bval : u8, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI2(uiin : u16, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI4(ulin : u32, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarBstrFromUI8(ui64in : u64, lcid : super::winnt::LCID, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarCat(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarCmp(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, lcid : super::winnt::LCID, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyAbs(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyAdd(cyleft : super::wtypes::CY, cyright : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyCmp(cyleft : super::wtypes::CY, cyright : super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyCmpR8(cyleft : super::wtypes::CY, dblright : f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFix(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromBool(boolin : super::wtypes::VARIANT_BOOL, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromDate(datein : f64, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromDec(pdecin : *const super::wtypes::DECIMAL, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI1(cin : i8, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI2(sin : i16, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI4(lin : i32, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromI8(i64in : i64, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromR4(fltin : f32, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromR8(dblin : f64, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI1(bin : u8, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI2(uiin : u16, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI4(ulin : u32, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyFromUI8(ui64in : u64, pcyout : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyInt(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyMul(cyleft : super::wtypes::CY, cyright : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyMulI4(cyleft : super::wtypes::CY, lright : i32, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyMulI8(cyleft : super::wtypes::CY, lright : i64, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyNeg(cyin : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCyRound(cyin : super::wtypes::CY, cdecimals : i32, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarCySub(cyleft : super::wtypes::CY, cyright : super::wtypes::CY, pcyresult : *mut super::wtypes::CY) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromBool(boolin : super::wtypes::VARIANT_BOOL, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromCy(cyin : super::wtypes::CY, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromDec(pdecin : *const super::wtypes::DECIMAL, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI1(cin : i8, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI2(sin : i16, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI4(lin : i32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromI8(i64in : i64, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromR4(fltin : f32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromR8(dblin : f64, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI1(bin : u8, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI2(uiin : u16, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI4(ulin : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUI8(ui64in : u64, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUdate(pudatein : *const UDATE, dwflags : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarDateFromUdateEx(pudatein : *const UDATE, lcid : super::winnt::LCID, dwflags : u32, pdateout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecAbs(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecAdd(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecCmp(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecCmpR8(pdecleft : *const super::wtypes::DECIMAL, dblright : f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecDiv(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFix(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromBool(boolin : super::wtypes::VARIANT_BOOL, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromCy(cyin : super::wtypes::CY, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromDate(datein : f64, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI1(cin : i8, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI2(uiin : i16, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI4(lin : i32, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromI8(i64in : i64, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromR4(fltin : f32, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromR8(dblin : f64, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wtypes"))]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI1(bin : u8, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI2(uiin : u16, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI4(ulin : u32, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecFromUI8(ui64in : u64, pdecout : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecInt(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecMul(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecNeg(pdecin : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecRound(pdecin : *const super::wtypes::DECIMAL, cdecimals : i32, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarDecSub(pdecleft : *const super::wtypes::DECIMAL, pdecright : *const super::wtypes::DECIMAL, pdecresult : *mut super::wtypes::DECIMAL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarDiv(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarEqv(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFix(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormat(pvarin : *const super::oaidl::VARIANT, pstrformat : windows_sys::core::PCWSTR, ifirstday : i32, ifirstweek : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatCurrency(pvarin : *const super::oaidl::VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatDateTime(pvarin : *const super::oaidl::VARIANT, inamedformat : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatFromTokens(pvarin : *const super::oaidl::VARIANT, pstrformat : windows_sys::core::PCWSTR, pbtokcur : *const u8, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR, lcid : super::winnt::LCID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatNumber(pvarin : *const super::oaidl::VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarFormatPercent(pvarin : *const super::oaidl::VARIANT, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromBool(boolin : super::wtypes::VARIANT_BOOL, pcout : *mut i8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromCy(cyin : super::wtypes::CY, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : f64, pcout : *mut i8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromDec(pdecin : *const super::wtypes::DECIMAL, pcout : *mut i8) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromI2(uiin : i16, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromI4(lin : i32, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromI8(i64in : i64, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromR4(fltin : f32, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromR8(dblin : f64, pcout : *mut i8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarI1FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI1(bin : u8, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI2(uiin : u16, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI4(ulin : u32, pcout : *mut i8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI1FromUI8(i64in : u64, pcout : *mut i8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromBool(boolin : super::wtypes::VARIANT_BOOL, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromCy(cyin : super::wtypes::CY, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromDate(datein : f64, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromDec(pdecin : *const super::wtypes::DECIMAL, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromI1(cin : i8, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromI4(lin : i32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromI8(i64in : i64, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromR4(fltin : f32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromR8(dblin : f64, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarI2FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI1(bin : u8, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI2(uiin : u16, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI4(ulin : u32, psout : *mut i16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI2FromUI8(ui64in : u64, psout : *mut i16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromBool(boolin : super::wtypes::VARIANT_BOOL, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromCy(cyin : super::wtypes::CY, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromDate(datein : f64, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromDec(pdecin : *const super::wtypes::DECIMAL, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromI1(cin : i8, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromI2(sin : i16, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromI8(i64in : i64, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromR4(fltin : f32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromR8(dblin : f64, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarI4FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI1(bin : u8, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI2(uiin : u16, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI4(ulin : u32, plout : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI4FromUI8(ui64in : u64, plout : *mut i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromBool(boolin : super::wtypes::VARIANT_BOOL, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromCy(cyin : super::wtypes::CY, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromDate(datein : f64, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromDec(pdecin : *const super::wtypes::DECIMAL, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromI1(cin : i8, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromI2(sin : i16, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromR4(fltin : f32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromR8(dblin : f64, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarI8FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI1(bin : u8, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI2(uiin : u16, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI4(ulin : u32, pi64out : *mut i64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarI8FromUI8(ui64in : u64, pi64out : *mut i64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarIdiv(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarImp(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarInt(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarMod(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarMonthName(imonth : i32, fabbrev : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarMul(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarNeg(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarNot(pvarin : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarNumFromParseNum(pnumprs : *const NUMPARSE, rgbdig : *const u8, dwvtbits : u32, pvar : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarOr(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarParseNumFromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pnumprs : *mut NUMPARSE, rgbdig : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarPow(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4CmpR8(fltleft : f32, dblright : f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromBool(boolin : super::wtypes::VARIANT_BOOL, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromCy(cyin : super::wtypes::CY, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromDate(datein : f64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromDec(pdecin : *const super::wtypes::DECIMAL, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI1(cin : i8, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI2(sin : i16, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI4(lin : i32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromI8(i64in : i64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromR8(dblin : f64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarR4FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI1(bin : u8, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI2(uiin : u16, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI4(ulin : u32, pfltout : *mut f32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR4FromUI8(ui64in : u64, pfltout : *mut f32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromBool(boolin : super::wtypes::VARIANT_BOOL, pdblout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromCy(cyin : super::wtypes::CY, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromDate(datein : f64, pdblout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromDec(pdecin : *const super::wtypes::DECIMAL, pdblout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI1(cin : i8, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI2(sin : i16, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI4(lin : i32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromI8(i64in : i64, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromR4(fltin : f32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarR8FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI1(bin : u8, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI2(uiin : u16, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI4(ulin : u32, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8FromUI8(ui64in : u64, pdblout : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8Pow(dblleft : f64, dblright : f64, pdblresult : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarR8Round(dblin : f64, cdecimals : i32, pdblresult : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarRound(pvarin : *const super::oaidl::VARIANT, cdecimals : i32, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarSub(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarTokenizeFormatString(pstrformat : windows_sys::core::PCWSTR, rgbtok : *mut u8, cbtok : i32, ifirstday : i32, ifirstweek : i32, lcid : super::winnt::LCID, pcbactual : *const i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromBool(boolin : super::wtypes::VARIANT_BOOL, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromCy(cyin : super::wtypes::CY, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromDate(datein : f64, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromDec(pdecin : *const super::wtypes::DECIMAL, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI1(cin : i8, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI2(sin : i16, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI4(lin : i32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromI8(i64in : i64, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromR4(fltin : f32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromR8(dblin : f64, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromUI2(uiin : u16, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromUI4(ulin : u32, pbout : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI1FromUI8(ui64in : u64, pbout : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromBool(boolin : super::wtypes::VARIANT_BOOL, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromCy(cyin : super::wtypes::CY, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromDate(datein : f64, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromDec(pdecin : *const super::wtypes::DECIMAL, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI1(cin : i8, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI2(uiin : i16, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI4(lin : i32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromI8(i64in : i64, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromR4(fltin : f32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromR8(dblin : f64, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromUI1(bin : u8, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromUI4(ulin : u32, puiout : *mut u16) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI2FromUI8(i64in : u64, puiout : *mut u16) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromBool(boolin : super::wtypes::VARIANT_BOOL, pulout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromCy(cyin : super::wtypes::CY, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromDate(datein : f64, pulout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromDec(pdecin : *const super::wtypes::DECIMAL, pulout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI1(cin : i8, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI2(uiin : i16, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI4(lin : i32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromI8(i64in : i64, plout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromR4(fltin : f32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromR8(dblin : f64, pulout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromUI1(bin : u8, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromUI2(uiin : u16, pulout : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI4FromUI8(ui64in : u64, plout : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromBool(boolin : super::wtypes::VARIANT_BOOL, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromCy(cyin : super::wtypes::CY, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromDate(datein : f64, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_wtypes")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromDec(pdecin : *const super::wtypes::DECIMAL, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt"))]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromDisp(pdispin : *mut core::ffi::c_void, lcid : super::winnt::LCID, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromI1(cin : i8, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromI2(sin : i16, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromI8(ui64in : i64, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromR4(fltin : f32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromR8(dblin : f64, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromStr(strin : windows_sys::core::PCWSTR, lcid : super::winnt::LCID, dwflags : u32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromUI1(bin : u8, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromUI2(uiin : u16, pi64out : *mut u64) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarUI8FromUI4(ulin : u32, pi64out : *mut u64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("oleaut32.dll" "system" fn VarUdateFromDate(datein : f64, dwflags : u32, pudateout : *mut UDATE) -> windows_sys::core::HRESULT);
windows_link::link!("oleaut32.dll" "system" fn VarWeekdayName(iweekday : i32, fabbrev : i32, ifirstday : i32, dwflags : u32, pbstrout : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VarXor(pvarleft : *const super::oaidl::VARIANT, pvarright : *const super::oaidl::VARIANT, pvarresult : *mut super::oaidl::VARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantChangeType(pvargdest : *mut super::oaidl::VARIANTARG, pvarsrc : *const super::oaidl::VARIANTARG, wflags : u16, vt : super::wtypes::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantChangeTypeEx(pvargdest : *mut super::oaidl::VARIANTARG, pvarsrc : *const super::oaidl::VARIANTARG, lcid : super::winnt::LCID, wflags : u16, vt : super::wtypes::VARTYPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut super::oaidl::VARIANTARG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut super::oaidl::VARIANTARG, pvargsrc : *const super::oaidl::VARIANTARG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantCopyInd(pvardest : *mut super::oaidl::VARIANT, pvargsrc : *const super::oaidl::VARIANTARG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn VariantInit(pvarg : *mut super::oaidl::VARIANTARG));
windows_link::link!("oleaut32.dll" "system" fn VariantTimeToDosDateTime(vtime : f64, pwdosdate : *mut u16, pwdostime : *mut u16) -> i32);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("oleaut32.dll" "system" fn VariantTimeToSystemTime(vtime : f64, lpsystemtime : *mut super::minwinbase::SYSTEMTIME) -> i32);
#[cfg(feature = "Win32_oaidl")]
windows_link::link!("oleaut32.dll" "system" fn VectorFromBstr(bstr : windows_sys::core::BSTR, ppsa : *mut *mut super::oaidl::SAFEARRAY) -> windows_sys::core::HRESULT);
pub const ACTIVEOBJECT_STRONG: u32 = 0;
pub const ACTIVEOBJECT_WEAK: u32 = 1;
pub const DISPATCH_METHOD: u32 = 1;
pub const DISPATCH_PROPERTYGET: u32 = 2;
pub const DISPATCH_PROPERTYPUT: u32 = 4;
pub const DISPATCH_PROPERTYPUTREF: u32 = 8;
pub const ID_DEFAULTINST: i32 = -2;
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOAD_TLB_AS_32BIT: u32 = 32;
pub const LOAD_TLB_AS_64BIT: u32 = 64;
pub const LOCALE_USE_NLS: u32 = 268435456;
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPINTERFACEDATA = *mut INTERFACEDATA;
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPMETHODDATA = *mut METHODDATA;
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub type LPPARAMDATA = *mut PARAMDATA;
pub const MASK_TO_RESET_TLB_BITS: i32 = -97;
pub const MEMBERID_NIL: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
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
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl Default for METHODDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
#[derive(Clone, Copy)]
pub struct PARAMDATA {
    pub szName: *mut super::wtypesbase::OLECHAR,
    pub vt: super::wtypes::VARTYPE,
}
#[cfg(all(feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
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
#[cfg(feature = "Win32_minwinbase")]
#[derive(Clone, Copy, Default)]
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
