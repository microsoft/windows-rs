#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserFree(param0 : *const u32, param1 : *const *mut core::ffi::c_void));
    unsafe { BSTR_UserFree(param0, core::mem::transmute(param1)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserFree64(param0 : *const u32, param1 : *const *mut core::ffi::c_void));
    unsafe { BSTR_UserFree64(param0, core::mem::transmute(param1)) }
}
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const windows_core::BSTR) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const *mut core::ffi::c_void) -> *mut u8);
    unsafe { BSTR_UserMarshal(param0, param1 as _, core::mem::transmute(param2)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const windows_core::BSTR) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const *mut core::ffi::c_void) -> *mut u8);
    unsafe { BSTR_UserMarshal64(param0, param1 as _, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const windows_core::BSTR) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserSize(param0 : *const u32, param1 : u32, param2 : *const *mut core::ffi::c_void) -> u32);
    unsafe { BSTR_UserSize(param0, param1, core::mem::transmute(param2)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const windows_core::BSTR) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserSize64(param0 : *const u32, param1 : u32, param2 : *const *mut core::ffi::c_void) -> u32);
    unsafe { BSTR_UserSize64(param0, param1, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut windows_core::BSTR) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut *mut core::ffi::c_void) -> *mut u8);
    unsafe { BSTR_UserUnmarshal(param0, param1.as_ptr(), core::mem::transmute(param2)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut windows_core::BSTR) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut *mut core::ffi::c_void) -> *mut u8);
    unsafe { BSTR_UserUnmarshal64(param0, param1.as_ptr(), core::mem::transmute(param2)) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const super::CLIPFORMAT) {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserFree(param0 : *const u32, param1 : *const super::CLIPFORMAT));
    unsafe { CLIPFORMAT_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const super::CLIPFORMAT) {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserFree64(param0 : *const u32, param1 : *const super::CLIPFORMAT));
    unsafe { CLIPFORMAT_UserFree64(param0, param1) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::CLIPFORMAT) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::CLIPFORMAT) -> *mut u8);
    unsafe { CLIPFORMAT_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::CLIPFORMAT) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::CLIPFORMAT) -> *mut u8);
    unsafe { CLIPFORMAT_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const super::CLIPFORMAT) -> u32 {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::CLIPFORMAT) -> u32);
    unsafe { CLIPFORMAT_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const super::CLIPFORMAT) -> u32 {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::CLIPFORMAT) -> u32);
    unsafe { CLIPFORMAT_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::CLIPFORMAT) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::CLIPFORMAT) -> *mut u8);
    unsafe { CLIPFORMAT_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "wtypes")]
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::CLIPFORMAT) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn CLIPFORMAT_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::CLIPFORMAT) -> *mut u8);
    unsafe { CLIPFORMAT_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn CLSIDFromProgIDEx(lpszprogid: super::LPCOLESTR) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn CLSIDFromProgIDEx(lpszprogid : super::LPCOLESTR, lpclsid : super::LPCLSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CoFileTimeNow() -> windows_core::Result<super::FILETIME> {
    windows_core::link!("ole32.dll" "system" fn CoFileTimeNow(lpfiletime : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoFileTimeNow(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> windows_core::Result<super::LPDATAADVISEHOLDER> {
    windows_core::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDataAdviseHolder(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn DriveType(idrive: i32) -> i32 {
    windows_core::link!("shell32.dll" "system" fn DriveType(idrive : i32) -> i32);
    unsafe { DriveType(idrive) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn FreePropVariantArray(rgvars: &mut [super::PROPVARIANT]) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn FreePropVariantArray(cvariants : ULONG, rgvars : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { FreePropVariantArray(rgvars.len().try_into().unwrap(), rgvars.as_mut_ptr()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetFileNameFromBrowse<P3, P4, P5, P6>(hwnd: Option<super::HWND>, pszfilepath: &mut [u16], pszworkingdir: P3, pszdefext: P4, pszfilters: P5, psztitle: P6) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn GetFileNameFromBrowse(hwnd : super::HWND, pszfilepath : windows_core::PWSTR, cchfilepath : UINT, pszworkingdir : windows_core::PCWSTR, pszdefext : windows_core::PCWSTR, pszfilters : windows_core::PCWSTR, psztitle : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { GetFileNameFromBrowse(hwnd.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszfilepath.as_mut_ptr()), pszfilepath.len().try_into().unwrap(), pszworkingdir.param().abi(), pszdefext.param().abi(), pszfilters.param().abi(), psztitle.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const super::HACCEL) {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserFree(param0 : *const u32, param1 : *const super::HACCEL));
    unsafe { HACCEL_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const super::HACCEL) {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserFree64(param0 : *const u32, param1 : *const super::HACCEL));
    unsafe { HACCEL_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HACCEL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HACCEL) -> *mut u8);
    unsafe { HACCEL_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HACCEL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HACCEL) -> *mut u8);
    unsafe { HACCEL_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::HACCEL) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HACCEL) -> u32);
    unsafe { HACCEL_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::HACCEL) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HACCEL) -> u32);
    unsafe { HACCEL_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HACCEL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HACCEL) -> *mut u8);
    unsafe { HACCEL_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HACCEL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HACCEL) -> *mut u8);
    unsafe { HACCEL_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const super::HBITMAP) {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserFree(param0 : *const u32, param1 : *const super::HBITMAP));
    unsafe { HBITMAP_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::HBITMAP) {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserFree64(param0 : *const u32, param1 : *const super::HBITMAP));
    unsafe { HBITMAP_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HBITMAP) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HBITMAP) -> *mut u8);
    unsafe { HBITMAP_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HBITMAP) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HBITMAP) -> *mut u8);
    unsafe { HBITMAP_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::HBITMAP) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HBITMAP) -> u32);
    unsafe { HBITMAP_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::HBITMAP) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HBITMAP) -> u32);
    unsafe { HBITMAP_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HBITMAP) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HBITMAP) -> *mut u8);
    unsafe { HBITMAP_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HBITMAP) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HBITMAP_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HBITMAP) -> *mut u8);
    unsafe { HBITMAP_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const super::HDC) {
    windows_core::link!("ole32.dll" "system" fn HDC_UserFree(param0 : *const u32, param1 : *const super::HDC));
    unsafe { HDC_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const super::HDC) {
    windows_core::link!("ole32.dll" "system" fn HDC_UserFree64(param0 : *const u32, param1 : *const super::HDC));
    unsafe { HDC_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HDC) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HDC_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HDC) -> *mut u8);
    unsafe { HDC_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HDC) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HDC_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HDC) -> *mut u8);
    unsafe { HDC_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::HDC) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HDC_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HDC) -> u32);
    unsafe { HDC_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::HDC) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HDC_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HDC) -> u32);
    unsafe { HDC_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HDC) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HDC_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HDC) -> *mut u8);
    unsafe { HDC_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HDC) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HDC_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HDC) -> *mut u8);
    unsafe { HDC_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const super::HGLOBAL) {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserFree(param0 : *const u32, param1 : *const super::HGLOBAL));
    unsafe { HGLOBAL_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const super::HGLOBAL) {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserFree64(param0 : *const u32, param1 : *const super::HGLOBAL));
    unsafe { HGLOBAL_UserFree64(param0, param1) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HGLOBAL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HGLOBAL) -> *mut u8);
    unsafe { HGLOBAL_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HGLOBAL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HGLOBAL) -> *mut u8);
    unsafe { HGLOBAL_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const super::HGLOBAL) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HGLOBAL) -> u32);
    unsafe { HGLOBAL_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const super::HGLOBAL) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HGLOBAL) -> u32);
    unsafe { HGLOBAL_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HGLOBAL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HGLOBAL) -> *mut u8);
    unsafe { HGLOBAL_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HGLOBAL) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HGLOBAL) -> *mut u8);
    unsafe { HGLOBAL_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const super::HICON) {
    windows_core::link!("ole32.dll" "system" fn HICON_UserFree(param0 : *const u32, param1 : *const super::HICON));
    unsafe { HICON_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const super::HICON) {
    windows_core::link!("ole32.dll" "system" fn HICON_UserFree64(param0 : *const u32, param1 : *const super::HICON));
    unsafe { HICON_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HICON) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HICON_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HICON) -> *mut u8);
    unsafe { HICON_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HICON) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HICON_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HICON) -> *mut u8);
    unsafe { HICON_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::HICON) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HICON_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HICON) -> u32);
    unsafe { HICON_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::HICON) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HICON_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HICON) -> u32);
    unsafe { HICON_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HICON) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HICON_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HICON) -> *mut u8);
    unsafe { HICON_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HICON) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HICON_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HICON) -> *mut u8);
    unsafe { HICON_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const super::HMENU) {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserFree(param0 : *const u32, param1 : *const super::HMENU));
    unsafe { HMENU_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const super::HMENU) {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserFree64(param0 : *const u32, param1 : *const super::HMENU));
    unsafe { HMENU_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HMENU) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HMENU) -> *mut u8);
    unsafe { HMENU_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HMENU) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HMENU) -> *mut u8);
    unsafe { HMENU_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::HMENU) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HMENU) -> u32);
    unsafe { HMENU_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::HMENU) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HMENU) -> u32);
    unsafe { HMENU_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HMENU) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HMENU) -> *mut u8);
    unsafe { HMENU_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HMENU) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMENU_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HMENU) -> *mut u8);
    unsafe { HMENU_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserFree(param0: *const u32, param1: *const super::HMONITOR) {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserFree(param0 : *const u32, param1 : *const super::HMONITOR));
    unsafe { HMONITOR_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserFree64(param0: *const u32, param1: *const super::HMONITOR) {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserFree64(param0 : *const u32, param1 : *const super::HMONITOR));
    unsafe { HMONITOR_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HMONITOR) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HMONITOR) -> *mut u8);
    unsafe { HMONITOR_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HMONITOR) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HMONITOR) -> *mut u8);
    unsafe { HMONITOR_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserSize(param0: *const u32, param1: u32, param2: *const super::HMONITOR) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HMONITOR) -> u32);
    unsafe { HMONITOR_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserSize64(param0: *const u32, param1: u32, param2: *const super::HMONITOR) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HMONITOR) -> u32);
    unsafe { HMONITOR_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HMONITOR) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HMONITOR) -> *mut u8);
    unsafe { HMONITOR_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HMONITOR_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HMONITOR) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HMONITOR_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HMONITOR) -> *mut u8);
    unsafe { HMONITOR_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const super::HPALETTE) {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserFree(param0 : *const u32, param1 : *const super::HPALETTE));
    unsafe { HPALETTE_UserFree(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HPALETTE) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HPALETTE) -> *mut u8);
    unsafe { HPALETTE_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::HPALETTE) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HPALETTE) -> u32);
    unsafe { HPALETTE_UserSize(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HPALETTE) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HPALETTE) -> *mut u8);
    unsafe { HPALETTE_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[inline]
pub unsafe fn HSTRING_UserFree(pflags: *const ULONG, ppidl: *const windows_core::HSTRING) {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree(pflags : *const ULONG, ppidl : *const *mut core::ffi::c_void));
    unsafe { HSTRING_UserFree(pflags, core::mem::transmute(ppidl)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const windows_core::HSTRING) {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserFree64(param0 : *const u32, param1 : *const *mut core::ffi::c_void));
    unsafe { HSTRING_UserFree64(param0, core::mem::transmute(param1)) }
}
#[inline]
pub unsafe fn HSTRING_UserMarshal(pflags: *const ULONG, pbuffer: *mut UCHAR, ppidl: *const windows_core::HSTRING) -> *mut UCHAR {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal(pflags : *const ULONG, pbuffer : *mut UCHAR, ppidl : *const *mut core::ffi::c_void) -> *mut UCHAR);
    unsafe { HSTRING_UserMarshal(pflags, pbuffer as _, core::mem::transmute(ppidl)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const windows_core::HSTRING) -> *mut u8 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const *mut core::ffi::c_void) -> *mut u8);
    unsafe { HSTRING_UserMarshal64(param0, param1 as _, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserSize(pflags: *const ULONG, startingsize: ULONG, ppidl: *const windows_core::HSTRING) -> ULONG {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize(pflags : *const ULONG, startingsize : ULONG, ppidl : *const *mut core::ffi::c_void) -> ULONG);
    unsafe { HSTRING_UserSize(pflags, startingsize, core::mem::transmute(ppidl)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const windows_core::HSTRING) -> u32 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserSize64(param0 : *const u32, param1 : u32, param2 : *const *mut core::ffi::c_void) -> u32);
    unsafe { HSTRING_UserSize64(param0, param1, core::mem::transmute(param2)) }
}
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(pflags: *const ULONG, pbuffer: &[UCHAR; 0], ppidl: *mut windows_core::HSTRING) -> *mut UCHAR {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal(pflags : *const ULONG, pbuffer : *const UCHAR, ppidl : *mut *mut core::ffi::c_void) -> *mut UCHAR);
    unsafe { HSTRING_UserUnmarshal(pflags, pbuffer.as_ptr(), core::mem::transmute(ppidl)) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut windows_core::HSTRING) -> *mut u8 {
    windows_core::link!("api-ms-win-core-winrt-string-l1-1-0.dll" "system" fn HSTRING_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut *mut core::ffi::c_void) -> *mut u8);
    unsafe { HSTRING_UserUnmarshal64(param0, param1.as_ptr(), core::mem::transmute(param2)) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::HWND) {
    windows_core::link!("ole32.dll" "system" fn HWND_UserFree(param0 : *const u32, param1 : *const super::HWND));
    unsafe { HWND_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::HWND) {
    windows_core::link!("ole32.dll" "system" fn HWND_UserFree64(param0 : *const u32, param1 : *const super::HWND));
    unsafe { HWND_UserFree64(param0, param1) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::HWND) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HWND_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::HWND) -> *mut u8);
    unsafe { HWND_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::HWND) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HWND_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::HWND) -> *mut u8);
    unsafe { HWND_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::HWND) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HWND_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::HWND) -> u32);
    unsafe { HWND_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::HWND) -> u32 {
    windows_core::link!("ole32.dll" "system" fn HWND_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::HWND) -> u32);
    unsafe { HWND_UserSize64(param0, param1, param2) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::HWND) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HWND_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::HWND) -> *mut u8);
    unsafe { HWND_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::HWND) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn HWND_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::HWND) -> *mut u8);
    unsafe { HWND_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const super::LPSAFEARRAY) {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserFree(param0 : *const u32, param1 : *const super::LPSAFEARRAY));
    unsafe { LPSAFEARRAY_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const super::LPSAFEARRAY) {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserFree64(param0 : *const u32, param1 : *const super::LPSAFEARRAY));
    unsafe { LPSAFEARRAY_UserFree64(param0, param1) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::LPSAFEARRAY) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::LPSAFEARRAY) -> *mut u8);
    unsafe { LPSAFEARRAY_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::LPSAFEARRAY) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::LPSAFEARRAY) -> *mut u8);
    unsafe { LPSAFEARRAY_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const super::LPSAFEARRAY) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::LPSAFEARRAY) -> u32);
    unsafe { LPSAFEARRAY_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const super::LPSAFEARRAY) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::LPSAFEARRAY) -> u32);
    unsafe { LPSAFEARRAY_UserSize64(param0, param1, param2) }
}
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::LPSAFEARRAY) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::LPSAFEARRAY) -> *mut u8);
    unsafe { LPSAFEARRAY_UserUnmarshal(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "oaidl", feature = "winnt"))]
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::LPSAFEARRAY) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::LPSAFEARRAY) -> *mut u8);
    unsafe { LPSAFEARRAY_UserUnmarshal64(param0, param1.as_ptr(), param2 as _) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnBlockOffset(plsn: *const super::CLFS_LSN) -> ULONG {
    windows_core::link!("clfsw32.dll" "system" fn LsnBlockOffset(plsn : *const super::CLFS_LSN) -> ULONG);
    unsafe { LsnBlockOffset(plsn) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnContainer(plsn: *const super::CLFS_LSN) -> super::CLFS_CONTAINER_ID {
    windows_core::link!("clfsw32.dll" "system" fn LsnContainer(plsn : *const super::CLFS_LSN) -> super::CLFS_CONTAINER_ID);
    unsafe { LsnContainer(plsn) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnCreate(cidcontainer: super::CLFS_CONTAINER_ID, offblock: ULONG, crecord: ULONG) -> super::CLFS_LSN {
    windows_core::link!("clfsw32.dll" "system" fn LsnCreate(cidcontainer : super::CLFS_CONTAINER_ID, offblock : ULONG, crecord : ULONG) -> super::CLFS_LSN);
    unsafe { LsnCreate(cidcontainer, offblock, crecord) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnRecordSequence(plsn: *const super::CLFS_LSN) -> ULONG {
    windows_core::link!("clfsw32.dll" "system" fn LsnRecordSequence(plsn : *const super::CLFS_LSN) -> ULONG);
    unsafe { LsnRecordSequence(plsn) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantClear(pvar: *mut super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantClear(pvar) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn PropVariantCopy(pvardest: *mut super::PROPVARIANT, pvarsrc: *const super::PROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn PropVariantCopy(pvardest : *mut super::PROPVARIANT, pvarsrc : *const super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe { PropVariantCopy(pvardest, pvarsrc) }
}
#[cfg(all(feature = "ntdef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlInitUTF8StringEx(destinationstring: super::PUTF8_STRING, sourcestring: Option<super::PCSZ>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitUTF8StringEx(destinationstring : super::PUTF8_STRING, sourcestring : super::PCSZ) -> windows_core::NTSTATUS);
    unsafe { RtlInitUTF8StringEx(destinationstring as _, sourcestring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlIsUntrustedObject(handle: Option<super::HANDLE>, object: Option<super::PVOID>, untrustedobject: super::PBOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIsUntrustedObject(handle : super::HANDLE, object : super::PVOID, untrustedobject : super::PBOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlIsUntrustedObject(handle.unwrap_or(core::mem::zeroed()) as _, object.unwrap_or(core::mem::zeroed()) as _, untrustedobject as _) }
}
#[cfg(all(feature = "ntdef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUTF8StringToUnicodeString(destinationstring: super::PUNICODE_STRING, sourcestring: super::PUTF8_STRING, allocatedestinationstring: BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUTF8StringToUnicodeString(destinationstring : super::PUNICODE_STRING, sourcestring : super::PUTF8_STRING, allocatedestinationstring : BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUTF8StringToUnicodeString(destinationstring as _, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUTF8ToUnicodeN(unicodestringdestination: windows_core::PWSTR, unicodestringmaxbytecount: ULONG, unicodestringactualbytecount: super::PULONG, utf8stringsource: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUTF8ToUnicodeN(unicodestringdestination : windows_core::PWSTR, unicodestringmaxbytecount : ULONG, unicodestringactualbytecount : super::PULONG, utf8stringsource : super::PCCH, utf8stringbytecount : ULONG) -> windows_core::NTSTATUS);
    unsafe { RtlUTF8ToUnicodeN(unicodestringdestination, unicodestringmaxbytecount, unicodestringactualbytecount as _, core::mem::transmute(utf8stringsource.as_ptr()), utf8stringsource.len().try_into().unwrap()) }
}
#[cfg(all(feature = "ntdef", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlUnicodeStringToUTF8String(destinationstring: super::PUTF8_STRING, sourcestring: super::PCUNICODE_STRING, allocatedestinationstring: BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToUTF8String(destinationstring : super::PUTF8_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeStringToUTF8String(destinationstring as _, sourcestring, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeToUTF8N(utf8stringdestination: super::PCHAR, utf8stringmaxbytecount: ULONG, utf8stringactualbytecount: super::PULONG, unicodestringsource: super::PCWCH, unicodestringbytecount: ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToUTF8N(utf8stringdestination : super::PCHAR, utf8stringmaxbytecount : ULONG, utf8stringactualbytecount : super::PULONG, unicodestringsource : super::PCWCH, unicodestringbytecount : ULONG) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeToUTF8N(utf8stringdestination as _, utf8stringmaxbytecount, utf8stringactualbytecount as _, unicodestringsource, unicodestringbytecount) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserFree(param0 : *const u32, param1 : *const super::STGMEDIUM));
    unsafe { STGMEDIUM_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserFree64(param0 : *const u32, param1 : *const super::STGMEDIUM));
    unsafe { STGMEDIUM_UserFree64(param0, param1) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::STGMEDIUM) -> *mut u8);
    unsafe { STGMEDIUM_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::STGMEDIUM) -> *mut u8);
    unsafe { STGMEDIUM_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::STGMEDIUM) -> u32);
    unsafe { STGMEDIUM_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::STGMEDIUM) -> u32);
    unsafe { STGMEDIUM_UserSize64(param0, param1, param2) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::STGMEDIUM) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::STGMEDIUM) -> *mut u8);
    unsafe { STGMEDIUM_UserUnmarshal(param0, param1.as_ptr(), param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::STGMEDIUM) -> *mut u8 {
    windows_core::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::STGMEDIUM) -> *mut u8);
    unsafe { STGMEDIUM_UserUnmarshal64(param0, param1.as_ptr(), param2) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SignalObjectAndWait(hobjecttosignal: super::HANDLE, hobjecttowaiton: super::HANDLE, dwmilliseconds: DWORD, balertable: bool) -> DWORD {
    windows_core::link!("kernel32.dll" "system" fn SignalObjectAndWait(hobjecttosignal : super::HANDLE, hobjecttowaiton : super::HANDLE, dwmilliseconds : DWORD, balertable : windows_core::BOOL) -> DWORD);
    unsafe { SignalObjectAndWait(hobjecttosignal, hobjecttowaiton, dwmilliseconds, balertable.into()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT) {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserFree(param0 : *const u32, param1 : *const super::VARIANT));
    unsafe { VARIANT_UserFree(param0, param1) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT) {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserFree64(param0 : *const u32, param1 : *const super::VARIANT));
    unsafe { VARIANT_UserFree64(param0, param1) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::VARIANT) -> *mut u8);
    unsafe { VARIANT_UserMarshal(param0, param1 as _, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::VARIANT) -> *mut u8);
    unsafe { VARIANT_UserMarshal64(param0, param1 as _, param2) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::VARIANT) -> u32);
    unsafe { VARIANT_UserSize(param0, param1, param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::VARIANT) -> u32);
    unsafe { VARIANT_UserSize64(param0, param1, param2) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: &[u8; 0], param2: *mut super::VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::VARIANT) -> *mut u8);
    unsafe { VARIANT_UserUnmarshal(param0, param1.as_ptr(), param2) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: &[u8; 0], param2: *mut super::VARIANT) -> *mut u8 {
    windows_core::link!("oleaut32.dll" "system" fn VARIANT_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::VARIANT) -> *mut u8);
    unsafe { VARIANT_UserUnmarshal64(param0, param1.as_ptr(), param2) }
}
#[inline]
pub unsafe fn VerSetConditionMask(conditionmask: ULONGLONG, typemask: DWORD, condition: BYTE) -> ULONGLONG {
    windows_core::link!("kernel32.dll" "system" fn VerSetConditionMask(conditionmask : ULONGLONG, typemask : DWORD, condition : BYTE) -> ULONGLONG);
    unsafe { VerSetConditionMask(conditionmask, typemask, condition) }
}
#[cfg(all(feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwDeviceIoControlFile(filehandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::PIO_APC_ROUTINE, apccontext: Option<super::PVOID>, iostatusblock: super::PIO_STATUS_BLOCK, iocontrolcode: ULONG, inputbuffer: Option<super::PVOID>, inputbufferlength: ULONG, outputbuffer: Option<super::PVOID>, outputbufferlength: ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeviceIoControlFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : super::PIO_APC_ROUTINE, apccontext : super::PVOID, iostatusblock : super::PIO_STATUS_BLOCK, iocontrolcode : ULONG, inputbuffer : super::PVOID, inputbufferlength : ULONG, outputbuffer : super::PVOID, outputbufferlength : ULONG) -> windows_core::NTSTATUS);
    unsafe { ZwDeviceIoControlFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, iocontrolcode, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(all(feature = "wdm", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryVolumeInformationFile(filehandle: super::HANDLE, iostatusblock: super::PIO_STATUS_BLOCK, fsinformation: super::PVOID, length: ULONG, fsinformationclass: super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryVolumeInformationFile(filehandle : super::HANDLE, iostatusblock : super::PIO_STATUS_BLOCK, fsinformation : super::PVOID, length : ULONG, fsinformationclass : super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS);
    unsafe { ZwQueryVolumeInformationFile(filehandle, iostatusblock as _, fsinformation as _, length, fsinformationclass) }
}
#[cfg(feature = "mmsyscom")]
#[inline]
pub unsafe fn joyConfigChanged(dwflags: DWORD) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : DWORD) -> super::MMRESULT);
    unsafe { joyConfigChanged(dwflags) }
}
pub type ACCESS_MASK = u32;
pub type ALG_ID = u32;
pub type BOOLEAN = u8;
pub type BTH_ADDR = u64;
pub type BYTE = u8;
pub const CDN_FIRST: u32 = 4294966695;
pub const CDN_LAST: u32 = 4294966597;
pub type CHAR = i8;
pub const CLSID_BroadcastEventService: windows_core::GUID = windows_core::GUID::from_u128(0x0b3ffb92_0919_4934_9d5b_619c719d0202);
#[cfg(feature = "windef")]
pub type D2D1_POINT_2L = super::POINT;
#[cfg(feature = "windef")]
pub type D2D1_RECT_L = super::RECT;
pub const DDPF_FOURCC: i32 = 4;
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Disk_Number: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 5 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Gpt_Name: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 9 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Gpt_Type: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 8 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Mbr_Type: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 7 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Partition_Number: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 6 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Portable: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 2 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_Removable_Media: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 3 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_Storage_System_Critical: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x4d1ebee8_0803_4774_9842_b77db50265e9), pid: 4 };
pub type DISPID = i32;
pub type DWORD = u32;
pub type DWORD64 = u64;
pub type DWORDLONG = u64;
pub const DWORDLONG_MAX: u64 = 18446744073709551615;
pub const DWORD_MAX: u32 = 4294967295;
pub type DWORD_PTR = usize;
pub const E_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070490_u32 as _);
pub const FR_MATCHALEFHAMZA: u32 = 2147483648;
pub const FR_MATCHDIAC: i32 = 536870912;
pub const FR_MATCHKASHIDA: i32 = 1073741824;
pub const GUID_DEVINTERFACE_CDCHANGER: windows_core::GUID = windows_core::GUID::from_u128(0x53f56312_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_CDROM: windows_core::GUID = windows_core::GUID::from_u128(0x53f56308_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_COMPORT: windows_core::GUID = windows_core::GUID::from_u128(0x86e0d1e0_8089_11d0_9ce4_08003e301f73);
pub const GUID_DEVINTERFACE_DISK: windows_core::GUID = windows_core::GUID::from_u128(0x53f56307_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_FLOPPY: windows_core::GUID = windows_core::GUID::from_u128(0x53f56311_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_HIDDEN_DISK: windows_core::GUID = windows_core::GUID::from_u128(0x7fccc86c_228a_40ad_8a58_f590af7bfdce);
pub const GUID_DEVINTERFACE_HIDDEN_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x7f108a28_9833_4b3b_b780_2c6b5fa5c062);
pub const GUID_DEVINTERFACE_MEDIUMCHANGER: windows_core::GUID = windows_core::GUID::from_u128(0x53f56310_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_PARTITION: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630a_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);
pub const GUID_DEVINTERFACE_SERVICE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x6ead3d82_25ec_46bc_b7fd_c1f0df8f5037);
pub const GUID_DEVINTERFACE_SES: windows_core::GUID = windows_core::GUID::from_u128(0x1790c9ec_47d5_4df3_b5af_9adf3cf23e48);
pub const GUID_DEVINTERFACE_STORAGEPORT: windows_core::GUID = windows_core::GUID::from_u128(0x2accfe60_c130_11d2_b082_00a0c91efb8b);
pub const GUID_DEVINTERFACE_TAPE: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630b_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_UNIFIED_ACCESS_RPMB: windows_core::GUID = windows_core::GUID::from_u128(0x27447c21_bcc3_4d07_a05b_a3395bb4eee7);
pub const GUID_DEVINTERFACE_VMLUN: windows_core::GUID = windows_core::GUID::from_u128(0x6f416619_9f29_42a5_b20b_37e219ca02b0);
pub const GUID_DEVINTERFACE_VOLUME: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630d_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_WRITEONCEDISK: windows_core::GUID = windows_core::GUID::from_u128(0x53f5630c_b6bf_11d0_94f2_00a0c91efb8b);
pub const GUID_DEVINTERFACE_ZNSDISK: windows_core::GUID = windows_core::GUID::from_u128(0xb87941c5_ffdb_43c7_b6b1_20b632f0b109);
pub type HCATADMIN = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: super::PUCHAR,
    pub reportBufferLen: ULONG,
    pub reportId: UCHAR,
}
pub type HINTERNET = *mut core::ffi::c_void;
pub type HTHEME = *mut core::ffi::c_void;
pub const IID_ICommDlgBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x000214f1_0000_0000_c000_000000000046);
pub const IID_ICommDlgBrowser2: windows_core::GUID = windows_core::GUID::from_u128(0x10339516_2894_11d2_9039_00c04f8eeb3e);
pub const IID_IContextMenu: windows_core::GUID = windows_core::GUID::from_u128(0x000214e4_0000_0000_c000_000000000046);
pub const IID_IContextMenu2: windows_core::GUID = windows_core::GUID::from_u128(0x000214f4_0000_0000_c000_000000000046);
pub const IID_IContextMenu3: windows_core::GUID = windows_core::GUID::from_u128(0xbcfce0a0_ec17_11d0_8d10_00a0c90f2719);
pub const IID_IEnumExtraSearch: windows_core::GUID = windows_core::GUID::from_u128(0x0e700be1_9db6_11d1_a1ce_00c04fd75d13);
pub const IID_IEnumIDList: windows_core::GUID = windows_core::GUID::from_u128(0x000214f2_0000_0000_c000_000000000046);
pub const IID_IFileSystemBindData: windows_core::GUID = windows_core::GUID::from_u128(0x01e18d10_4d8b_11d2_855d_006008059367);
pub const IID_IKsControl: windows_core::GUID = windows_core::GUID::from_u128(0x28f54685_06fd_11d2_b27a_00a0c9223196);
pub const IID_IPersistFolder: windows_core::GUID = windows_core::GUID::from_u128(0x000214ea_0000_0000_c000_000000000046);
pub const IID_IRemoteComputer: windows_core::GUID = windows_core::GUID::from_u128(0x000214fe_0000_0000_c000_000000000046);
pub const IID_IShellBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x000214e2_0000_0000_c000_000000000046);
pub const IID_IShellExtInit: windows_core::GUID = windows_core::GUID::from_u128(0x000214e8_0000_0000_c000_000000000046);
pub const IID_IShellFolder: windows_core::GUID = windows_core::GUID::from_u128(0x000214e6_0000_0000_c000_000000000046);
pub const IID_IShellFolder2: windows_core::GUID = windows_core::GUID::from_u128(0x93f2f68c_1d1b_11d3_a30e_00c04f79abd1);
pub const IID_IShellIcon: windows_core::GUID = windows_core::GUID::from_u128(0x000214e5_0000_0000_c000_000000000046);
pub const IID_IShellIconOverlayIdentifier: windows_core::GUID = windows_core::GUID::from_u128(0x0c6c4200_c589_11d0_999a_00c04fd655e1);
pub const IID_IShellLinkA: windows_core::GUID = windows_core::GUID::from_u128(0x000214ee_0000_0000_c000_000000000046);
pub const IID_IShellLinkW: windows_core::GUID = windows_core::GUID::from_u128(0x000214f9_0000_0000_c000_000000000046);
pub const IID_IShellPropSheetExt: windows_core::GUID = windows_core::GUID::from_u128(0x000214e9_0000_0000_c000_000000000046);
pub const IID_IShellView: windows_core::GUID = windows_core::GUID::from_u128(0x000214e3_0000_0000_c000_000000000046);
pub const IID_IShellView2: windows_core::GUID = windows_core::GUID::from_u128(0x88e39e80_3578_11cf_ae69_08002b2e1262);
pub type INT = i32;
pub type INT16 = i16;
pub type INT32 = i32;
pub type INT64 = i64;
pub type INT8 = i8;
pub type INTERNET_PORT = u16;
pub type INT_PTR = isize;
#[cfg(feature = "winnt")]
pub const INVALID_HANDLE_VALUE: super::HANDLE = -1 as _;
pub const IOCTL_SERENUM_EXPOSE_HARDWARE: i32 = 3604992;
pub const IOCTL_SERENUM_GET_PORT_NAME: i32 = 3605004;
pub const IOCTL_SERENUM_PORT_DESC: i32 = 3605000;
pub const IOCTL_SERENUM_REMOVE_HARDWARE: i32 = 3604996;
pub const IOCTL_SERIAL_LSRMST_INSERT: i32 = 1769596;
pub type IPAddr = u32;
pub type IPMask = u32;
pub const KEYBOARD_OVERRUN_MAKE_CODE: i32 = 255;
pub type KF_REDIRECT_FLAGS = u32;
pub const KSDATAFORMAT_SUBTYPE_PCM: windows_core::GUID = windows_core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const KSDATAFORMAT_SUBTYPE_WAVEFORMATEX: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub type LANGID = u16;
pub type LCID = u32;
pub type LONG = i32;
pub type LONG64 = i64;
pub type LONGLONG = i64;
pub type LONG_PTR = isize;
pub type LPBYTE = *mut u8;
pub type LPCVOID = *const core::ffi::c_void;
pub type LPHINTERNET = *mut *mut core::ffi::c_void;
pub type LPINTERNET_PORT = *mut u16;
pub type LPUINT = *mut u32;
pub const MAX_PROFILE_LEN: i32 = 80;
pub type MEMBERID = i32;
pub const MOUSE_ATTRIBUTES_CHANGED: i32 = 4;
pub const MOUSE_MOVE_ABSOLUTE: i32 = 1;
pub const MOUSE_MOVE_NOCOALESCE: i32 = 8;
pub const MOUSE_MOVE_RELATIVE: i32 = 0;
pub const MOUSE_VIRTUAL_DESKTOP: i32 = 2;
pub const MR_BUFFER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0xa562248c_9ac6_4ffc_9fba_3af8f8ad1a4d);
pub type NODE_REQUIREMENT = ULONG;
#[cfg(feature = "ntddk")]
pub type PBUS_HANDLER = *mut super::_BUS_HANDLER;
pub type PBYTE = *mut u8;
#[cfg(feature = "wdm")]
pub type PCALLBACK_OBJECT = *mut super::_CALLBACK_OBJECT;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "mce", feature = "winnt"))]
pub type PCMC_EXCEPTION = *mut super::MCA_EXCEPTION;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
pub type PCMC_EXCEPTION = super::PVOID;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(all(feature = "mce", feature = "winnt"))]
pub type PCPE_EXCEPTION = *mut super::MCA_EXCEPTION;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
pub type PCPE_EXCEPTION = super::PVOID;
#[cfg(feature = "ntddk")]
pub type PDEVICE_HANDLER_OBJECT = *mut super::_DEVICE_HANDLER_OBJECT;
#[cfg(feature = "wdm")]
pub type PEPROCESS = *mut super::_KPROCESS;
#[cfg(feature = "wdm")]
pub type PETHREAD = *mut super::_KTHREAD;
#[cfg(feature = "minwindef")]
pub type PHID_XFER_PACKET = *mut HID_XFER_PACKET;
#[cfg(feature = "winnt")]
pub type PHYSICAL_ADDRESS = super::LARGE_INTEGER;
#[cfg(feature = "wdm")]
pub type PIO_TIMER = *mut super::_IO_TIMER;
#[cfg(feature = "wdm")]
pub type PKAFFINITY_EX = *mut super::_KAFFINITY_EX;
#[cfg(feature = "wdm")]
pub type PKINTERRUPT = *mut super::_KINTERRUPT;
#[cfg(feature = "wdm")]
pub type PKPROCESS = *mut super::_KPROCESS;
#[cfg(feature = "wdm")]
pub type PKTHREAD = *mut super::_KTHREAD;
pub type PNTSTATUS = *mut i32;
#[cfg(feature = "wdm")]
pub type POBJECT_TYPE = *mut super::_OBJECT_TYPE;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type POWER_SETTING_CALLBACK = Option<unsafe extern "system" fn(settingguid: super::LPCGUID, value: super::PVOID, valuelength: ULONG, context: super::PVOID) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type PPOWER_SETTING_CALLBACK = Option<unsafe extern "system" fn(settingguid: super::LPCGUID, value: super::PVOID, valuelength: ULONG, context: super::PVOID) -> windows_core::NTSTATUS>;
#[cfg(feature = "wdm")]
pub type PRKPROCESS = *mut super::_KPROCESS;
#[cfg(feature = "wdm")]
pub type PRKTHREAD = *mut super::_KTHREAD;
pub type PROPERTYID = i32;
#[cfg(feature = "winnt")]
pub type PS_AVAILABLE_CPUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(parameter: super::PVOID)>;
#[cfg(feature = "winnt")]
pub type PS_AVAILABLE_CPUS_CHANGE_REGISTRATION = super::PVOID;
#[cfg(target_arch = "x86")]
pub type PULONG_PTR = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PULONG_PTR = *mut u64;
#[cfg(feature = "mmeapi")]
pub type PWAVEFORMATEXTENSIBLE = *mut WAVEFORMATEXTENSIBLE;
pub type QWORD = u64;
pub type REFERENCE_TIME = i64;
pub type REFTIME = f64;
#[cfg(target_arch = "x86")]
pub const RESULT_NEGATIVE: i32 = 32768;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RESULT_NEGATIVE: i32 = 1;
#[cfg(target_arch = "x86")]
pub const RESULT_POSITIVE: i32 = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RESULT_POSITIVE: i32 = 2;
#[cfg(target_arch = "x86")]
pub const RESULT_ZERO: i32 = 16384;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RESULT_ZERO: i32 = 0;
pub const SERIAL_IOC_FCR_DMA_MODE: u32 = 8;
pub const SERIAL_IOC_FCR_FIFO_ENABLE: u32 = 1;
pub const SERIAL_IOC_FCR_RCVR_RESET: u32 = 2;
pub const SERIAL_IOC_FCR_RCVR_TRIGGER_LSB: u32 = 64;
pub const SERIAL_IOC_FCR_RCVR_TRIGGER_MSB: u32 = 128;
pub const SERIAL_IOC_FCR_RES1: u32 = 16;
pub const SERIAL_IOC_FCR_RES2: u32 = 32;
pub const SERIAL_IOC_FCR_XMIT_RESET: u32 = 4;
pub const SERIAL_IOC_MCR_DTR: u32 = 1;
pub const SERIAL_IOC_MCR_LOOP: u32 = 16;
pub const SERIAL_IOC_MCR_OUT1: u32 = 4;
pub const SERIAL_IOC_MCR_OUT2: u32 = 8;
pub const SERIAL_IOC_MCR_RTS: u32 = 2;
pub const SERIAL_LSRMST_ESCAPE: u8 = 0;
pub const SERIAL_LSRMST_LSR_DATA: u8 = 1;
pub const SERIAL_LSRMST_LSR_NODATA: u8 = 2;
pub const SERIAL_LSRMST_MST: u8 = 3;
pub type SHORT = i16;
pub type SIZE_T = usize;
pub type SSIZE_T = isize;
pub const STATUS_ACCESS_DENIED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000022_u32 as _);
pub const STATUS_ACCOUNT_DISABLED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000072_u32 as _);
pub const STATUS_ACCOUNT_EXPIRED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000193_u32 as _);
pub const STATUS_ACCOUNT_LOCKED_OUT: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000234_u32 as _);
pub const STATUS_ACCOUNT_RESTRICTION: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC000006E_u32 as _);
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000413_u32 as _);
pub const STATUS_DOWNGRADE_DETECTED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000388_u32 as _);
pub const STATUS_LOGON_FAILURE: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC000006D_u32 as _);
pub const STATUS_LOGON_TYPE_NOT_GRANTED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC000015B_u32 as _);
pub const STATUS_NO_SUCH_LOGON_SESSION: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC000005F_u32 as _);
pub const STATUS_NO_SUCH_USER: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000064_u32 as _);
pub const STATUS_PASSWORD_EXPIRED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000071_u32 as _);
pub const STATUS_PASSWORD_MUST_CHANGE: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0000224_u32 as _);
pub const STATUS_SEVERITY_ERROR: i32 = 3;
pub const STATUS_SEVERITY_INFORMATIONAL: i32 = 1;
pub const STATUS_SEVERITY_SUCCESS: i32 = 0;
pub const STATUS_SEVERITY_WARNING: i32 = 2;
pub const STATUS_WRONG_PASSWORD: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC000006A_u32 as _);
pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub type UCHAR = u8;
pub type UINT = u32;
pub type UINT16 = u16;
pub type UINT32 = u32;
pub type UINT64 = u64;
pub type UINT8 = u8;
pub type UINT_PTR = usize;
pub type ULONG = u32;
pub type ULONG64 = u64;
pub type ULONGLONG = u64;
pub type ULONG_PTR = usize;
pub type USHORT = u16;
pub type UWORD = u16;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: super::WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: DWORD,
    pub SubFormat: windows_core::GUID,
}
#[cfg(feature = "mmeapi")]
impl Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: WORD,
    pub wSamplesPerBlock: WORD,
    pub wReserved: WORD,
}
#[cfg(feature = "mmeapi")]
impl Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVE_FORMAT_EXTENSIBLE: i32 = 65534;
pub type WORD = u16;
pub type __C_ASSERT__ = [i8; 1];
pub type error_status_t = u32;
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type va_list = *mut i8;
