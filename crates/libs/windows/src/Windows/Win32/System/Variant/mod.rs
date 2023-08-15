#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ClearVariantArray(pvars: &mut [VARIANT]) {
    ::windows_targets::link!("propsys.dll" "system" fn ClearVariantArray(pvars : *mut VARIANT, cvars : u32) -> ());
    ClearVariantArray(::core::mem::transmute(pvars.as_ptr()), pvars.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
#[inline]
pub unsafe fn DosDateTimeToVariantTime(wdosdate: u16, wdostime: u16, pvtime: *mut f64) -> i32 {
    ::windows_targets::link!("oleaut32.dll" "system" fn DosDateTimeToVariantTime(wdosdate : u16, wdostime : u16, pvtime : *mut f64) -> i32);
    DosDateTimeToVariantTime(wdosdate, wdostime, pvtime)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBooleanArray(prgf: &[super::super::Foundation::BOOL]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromBooleanArray(prgf : *const super::super::Foundation:: BOOL, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromBooleanArray(::core::mem::transmute(prgf.as_ptr()), prgf.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromBuffer(pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromBuffer(pv : *const ::core::ffi::c_void, cb : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromBuffer(pv, cb, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromDoubleArray(prgn: &[f64]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromDoubleArray(prgn : *const f64, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromDoubleArray(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTime(pft: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromFileTime(pft : *const super::super::Foundation:: FILETIME, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromFileTime(pft, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromFileTimeArray(prgft: ::core::option::Option<&[super::super::Foundation::FILETIME]>) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromFileTimeArray(prgft : *const super::super::Foundation:: FILETIME, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromFileTimeArray(::core::mem::transmute(prgft.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prgft.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromGUIDAsString(guid: *const ::windows_core::GUID) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromGUIDAsString(guid : *const ::windows_core::GUID, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromGUIDAsString(guid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt16Array(prgn: &[i16]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromInt16Array(prgn : *const i16, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromInt16Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt32Array(prgn: &[i32]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromInt32Array(prgn : *const i32, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromInt32Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromInt64Array(prgn: &[i64]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromInt64Array(prgn : *const i64, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromInt64Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromResource<P0>(hinst: P0, id: u32) -> ::windows_core::Result<VARIANT>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromResource(hinst : super::super::Foundation:: HINSTANCE, id : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromResource(hinst.into_param().abi(), id, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromStringArray(prgsz: &[::windows_core::PCWSTR]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromStringArray(prgsz : *const ::windows_core::PCWSTR, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromStringArray(::core::mem::transmute(prgsz.as_ptr()), prgsz.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt16Array(prgn: &[u16]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromUInt16Array(prgn : *const u16, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromUInt16Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt32Array(prgn: &[u32]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromUInt32Array(prgn : *const u32, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromUInt32Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromUInt64Array(prgn: &[u64]) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromUInt64Array(prgn : *const u64, celems : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromUInt64Array(::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn InitVariantFromVariantArrayElem(varin: *const VARIANT, ielem: u32) -> ::windows_core::Result<VARIANT> {
    ::windows_targets::link!("propsys.dll" "system" fn InitVariantFromVariantArrayElem(varin : *const VARIANT, ielem : u32, pvar : *mut VARIANT) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    InitVariantFromVariantArrayElem(varin, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemTimeToVariantTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME, pvtime: *mut f64) -> i32 {
    ::windows_targets::link!("oleaut32.dll" "system" fn SystemTimeToVariantTime(lpsystemtime : *const super::super::Foundation:: SYSTEMTIME, pvtime : *mut f64) -> i32);
    SystemTimeToVariantTime(lpsystemtime, pvtime)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const VARIANT) {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserFree(param0 : *const u32, param1 : *const VARIANT) -> ());
    VARIANT_UserFree(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const VARIANT) {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserFree64(param0 : *const u32, param1 : *const VARIANT) -> ());
    VARIANT_UserFree64(param0, param1)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const VARIANT) -> *mut u8 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const VARIANT) -> *mut u8);
    VARIANT_UserMarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const VARIANT) -> *mut u8 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const VARIANT) -> *mut u8);
    VARIANT_UserMarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const VARIANT) -> u32 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserSize(param0 : *const u32, param1 : u32, param2 : *const VARIANT) -> u32);
    VARIANT_UserSize(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const VARIANT) -> u32 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserSize64(param0 : *const u32, param1 : u32, param2 : *const VARIANT) -> u32);
    VARIANT_UserSize64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut VARIANT) -> *mut u8 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut VARIANT) -> *mut u8);
    VARIANT_UserUnmarshal(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut VARIANT) -> *mut u8 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut VARIANT) -> *mut u8);
    VARIANT_UserUnmarshal64(param0, param1, param2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantChangeType(pvargdest: *mut VARIANT, pvarsrc: *const VARIANT, wflags: VAR_CHANGE_FLAGS, vt: VARENUM) -> ::windows_core::Result<()> {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantChangeType(pvargdest : *mut VARIANT, pvarsrc : *const VARIANT, wflags : VAR_CHANGE_FLAGS, vt : VARENUM) -> ::windows_core::HRESULT);
    VariantChangeType(pvargdest, pvarsrc, wflags, vt).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantChangeTypeEx(pvargdest: *mut VARIANT, pvarsrc: *const VARIANT, lcid: u32, wflags: VAR_CHANGE_FLAGS, vt: VARENUM) -> ::windows_core::Result<()> {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantChangeTypeEx(pvargdest : *mut VARIANT, pvarsrc : *const VARIANT, lcid : u32, wflags : VAR_CHANGE_FLAGS, vt : VARENUM) -> ::windows_core::HRESULT);
    VariantChangeTypeEx(pvargdest, pvarsrc, lcid, wflags, vt).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantClear(pvarg: *mut VARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantClear(pvarg : *mut VARIANT) -> ::windows_core::HRESULT);
    VariantClear(pvarg).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCompare(var1: *const VARIANT, var2: *const VARIANT) -> i32 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantCompare(var1 : *const VARIANT, var2 : *const VARIANT) -> i32);
    VariantCompare(var1, var2)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCopy(pvargdest: *mut VARIANT, pvargsrc: *const VARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantCopy(pvargdest : *mut VARIANT, pvargsrc : *const VARIANT) -> ::windows_core::HRESULT);
    VariantCopy(pvargdest, pvargsrc).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantCopyInd(pvardest: *mut VARIANT, pvargsrc: *const VARIANT) -> ::windows_core::Result<()> {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantCopyInd(pvardest : *mut VARIANT, pvargsrc : *const VARIANT) -> ::windows_core::HRESULT);
    VariantCopyInd(pvardest, pvargsrc).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetBooleanElem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetBooleanElem(var : *const VARIANT, ielem : u32, pfval : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetBooleanElem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetDoubleElem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<f64> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetDoubleElem(var : *const VARIANT, ielem : u32, pnval : *mut f64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetDoubleElem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetElementCount(varin: *const VARIANT) -> u32 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetElementCount(varin : *const VARIANT) -> u32);
    VariantGetElementCount(varin)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt16Elem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<i16> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetInt16Elem(var : *const VARIANT, ielem : u32, pnval : *mut i16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetInt16Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt32Elem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<i32> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetInt32Elem(var : *const VARIANT, ielem : u32, pnval : *mut i32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetInt32Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetInt64Elem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<i64> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetInt64Elem(var : *const VARIANT, ielem : u32, pnval : *mut i64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetInt64Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetStringElem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetStringElem(var : *const VARIANT, ielem : u32, ppszval : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetStringElem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt16Elem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<u16> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetUInt16Elem(var : *const VARIANT, ielem : u32, pnval : *mut u16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetUInt16Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt32Elem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<u32> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetUInt32Elem(var : *const VARIANT, ielem : u32, pnval : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetUInt32Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantGetUInt64Elem(var: *const VARIANT, ielem: u32) -> ::windows_core::Result<u64> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantGetUInt64Elem(var : *const VARIANT, ielem : u32, pnval : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantGetUInt64Elem(var, ielem, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantInit() -> VARIANT {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantInit(pvarg : *mut VARIANT) -> ());
    let mut result__ = ::std::mem::zeroed();
    VariantInit(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
#[inline]
pub unsafe fn VariantTimeToDosDateTime(vtime: f64, pwdosdate: *mut u16, pwdostime: *mut u16) -> i32 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantTimeToDosDateTime(vtime : f64, pwdosdate : *mut u16, pwdostime : *mut u16) -> i32);
    VariantTimeToDosDateTime(vtime, pwdosdate, pwdostime)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VariantTimeToSystemTime(vtime: f64, lpsystemtime: *mut super::super::Foundation::SYSTEMTIME) -> i32 {
    ::windows_targets::link!("oleaut32.dll" "system" fn VariantTimeToSystemTime(vtime : f64, lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME) -> i32);
    VariantTimeToSystemTime(vtime, lpsystemtime)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBoolean(varin: *const VARIANT) -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToBoolean(varin : *const VARIANT, pfret : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToBoolean(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArray(var: *const VARIANT, prgf: &mut [super::super::Foundation::BOOL], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToBooleanArray(var : *const VARIANT, prgf : *mut super::super::Foundation:: BOOL, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToBooleanArray(var, ::core::mem::transmute(prgf.as_ptr()), prgf.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanArrayAlloc(var: *const VARIANT, pprgf: *mut *mut super::super::Foundation::BOOL, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToBooleanArrayAlloc(var : *const VARIANT, pprgf : *mut *mut super::super::Foundation:: BOOL, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToBooleanArrayAlloc(var, pprgf, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBooleanWithDefault<P0>(varin: *const VARIANT, fdefault: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("propsys.dll" "system" fn VariantToBooleanWithDefault(varin : *const VARIANT, fdefault : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    VariantToBooleanWithDefault(varin, fdefault.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToBuffer(varin: *const VARIANT, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToBuffer(varin : *const VARIANT, pv : *mut ::core::ffi::c_void, cb : u32) -> ::windows_core::HRESULT);
    VariantToBuffer(varin, pv, cb).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDosDateTime(varin: *const VARIANT, pwdate: *mut u16, pwtime: *mut u16) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToDosDateTime(varin : *const VARIANT, pwdate : *mut u16, pwtime : *mut u16) -> ::windows_core::HRESULT);
    VariantToDosDateTime(varin, pwdate, pwtime).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDouble(varin: *const VARIANT) -> ::windows_core::Result<f64> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToDouble(varin : *const VARIANT, pdblret : *mut f64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToDouble(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArray(var: *const VARIANT, prgn: &mut [f64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToDoubleArray(var : *const VARIANT, prgn : *mut f64, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToDoubleArray(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleArrayAlloc(var: *const VARIANT, pprgn: *mut *mut f64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToDoubleArrayAlloc(var : *const VARIANT, pprgn : *mut *mut f64, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToDoubleArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToDoubleWithDefault(varin: *const VARIANT, dbldefault: f64) -> f64 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToDoubleWithDefault(varin : *const VARIANT, dbldefault : f64) -> f64);
    VariantToDoubleWithDefault(varin, dbldefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToFileTime(varin: *const VARIANT, stfout: PSTIME_FLAGS) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToFileTime(varin : *const VARIANT, stfout : PSTIME_FLAGS, pftout : *mut super::super::Foundation:: FILETIME) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToFileTime(varin, stfout, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToGUID(varin: *const VARIANT) -> ::windows_core::Result<::windows_core::GUID> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToGUID(varin : *const VARIANT, pguid : *mut ::windows_core::GUID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToGUID(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16(varin: *const VARIANT) -> ::windows_core::Result<i16> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt16(varin : *const VARIANT, piret : *mut i16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToInt16(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16Array(var: *const VARIANT, prgn: &mut [i16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt16Array(var : *const VARIANT, prgn : *mut i16, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToInt16Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut i16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt16ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut i16, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToInt16ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt16WithDefault(varin: *const VARIANT, idefault: i16) -> i16 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt16WithDefault(varin : *const VARIANT, idefault : i16) -> i16);
    VariantToInt16WithDefault(varin, idefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32(varin: *const VARIANT) -> ::windows_core::Result<i32> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt32(varin : *const VARIANT, plret : *mut i32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToInt32(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32Array(var: *const VARIANT, prgn: &mut [i32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt32Array(var : *const VARIANT, prgn : *mut i32, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToInt32Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut i32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt32ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut i32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToInt32ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt32WithDefault(varin: *const VARIANT, ldefault: i32) -> i32 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt32WithDefault(varin : *const VARIANT, ldefault : i32) -> i32);
    VariantToInt32WithDefault(varin, ldefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64(varin: *const VARIANT) -> ::windows_core::Result<i64> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt64(varin : *const VARIANT, pllret : *mut i64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToInt64(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64Array(var: *const VARIANT, prgn: &mut [i64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt64Array(var : *const VARIANT, prgn : *mut i64, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToInt64Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut i64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt64ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut i64, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToInt64ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToInt64WithDefault(varin: *const VARIANT, lldefault: i64) -> i64 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToInt64WithDefault(varin : *const VARIANT, lldefault : i64) -> i64);
    VariantToInt64WithDefault(varin, lldefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToString(varin: *const VARIANT, pszbuf: &mut [u16]) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToString(varin : *const VARIANT, pszbuf : ::windows_core::PWSTR, cchbuf : u32) -> ::windows_core::HRESULT);
    VariantToString(varin, ::core::mem::transmute(pszbuf.as_ptr()), pszbuf.len() as _).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringAlloc(varin: *const VARIANT) -> ::windows_core::Result<::windows_core::PWSTR> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToStringAlloc(varin : *const VARIANT, ppszbuf : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToStringAlloc(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArray(var: *const VARIANT, prgsz: &mut [::windows_core::PWSTR], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToStringArray(var : *const VARIANT, prgsz : *mut ::windows_core::PWSTR, crgsz : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToStringArray(var, ::core::mem::transmute(prgsz.as_ptr()), prgsz.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringArrayAlloc(var: *const VARIANT, pprgsz: *mut *mut ::windows_core::PWSTR, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToStringArrayAlloc(var : *const VARIANT, pprgsz : *mut *mut ::windows_core::PWSTR, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToStringArrayAlloc(var, pprgsz, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToStringWithDefault<P0>(varin: *const VARIANT, pszdefault: P0) -> ::windows_core::PCWSTR
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("propsys.dll" "system" fn VariantToStringWithDefault(varin : *const VARIANT, pszdefault : ::windows_core::PCWSTR) -> ::windows_core::PCWSTR);
    VariantToStringWithDefault(varin, pszdefault.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16(varin: *const VARIANT) -> ::windows_core::Result<u16> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt16(varin : *const VARIANT, puiret : *mut u16) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToUInt16(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16Array(var: *const VARIANT, prgn: &mut [u16], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt16Array(var : *const VARIANT, prgn : *mut u16, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToUInt16Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut u16, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt16ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut u16, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToUInt16ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt16WithDefault(varin: *const VARIANT, uidefault: u16) -> u16 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt16WithDefault(varin : *const VARIANT, uidefault : u16) -> u16);
    VariantToUInt16WithDefault(varin, uidefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32(varin: *const VARIANT) -> ::windows_core::Result<u32> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt32(varin : *const VARIANT, pulret : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToUInt32(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32Array(var: *const VARIANT, prgn: &mut [u32], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt32Array(var : *const VARIANT, prgn : *mut u32, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToUInt32Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut u32, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt32ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToUInt32ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt32WithDefault(varin: *const VARIANT, uldefault: u32) -> u32 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt32WithDefault(varin : *const VARIANT, uldefault : u32) -> u32);
    VariantToUInt32WithDefault(varin, uldefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64(varin: *const VARIANT) -> ::windows_core::Result<u64> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt64(varin : *const VARIANT, pullret : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    VariantToUInt64(varin, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64Array(var: *const VARIANT, prgn: &mut [u64], pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt64Array(var : *const VARIANT, prgn : *mut u64, crgn : u32, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToUInt64Array(var, ::core::mem::transmute(prgn.as_ptr()), prgn.len() as _, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64ArrayAlloc(var: *const VARIANT, pprgn: *mut *mut u64, pcelem: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt64ArrayAlloc(var : *const VARIANT, pprgn : *mut *mut u64, pcelem : *mut u32) -> ::windows_core::HRESULT);
    VariantToUInt64ArrayAlloc(var, pprgn, pcelem).ok()
}
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VariantToUInt64WithDefault(varin: *const VARIANT, ulldefault: u64) -> u64 {
    ::windows_targets::link!("propsys.dll" "system" fn VariantToUInt64WithDefault(varin : *const VARIANT, ulldefault : u64) -> u64);
    VariantToUInt64WithDefault(varin, ulldefault)
}
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const DPF_ERROR: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const DPF_MARQUEE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const DPF_MARQUEE_COMPLETE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const DPF_NONE: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const DPF_STOPPED: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const DPF_WARNING: DRAWPROGRESSFLAGS = DRAWPROGRESSFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const PSTF_LOCAL: PSTIME_FLAGS = PSTIME_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const PSTF_UTC: PSTIME_FLAGS = PSTIME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_ALPHABOOL: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(2u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_CALENDAR_GREGORIAN: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(64u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_CALENDAR_HIJRI: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(8u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_CALENDAR_THAI: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(32u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_LOCALBOOL: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(16u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_NOUSEROVERRIDE: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(4u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_NOVALUEPROP: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(1u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VARIANT_USE_NLS: VAR_CHANGE_FLAGS = VAR_CHANGE_FLAGS(128u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_ARRAY: VARENUM = VARENUM(8192u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_BLOB: VARENUM = VARENUM(65u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_BLOB_OBJECT: VARENUM = VARENUM(70u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_BOOL: VARENUM = VARENUM(11u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_BSTR: VARENUM = VARENUM(8u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_BSTR_BLOB: VARENUM = VARENUM(4095u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_BYREF: VARENUM = VARENUM(16384u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_CARRAY: VARENUM = VARENUM(28u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_CF: VARENUM = VARENUM(71u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_CLSID: VARENUM = VARENUM(72u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_CY: VARENUM = VARENUM(6u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_DATE: VARENUM = VARENUM(7u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_DECIMAL: VARENUM = VARENUM(14u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_DISPATCH: VARENUM = VARENUM(9u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_EMPTY: VARENUM = VARENUM(0u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_ERROR: VARENUM = VARENUM(10u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_FILETIME: VARENUM = VARENUM(64u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_HRESULT: VARENUM = VARENUM(25u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_I1: VARENUM = VARENUM(16u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_I2: VARENUM = VARENUM(2u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_I4: VARENUM = VARENUM(3u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_I8: VARENUM = VARENUM(20u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_ILLEGAL: VARENUM = VARENUM(65535u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_ILLEGALMASKED: VARENUM = VARENUM(4095u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_INT: VARENUM = VARENUM(22u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_INT_PTR: VARENUM = VARENUM(37u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_LPSTR: VARENUM = VARENUM(30u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_LPWSTR: VARENUM = VARENUM(31u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_NULL: VARENUM = VARENUM(1u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_PTR: VARENUM = VARENUM(26u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_R4: VARENUM = VARENUM(4u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_R8: VARENUM = VARENUM(5u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_RECORD: VARENUM = VARENUM(36u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_RESERVED: VARENUM = VARENUM(32768u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_SAFEARRAY: VARENUM = VARENUM(27u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_STORAGE: VARENUM = VARENUM(67u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_STORED_OBJECT: VARENUM = VARENUM(69u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_STREAM: VARENUM = VARENUM(66u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_STREAMED_OBJECT: VARENUM = VARENUM(68u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_TYPEMASK: VARENUM = VARENUM(4095u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UI1: VARENUM = VARENUM(17u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UI2: VARENUM = VARENUM(18u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UI4: VARENUM = VARENUM(19u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UI8: VARENUM = VARENUM(21u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UINT: VARENUM = VARENUM(23u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UINT_PTR: VARENUM = VARENUM(38u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_UNKNOWN: VARENUM = VARENUM(13u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_USERDEFINED: VARENUM = VARENUM(29u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_VARIANT: VARENUM = VARENUM(12u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_VECTOR: VARENUM = VARENUM(4096u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_VERSIONED_STREAM: VARENUM = VARENUM(73u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
pub const VT_VOID: VARENUM = VARENUM(24u16);
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DRAWPROGRESSFLAGS(pub i32);
impl ::core::marker::Copy for DRAWPROGRESSFLAGS {}
impl ::core::clone::Clone for DRAWPROGRESSFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWPROGRESSFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DRAWPROGRESSFLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DRAWPROGRESSFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWPROGRESSFLAGS").field(&self.0).finish()
    }
}
impl DRAWPROGRESSFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWPROGRESSFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWPROGRESSFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWPROGRESSFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSTIME_FLAGS(pub i32);
impl ::core::marker::Copy for PSTIME_FLAGS {}
impl ::core::clone::Clone for PSTIME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSTIME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PSTIME_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PSTIME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSTIME_FLAGS").field(&self.0).finish()
    }
}
impl PSTIME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PSTIME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSTIME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSTIME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSTIME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSTIME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VARENUM(pub u16);
impl ::core::marker::Copy for VARENUM {}
impl ::core::clone::Clone for VARENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VARENUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VARENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Variant\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VAR_CHANGE_FLAGS(pub u16);
impl ::core::marker::Copy for VAR_CHANGE_FLAGS {}
impl ::core::clone::Clone for VAR_CHANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VAR_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VAR_CHANGE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VAR_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VAR_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl VAR_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VAR_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VAR_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VAR_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VAR_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::TypeKind for VARIANT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::TypeKind for VARIANT_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::TypeKind for VARIANT_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: ::std::mem::ManuallyDrop<::windows_core::BSTR>,
    pub punkVal: ::std::mem::ManuallyDrop<::core::option::Option<::windows_core::IUnknown>>,
    pub pdispVal: ::std::mem::ManuallyDrop<::core::option::Option<super::Com::IDispatch>>,
    pub parray: *mut super::Com::SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::windows_core::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows_core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<super::Com::IDispatch>,
    pub pparray: *mut *mut super::Com::SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: u8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: ::windows_core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::TypeKind for VARIANT_0_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Variant\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::std::mem::ManuallyDrop<::core::option::Option<super::Ole::IRecordInfo>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0_0").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows_core::TypeKind for VARIANT_0_0_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
