#[inline]
pub unsafe fn AssocCreate(clsid: windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn AssocCreate(clsid : windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { AssocCreate(core::mem::transmute(clsid), riid, ppv as _) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn AssocGetPerceivedType<P0>(pszext: P0, ptype: *mut super::shtypes::PERCEIVED, pflag: *mut super::shtypes::PERCEIVEDFLAG, ppsztype: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocGetPerceivedType(pszext : windows_core::PCWSTR, ptype : *mut super::shtypes::PERCEIVED, pflag : *mut super::shtypes::PERCEIVEDFLAG, ppsztype : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { AssocGetPerceivedType(pszext.param().abi(), ptype as _, pflag as _, ppsztype as _) }
}
#[inline]
pub unsafe fn AssocIsDangerous<P0>(pszassoc: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocIsDangerous(pszassoc : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AssocIsDangerous(pszassoc.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn AssocQueryKeyA<P2, P3>(flags: ASSOCF, key: ASSOCKEY, pszassoc: P2, pszextra: P3) -> windows_core::Result<super::minwindef::HKEY>
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocQueryKeyA(flags : ASSOCF, key : ASSOCKEY, pszassoc : windows_core::PCSTR, pszextra : windows_core::PCSTR, phkeyout : *mut super::minwindef::HKEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AssocQueryKeyA(flags, key, pszassoc.param().abi(), pszextra.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn AssocQueryKeyW<P2, P3>(flags: ASSOCF, key: ASSOCKEY, pszassoc: P2, pszextra: P3) -> windows_core::Result<super::minwindef::HKEY>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocQueryKeyW(flags : ASSOCF, key : ASSOCKEY, pszassoc : windows_core::PCWSTR, pszextra : windows_core::PCWSTR, phkeyout : *mut super::minwindef::HKEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AssocQueryKeyW(flags, key, pszassoc.param().abi(), pszextra.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn AssocQueryStringA<P2, P3>(flags: ASSOCF, str: ASSOCSTR, pszassoc: P2, pszextra: P3, pszout: Option<windows_core::PSTR>, pcchout: *mut u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocQueryStringA(flags : ASSOCF, str : ASSOCSTR, pszassoc : windows_core::PCSTR, pszextra : windows_core::PCSTR, pszout : windows_core::PSTR, pcchout : *mut u32) -> windows_core::HRESULT);
    unsafe { AssocQueryStringA(flags, str, pszassoc.param().abi(), pszextra.param().abi(), pszout.unwrap_or(core::mem::zeroed()) as _, pcchout as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn AssocQueryStringByKeyA<P3>(flags: ASSOCF, str: ASSOCSTR, hkassoc: super::minwindef::HKEY, pszextra: P3, pszout: Option<windows_core::PSTR>, pcchout: *mut u32) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocQueryStringByKeyA(flags : ASSOCF, str : ASSOCSTR, hkassoc : super::minwindef::HKEY, pszextra : windows_core::PCSTR, pszout : windows_core::PSTR, pcchout : *mut u32) -> windows_core::HRESULT);
    unsafe { AssocQueryStringByKeyA(flags, str, hkassoc, pszextra.param().abi(), pszout.unwrap_or(core::mem::zeroed()) as _, pcchout as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn AssocQueryStringByKeyW<P3>(flags: ASSOCF, str: ASSOCSTR, hkassoc: super::minwindef::HKEY, pszextra: P3, pszout: Option<windows_core::PWSTR>, pcchout: *mut u32) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocQueryStringByKeyW(flags : ASSOCF, str : ASSOCSTR, hkassoc : super::minwindef::HKEY, pszextra : windows_core::PCWSTR, pszout : windows_core::PWSTR, pcchout : *mut u32) -> windows_core::HRESULT);
    unsafe { AssocQueryStringByKeyW(flags, str, hkassoc, pszextra.param().abi(), pszout.unwrap_or(core::mem::zeroed()) as _, pcchout as _) }
}
#[inline]
pub unsafe fn AssocQueryStringW<P2, P3>(flags: ASSOCF, str: ASSOCSTR, pszassoc: P2, pszextra: P3, pszout: Option<windows_core::PWSTR>, pcchout: *mut u32) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn AssocQueryStringW(flags : ASSOCF, str : ASSOCSTR, pszassoc : windows_core::PCWSTR, pszextra : windows_core::PCWSTR, pszout : windows_core::PWSTR, pcchout : *mut u32) -> windows_core::HRESULT);
    unsafe { AssocQueryStringW(flags, str, pszassoc.param().abi(), pszextra.param().abi(), pszout.unwrap_or(core::mem::zeroed()) as _, pcchout as _) }
}
#[inline]
pub unsafe fn ChrCmpIA(w1: u16, w2: u16) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn ChrCmpIA(w1 : u16, w2 : u16) -> windows_core::BOOL);
    unsafe { ChrCmpIA(w1, w2) }
}
#[inline]
pub unsafe fn ChrCmpIW(w1: u16, w2: u16) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn ChrCmpIW(w1 : u16, w2 : u16) -> windows_core::BOOL);
    unsafe { ChrCmpIW(w1, w2) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn ColorAdjustLuma(clrrgb: super::windef::COLORREF, n: i32, fscale: bool) -> super::windef::COLORREF {
    windows_core::link!("shlwapi.dll" "system" fn ColorAdjustLuma(clrrgb : super::windef::COLORREF, n : i32, fscale : windows_core::BOOL) -> super::windef::COLORREF);
    unsafe { ColorAdjustLuma(clrrgb, n, fscale.into()) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn ColorHLSToRGB(whue: u16, wluminance: u16, wsaturation: u16) -> super::windef::COLORREF {
    windows_core::link!("shlwapi.dll" "system" fn ColorHLSToRGB(whue : u16, wluminance : u16, wsaturation : u16) -> super::windef::COLORREF);
    unsafe { ColorHLSToRGB(whue, wluminance, wsaturation) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn ColorRGBToHLS(clrrgb: super::windef::COLORREF, pwhue: *mut u16, pwluminance: *mut u16, pwsaturation: *mut u16) {
    windows_core::link!("shlwapi.dll" "system" fn ColorRGBToHLS(clrrgb : super::windef::COLORREF, pwhue : *mut u16, pwluminance : *mut u16, pwsaturation : *mut u16));
    unsafe { ColorRGBToHLS(clrrgb, pwhue as _, pwluminance as _, pwsaturation as _) }
}
#[cfg(feature = "Win32_ocidl")]
#[inline]
pub unsafe fn ConnectToConnectionPoint<P0, P3>(punk: P0, riidevent: *const windows_core::GUID, fconnect: bool, punktarget: P3, pdwcookie: *mut u32, ppcpout: *mut Option<super::ocidl::IConnectionPoint>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P3: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shlwapi.dll" "system" fn ConnectToConnectionPoint(punk : *mut core::ffi::c_void, riidevent : *const windows_core::GUID, fconnect : windows_core::BOOL, punktarget : *mut core::ffi::c_void, pdwcookie : *mut u32, ppcpout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ConnectToConnectionPoint(punk.param().abi(), riidevent, fconnect.into(), punktarget.param().abi(), pdwcookie as _, core::mem::transmute(ppcpout)) }
}
#[inline]
pub unsafe fn GetAcceptLanguagesA(pszlanguages: windows_core::PSTR, pcchlanguages: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn GetAcceptLanguagesA(pszlanguages : windows_core::PSTR, pcchlanguages : *mut u32) -> windows_core::HRESULT);
    unsafe { GetAcceptLanguagesA(core::mem::transmute(pszlanguages), pcchlanguages as _) }
}
#[inline]
pub unsafe fn GetAcceptLanguagesW(pszlanguages: windows_core::PWSTR, pcchlanguages: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn GetAcceptLanguagesW(pszlanguages : windows_core::PWSTR, pcchlanguages : *mut u32) -> windows_core::HRESULT);
    unsafe { GetAcceptLanguagesW(core::mem::transmute(pszlanguages), pcchlanguages as _) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn GetMenuPosFromID(hmenu: super::windef::HMENU, id: u32) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn GetMenuPosFromID(hmenu : super::windef::HMENU, id : u32) -> i32);
    unsafe { GetMenuPosFromID(hmenu, id) }
}
#[inline]
pub unsafe fn HashData(pbdata: &[u8], pbhash: &mut [u8]) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn HashData(pbdata : *const u8, cbdata : u32, pbhash : *mut u8, cbhash : u32) -> windows_core::HRESULT);
    unsafe { HashData(core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_Copy<P0, P1>(pstmfrom: P0, pstmto: P1, cb: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
    P1: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_Copy(pstmfrom : *mut core::ffi::c_void, pstmto : *mut core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { IStream_Copy(pstmfrom.param().abi(), pstmto.param().abi(), cb) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_Read<P0>(pstm: P0, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_Read(pstm : *mut core::ffi::c_void, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { IStream_Read(pstm.param().abi(), pv as _, cb) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_shtypes"))]
#[inline]
pub unsafe fn IStream_ReadPidl<P0>(pstm: P0) -> windows_core::Result<super::shtypes::LPITEMIDLIST>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_ReadPidl(pstm : *mut core::ffi::c_void, ppidlout : *mut super::shtypes::LPITEMIDLIST) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IStream_ReadPidl(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_ReadStr<P0>(pstm: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_ReadStr(pstm : *mut core::ffi::c_void, ppsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IStream_ReadStr(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_Reset<P0>(pstm: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_Reset(pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { IStream_Reset(pstm.param().abi()) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_Size<P0>(pstm: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_Size(pstm : *mut core::ffi::c_void, pui : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IStream_Size(pstm.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_Write<P0>(pstm: P0, pv: *const core::ffi::c_void, cb: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_Write(pstm : *mut core::ffi::c_void, pv : *const core::ffi::c_void, cb : u32) -> windows_core::HRESULT);
    unsafe { IStream_Write(pstm.param().abi(), pv, cb) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_shtypes"))]
#[inline]
pub unsafe fn IStream_WritePidl<P0>(pstm: P0, pidlwrite: *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_WritePidl(pstm : *mut core::ffi::c_void, pidlwrite : *const super::shtypes::ITEMIDLIST) -> windows_core::HRESULT);
    unsafe { IStream_WritePidl(pstm.param().abi(), pidlwrite) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn IStream_WriteStr<P0, P1>(pstm: P0, psz: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidlbase::IStream>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn IStream_WriteStr(pstm : *mut core::ffi::c_void, psz : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { IStream_WriteStr(pstm.param().abi(), psz.param().abi()) }
}
#[inline]
pub unsafe fn IUnknown_AtomicRelease(ppunk: Option<*mut *mut core::ffi::c_void>) {
    windows_core::link!("shlwapi.dll" "system" fn IUnknown_AtomicRelease(ppunk : *mut *mut core::ffi::c_void));
    unsafe { IUnknown_AtomicRelease(ppunk.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IUnknown_GetSite<P0>(punk: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shlwapi.dll" "system" fn IUnknown_GetSite(punk : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { IUnknown_GetSite(punk.param().abi(), riid, ppv as _) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn IUnknown_GetWindow<P0>(punk: P0) -> windows_core::Result<super::windef::HWND>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shlwapi.dll" "system" fn IUnknown_GetWindow(punk : *mut core::ffi::c_void, phwnd : *mut super::windef::HWND) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IUnknown_GetWindow(punk.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IUnknown_QueryService<P0, T>(punk: P0, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("shlwapi.dll" "system" fn IUnknown_QueryService(punk : *mut core::ffi::c_void, guidservice : *const windows_core::GUID, riid : *const windows_core::GUID, ppvout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { IUnknown_QueryService(punk.param().abi(), guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn IUnknown_Set<P1>(ppunk: *mut Option<windows_core::IUnknown>, punk: P1)
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shlwapi.dll" "system" fn IUnknown_Set(ppunk : *mut *mut core::ffi::c_void, punk : *mut core::ffi::c_void));
    unsafe { IUnknown_Set(core::mem::transmute(ppunk), punk.param().abi()) }
}
#[inline]
pub unsafe fn IUnknown_SetSite<P0, P1>(punk: P0, punksite: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shlwapi.dll" "system" fn IUnknown_SetSite(punk : *mut core::ffi::c_void, punksite : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { IUnknown_SetSite(punk.param().abi(), punksite.param().abi()) }
}
#[inline]
pub unsafe fn IntlStrEqWorkerA<P1, P2>(fcasesens: bool, lpstring1: P1, lpstring2: P2, nchar: i32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn IntlStrEqWorkerA(fcasesens : windows_core::BOOL, lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR, nchar : i32) -> windows_core::BOOL);
    unsafe { IntlStrEqWorkerA(fcasesens.into(), lpstring1.param().abi(), lpstring2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn IntlStrEqWorkerW<P1, P2>(fcasesens: bool, lpstring1: P1, lpstring2: P2, nchar: i32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn IntlStrEqWorkerW(fcasesens : windows_core::BOOL, lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR, nchar : i32) -> windows_core::BOOL);
    unsafe { IntlStrEqWorkerW(fcasesens.into(), lpstring1.param().abi(), lpstring2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn IsCharSpaceA(wch: i8) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn IsCharSpaceA(wch : i8) -> windows_core::BOOL);
    unsafe { IsCharSpaceA(wch) }
}
#[inline]
pub unsafe fn IsCharSpaceW(wch: u16) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn IsCharSpaceW(wch : u16) -> windows_core::BOOL);
    unsafe { IsCharSpaceW(wch) }
}
#[inline]
pub unsafe fn IsInternetESCEnabled() -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn IsInternetESCEnabled() -> windows_core::BOOL);
    unsafe { IsInternetESCEnabled() }
}
#[inline]
pub unsafe fn IsOS(dwos: u32) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn IsOS(dwos : u32) -> windows_core::BOOL);
    unsafe { IsOS(dwos) }
}
#[inline]
pub unsafe fn ParseURLA<P0>(pcszurl: P0, ppu: *mut PARSEDURLA) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn ParseURLA(pcszurl : windows_core::PCSTR, ppu : *mut PARSEDURLA) -> windows_core::HRESULT);
    unsafe { ParseURLA(pcszurl.param().abi(), ppu as _) }
}
#[inline]
pub unsafe fn ParseURLW<P0>(pcszurl: P0, ppu: *mut PARSEDURLW) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn ParseURLW(pcszurl : windows_core::PCWSTR, ppu : *mut PARSEDURLW) -> windows_core::HRESULT);
    unsafe { ParseURLW(pcszurl.param().abi(), ppu as _) }
}
#[inline]
pub unsafe fn PathAddBackslashA(pszpath: windows_core::PSTR) -> windows_core::PSTR {
    windows_core::link!("shlwapi.dll" "system" fn PathAddBackslashA(pszpath : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { PathAddBackslashA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathAddBackslashW(pszpath: windows_core::PWSTR) -> windows_core::PWSTR {
    windows_core::link!("shlwapi.dll" "system" fn PathAddBackslashW(pszpath : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { PathAddBackslashW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathAddExtensionA<P1>(pszpath: windows_core::PSTR, pszext: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathAddExtensionA(pszpath : windows_core::PSTR, pszext : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathAddExtensionA(core::mem::transmute(pszpath), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathAddExtensionW<P1>(pszpath: windows_core::PWSTR, pszext: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathAddExtensionW(pszpath : windows_core::PWSTR, pszext : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathAddExtensionW(core::mem::transmute(pszpath), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathAppendA<P1>(pszpath: windows_core::PSTR, pszmore: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathAppendA(pszpath : windows_core::PSTR, pszmore : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathAppendA(core::mem::transmute(pszpath), pszmore.param().abi()) }
}
#[inline]
pub unsafe fn PathAppendW<P1>(pszpath: windows_core::PWSTR, pszmore: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathAppendW(pszpath : windows_core::PWSTR, pszmore : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathAppendW(core::mem::transmute(pszpath), pszmore.param().abi()) }
}
#[inline]
pub unsafe fn PathBuildRootA(pszroot: &mut [u8; 4], idrive: i32) -> windows_core::PSTR {
    windows_core::link!("shlwapi.dll" "system" fn PathBuildRootA(pszroot : windows_core::PSTR, idrive : i32) -> windows_core::PSTR);
    unsafe { PathBuildRootA(core::mem::transmute(pszroot.as_ptr()), idrive) }
}
#[inline]
pub unsafe fn PathBuildRootW(pszroot: &mut [u16; 4], idrive: i32) -> windows_core::PWSTR {
    windows_core::link!("shlwapi.dll" "system" fn PathBuildRootW(pszroot : windows_core::PWSTR, idrive : i32) -> windows_core::PWSTR);
    unsafe { PathBuildRootW(core::mem::transmute(pszroot.as_ptr()), idrive) }
}
#[inline]
pub unsafe fn PathCanonicalizeA<P1>(pszbuf: windows_core::PSTR, pszpath: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCanonicalizeA(pszbuf : windows_core::PSTR, pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathCanonicalizeA(core::mem::transmute(pszbuf), pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathCanonicalizeW<P1>(pszbuf: windows_core::PWSTR, pszpath: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCanonicalizeW(pszbuf : windows_core::PWSTR, pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathCanonicalizeW(core::mem::transmute(pszbuf), pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathCombineA<P1, P2>(pszdest: windows_core::PSTR, pszdir: P1, pszfile: P2) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCombineA(pszdest : windows_core::PSTR, pszdir : windows_core::PCSTR, pszfile : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { PathCombineA(core::mem::transmute(pszdest), pszdir.param().abi(), pszfile.param().abi()) }
}
#[inline]
pub unsafe fn PathCombineW<P1, P2>(pszdest: windows_core::PWSTR, pszdir: P1, pszfile: P2) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCombineW(pszdest : windows_core::PWSTR, pszdir : windows_core::PCWSTR, pszfile : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { PathCombineW(core::mem::transmute(pszdest), pszdir.param().abi(), pszfile.param().abi()) }
}
#[inline]
pub unsafe fn PathCommonPrefixA<P0, P1>(pszfile1: P0, pszfile2: P1, achpath: Option<windows_core::PSTR>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCommonPrefixA(pszfile1 : windows_core::PCSTR, pszfile2 : windows_core::PCSTR, achpath : windows_core::PSTR) -> i32);
    unsafe { PathCommonPrefixA(pszfile1.param().abi(), pszfile2.param().abi(), achpath.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PathCommonPrefixW<P0, P1>(pszfile1: P0, pszfile2: P1, achpath: Option<windows_core::PWSTR>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCommonPrefixW(pszfile1 : windows_core::PCWSTR, pszfile2 : windows_core::PCWSTR, achpath : windows_core::PWSTR) -> i32);
    unsafe { PathCommonPrefixW(pszfile1.param().abi(), pszfile2.param().abi(), achpath.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn PathCompactPathA(hdc: Option<super::windef::HDC>, pszpath: windows_core::PSTR, dx: u32) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathCompactPathA(hdc : super::windef::HDC, pszpath : windows_core::PSTR, dx : u32) -> windows_core::BOOL);
    unsafe { PathCompactPathA(hdc.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszpath), dx) }
}
#[inline]
pub unsafe fn PathCompactPathExA<P1>(pszout: &mut [u8], pszsrc: P1, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCompactPathExA(pszout : windows_core::PSTR, pszsrc : windows_core::PCSTR, cchmax : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { PathCompactPathExA(core::mem::transmute(pszout.as_ptr()), pszsrc.param().abi(), pszout.len().try_into().unwrap(), dwflags) }
}
#[inline]
pub unsafe fn PathCompactPathExW<P1>(pszout: &mut [u16], pszsrc: P1, dwflags: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCompactPathExW(pszout : windows_core::PWSTR, pszsrc : windows_core::PCWSTR, cchmax : u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { PathCompactPathExW(core::mem::transmute(pszout.as_ptr()), pszsrc.param().abi(), pszout.len().try_into().unwrap(), dwflags) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn PathCompactPathW(hdc: Option<super::windef::HDC>, pszpath: windows_core::PWSTR, dx: u32) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathCompactPathW(hdc : super::windef::HDC, pszpath : windows_core::PWSTR, dx : u32) -> windows_core::BOOL);
    unsafe { PathCompactPathW(hdc.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszpath), dx) }
}
#[inline]
pub unsafe fn PathCreateFromUrlA<P0>(pszurl: P0, pszpath: windows_core::PSTR, pcchpath: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCreateFromUrlA(pszurl : windows_core::PCSTR, pszpath : windows_core::PSTR, pcchpath : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathCreateFromUrlA(pszurl.param().abi(), core::mem::transmute(pszpath), pcchpath as _, dwflags) }
}
#[inline]
pub unsafe fn PathCreateFromUrlAlloc<P0>(pszin: P0, ppszout: *mut windows_core::PWSTR, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCreateFromUrlAlloc(pszin : windows_core::PCWSTR, ppszout : *mut windows_core::PWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathCreateFromUrlAlloc(pszin.param().abi(), ppszout as _, dwflags) }
}
#[inline]
pub unsafe fn PathCreateFromUrlW<P0>(pszurl: P0, pszpath: windows_core::PWSTR, pcchpath: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathCreateFromUrlW(pszurl : windows_core::PCWSTR, pszpath : windows_core::PWSTR, pcchpath : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathCreateFromUrlW(pszurl.param().abi(), core::mem::transmute(pszpath), pcchpath as _, dwflags) }
}
#[inline]
pub unsafe fn PathFileExistsA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFileExistsA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathFileExistsA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFileExistsW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFileExistsW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathFileExistsW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindExtensionA<P0>(pszpath: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindExtensionA(pszpath : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { PathFindExtensionA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindExtensionW<P0>(pszpath: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindExtensionW(pszpath : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { PathFindExtensionW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindFileNameA<P0>(pszpath: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindFileNameA(pszpath : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { PathFindFileNameA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindFileNameW<P0>(pszpath: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindFileNameW(pszpath : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { PathFindFileNameW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindNextComponentA<P0>(pszpath: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindNextComponentA(pszpath : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { PathFindNextComponentA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindNextComponentW<P0>(pszpath: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindNextComponentW(pszpath : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { PathFindNextComponentW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathFindOnPathA(pszpath: windows_core::PSTR, ppszotherdirs: Option<*const windows_core::PCSTR>) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathFindOnPathA(pszpath : windows_core::PSTR, ppszotherdirs : *const windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathFindOnPathA(core::mem::transmute(pszpath), ppszotherdirs.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PathFindOnPathW(pszpath: windows_core::PWSTR, ppszotherdirs: Option<*const windows_core::PCWSTR>) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathFindOnPathW(pszpath : windows_core::PWSTR, ppszotherdirs : *const windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathFindOnPathW(core::mem::transmute(pszpath), ppszotherdirs.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PathFindSuffixArrayA<P0>(pszpath: P0, apszsuffix: &[windows_core::PCSTR]) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindSuffixArrayA(pszpath : windows_core::PCSTR, apszsuffix : *const windows_core::PCSTR, iarraysize : i32) -> windows_core::PCSTR);
    unsafe { PathFindSuffixArrayA(pszpath.param().abi(), core::mem::transmute(apszsuffix.as_ptr()), apszsuffix.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathFindSuffixArrayW<P0>(pszpath: P0, apszsuffix: &[windows_core::PCWSTR]) -> windows_core::PCWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathFindSuffixArrayW(pszpath : windows_core::PCWSTR, apszsuffix : *const windows_core::PCWSTR, iarraysize : i32) -> windows_core::PCWSTR);
    unsafe { PathFindSuffixArrayW(pszpath.param().abi(), core::mem::transmute(apszsuffix.as_ptr()), apszsuffix.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathGetArgsA<P0>(pszpath: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathGetArgsA(pszpath : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { PathGetArgsA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathGetArgsW<P0>(pszpath: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathGetArgsW(pszpath : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { PathGetArgsW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathGetCharTypeA(ch: u8) -> u32 {
    windows_core::link!("shlwapi.dll" "system" fn PathGetCharTypeA(ch : u8) -> u32);
    unsafe { PathGetCharTypeA(ch) }
}
#[inline]
pub unsafe fn PathGetCharTypeW(ch: u16) -> u32 {
    windows_core::link!("shlwapi.dll" "system" fn PathGetCharTypeW(ch : u16) -> u32);
    unsafe { PathGetCharTypeW(ch) }
}
#[inline]
pub unsafe fn PathGetDriveNumberA<P0>(pszpath: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathGetDriveNumberA(pszpath : windows_core::PCSTR) -> i32);
    unsafe { PathGetDriveNumberA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathGetDriveNumberW<P0>(pszpath: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathGetDriveNumberW(pszpath : windows_core::PCWSTR) -> i32);
    unsafe { PathGetDriveNumberW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsContentTypeA<P0, P1>(pszpath: P0, pszcontenttype: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsContentTypeA(pszpath : windows_core::PCSTR, pszcontenttype : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsContentTypeA(pszpath.param().abi(), pszcontenttype.param().abi()) }
}
#[inline]
pub unsafe fn PathIsContentTypeW<P0, P1>(pszpath: P0, pszcontenttype: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsContentTypeW(pszpath : windows_core::PCWSTR, pszcontenttype : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsContentTypeW(pszpath.param().abi(), pszcontenttype.param().abi()) }
}
#[inline]
pub unsafe fn PathIsDirectoryA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsDirectoryA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsDirectoryA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsDirectoryEmptyA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsDirectoryEmptyA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsDirectoryEmptyA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsDirectoryEmptyW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsDirectoryEmptyW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsDirectoryEmptyW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsDirectoryW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsDirectoryW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsDirectoryW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsFileSpecA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsFileSpecA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsFileSpecA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsFileSpecW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsFileSpecW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsFileSpecW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsLFNFileSpecA<P0>(pszname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsLFNFileSpecA(pszname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsLFNFileSpecA(pszname.param().abi()) }
}
#[inline]
pub unsafe fn PathIsLFNFileSpecW<P0>(pszname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsLFNFileSpecW(pszname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsLFNFileSpecW(pszname.param().abi()) }
}
#[inline]
pub unsafe fn PathIsNetworkPathA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsNetworkPathA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsNetworkPathA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsNetworkPathW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsNetworkPathW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsNetworkPathW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsPrefixA<P0, P1>(pszprefix: P0, pszpath: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsPrefixA(pszprefix : windows_core::PCSTR, pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsPrefixA(pszprefix.param().abi(), pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsPrefixW<P0, P1>(pszprefix: P0, pszpath: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsPrefixW(pszprefix : windows_core::PCWSTR, pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsPrefixW(pszprefix.param().abi(), pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsRelativeA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsRelativeA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsRelativeA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsRelativeW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsRelativeW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsRelativeW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsRootA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsRootA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsRootA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsRootW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsRootW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsRootW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsSameRootA<P0, P1>(pszpath1: P0, pszpath2: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsSameRootA(pszpath1 : windows_core::PCSTR, pszpath2 : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsSameRootA(pszpath1.param().abi(), pszpath2.param().abi()) }
}
#[inline]
pub unsafe fn PathIsSameRootW<P0, P1>(pszpath1: P0, pszpath2: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsSameRootW(pszpath1 : windows_core::PCWSTR, pszpath2 : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsSameRootW(pszpath1.param().abi(), pszpath2.param().abi()) }
}
#[inline]
pub unsafe fn PathIsSystemFolderA<P0>(pszpath: P0, dwattrb: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsSystemFolderA(pszpath : windows_core::PCSTR, dwattrb : u32) -> windows_core::BOOL);
    unsafe { PathIsSystemFolderA(pszpath.param().abi(), dwattrb) }
}
#[inline]
pub unsafe fn PathIsSystemFolderW<P0>(pszpath: P0, dwattrb: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsSystemFolderW(pszpath : windows_core::PCWSTR, dwattrb : u32) -> windows_core::BOOL);
    unsafe { PathIsSystemFolderW(pszpath.param().abi(), dwattrb) }
}
#[inline]
pub unsafe fn PathIsUNCA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsUNCA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsUNCServerA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsUNCServerA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCServerA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsUNCServerShareA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsUNCServerShareA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCServerShareA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsUNCServerShareW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsUNCServerShareW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCServerShareW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsUNCServerW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsUNCServerW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCServerW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsUNCW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsUNCW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsUNCW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsURLA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsURLA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathIsURLA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathIsURLW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathIsURLW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathIsURLW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathMakePrettyA(pszpath: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathMakePrettyA(pszpath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { PathMakePrettyA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathMakePrettyW(pszpath: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathMakePrettyW(pszpath : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { PathMakePrettyW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathMakeSystemFolderA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathMakeSystemFolderA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathMakeSystemFolderA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathMakeSystemFolderW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathMakeSystemFolderW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathMakeSystemFolderW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathMatchSpecA<P0, P1>(pszfile: P0, pszspec: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathMatchSpecA(pszfile : windows_core::PCSTR, pszspec : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathMatchSpecA(pszfile.param().abi(), pszspec.param().abi()) }
}
#[inline]
pub unsafe fn PathMatchSpecExA<P0, P1>(pszfile: P0, pszspec: P1, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathMatchSpecExA(pszfile : windows_core::PCSTR, pszspec : windows_core::PCSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathMatchSpecExA(pszfile.param().abi(), pszspec.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn PathMatchSpecExW<P0, P1>(pszfile: P0, pszspec: P1, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathMatchSpecExW(pszfile : windows_core::PCWSTR, pszspec : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { PathMatchSpecExW(pszfile.param().abi(), pszspec.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn PathMatchSpecW<P0, P1>(pszfile: P0, pszspec: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathMatchSpecW(pszfile : windows_core::PCWSTR, pszspec : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathMatchSpecW(pszfile.param().abi(), pszspec.param().abi()) }
}
#[inline]
pub unsafe fn PathParseIconLocationA(psziconfile: windows_core::PSTR) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn PathParseIconLocationA(psziconfile : windows_core::PSTR) -> i32);
    unsafe { PathParseIconLocationA(core::mem::transmute(psziconfile)) }
}
#[inline]
pub unsafe fn PathParseIconLocationW(psziconfile: windows_core::PWSTR) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn PathParseIconLocationW(psziconfile : windows_core::PWSTR) -> i32);
    unsafe { PathParseIconLocationW(core::mem::transmute(psziconfile)) }
}
#[inline]
pub unsafe fn PathQuoteSpacesA(lpsz: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathQuoteSpacesA(lpsz : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { PathQuoteSpacesA(core::mem::transmute(lpsz)) }
}
#[inline]
pub unsafe fn PathQuoteSpacesW(lpsz: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathQuoteSpacesW(lpsz : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { PathQuoteSpacesW(core::mem::transmute(lpsz)) }
}
#[inline]
pub unsafe fn PathRelativePathToA<P1, P3>(pszpath: windows_core::PSTR, pszfrom: P1, dwattrfrom: u32, pszto: P3, dwattrto: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathRelativePathToA(pszpath : windows_core::PSTR, pszfrom : windows_core::PCSTR, dwattrfrom : u32, pszto : windows_core::PCSTR, dwattrto : u32) -> windows_core::BOOL);
    unsafe { PathRelativePathToA(core::mem::transmute(pszpath), pszfrom.param().abi(), dwattrfrom, pszto.param().abi(), dwattrto) }
}
#[inline]
pub unsafe fn PathRelativePathToW<P1, P3>(pszpath: windows_core::PWSTR, pszfrom: P1, dwattrfrom: u32, pszto: P3, dwattrto: u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathRelativePathToW(pszpath : windows_core::PWSTR, pszfrom : windows_core::PCWSTR, dwattrfrom : u32, pszto : windows_core::PCWSTR, dwattrto : u32) -> windows_core::BOOL);
    unsafe { PathRelativePathToW(core::mem::transmute(pszpath), pszfrom.param().abi(), dwattrfrom, pszto.param().abi(), dwattrto) }
}
#[inline]
pub unsafe fn PathRemoveArgsA(pszpath: windows_core::PSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveArgsA(pszpath : windows_core::PSTR));
    unsafe { PathRemoveArgsA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveArgsW(pszpath: windows_core::PWSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveArgsW(pszpath : windows_core::PWSTR));
    unsafe { PathRemoveArgsW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveBackslashA(pszpath: windows_core::PSTR) -> windows_core::PSTR {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveBackslashA(pszpath : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { PathRemoveBackslashA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveBackslashW(pszpath: windows_core::PWSTR) -> windows_core::PWSTR {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveBackslashW(pszpath : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { PathRemoveBackslashW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveBlanksA(pszpath: windows_core::PSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveBlanksA(pszpath : windows_core::PSTR));
    unsafe { PathRemoveBlanksA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveBlanksW(pszpath: windows_core::PWSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveBlanksW(pszpath : windows_core::PWSTR));
    unsafe { PathRemoveBlanksW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveExtensionA(pszpath: windows_core::PSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveExtensionA(pszpath : windows_core::PSTR));
    unsafe { PathRemoveExtensionA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveExtensionW(pszpath: windows_core::PWSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveExtensionW(pszpath : windows_core::PWSTR));
    unsafe { PathRemoveExtensionW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveFileSpecA(pszpath: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveFileSpecA(pszpath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { PathRemoveFileSpecA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRemoveFileSpecW(pszpath: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathRemoveFileSpecW(pszpath : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { PathRemoveFileSpecW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathRenameExtensionA<P1>(pszpath: windows_core::PSTR, pszext: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathRenameExtensionA(pszpath : windows_core::PSTR, pszext : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathRenameExtensionA(core::mem::transmute(pszpath), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathRenameExtensionW<P1>(pszpath: windows_core::PWSTR, pszext: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathRenameExtensionW(pszpath : windows_core::PWSTR, pszext : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathRenameExtensionW(core::mem::transmute(pszpath), pszext.param().abi()) }
}
#[inline]
pub unsafe fn PathSearchAndQualifyA<P0>(pszpath: P0, pszbuf: &mut [u8]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathSearchAndQualifyA(pszpath : windows_core::PCSTR, pszbuf : windows_core::PSTR, cchbuf : u32) -> windows_core::BOOL);
    unsafe { PathSearchAndQualifyA(pszpath.param().abi(), core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathSearchAndQualifyW<P0>(pszpath: P0, pszbuf: &mut [u16]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathSearchAndQualifyW(pszpath : windows_core::PCWSTR, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::BOOL);
    unsafe { PathSearchAndQualifyW(pszpath.param().abi(), core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn PathSetDlgItemPathA<P2>(hdlg: super::windef::HWND, id: i32, pszpath: P2)
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathSetDlgItemPathA(hdlg : super::windef::HWND, id : i32, pszpath : windows_core::PCSTR));
    unsafe { PathSetDlgItemPathA(hdlg, id, pszpath.param().abi()) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn PathSetDlgItemPathW<P2>(hdlg: super::windef::HWND, id: i32, pszpath: P2)
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathSetDlgItemPathW(hdlg : super::windef::HWND, id : i32, pszpath : windows_core::PCWSTR));
    unsafe { PathSetDlgItemPathW(hdlg, id, pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathSkipRootA<P0>(pszpath: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathSkipRootA(pszpath : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { PathSkipRootA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathSkipRootW<P0>(pszpath: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathSkipRootW(pszpath : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { PathSkipRootW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathStripPathA(pszpath: windows_core::PSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathStripPathA(pszpath : windows_core::PSTR));
    unsafe { PathStripPathA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathStripPathW(pszpath: windows_core::PWSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathStripPathW(pszpath : windows_core::PWSTR));
    unsafe { PathStripPathW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathStripToRootA(pszpath: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathStripToRootA(pszpath : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { PathStripToRootA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathStripToRootW(pszpath: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathStripToRootW(pszpath : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { PathStripToRootW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathUnExpandEnvStringsA<P0>(pszpath: P0, pszbuf: &mut [u8]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathUnExpandEnvStringsA(pszpath : windows_core::PCSTR, pszbuf : windows_core::PSTR, cchbuf : u32) -> windows_core::BOOL);
    unsafe { PathUnExpandEnvStringsA(pszpath.param().abi(), core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathUnExpandEnvStringsW<P0>(pszpath: P0, pszbuf: &mut [u16]) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathUnExpandEnvStringsW(pszpath : windows_core::PCWSTR, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::BOOL);
    unsafe { PathUnExpandEnvStringsW(pszpath.param().abi(), core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn PathUndecorateA(pszpath: windows_core::PSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathUndecorateA(pszpath : windows_core::PSTR));
    unsafe { PathUndecorateA(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathUndecorateW(pszpath: windows_core::PWSTR) {
    windows_core::link!("shlwapi.dll" "system" fn PathUndecorateW(pszpath : windows_core::PWSTR));
    unsafe { PathUndecorateW(core::mem::transmute(pszpath)) }
}
#[inline]
pub unsafe fn PathUnmakeSystemFolderA<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathUnmakeSystemFolderA(pszpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { PathUnmakeSystemFolderA(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathUnmakeSystemFolderW<P0>(pszpath: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn PathUnmakeSystemFolderW(pszpath : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { PathUnmakeSystemFolderW(pszpath.param().abi()) }
}
#[inline]
pub unsafe fn PathUnquoteSpacesA(lpsz: windows_core::PSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathUnquoteSpacesA(lpsz : windows_core::PSTR) -> windows_core::BOOL);
    unsafe { PathUnquoteSpacesA(core::mem::transmute(lpsz)) }
}
#[inline]
pub unsafe fn PathUnquoteSpacesW(lpsz: windows_core::PWSTR) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn PathUnquoteSpacesW(lpsz : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { PathUnquoteSpacesW(core::mem::transmute(lpsz)) }
}
#[inline]
pub unsafe fn QISearch<T>(that: *mut core::ffi::c_void, pqit: *const QITAB) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("shlwapi.dll" "system" fn QISearch(that : *mut core::ffi::c_void, pqit : *const QITAB, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { QISearch(that as _, pqit, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SHAllocShared(pvdata: Option<*const core::ffi::c_void>, dwsize: u32, dwprocessid: u32) -> super::winnt::HANDLE {
    windows_core::link!("shlwapi.dll" "system" fn SHAllocShared(pvdata : *const core::ffi::c_void, dwsize : u32, dwprocessid : u32) -> super::winnt::HANDLE);
    unsafe { SHAllocShared(pvdata.unwrap_or(core::mem::zeroed()) as _, dwsize, dwprocessid) }
}
#[inline]
pub unsafe fn SHAnsiToAnsi<P0>(pszsrc: P0, pszdst: &mut [u8]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHAnsiToAnsi(pszsrc : windows_core::PCSTR, pszdst : windows_core::PSTR, cchbuf : i32) -> i32);
    unsafe { SHAnsiToAnsi(pszsrc.param().abi(), core::mem::transmute(pszdst.as_ptr()), pszdst.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SHAnsiToUnicode<P0>(pszsrc: P0, pwszdst: &mut [u16]) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHAnsiToUnicode(pszsrc : windows_core::PCSTR, pwszdst : windows_core::PWSTR, cwchbuf : i32) -> i32);
    unsafe { SHAnsiToUnicode(pszsrc.param().abi(), core::mem::transmute(pwszdst.as_ptr()), pwszdst.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn SHAutoComplete(hwndedit: super::windef::HWND, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn SHAutoComplete(hwndedit : super::windef::HWND, dwflags : u32) -> windows_core::HRESULT);
    unsafe { SHAutoComplete(hwndedit, dwflags) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHCopyKeyA<P1>(hkeysrc: super::minwindef::HKEY, pszsrcsubkey: P1, hkeydest: super::minwindef::HKEY, freserved: Option<u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHCopyKeyA(hkeysrc : super::minwindef::HKEY, pszsrcsubkey : windows_core::PCSTR, hkeydest : super::minwindef::HKEY, freserved : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHCopyKeyA(hkeysrc, pszsrcsubkey.param().abi(), hkeydest, freserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHCopyKeyW<P1>(hkeysrc: super::minwindef::HKEY, pszsrcsubkey: P1, hkeydest: super::minwindef::HKEY, freserved: Option<u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHCopyKeyW(hkeysrc : super::minwindef::HKEY, pszsrcsubkey : windows_core::PCWSTR, hkeydest : super::minwindef::HKEY, freserved : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHCopyKeyW(hkeysrc, pszsrcsubkey.param().abi(), hkeydest, freserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn SHCreateMemStream(pinit: Option<&[u8]>) -> Option<super::objidlbase::IStream> {
    windows_core::link!("shlwapi.dll" "system" fn SHCreateMemStream(pinit : *const u8, cbinit : u32) -> Option < super::objidlbase::IStream >);
    unsafe { SHCreateMemStream(core::mem::transmute(pinit.map_or(core::ptr::null(), |slice| slice.as_ptr())), pinit.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn SHCreateShellPalette(hdc: Option<super::windef::HDC>) -> super::windef::HPALETTE {
    windows_core::link!("shlwapi.dll" "system" fn SHCreateShellPalette(hdc : super::windef::HDC) -> super::windef::HPALETTE);
    unsafe { SHCreateShellPalette(hdc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn SHCreateStreamOnFileA<P0>(pszfile: P0, grfmode: u32) -> windows_core::Result<super::objidlbase::IStream>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHCreateStreamOnFileA(pszfile : windows_core::PCSTR, grfmode : u32, ppstm : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateStreamOnFileA(pszfile.param().abi(), grfmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn SHCreateStreamOnFileEx<P0, P4>(pszfile: P0, grfmode: u32, dwattributes: u32, fcreate: bool, pstmtemplate: P4) -> windows_core::Result<super::objidlbase::IStream>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHCreateStreamOnFileEx(pszfile : windows_core::PCWSTR, grfmode : u32, dwattributes : u32, fcreate : windows_core::BOOL, pstmtemplate : *mut core::ffi::c_void, ppstm : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateStreamOnFileEx(pszfile.param().abi(), grfmode, dwattributes, fcreate.into(), pstmtemplate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn SHCreateStreamOnFileW<P0>(pszfile: P0, grfmode: u32) -> windows_core::Result<super::objidlbase::IStream>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHCreateStreamOnFileW(pszfile : windows_core::PCWSTR, grfmode : u32, ppstm : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHCreateStreamOnFileW(pszfile.param().abi(), grfmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_minwinbase")]
#[inline]
pub unsafe fn SHCreateThread(pfnthreadproc: super::minwinbase::LPTHREAD_START_ROUTINE, pdata: Option<*const core::ffi::c_void>, flags: SHCT_FLAGS, pfncallback: Option<super::minwinbase::LPTHREAD_START_ROUTINE>) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn SHCreateThread(pfnthreadproc : super::minwinbase::LPTHREAD_START_ROUTINE, pdata : *const core::ffi::c_void, flags : SHCT_FLAGS, pfncallback : super::minwinbase::LPTHREAD_START_ROUTINE) -> windows_core::BOOL);
    unsafe { SHCreateThread(pfnthreadproc, pdata.unwrap_or(core::mem::zeroed()) as _, flags, pfncallback.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SHCreateThreadRef(pcref: *mut i32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn SHCreateThreadRef(pcref : *mut i32, ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHCreateThreadRef(pcref as _, core::mem::transmute(ppunk)) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SHCreateThreadWithHandle(pfnthreadproc: super::minwinbase::LPTHREAD_START_ROUTINE, pdata: Option<*const core::ffi::c_void>, flags: SHCT_FLAGS, pfncallback: Option<super::minwinbase::LPTHREAD_START_ROUTINE>, phandle: Option<*mut super::winnt::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn SHCreateThreadWithHandle(pfnthreadproc : super::minwinbase::LPTHREAD_START_ROUTINE, pdata : *const core::ffi::c_void, flags : SHCT_FLAGS, pfncallback : super::minwinbase::LPTHREAD_START_ROUTINE, phandle : *mut super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { SHCreateThreadWithHandle(pfnthreadproc, pdata.unwrap_or(core::mem::zeroed()) as _, flags, pfncallback.unwrap_or(core::mem::zeroed()) as _, phandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHDeleteEmptyKeyA<P1>(hkey: super::minwindef::HKEY, pszsubkey: P1) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHDeleteEmptyKeyA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR) -> super::winreg::WIN32_ERROR);
    unsafe { SHDeleteEmptyKeyA(hkey, pszsubkey.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHDeleteEmptyKeyW<P1>(hkey: super::minwindef::HKEY, pszsubkey: P1) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHDeleteEmptyKeyW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR) -> super::winreg::WIN32_ERROR);
    unsafe { SHDeleteEmptyKeyW(hkey, pszsubkey.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHDeleteKeyA<P1>(hkey: super::minwindef::HKEY, pszsubkey: P1) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHDeleteKeyA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR) -> super::winreg::WIN32_ERROR);
    unsafe { SHDeleteKeyA(hkey, pszsubkey.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHDeleteKeyW<P1>(hkey: super::minwindef::HKEY, pszsubkey: P1) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHDeleteKeyW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR) -> super::winreg::WIN32_ERROR);
    unsafe { SHDeleteKeyW(hkey, pszsubkey.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHDeleteValueA<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHDeleteValueA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR) -> super::winreg::WIN32_ERROR);
    unsafe { SHDeleteValueA(hkey, pszsubkey.param().abi(), pszvalue.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHDeleteValueW<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHDeleteValueW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR) -> super::winreg::WIN32_ERROR);
    unsafe { SHDeleteValueW(hkey, pszsubkey.param().abi(), pszvalue.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHEnumKeyExA(hkey: super::minwindef::HKEY, dwindex: u32, pszname: windows_core::PSTR, pcchname: *mut u32) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHEnumKeyExA(hkey : super::minwindef::HKEY, dwindex : u32, pszname : windows_core::PSTR, pcchname : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHEnumKeyExA(hkey, dwindex, core::mem::transmute(pszname), pcchname as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHEnumKeyExW(hkey: super::minwindef::HKEY, dwindex: u32, pszname: windows_core::PWSTR, pcchname: *mut u32) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHEnumKeyExW(hkey : super::minwindef::HKEY, dwindex : u32, pszname : windows_core::PWSTR, pcchname : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHEnumKeyExW(hkey, dwindex, core::mem::transmute(pszname), pcchname as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHEnumValueA(hkey: super::minwindef::HKEY, dwindex: u32, pszvaluename: Option<windows_core::PSTR>, pcchvaluename: Option<*mut u32>, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHEnumValueA(hkey : super::minwindef::HKEY, dwindex : u32, pszvaluename : windows_core::PSTR, pcchvaluename : *mut u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHEnumValueA(hkey, dwindex, pszvaluename.unwrap_or(core::mem::zeroed()) as _, pcchvaluename.unwrap_or(core::mem::zeroed()) as _, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHEnumValueW(hkey: super::minwindef::HKEY, dwindex: u32, pszvaluename: Option<windows_core::PWSTR>, pcchvaluename: Option<*mut u32>, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHEnumValueW(hkey : super::minwindef::HKEY, dwindex : u32, pszvaluename : windows_core::PWSTR, pcchvaluename : *mut u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHEnumValueW(hkey, dwindex, pszvaluename.unwrap_or(core::mem::zeroed()) as _, pcchvaluename.unwrap_or(core::mem::zeroed()) as _, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn SHFormatDateTimeA(pft: *const super::minwindef::FILETIME, pdwflags: Option<*mut u32>, pszbuf: &mut [u8]) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn SHFormatDateTimeA(pft : *const super::minwindef::FILETIME, pdwflags : *mut u32, pszbuf : windows_core::PSTR, cchbuf : u32) -> i32);
    unsafe { SHFormatDateTimeA(pft, pdwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn SHFormatDateTimeW(pft: *const super::minwindef::FILETIME, pdwflags: Option<*mut u32>, pszbuf: &mut [u16]) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn SHFormatDateTimeW(pft : *const super::minwindef::FILETIME, pdwflags : *mut u32, pszbuf : windows_core::PWSTR, cchbuf : u32) -> i32);
    unsafe { SHFormatDateTimeW(pft, pdwflags.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SHFreeShared(hdata: super::winnt::HANDLE, dwprocessid: u32) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn SHFreeShared(hdata : super::winnt::HANDLE, dwprocessid : u32) -> windows_core::BOOL);
    unsafe { SHFreeShared(hdata, dwprocessid) }
}
#[inline]
pub unsafe fn SHGetInverseCMAP(pbmap: &mut [u8]) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn SHGetInverseCMAP(pbmap : *mut u8, cbmap : u32) -> windows_core::HRESULT);
    unsafe { SHGetInverseCMAP(core::mem::transmute(pbmap.as_ptr()), pbmap.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SHGetThreadRef() -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("shlwapi.dll" "system" fn SHGetThreadRef(ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetThreadRef(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHGetValueA<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHGetValueA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHGetValueA(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHGetValueW<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHGetValueW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHGetValueW(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn SHGetViewStatePropertyBag<P1>(pidl: Option<*const super::shtypes::ITEMIDLIST>, pszbagname: P1, dwflags: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHGetViewStatePropertyBag(pidl : *const super::shtypes::ITEMIDLIST, pszbagname : windows_core::PCWSTR, dwflags : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHGetViewStatePropertyBag(pidl.unwrap_or(core::mem::zeroed()) as _, pszbagname.param().abi(), dwflags, riid, ppv as _) }
}
#[inline]
pub unsafe fn SHGlobalCounterDecrement(id: SHGLOBALCOUNTER) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn SHGlobalCounterDecrement(id : SHGLOBALCOUNTER) -> i32);
    unsafe { SHGlobalCounterDecrement(id) }
}
#[inline]
pub unsafe fn SHGlobalCounterGetValue(id: SHGLOBALCOUNTER) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn SHGlobalCounterGetValue(id : SHGLOBALCOUNTER) -> i32);
    unsafe { SHGlobalCounterGetValue(id) }
}
#[inline]
pub unsafe fn SHGlobalCounterIncrement(id: SHGLOBALCOUNTER) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn SHGlobalCounterIncrement(id : SHGLOBALCOUNTER) -> i32);
    unsafe { SHGlobalCounterIncrement(id) }
}
#[inline]
pub unsafe fn SHIsLowMemoryMachine(dwtype: u32) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn SHIsLowMemoryMachine(dwtype : u32) -> windows_core::BOOL);
    unsafe { SHIsLowMemoryMachine(dwtype) }
}
#[inline]
pub unsafe fn SHLoadIndirectString<P0>(pszsource: P0, pszoutbuf: &mut [u16], ppvreserved: Option<*const *const core::ffi::c_void>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHLoadIndirectString(pszsource : windows_core::PCWSTR, pszoutbuf : windows_core::PWSTR, cchoutbuf : u32, ppvreserved : *const *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHLoadIndirectString(pszsource.param().abi(), core::mem::transmute(pszoutbuf.as_ptr()), pszoutbuf.len().try_into().unwrap(), ppvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SHLockShared(hdata: super::winnt::HANDLE, dwprocessid: u32) -> *mut core::ffi::c_void {
    windows_core::link!("shlwapi.dll" "system" fn SHLockShared(hdata : super::winnt::HANDLE, dwprocessid : u32) -> *mut core::ffi::c_void);
    unsafe { SHLockShared(hdata, dwprocessid) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn SHMessageBoxCheckA<P1, P2, P5>(hwnd: Option<super::windef::HWND>, psztext: P1, pszcaption: P2, utype: u32, idefault: i32, pszregval: P5) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHMessageBoxCheckA(hwnd : super::windef::HWND, psztext : windows_core::PCSTR, pszcaption : windows_core::PCSTR, utype : u32, idefault : i32, pszregval : windows_core::PCSTR) -> i32);
    unsafe { SHMessageBoxCheckA(hwnd.unwrap_or(core::mem::zeroed()) as _, psztext.param().abi(), pszcaption.param().abi(), utype, idefault, pszregval.param().abi()) }
}
#[cfg(feature = "Win32_windef")]
#[inline]
pub unsafe fn SHMessageBoxCheckW<P1, P2, P5>(hwnd: Option<super::windef::HWND>, psztext: P1, pszcaption: P2, utype: u32, idefault: i32, pszregval: P5) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHMessageBoxCheckW(hwnd : super::windef::HWND, psztext : windows_core::PCWSTR, pszcaption : windows_core::PCWSTR, utype : u32, idefault : i32, pszregval : windows_core::PCWSTR) -> i32);
    unsafe { SHMessageBoxCheckW(hwnd.unwrap_or(core::mem::zeroed()) as _, psztext.param().abi(), pszcaption.param().abi(), utype, idefault, pszregval.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
#[inline]
pub unsafe fn SHOpenRegStream2A<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, grfmode: u32) -> Option<super::objidlbase::IStream>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHOpenRegStream2A(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, grfmode : u32) -> Option < super::objidlbase::IStream >);
    unsafe { SHOpenRegStream2A(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), grfmode) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
#[inline]
pub unsafe fn SHOpenRegStream2W<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, grfmode: u32) -> Option<super::objidlbase::IStream>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHOpenRegStream2W(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, grfmode : u32) -> Option < super::objidlbase::IStream >);
    unsafe { SHOpenRegStream2W(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), grfmode) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
#[inline]
pub unsafe fn SHOpenRegStreamA<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, grfmode: u32) -> Option<super::objidlbase::IStream>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHOpenRegStreamA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, grfmode : u32) -> Option < super::objidlbase::IStream >);
    unsafe { SHOpenRegStreamA(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), grfmode) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_objidlbase"))]
#[inline]
pub unsafe fn SHOpenRegStreamW<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, grfmode: u32) -> Option<super::objidlbase::IStream>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHOpenRegStreamW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, grfmode : u32) -> Option < super::objidlbase::IStream >);
    unsafe { SHOpenRegStreamW(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), grfmode) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHQueryInfoKeyA(hkey: super::minwindef::HKEY, pcsubkeys: Option<*mut u32>, pcchmaxsubkeylen: Option<*mut u32>, pcvalues: Option<*mut u32>, pcchmaxvaluenamelen: Option<*mut u32>) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHQueryInfoKeyA(hkey : super::minwindef::HKEY, pcsubkeys : *mut u32, pcchmaxsubkeylen : *mut u32, pcvalues : *mut u32, pcchmaxvaluenamelen : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHQueryInfoKeyA(hkey, pcsubkeys.unwrap_or(core::mem::zeroed()) as _, pcchmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _, pcvalues.unwrap_or(core::mem::zeroed()) as _, pcchmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHQueryInfoKeyW(hkey: super::minwindef::HKEY, pcsubkeys: Option<*mut u32>, pcchmaxsubkeylen: Option<*mut u32>, pcvalues: Option<*mut u32>, pcchmaxvaluenamelen: Option<*mut u32>) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHQueryInfoKeyW(hkey : super::minwindef::HKEY, pcsubkeys : *mut u32, pcchmaxsubkeylen : *mut u32, pcvalues : *mut u32, pcchmaxvaluenamelen : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHQueryInfoKeyW(hkey, pcsubkeys.unwrap_or(core::mem::zeroed()) as _, pcchmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _, pcvalues.unwrap_or(core::mem::zeroed()) as _, pcchmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHQueryValueExA<P1>(hkey: super::minwindef::HKEY, pszvalue: P1, pdwreserved: Option<*const u32>, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHQueryValueExA(hkey : super::minwindef::HKEY, pszvalue : windows_core::PCSTR, pdwreserved : *const u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHQueryValueExA(hkey, pszvalue.param().abi(), pdwreserved.unwrap_or(core::mem::zeroed()) as _, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHQueryValueExW<P1>(hkey: super::minwindef::HKEY, pszvalue: P1, pdwreserved: Option<*const u32>, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHQueryValueExW(hkey : super::minwindef::HKEY, pszvalue : windows_core::PCWSTR, pdwreserved : *const u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHQueryValueExW(hkey, pszvalue.param().abi(), pdwreserved.unwrap_or(core::mem::zeroed()) as _, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegCloseUSKey(huskey: HUSKEY) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegCloseUSKey(huskey : HUSKEY) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegCloseUSKey(huskey) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegCreateUSKeyA<P0>(pszpath: P0, samdesired: super::winreg::REGSAM, hrelativeuskey: Option<HUSKEY>, phnewuskey: *mut HUSKEY, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegCreateUSKeyA(pszpath : windows_core::PCSTR, samdesired : super::winreg::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : *mut HUSKEY, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegCreateUSKeyA(pszpath.param().abi(), samdesired, hrelativeuskey.unwrap_or(core::mem::zeroed()) as _, phnewuskey as _, dwflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegCreateUSKeyW<P0>(pwzpath: P0, samdesired: super::winreg::REGSAM, hrelativeuskey: Option<HUSKEY>, phnewuskey: *mut HUSKEY, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegCreateUSKeyW(pwzpath : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : *mut HUSKEY, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegCreateUSKeyW(pwzpath.param().abi(), samdesired, hrelativeuskey.unwrap_or(core::mem::zeroed()) as _, phnewuskey as _, dwflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegDeleteEmptyUSKeyA<P1>(huskey: HUSKEY, pszsubkey: P1, delregflags: SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegDeleteEmptyUSKeyA(huskey : HUSKEY, pszsubkey : windows_core::PCSTR, delregflags : SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegDeleteEmptyUSKeyA(huskey, pszsubkey.param().abi(), delregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegDeleteEmptyUSKeyW<P1>(huskey: HUSKEY, pwzsubkey: P1, delregflags: SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegDeleteEmptyUSKeyW(huskey : HUSKEY, pwzsubkey : windows_core::PCWSTR, delregflags : SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegDeleteEmptyUSKeyW(huskey, pwzsubkey.param().abi(), delregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegDeleteUSValueA<P1>(huskey: HUSKEY, pszvalue: P1, delregflags: SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegDeleteUSValueA(huskey : HUSKEY, pszvalue : windows_core::PCSTR, delregflags : SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegDeleteUSValueA(huskey, pszvalue.param().abi(), delregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegDeleteUSValueW<P1>(huskey: HUSKEY, pwzvalue: P1, delregflags: SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegDeleteUSValueW(huskey : HUSKEY, pwzvalue : windows_core::PCWSTR, delregflags : SHREGDEL_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegDeleteUSValueW(huskey, pwzvalue.param().abi(), delregflags) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn SHRegDuplicateHKey(hkey: super::minwindef::HKEY) -> super::minwindef::HKEY {
    windows_core::link!("shlwapi.dll" "system" fn SHRegDuplicateHKey(hkey : super::minwindef::HKEY) -> super::minwindef::HKEY);
    unsafe { SHRegDuplicateHKey(hkey) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegEnumUSKeyA(huskey: HUSKEY, dwindex: u32, pszname: windows_core::PSTR, pcchname: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegEnumUSKeyA(huskey : HUSKEY, dwindex : u32, pszname : windows_core::PSTR, pcchname : *mut u32, enumregflags : SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegEnumUSKeyA(huskey, dwindex, core::mem::transmute(pszname), pcchname as _, enumregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegEnumUSKeyW(huskey: HUSKEY, dwindex: u32, pwzname: windows_core::PWSTR, pcchname: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegEnumUSKeyW(huskey : HUSKEY, dwindex : u32, pwzname : windows_core::PWSTR, pcchname : *mut u32, enumregflags : SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegEnumUSKeyW(huskey, dwindex, core::mem::transmute(pwzname), pcchname as _, enumregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegEnumUSValueA(huskey: HUSKEY, dwindex: u32, pszvaluename: windows_core::PSTR, pcchvaluename: *mut u32, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>, enumregflags: SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegEnumUSValueA(huskey : HUSKEY, dwindex : u32, pszvaluename : windows_core::PSTR, pcchvaluename : *mut u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32, enumregflags : SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegEnumUSValueA(huskey, dwindex, core::mem::transmute(pszvaluename), pcchvaluename as _, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _, enumregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegEnumUSValueW(huskey: HUSKEY, dwindex: u32, pszvaluename: windows_core::PWSTR, pcchvaluename: *mut u32, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>, enumregflags: SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegEnumUSValueW(huskey : HUSKEY, dwindex : u32, pszvaluename : windows_core::PWSTR, pcchvaluename : *mut u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32, enumregflags : SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegEnumUSValueW(huskey, dwindex, core::mem::transmute(pszvaluename), pcchvaluename as _, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _, enumregflags) }
}
#[inline]
pub unsafe fn SHRegGetBoolUSValueA<P0, P1>(pszsubkey: P0, pszvalue: P1, fignorehkcu: bool, fdefault: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetBoolUSValueA(pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, fignorehkcu : windows_core::BOOL, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SHRegGetBoolUSValueA(pszsubkey.param().abi(), pszvalue.param().abi(), fignorehkcu.into(), fdefault.into()) }
}
#[inline]
pub unsafe fn SHRegGetBoolUSValueW<P0, P1>(pszsubkey: P0, pszvalue: P1, fignorehkcu: bool, fdefault: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetBoolUSValueW(pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, fignorehkcu : windows_core::BOOL, fdefault : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SHRegGetBoolUSValueW(pszsubkey.param().abi(), pszvalue.param().abi(), fignorehkcu.into(), fdefault.into()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn SHRegGetIntW<P1>(hk: super::minwindef::HKEY, pwzkey: P1, idefault: i32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetIntW(hk : super::minwindef::HKEY, pwzkey : windows_core::PCWSTR, idefault : i32) -> i32);
    unsafe { SHRegGetIntW(hk, pwzkey.param().abi(), idefault) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegGetPathA<P1, P2>(hkey: super::minwindef::HKEY, pcszsubkey: P1, pcszvalue: P2, pszpath: windows_core::PSTR, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetPathA(hkey : super::minwindef::HKEY, pcszsubkey : windows_core::PCSTR, pcszvalue : windows_core::PCSTR, pszpath : windows_core::PSTR, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetPathA(hkey, pcszsubkey.param().abi(), pcszvalue.param().abi(), core::mem::transmute(pszpath), dwflags) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegGetPathW<P1, P2>(hkey: super::minwindef::HKEY, pcszsubkey: P1, pcszvalue: P2, pszpath: windows_core::PWSTR, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetPathW(hkey : super::minwindef::HKEY, pcszsubkey : windows_core::PCWSTR, pcszvalue : windows_core::PCWSTR, pszpath : windows_core::PWSTR, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetPathW(hkey, pcszsubkey.param().abi(), pcszvalue.param().abi(), core::mem::transmute(pszpath), dwflags) }
}
#[cfg(feature = "Win32_winreg")]
#[inline]
pub unsafe fn SHRegGetUSValueA<P0, P1>(pszsubkey: P0, pszvalue: P1, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>, fignorehkcu: bool, pvdefaultdata: Option<*const core::ffi::c_void>, dwdefaultdatasize: u32) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetUSValueA(pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32, fignorehkcu : windows_core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetUSValueA(pszsubkey.param().abi(), pszvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _, fignorehkcu.into(), pvdefaultdata.unwrap_or(core::mem::zeroed()) as _, dwdefaultdatasize) }
}
#[cfg(feature = "Win32_winreg")]
#[inline]
pub unsafe fn SHRegGetUSValueW<P0, P1>(pszsubkey: P0, pszvalue: P1, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>, fignorehkcu: bool, pvdefaultdata: Option<*const core::ffi::c_void>, dwdefaultdatasize: u32) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetUSValueW(pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32, fignorehkcu : windows_core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetUSValueW(pszsubkey.param().abi(), pszvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _, fignorehkcu.into(), pvdefaultdata.unwrap_or(core::mem::zeroed()) as _, dwdefaultdatasize) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegGetValueA<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, srrfflags: SRRF, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetValueA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, srrfflags : SRRF, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetValueA(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), srrfflags, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winreg")]
#[inline]
pub unsafe fn SHRegGetValueFromHKCUHKLM<P0, P1>(pwszkey: P0, pwszvalue: P1, srrfflags: SRRF, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetValueFromHKCUHKLM(pwszkey : windows_core::PCWSTR, pwszvalue : windows_core::PCWSTR, srrfflags : SRRF, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetValueFromHKCUHKLM(pwszkey.param().abi(), pwszvalue.param().abi(), srrfflags, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegGetValueW<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, srrfflags: SRRF, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegGetValueW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, srrfflags : SRRF, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegGetValueW(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), srrfflags, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegOpenUSKeyA<P0>(pszpath: P0, samdesired: super::winreg::REGSAM, hrelativeuskey: Option<HUSKEY>, phnewuskey: *mut HUSKEY, fignorehkcu: bool) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegOpenUSKeyA(pszpath : windows_core::PCSTR, samdesired : super::winreg::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : *mut HUSKEY, fignorehkcu : windows_core::BOOL) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegOpenUSKeyA(pszpath.param().abi(), samdesired, hrelativeuskey.unwrap_or(core::mem::zeroed()) as _, phnewuskey as _, fignorehkcu.into()) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegOpenUSKeyW<P0>(pwzpath: P0, samdesired: super::winreg::REGSAM, hrelativeuskey: Option<HUSKEY>, phnewuskey: *mut HUSKEY, fignorehkcu: bool) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegOpenUSKeyW(pwzpath : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, hrelativeuskey : HUSKEY, phnewuskey : *mut HUSKEY, fignorehkcu : windows_core::BOOL) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegOpenUSKeyW(pwzpath.param().abi(), samdesired, hrelativeuskey.unwrap_or(core::mem::zeroed()) as _, phnewuskey as _, fignorehkcu.into()) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegQueryInfoUSKeyA(huskey: HUSKEY, pcsubkeys: Option<*mut u32>, pcchmaxsubkeylen: Option<*mut u32>, pcvalues: Option<*mut u32>, pcchmaxvaluenamelen: Option<*mut u32>, enumregflags: SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegQueryInfoUSKeyA(huskey : HUSKEY, pcsubkeys : *mut u32, pcchmaxsubkeylen : *mut u32, pcvalues : *mut u32, pcchmaxvaluenamelen : *mut u32, enumregflags : SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegQueryInfoUSKeyA(huskey, pcsubkeys.unwrap_or(core::mem::zeroed()) as _, pcchmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _, pcvalues.unwrap_or(core::mem::zeroed()) as _, pcchmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _, enumregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegQueryInfoUSKeyW(huskey: HUSKEY, pcsubkeys: Option<*mut u32>, pcchmaxsubkeylen: Option<*mut u32>, pcvalues: Option<*mut u32>, pcchmaxvaluenamelen: Option<*mut u32>, enumregflags: SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR {
    windows_core::link!("shlwapi.dll" "system" fn SHRegQueryInfoUSKeyW(huskey : HUSKEY, pcsubkeys : *mut u32, pcchmaxsubkeylen : *mut u32, pcvalues : *mut u32, pcchmaxvaluenamelen : *mut u32, enumregflags : SHREGENUM_FLAGS) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegQueryInfoUSKeyW(huskey, pcsubkeys.unwrap_or(core::mem::zeroed()) as _, pcchmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _, pcvalues.unwrap_or(core::mem::zeroed()) as _, pcchmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _, enumregflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegQueryUSValueA<P1>(huskey: HUSKEY, pszvalue: P1, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>, fignorehkcu: bool, pvdefaultdata: Option<*const core::ffi::c_void>, dwdefaultdatasize: Option<u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegQueryUSValueA(huskey : HUSKEY, pszvalue : windows_core::PCSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32, fignorehkcu : windows_core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegQueryUSValueA(huskey, pszvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _, fignorehkcu.into(), pvdefaultdata.unwrap_or(core::mem::zeroed()) as _, dwdefaultdatasize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegQueryUSValueW<P1>(huskey: HUSKEY, pszvalue: P1, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>, fignorehkcu: bool, pvdefaultdata: Option<*const core::ffi::c_void>, dwdefaultdatasize: Option<u32>) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegQueryUSValueW(huskey : HUSKEY, pszvalue : windows_core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32, fignorehkcu : windows_core::BOOL, pvdefaultdata : *const core::ffi::c_void, dwdefaultdatasize : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegQueryUSValueW(huskey, pszvalue.param().abi(), pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _, fignorehkcu.into(), pvdefaultdata.unwrap_or(core::mem::zeroed()) as _, dwdefaultdatasize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegSetPathA<P1, P2, P3>(hkey: super::minwindef::HKEY, pcszsubkey: P1, pcszvalue: P2, pcszpath: P3, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegSetPathA(hkey : super::minwindef::HKEY, pcszsubkey : windows_core::PCSTR, pcszvalue : windows_core::PCSTR, pcszpath : windows_core::PCSTR, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegSetPathA(hkey, pcszsubkey.param().abi(), pcszvalue.param().abi(), pcszpath.param().abi(), dwflags) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegSetPathW<P1, P2, P3>(hkey: super::minwindef::HKEY, pcszsubkey: P1, pcszvalue: P2, pcszpath: P3, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegSetPathW(hkey : super::minwindef::HKEY, pcszsubkey : windows_core::PCWSTR, pcszvalue : windows_core::PCWSTR, pcszpath : windows_core::PCWSTR, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegSetPathW(hkey, pcszsubkey.param().abi(), pcszvalue.param().abi(), pcszpath.param().abi(), dwflags) }
}
#[cfg(feature = "Win32_winreg")]
#[inline]
pub unsafe fn SHRegSetUSValueA<P0, P1>(pszsubkey: P0, pszvalue: P1, dwtype: u32, pvdata: Option<*const core::ffi::c_void>, cbdata: Option<u32>, dwflags: Option<u32>) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegSetUSValueA(pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, dwtype : u32, pvdata : *const core::ffi::c_void, cbdata : u32, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegSetUSValueA(pszsubkey.param().abi(), pszvalue.param().abi(), dwtype, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winreg")]
#[inline]
pub unsafe fn SHRegSetUSValueW<P0, P1>(pwzsubkey: P0, pwzvalue: P1, dwtype: u32, pvdata: Option<*const core::ffi::c_void>, cbdata: Option<u32>, dwflags: Option<u32>) -> super::winreg::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegSetUSValueW(pwzsubkey : windows_core::PCWSTR, pwzvalue : windows_core::PCWSTR, dwtype : u32, pvdata : *const core::ffi::c_void, cbdata : u32, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegSetUSValueW(pwzsubkey.param().abi(), pwzvalue.param().abi(), dwtype, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata.unwrap_or(core::mem::zeroed()) as _, dwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegWriteUSValueA<P1>(huskey: HUSKEY, pszvalue: P1, dwtype: u32, pvdata: *const core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegWriteUSValueA(huskey : HUSKEY, pszvalue : windows_core::PCSTR, dwtype : u32, pvdata : *const core::ffi::c_void, cbdata : u32, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegWriteUSValueA(huskey, pszvalue.param().abi(), dwtype, pvdata, cbdata, dwflags) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHRegWriteUSValueW<P1>(huskey: HUSKEY, pwzvalue: P1, dwtype: u32, pvdata: *const core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHRegWriteUSValueW(huskey : HUSKEY, pwzvalue : windows_core::PCWSTR, dwtype : u32, pvdata : *const core::ffi::c_void, cbdata : u32, dwflags : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHRegWriteUSValueW(huskey, pwzvalue.param().abi(), dwtype, pvdata, cbdata, dwflags) }
}
#[inline]
pub unsafe fn SHReleaseThreadRef() -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn SHReleaseThreadRef() -> windows_core::HRESULT);
    unsafe { SHReleaseThreadRef() }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn SHSendMessageBroadcastA(umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("shlwapi.dll" "system" fn SHSendMessageBroadcastA(umsg : u32, wparam : super::minwindef::WPARAM, lparam : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { SHSendMessageBroadcastA(umsg, wparam, lparam) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn SHSendMessageBroadcastW(umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("shlwapi.dll" "system" fn SHSendMessageBroadcastW(umsg : u32, wparam : super::minwindef::WPARAM, lparam : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { SHSendMessageBroadcastW(umsg, wparam, lparam) }
}
#[inline]
pub unsafe fn SHSetThreadRef<P0>(punk: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHSetThreadRef(punk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SHSetThreadRef(punk.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHSetValueA<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, dwtype: u32, pvdata: Option<*const core::ffi::c_void>, cbdata: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHSetValueA(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCSTR, pszvalue : windows_core::PCSTR, dwtype : u32, pvdata : *const core::ffi::c_void, cbdata : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHSetValueA(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), dwtype, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn SHSetValueW<P1, P2>(hkey: super::minwindef::HKEY, pszsubkey: P1, pszvalue: P2, dwtype: u32, pvdata: Option<*const core::ffi::c_void>, cbdata: u32) -> super::winreg::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHSetValueW(hkey : super::minwindef::HKEY, pszsubkey : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, dwtype : u32, pvdata : *const core::ffi::c_void, cbdata : u32) -> super::winreg::WIN32_ERROR);
    unsafe { SHSetValueW(hkey, pszsubkey.param().abi(), pszvalue.param().abi(), dwtype, pvdata.unwrap_or(core::mem::zeroed()) as _, cbdata) }
}
#[cfg(feature = "Win32_objidl")]
#[inline]
pub unsafe fn SHSkipJunction<P0>(pbc: P0, pclsid: *const windows_core::GUID) -> windows_core::BOOL
where
    P0: windows_core::Param<super::objidl::IBindCtx>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHSkipJunction(pbc : *mut core::ffi::c_void, pclsid : *const windows_core::GUID) -> windows_core::BOOL);
    unsafe { SHSkipJunction(pbc.param().abi(), pclsid) }
}
#[inline]
pub unsafe fn SHStrDupA<P0>(psz: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHStrDupA(psz : windows_core::PCSTR, ppwsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHStrDupA(psz.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SHStrDupW<P0>(psz: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHStrDupW(psz : windows_core::PCWSTR, ppwsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHStrDupW(psz.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SHStripMneumonicA(pszmenu: windows_core::PSTR) -> i8 {
    windows_core::link!("shlwapi.dll" "system" fn SHStripMneumonicA(pszmenu : windows_core::PSTR) -> i8);
    unsafe { SHStripMneumonicA(core::mem::transmute(pszmenu)) }
}
#[inline]
pub unsafe fn SHStripMneumonicW(pszmenu: windows_core::PWSTR) -> u16 {
    windows_core::link!("shlwapi.dll" "system" fn SHStripMneumonicW(pszmenu : windows_core::PWSTR) -> u16);
    unsafe { SHStripMneumonicW(core::mem::transmute(pszmenu)) }
}
#[inline]
pub unsafe fn SHUnicodeToAnsi<P0>(pwszsrc: P0, pszdst: &mut [u8]) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHUnicodeToAnsi(pwszsrc : windows_core::PCWSTR, pszdst : windows_core::PSTR, cchbuf : i32) -> i32);
    unsafe { SHUnicodeToAnsi(pwszsrc.param().abi(), core::mem::transmute(pszdst.as_ptr()), pszdst.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SHUnicodeToUnicode<P0>(pwzsrc: P0, pwzdst: &mut [u16]) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn SHUnicodeToUnicode(pwzsrc : windows_core::PCWSTR, pwzdst : windows_core::PWSTR, cwchbuf : i32) -> i32);
    unsafe { SHUnicodeToUnicode(pwzsrc.param().abi(), core::mem::transmute(pwzdst.as_ptr()), pwzdst.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn SHUnlockShared(pvdata: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("shlwapi.dll" "system" fn SHUnlockShared(pvdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SHUnlockShared(pvdata) }
}
#[inline]
pub unsafe fn StrCSpnA<P0, P1>(pszstr: P0, pszset: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCSpnA(pszstr : windows_core::PCSTR, pszset : windows_core::PCSTR) -> i32);
    unsafe { StrCSpnA(pszstr.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrCSpnIA<P0, P1>(pszstr: P0, pszset: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCSpnIA(pszstr : windows_core::PCSTR, pszset : windows_core::PCSTR) -> i32);
    unsafe { StrCSpnIA(pszstr.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrCSpnIW<P0, P1>(pszstr: P0, pszset: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCSpnIW(pszstr : windows_core::PCWSTR, pszset : windows_core::PCWSTR) -> i32);
    unsafe { StrCSpnIW(pszstr.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrCSpnW<P0, P1>(pszstr: P0, pszset: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCSpnW(pszstr : windows_core::PCWSTR, pszset : windows_core::PCWSTR) -> i32);
    unsafe { StrCSpnW(pszstr.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrCatBuffA<P1>(pszdest: &mut [u8], pszsrc: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCatBuffA(pszdest : windows_core::PSTR, pszsrc : windows_core::PCSTR, cchdestbuffsize : i32) -> windows_core::PSTR);
    unsafe { StrCatBuffA(core::mem::transmute(pszdest.as_ptr()), pszsrc.param().abi(), pszdest.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrCatBuffW<P1>(pszdest: &mut [u16], pszsrc: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCatBuffW(pszdest : windows_core::PWSTR, pszsrc : windows_core::PCWSTR, cchdestbuffsize : i32) -> windows_core::PWSTR);
    unsafe { StrCatBuffW(core::mem::transmute(pszdest.as_ptr()), pszsrc.param().abi(), pszdest.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrCatChainW<P3>(pszdst: &mut [u16], ichat: u32, pszsrc: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCatChainW(pszdst : windows_core::PWSTR, cchdst : u32, ichat : u32, pszsrc : windows_core::PCWSTR) -> u32);
    unsafe { StrCatChainW(core::mem::transmute(pszdst.as_ptr()), pszdst.len().try_into().unwrap(), ichat, pszsrc.param().abi()) }
}
#[inline]
pub unsafe fn StrCatW<P1>(psz1: windows_core::PWSTR, psz2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCatW(psz1 : windows_core::PWSTR, psz2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrCatW(core::mem::transmute(psz1), psz2.param().abi()) }
}
#[inline]
pub unsafe fn StrChrA<P0>(pszstart: P0, wmatch: u16) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrChrA(pszstart : windows_core::PCSTR, wmatch : u16) -> windows_core::PSTR);
    unsafe { StrChrA(pszstart.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrChrIA<P0>(pszstart: P0, wmatch: u16) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrChrIA(pszstart : windows_core::PCSTR, wmatch : u16) -> windows_core::PSTR);
    unsafe { StrChrIA(pszstart.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrChrIW<P0>(pszstart: P0, wmatch: u16) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrChrIW(pszstart : windows_core::PCWSTR, wmatch : u16) -> windows_core::PWSTR);
    unsafe { StrChrIW(pszstart.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrChrNIW<P0>(pszstart: P0, wmatch: u16, cchmax: u32) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrChrNIW(pszstart : windows_core::PCWSTR, wmatch : u16, cchmax : u32) -> windows_core::PWSTR);
    unsafe { StrChrNIW(pszstart.param().abi(), wmatch, cchmax) }
}
#[inline]
pub unsafe fn StrChrNW<P0>(pszstart: P0, wmatch: u16, cchmax: u32) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrChrNW(pszstart : windows_core::PCWSTR, wmatch : u16, cchmax : u32) -> windows_core::PWSTR);
    unsafe { StrChrNW(pszstart.param().abi(), wmatch, cchmax) }
}
#[inline]
pub unsafe fn StrChrW<P0>(pszstart: P0, wmatch: u16) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrChrW(pszstart : windows_core::PCWSTR, wmatch : u16) -> windows_core::PWSTR);
    unsafe { StrChrW(pszstart.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrCmpCA<P0, P1>(pszstr1: P0, pszstr2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpCA(pszstr1 : windows_core::PCSTR, pszstr2 : windows_core::PCSTR) -> i32);
    unsafe { StrCmpCA(pszstr1.param().abi(), pszstr2.param().abi()) }
}
#[inline]
pub unsafe fn StrCmpCW<P0, P1>(pszstr1: P0, pszstr2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpCW(pszstr1 : windows_core::PCWSTR, pszstr2 : windows_core::PCWSTR) -> i32);
    unsafe { StrCmpCW(pszstr1.param().abi(), pszstr2.param().abi()) }
}
#[inline]
pub unsafe fn StrCmpICA<P0, P1>(pszstr1: P0, pszstr2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpICA(pszstr1 : windows_core::PCSTR, pszstr2 : windows_core::PCSTR) -> i32);
    unsafe { StrCmpICA(pszstr1.param().abi(), pszstr2.param().abi()) }
}
#[inline]
pub unsafe fn StrCmpICW<P0, P1>(pszstr1: P0, pszstr2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpICW(pszstr1 : windows_core::PCWSTR, pszstr2 : windows_core::PCWSTR) -> i32);
    unsafe { StrCmpICW(pszstr1.param().abi(), pszstr2.param().abi()) }
}
#[inline]
pub unsafe fn StrCmpIW<P0, P1>(psz1: P0, psz2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpIW(psz1 : windows_core::PCWSTR, psz2 : windows_core::PCWSTR) -> i32);
    unsafe { StrCmpIW(psz1.param().abi(), psz2.param().abi()) }
}
#[inline]
pub unsafe fn StrCmpLogicalW<P0, P1>(psz1: P0, psz2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpLogicalW(psz1 : windows_core::PCWSTR, psz2 : windows_core::PCWSTR) -> i32);
    unsafe { StrCmpLogicalW(psz1.param().abi(), psz2.param().abi()) }
}
#[inline]
pub unsafe fn StrCmpNA<P0, P1>(psz1: P0, psz2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNA(psz1 : windows_core::PCSTR, psz2 : windows_core::PCSTR, nchar : i32) -> i32);
    unsafe { StrCmpNA(psz1.param().abi(), psz2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNCA<P0, P1>(pszstr1: P0, pszstr2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNCA(pszstr1 : windows_core::PCSTR, pszstr2 : windows_core::PCSTR, nchar : i32) -> i32);
    unsafe { StrCmpNCA(pszstr1.param().abi(), pszstr2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNCW<P0, P1>(pszstr1: P0, pszstr2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNCW(pszstr1 : windows_core::PCWSTR, pszstr2 : windows_core::PCWSTR, nchar : i32) -> i32);
    unsafe { StrCmpNCW(pszstr1.param().abi(), pszstr2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNIA<P0, P1>(psz1: P0, psz2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNIA(psz1 : windows_core::PCSTR, psz2 : windows_core::PCSTR, nchar : i32) -> i32);
    unsafe { StrCmpNIA(psz1.param().abi(), psz2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNICA<P0, P1>(pszstr1: P0, pszstr2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNICA(pszstr1 : windows_core::PCSTR, pszstr2 : windows_core::PCSTR, nchar : i32) -> i32);
    unsafe { StrCmpNICA(pszstr1.param().abi(), pszstr2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNICW<P0, P1>(pszstr1: P0, pszstr2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNICW(pszstr1 : windows_core::PCWSTR, pszstr2 : windows_core::PCWSTR, nchar : i32) -> i32);
    unsafe { StrCmpNICW(pszstr1.param().abi(), pszstr2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNIW<P0, P1>(psz1: P0, psz2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNIW(psz1 : windows_core::PCWSTR, psz2 : windows_core::PCWSTR, nchar : i32) -> i32);
    unsafe { StrCmpNIW(psz1.param().abi(), psz2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpNW<P0, P1>(psz1: P0, psz2: P1, nchar: i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpNW(psz1 : windows_core::PCWSTR, psz2 : windows_core::PCWSTR, nchar : i32) -> i32);
    unsafe { StrCmpNW(psz1.param().abi(), psz2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrCmpW<P0, P1>(psz1: P0, psz2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCmpW(psz1 : windows_core::PCWSTR, psz2 : windows_core::PCWSTR) -> i32);
    unsafe { StrCmpW(psz1.param().abi(), psz2.param().abi()) }
}
#[inline]
pub unsafe fn StrCpyNW<P1>(pszdst: &mut [u16], pszsrc: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCpyNW(pszdst : windows_core::PWSTR, pszsrc : windows_core::PCWSTR, cchmax : i32) -> windows_core::PWSTR);
    unsafe { StrCpyNW(core::mem::transmute(pszdst.as_ptr()), pszsrc.param().abi(), pszdst.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrCpyW<P1>(psz1: windows_core::PWSTR, psz2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrCpyW(psz1 : windows_core::PWSTR, psz2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrCpyW(core::mem::transmute(psz1), psz2.param().abi()) }
}
#[inline]
pub unsafe fn StrDupA<P0>(pszsrch: P0) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrDupA(pszsrch : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { StrDupA(pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrDupW<P0>(pszsrch: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrDupW(pszsrch : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrDupW(pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrFormatByteSize64A(qdw: i64, pszbuf: &mut [u8]) -> windows_core::PSTR {
    windows_core::link!("shlwapi.dll" "system" fn StrFormatByteSize64A(qdw : i64, pszbuf : windows_core::PSTR, cchbuf : u32) -> windows_core::PSTR);
    unsafe { StrFormatByteSize64A(qdw, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrFormatByteSizeA(dw: u32, pszbuf: &mut [u8]) -> windows_core::PSTR {
    windows_core::link!("shlwapi.dll" "system" fn StrFormatByteSizeA(dw : u32, pszbuf : windows_core::PSTR, cchbuf : u32) -> windows_core::PSTR);
    unsafe { StrFormatByteSizeA(dw, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrFormatByteSizeEx(ull: u64, flags: SFBS_FLAGS, pszbuf: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn StrFormatByteSizeEx(ull : u64, flags : SFBS_FLAGS, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::HRESULT);
    unsafe { StrFormatByteSizeEx(ull, flags, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrFormatByteSizeW(qdw: i64, pszbuf: &mut [u16]) -> windows_core::PWSTR {
    windows_core::link!("shlwapi.dll" "system" fn StrFormatByteSizeW(qdw : i64, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::PWSTR);
    unsafe { StrFormatByteSizeW(qdw, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrFormatKBSizeA(qdw: i64, pszbuf: &mut [u8]) -> windows_core::PSTR {
    windows_core::link!("shlwapi.dll" "system" fn StrFormatKBSizeA(qdw : i64, pszbuf : windows_core::PSTR, cchbuf : u32) -> windows_core::PSTR);
    unsafe { StrFormatKBSizeA(qdw, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrFormatKBSizeW(qdw: i64, pszbuf: &mut [u16]) -> windows_core::PWSTR {
    windows_core::link!("shlwapi.dll" "system" fn StrFormatKBSizeW(qdw : i64, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::PWSTR);
    unsafe { StrFormatKBSizeW(qdw, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrFromTimeIntervalA(pszout: &mut [u8], dwtimems: u32, digits: i32) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn StrFromTimeIntervalA(pszout : windows_core::PSTR, cchmax : u32, dwtimems : u32, digits : i32) -> i32);
    unsafe { StrFromTimeIntervalA(core::mem::transmute(pszout.as_ptr()), pszout.len().try_into().unwrap(), dwtimems, digits) }
}
#[inline]
pub unsafe fn StrFromTimeIntervalW(pszout: &mut [u16], dwtimems: u32, digits: i32) -> i32 {
    windows_core::link!("shlwapi.dll" "system" fn StrFromTimeIntervalW(pszout : windows_core::PWSTR, cchmax : u32, dwtimems : u32, digits : i32) -> i32);
    unsafe { StrFromTimeIntervalW(core::mem::transmute(pszout.as_ptr()), pszout.len().try_into().unwrap(), dwtimems, digits) }
}
#[inline]
pub unsafe fn StrIsIntlEqualA<P1, P2>(fcasesens: bool, pszstring1: P1, pszstring2: P2, nchar: i32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrIsIntlEqualA(fcasesens : windows_core::BOOL, pszstring1 : windows_core::PCSTR, pszstring2 : windows_core::PCSTR, nchar : i32) -> windows_core::BOOL);
    unsafe { StrIsIntlEqualA(fcasesens.into(), pszstring1.param().abi(), pszstring2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrIsIntlEqualW<P1, P2>(fcasesens: bool, pszstring1: P1, pszstring2: P2, nchar: i32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrIsIntlEqualW(fcasesens : windows_core::BOOL, pszstring1 : windows_core::PCWSTR, pszstring2 : windows_core::PCWSTR, nchar : i32) -> windows_core::BOOL);
    unsafe { StrIsIntlEqualW(fcasesens.into(), pszstring1.param().abi(), pszstring2.param().abi(), nchar) }
}
#[inline]
pub unsafe fn StrNCatA<P1>(psz1: &mut [u8], psz2: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrNCatA(psz1 : windows_core::PSTR, psz2 : windows_core::PCSTR, cchmax : i32) -> windows_core::PSTR);
    unsafe { StrNCatA(core::mem::transmute(psz1.as_ptr()), psz2.param().abi(), psz1.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrNCatW<P1>(psz1: &mut [u16], psz2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrNCatW(psz1 : windows_core::PWSTR, psz2 : windows_core::PCWSTR, cchmax : i32) -> windows_core::PWSTR);
    unsafe { StrNCatW(core::mem::transmute(psz1.as_ptr()), psz2.param().abi(), psz1.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn StrPBrkA<P0, P1>(psz: P0, pszset: P1) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrPBrkA(psz : windows_core::PCSTR, pszset : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { StrPBrkA(psz.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrPBrkW<P0, P1>(psz: P0, pszset: P1) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrPBrkW(psz : windows_core::PCWSTR, pszset : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrPBrkW(psz.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrRChrA<P0, P1>(pszstart: P0, pszend: P1, wmatch: u16) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrRChrA(pszstart : windows_core::PCSTR, pszend : windows_core::PCSTR, wmatch : u16) -> windows_core::PSTR);
    unsafe { StrRChrA(pszstart.param().abi(), pszend.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrRChrIA<P0, P1>(pszstart: P0, pszend: P1, wmatch: u16) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrRChrIA(pszstart : windows_core::PCSTR, pszend : windows_core::PCSTR, wmatch : u16) -> windows_core::PSTR);
    unsafe { StrRChrIA(pszstart.param().abi(), pszend.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrRChrIW<P0, P1>(pszstart: P0, pszend: P1, wmatch: u16) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrRChrIW(pszstart : windows_core::PCWSTR, pszend : windows_core::PCWSTR, wmatch : u16) -> windows_core::PWSTR);
    unsafe { StrRChrIW(pszstart.param().abi(), pszend.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrRChrW<P0, P1>(pszstart: P0, pszend: P1, wmatch: u16) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrRChrW(pszstart : windows_core::PCWSTR, pszend : windows_core::PCWSTR, wmatch : u16) -> windows_core::PWSTR);
    unsafe { StrRChrW(pszstart.param().abi(), pszend.param().abi(), wmatch) }
}
#[inline]
pub unsafe fn StrRStrIA<P0, P1, P2>(pszsource: P0, pszlast: P1, pszsrch: P2) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrRStrIA(pszsource : windows_core::PCSTR, pszlast : windows_core::PCSTR, pszsrch : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { StrRStrIA(pszsource.param().abi(), pszlast.param().abi(), pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrRStrIW<P0, P1, P2>(pszsource: P0, pszlast: P1, pszsrch: P2) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrRStrIW(pszsource : windows_core::PCWSTR, pszlast : windows_core::PCWSTR, pszsrch : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrRStrIW(pszsource.param().abi(), pszlast.param().abi(), pszsrch.param().abi()) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn StrRetToBSTR(pstr: *mut super::shtypes::STRRET, pidl: Option<*const super::shtypes::ITEMIDLIST>, pbstr: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn StrRetToBSTR(pstr : *mut super::shtypes::STRRET, pidl : *const super::shtypes::ITEMIDLIST, pbstr : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { StrRetToBSTR(pstr as _, pidl.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pbstr)) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn StrRetToBufA(pstr: *mut super::shtypes::STRRET, pidl: Option<*const super::shtypes::ITEMIDLIST>, pszbuf: &mut [u8]) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn StrRetToBufA(pstr : *mut super::shtypes::STRRET, pidl : *const super::shtypes::ITEMIDLIST, pszbuf : windows_core::PSTR, cchbuf : u32) -> windows_core::HRESULT);
    unsafe { StrRetToBufA(pstr as _, pidl.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn StrRetToBufW(pstr: *mut super::shtypes::STRRET, pidl: Option<*const super::shtypes::ITEMIDLIST>, pszbuf: &mut [u16]) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn StrRetToBufW(pstr : *mut super::shtypes::STRRET, pidl : *const super::shtypes::ITEMIDLIST, pszbuf : windows_core::PWSTR, cchbuf : u32) -> windows_core::HRESULT);
    unsafe { StrRetToBufW(pstr as _, pidl.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszbuf.as_ptr()), pszbuf.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn StrRetToStrA(pstr: *mut super::shtypes::STRRET, pidl: Option<*const super::shtypes::ITEMIDLIST>, ppsz: *mut windows_core::PSTR) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn StrRetToStrA(pstr : *mut super::shtypes::STRRET, pidl : *const super::shtypes::ITEMIDLIST, ppsz : *mut windows_core::PSTR) -> windows_core::HRESULT);
    unsafe { StrRetToStrA(pstr as _, pidl.unwrap_or(core::mem::zeroed()) as _, ppsz as _) }
}
#[cfg(feature = "Win32_shtypes")]
#[inline]
pub unsafe fn StrRetToStrW(pstr: *mut super::shtypes::STRRET, pidl: Option<*const super::shtypes::ITEMIDLIST>, ppsz: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn StrRetToStrW(pstr : *mut super::shtypes::STRRET, pidl : *const super::shtypes::ITEMIDLIST, ppsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { StrRetToStrW(pstr as _, pidl.unwrap_or(core::mem::zeroed()) as _, ppsz as _) }
}
#[inline]
pub unsafe fn StrSpnA<P0, P1>(psz: P0, pszset: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrSpnA(psz : windows_core::PCSTR, pszset : windows_core::PCSTR) -> i32);
    unsafe { StrSpnA(psz.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrSpnW<P0, P1>(psz: P0, pszset: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrSpnW(psz : windows_core::PCWSTR, pszset : windows_core::PCWSTR) -> i32);
    unsafe { StrSpnW(psz.param().abi(), pszset.param().abi()) }
}
#[inline]
pub unsafe fn StrStrA<P0, P1>(pszfirst: P0, pszsrch: P1) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrStrA(pszfirst : windows_core::PCSTR, pszsrch : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { StrStrA(pszfirst.param().abi(), pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrStrIA<P0, P1>(pszfirst: P0, pszsrch: P1) -> windows_core::PSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrStrIA(pszfirst : windows_core::PCSTR, pszsrch : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { StrStrIA(pszfirst.param().abi(), pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrStrIW<P0, P1>(pszfirst: P0, pszsrch: P1) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrStrIW(pszfirst : windows_core::PCWSTR, pszsrch : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrStrIW(pszfirst.param().abi(), pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrStrNIW<P0, P1>(pszfirst: P0, pszsrch: P1, cchmax: u32) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrStrNIW(pszfirst : windows_core::PCWSTR, pszsrch : windows_core::PCWSTR, cchmax : u32) -> windows_core::PWSTR);
    unsafe { StrStrNIW(pszfirst.param().abi(), pszsrch.param().abi(), cchmax) }
}
#[inline]
pub unsafe fn StrStrNW<P0, P1>(pszfirst: P0, pszsrch: P1, cchmax: u32) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrStrNW(pszfirst : windows_core::PCWSTR, pszsrch : windows_core::PCWSTR, cchmax : u32) -> windows_core::PWSTR);
    unsafe { StrStrNW(pszfirst.param().abi(), pszsrch.param().abi(), cchmax) }
}
#[inline]
pub unsafe fn StrStrW<P0, P1>(pszfirst: P0, pszsrch: P1) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrStrW(pszfirst : windows_core::PCWSTR, pszsrch : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { StrStrW(pszfirst.param().abi(), pszsrch.param().abi()) }
}
#[inline]
pub unsafe fn StrToInt64ExA<P0>(pszstring: P0, dwflags: STIF_FLAGS, pllret: *mut i64) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrToInt64ExA(pszstring : windows_core::PCSTR, dwflags : STIF_FLAGS, pllret : *mut i64) -> windows_core::BOOL);
    unsafe { StrToInt64ExA(pszstring.param().abi(), dwflags, pllret as _) }
}
#[inline]
pub unsafe fn StrToInt64ExW<P0>(pszstring: P0, dwflags: STIF_FLAGS, pllret: *mut i64) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrToInt64ExW(pszstring : windows_core::PCWSTR, dwflags : STIF_FLAGS, pllret : *mut i64) -> windows_core::BOOL);
    unsafe { StrToInt64ExW(pszstring.param().abi(), dwflags, pllret as _) }
}
#[inline]
pub unsafe fn StrToIntA<P0>(pszsrc: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrToIntA(pszsrc : windows_core::PCSTR) -> i32);
    unsafe { StrToIntA(pszsrc.param().abi()) }
}
#[inline]
pub unsafe fn StrToIntExA<P0>(pszstring: P0, dwflags: STIF_FLAGS, piret: *mut i32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrToIntExA(pszstring : windows_core::PCSTR, dwflags : STIF_FLAGS, piret : *mut i32) -> windows_core::BOOL);
    unsafe { StrToIntExA(pszstring.param().abi(), dwflags, piret as _) }
}
#[inline]
pub unsafe fn StrToIntExW<P0>(pszstring: P0, dwflags: STIF_FLAGS, piret: *mut i32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrToIntExW(pszstring : windows_core::PCWSTR, dwflags : STIF_FLAGS, piret : *mut i32) -> windows_core::BOOL);
    unsafe { StrToIntExW(pszstring.param().abi(), dwflags, piret as _) }
}
#[inline]
pub unsafe fn StrToIntW<P0>(pszsrc: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrToIntW(pszsrc : windows_core::PCWSTR) -> i32);
    unsafe { StrToIntW(pszsrc.param().abi()) }
}
#[inline]
pub unsafe fn StrTrimA<P1>(psz: windows_core::PSTR, psztrimchars: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrTrimA(psz : windows_core::PSTR, psztrimchars : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { StrTrimA(core::mem::transmute(psz), psztrimchars.param().abi()) }
}
#[inline]
pub unsafe fn StrTrimW<P1>(psz: windows_core::PWSTR, psztrimchars: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn StrTrimW(psz : windows_core::PWSTR, psztrimchars : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { StrTrimW(core::mem::transmute(psz), psztrimchars.param().abi()) }
}
#[inline]
pub unsafe fn UrlApplySchemeA<P0>(pszin: P0, pszout: windows_core::PSTR, pcchout: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlApplySchemeA(pszin : windows_core::PCSTR, pszout : windows_core::PSTR, pcchout : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlApplySchemeA(pszin.param().abi(), core::mem::transmute(pszout), pcchout as _, dwflags) }
}
#[inline]
pub unsafe fn UrlApplySchemeW<P0>(pszin: P0, pszout: windows_core::PWSTR, pcchout: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlApplySchemeW(pszin : windows_core::PCWSTR, pszout : windows_core::PWSTR, pcchout : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlApplySchemeW(pszin.param().abi(), core::mem::transmute(pszout), pcchout as _, dwflags) }
}
#[inline]
pub unsafe fn UrlCanonicalizeA<P0>(pszurl: P0, pszcanonicalized: windows_core::PSTR, pcchcanonicalized: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCanonicalizeA(pszurl : windows_core::PCSTR, pszcanonicalized : windows_core::PSTR, pcchcanonicalized : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlCanonicalizeA(pszurl.param().abi(), core::mem::transmute(pszcanonicalized), pcchcanonicalized as _, dwflags) }
}
#[inline]
pub unsafe fn UrlCanonicalizeW<P0>(pszurl: P0, pszcanonicalized: windows_core::PWSTR, pcchcanonicalized: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCanonicalizeW(pszurl : windows_core::PCWSTR, pszcanonicalized : windows_core::PWSTR, pcchcanonicalized : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlCanonicalizeW(pszurl.param().abi(), core::mem::transmute(pszcanonicalized), pcchcanonicalized as _, dwflags) }
}
#[inline]
pub unsafe fn UrlCombineA<P0, P1>(pszbase: P0, pszrelative: P1, pszcombined: Option<windows_core::PSTR>, pcchcombined: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCombineA(pszbase : windows_core::PCSTR, pszrelative : windows_core::PCSTR, pszcombined : windows_core::PSTR, pcchcombined : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlCombineA(pszbase.param().abi(), pszrelative.param().abi(), pszcombined.unwrap_or(core::mem::zeroed()) as _, pcchcombined as _, dwflags) }
}
#[inline]
pub unsafe fn UrlCombineW<P0, P1>(pszbase: P0, pszrelative: P1, pszcombined: Option<windows_core::PWSTR>, pcchcombined: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCombineW(pszbase : windows_core::PCWSTR, pszrelative : windows_core::PCWSTR, pszcombined : windows_core::PWSTR, pcchcombined : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlCombineW(pszbase.param().abi(), pszrelative.param().abi(), pszcombined.unwrap_or(core::mem::zeroed()) as _, pcchcombined as _, dwflags) }
}
#[inline]
pub unsafe fn UrlCompareA<P0, P1>(psz1: P0, psz2: P1, fignoreslash: bool) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCompareA(psz1 : windows_core::PCSTR, psz2 : windows_core::PCSTR, fignoreslash : windows_core::BOOL) -> i32);
    unsafe { UrlCompareA(psz1.param().abi(), psz2.param().abi(), fignoreslash.into()) }
}
#[inline]
pub unsafe fn UrlCompareW<P0, P1>(psz1: P0, psz2: P1, fignoreslash: bool) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCompareW(psz1 : windows_core::PCWSTR, psz2 : windows_core::PCWSTR, fignoreslash : windows_core::BOOL) -> i32);
    unsafe { UrlCompareW(psz1.param().abi(), psz2.param().abi(), fignoreslash.into()) }
}
#[inline]
pub unsafe fn UrlCreateFromPathA<P0>(pszpath: P0, pszurl: windows_core::PSTR, pcchurl: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCreateFromPathA(pszpath : windows_core::PCSTR, pszurl : windows_core::PSTR, pcchurl : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlCreateFromPathA(pszpath.param().abi(), core::mem::transmute(pszurl), pcchurl as _, dwflags) }
}
#[inline]
pub unsafe fn UrlCreateFromPathW<P0>(pszpath: P0, pszurl: windows_core::PWSTR, pcchurl: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlCreateFromPathW(pszpath : windows_core::PCWSTR, pszurl : windows_core::PWSTR, pcchurl : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlCreateFromPathW(pszpath.param().abi(), core::mem::transmute(pszurl), pcchurl as _, dwflags) }
}
#[inline]
pub unsafe fn UrlEscapeA<P0>(pszurl: P0, pszescaped: windows_core::PSTR, pcchescaped: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlEscapeA(pszurl : windows_core::PCSTR, pszescaped : windows_core::PSTR, pcchescaped : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlEscapeA(pszurl.param().abi(), core::mem::transmute(pszescaped), pcchescaped as _, dwflags) }
}
#[inline]
pub unsafe fn UrlEscapeW<P0>(pszurl: P0, pszescaped: windows_core::PWSTR, pcchescaped: *mut u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlEscapeW(pszurl : windows_core::PCWSTR, pszescaped : windows_core::PWSTR, pcchescaped : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlEscapeW(pszurl.param().abi(), core::mem::transmute(pszescaped), pcchescaped as _, dwflags) }
}
#[inline]
pub unsafe fn UrlFixupW<P0>(pcszurl: P0, psztranslatedurl: &mut [u16]) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlFixupW(pcszurl : windows_core::PCWSTR, psztranslatedurl : windows_core::PWSTR, cchmax : u32) -> windows_core::HRESULT);
    unsafe { UrlFixupW(pcszurl.param().abi(), core::mem::transmute(psztranslatedurl.as_ptr()), psztranslatedurl.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn UrlGetLocationA<P0>(pszurl: P0) -> windows_core::PCSTR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlGetLocationA(pszurl : windows_core::PCSTR) -> windows_core::PCSTR);
    unsafe { UrlGetLocationA(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn UrlGetLocationW<P0>(pszurl: P0) -> windows_core::PCWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlGetLocationW(pszurl : windows_core::PCWSTR) -> windows_core::PCWSTR);
    unsafe { UrlGetLocationW(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn UrlGetPartA<P0>(pszin: P0, pszout: windows_core::PSTR, pcchout: *mut u32, dwpart: u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlGetPartA(pszin : windows_core::PCSTR, pszout : windows_core::PSTR, pcchout : *mut u32, dwpart : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlGetPartA(pszin.param().abi(), core::mem::transmute(pszout), pcchout as _, dwpart, dwflags) }
}
#[inline]
pub unsafe fn UrlGetPartW<P0>(pszin: P0, pszout: windows_core::PWSTR, pcchout: *mut u32, dwpart: u32, dwflags: u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlGetPartW(pszin : windows_core::PCWSTR, pszout : windows_core::PWSTR, pcchout : *mut u32, dwpart : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlGetPartW(pszin.param().abi(), core::mem::transmute(pszout), pcchout as _, dwpart, dwflags) }
}
#[inline]
pub unsafe fn UrlHashA<P0>(pszurl: P0, pbhash: &mut [u8]) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlHashA(pszurl : windows_core::PCSTR, pbhash : *mut u8, cbhash : u32) -> windows_core::HRESULT);
    unsafe { UrlHashA(pszurl.param().abi(), core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn UrlHashW<P0>(pszurl: P0, pbhash: &mut [u8]) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlHashW(pszurl : windows_core::PCWSTR, pbhash : *mut u8, cbhash : u32) -> windows_core::HRESULT);
    unsafe { UrlHashW(pszurl.param().abi(), core::mem::transmute(pbhash.as_ptr()), pbhash.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn UrlIsA<P0>(pszurl: P0, urlis: URLIS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlIsA(pszurl : windows_core::PCSTR, urlis : URLIS) -> windows_core::BOOL);
    unsafe { UrlIsA(pszurl.param().abi(), urlis) }
}
#[inline]
pub unsafe fn UrlIsNoHistoryA<P0>(pszurl: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlIsNoHistoryA(pszurl : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { UrlIsNoHistoryA(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn UrlIsNoHistoryW<P0>(pszurl: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlIsNoHistoryW(pszurl : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { UrlIsNoHistoryW(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn UrlIsOpaqueA<P0>(pszurl: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlIsOpaqueA(pszurl : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { UrlIsOpaqueA(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn UrlIsOpaqueW<P0>(pszurl: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlIsOpaqueW(pszurl : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { UrlIsOpaqueW(pszurl.param().abi()) }
}
#[inline]
pub unsafe fn UrlIsW<P0>(pszurl: P0, urlis: URLIS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn UrlIsW(pszurl : windows_core::PCWSTR, urlis : URLIS) -> windows_core::BOOL);
    unsafe { UrlIsW(pszurl.param().abi(), urlis) }
}
#[inline]
pub unsafe fn UrlUnescapeA(pszurl: windows_core::PSTR, pszunescaped: Option<windows_core::PSTR>, pcchunescaped: Option<*mut u32>, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn UrlUnescapeA(pszurl : windows_core::PSTR, pszunescaped : windows_core::PSTR, pcchunescaped : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlUnescapeA(core::mem::transmute(pszurl), pszunescaped.unwrap_or(core::mem::zeroed()) as _, pcchunescaped.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn UrlUnescapeW(pszurl: windows_core::PWSTR, pszunescaped: Option<windows_core::PWSTR>, pcchunescaped: Option<*mut u32>, dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("shlwapi.dll" "system" fn UrlUnescapeW(pszurl : windows_core::PWSTR, pszunescaped : windows_core::PWSTR, pcchunescaped : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { UrlUnescapeW(core::mem::transmute(pszurl), pszunescaped.unwrap_or(core::mem::zeroed()) as _, pcchunescaped.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn WhichPlatform() -> u32 {
    windows_core::link!("shlwapi.dll" "system" fn WhichPlatform() -> u32);
    unsafe { WhichPlatform() }
}
#[inline]
pub unsafe fn wnsprintfA<P2>(pszdest: &mut [u8], pszfmt: P2) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "C" fn wnsprintfA(pszdest : windows_core::PSTR, cchdest : i32, pszfmt : windows_core::PCSTR) -> i32);
    unsafe { wnsprintfA(core::mem::transmute(pszdest.as_ptr()), pszdest.len().try_into().unwrap(), pszfmt.param().abi()) }
}
#[inline]
pub unsafe fn wnsprintfW<P2>(pszdest: &mut [u16], pszfmt: P2) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "C" fn wnsprintfW(pszdest : windows_core::PWSTR, cchdest : i32, pszfmt : windows_core::PCWSTR) -> i32);
    unsafe { wnsprintfW(core::mem::transmute(pszdest.as_ptr()), pszdest.len().try_into().unwrap(), pszfmt.param().abi()) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn wvnsprintfA<P2>(pszdest: &mut [u8], pszfmt: P2, arglist: *const i8) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn wvnsprintfA(pszdest : windows_core::PSTR, cchdest : i32, pszfmt : windows_core::PCSTR, arglist : *const i8) -> i32);
    unsafe { wvnsprintfA(core::mem::transmute(pszdest.as_ptr()), pszdest.len().try_into().unwrap(), pszfmt.param().abi(), arglist) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "Win32_vadefs")]
#[inline]
pub unsafe fn wvnsprintfA<P2>(pszdest: &mut [u8], pszfmt: P2, arglist: super::vadefs::va_list) -> i32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn wvnsprintfA(pszdest : windows_core::PSTR, cchdest : i32, pszfmt : windows_core::PCSTR, arglist : super::vadefs::va_list) -> i32);
    unsafe { wvnsprintfA(core::mem::transmute(pszdest.as_ptr()), pszdest.len().try_into().unwrap(), pszfmt.param().abi(), arglist) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn wvnsprintfW<P2>(pszdest: &mut [u16], pszfmt: P2, arglist: *const i8) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn wvnsprintfW(pszdest : windows_core::PWSTR, cchdest : i32, pszfmt : windows_core::PCWSTR, arglist : *const i8) -> i32);
    unsafe { wvnsprintfW(core::mem::transmute(pszdest.as_ptr()), pszdest.len().try_into().unwrap(), pszfmt.param().abi(), arglist) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(feature = "Win32_vadefs")]
#[inline]
pub unsafe fn wvnsprintfW<P2>(pszdest: &mut [u16], pszfmt: P2, arglist: super::vadefs::va_list) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shlwapi.dll" "system" fn wvnsprintfW(pszdest : windows_core::PWSTR, cchdest : i32, pszfmt : windows_core::PCWSTR, arglist : super::vadefs::va_list) -> i32);
    unsafe { wvnsprintfW(core::mem::transmute(pszdest.as_ptr()), pszdest.len().try_into().unwrap(), pszfmt.param().abi(), arglist) }
}
pub type ASSOCDATA = i32;
pub const ASSOCDATA_EDITFLAGS: ASSOCDATA = 5;
pub const ASSOCDATA_HASPERUSERASSOC: ASSOCDATA = 4;
pub const ASSOCDATA_MAX: ASSOCDATA = 7;
pub const ASSOCDATA_MSIDESCRIPTOR: ASSOCDATA = 1;
pub const ASSOCDATA_NOACTIVATEHANDLER: ASSOCDATA = 2;
pub const ASSOCDATA_UNUSED1: ASSOCDATA = 3;
pub const ASSOCDATA_VALUE: ASSOCDATA = 6;
pub type ASSOCENUM = i32;
pub const ASSOCENUM_NONE: ASSOCENUM = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ASSOCF(pub u32);
pub const ASSOCF_APP_TO_APP: i32 = 65536;
pub const ASSOCF_IGNOREBASECLASS: i32 = 512;
pub const ASSOCF_INIT_BYEXENAME: i32 = 2;
pub const ASSOCF_INIT_DEFAULTTOFOLDER: i32 = 8;
pub const ASSOCF_INIT_DEFAULTTOSTAR: i32 = 4;
pub const ASSOCF_INIT_FIXED_PROGID: i32 = 2048;
pub const ASSOCF_INIT_FOR_FILE: i32 = 8192;
pub const ASSOCF_INIT_IGNOREUNKNOWN: i32 = 1024;
pub const ASSOCF_INIT_NOREMAPCLSID: i32 = 1;
pub const ASSOCF_IS_FULL_URI: i32 = 16384;
pub const ASSOCF_IS_PROTOCOL: i32 = 4096;
pub const ASSOCF_NOFIXUPS: i32 = 256;
pub const ASSOCF_NONE: i32 = 0;
pub const ASSOCF_NOTRUNCATE: i32 = 32;
pub const ASSOCF_NOUSERSETTINGS: i32 = 16;
pub const ASSOCF_OPEN_BYEXENAME: i32 = 2;
pub const ASSOCF_PER_MACHINE_ONLY: i32 = 32768;
pub const ASSOCF_REMAPRUNDLL: i32 = 128;
pub const ASSOCF_VERIFY: i32 = 64;
pub type ASSOCKEY = i32;
pub const ASSOCKEY_APP: ASSOCKEY = 2;
pub const ASSOCKEY_BASECLASS: ASSOCKEY = 4;
pub const ASSOCKEY_CLASS: ASSOCKEY = 3;
pub const ASSOCKEY_MAX: ASSOCKEY = 5;
pub const ASSOCKEY_SHELLEXECCLASS: ASSOCKEY = 1;
pub type ASSOCSTR = i32;
pub const ASSOCSTR_APPICONREFERENCE: ASSOCSTR = 23;
pub const ASSOCSTR_APPID: ASSOCSTR = 21;
pub const ASSOCSTR_APPPUBLISHER: ASSOCSTR = 22;
pub const ASSOCSTR_COMMAND: ASSOCSTR = 1;
pub const ASSOCSTR_CONTENTTYPE: ASSOCSTR = 14;
pub const ASSOCSTR_DDEAPPLICATION: ASSOCSTR = 9;
pub const ASSOCSTR_DDECOMMAND: ASSOCSTR = 7;
pub const ASSOCSTR_DDEIFEXEC: ASSOCSTR = 8;
pub const ASSOCSTR_DDETOPIC: ASSOCSTR = 10;
pub const ASSOCSTR_DEFAULTICON: ASSOCSTR = 15;
pub const ASSOCSTR_DELEGATEEXECUTE: ASSOCSTR = 18;
pub const ASSOCSTR_DROPTARGET: ASSOCSTR = 17;
pub const ASSOCSTR_EXECUTABLE: ASSOCSTR = 2;
pub const ASSOCSTR_FRIENDLYAPPNAME: ASSOCSTR = 4;
pub const ASSOCSTR_FRIENDLYDOCNAME: ASSOCSTR = 3;
pub const ASSOCSTR_INFOTIP: ASSOCSTR = 11;
pub const ASSOCSTR_MAX: ASSOCSTR = 24;
pub const ASSOCSTR_NOOPEN: ASSOCSTR = 5;
pub const ASSOCSTR_PROGID: ASSOCSTR = 20;
pub const ASSOCSTR_QUICKTIP: ASSOCSTR = 12;
pub const ASSOCSTR_SHELLEXTENSION: ASSOCSTR = 16;
pub const ASSOCSTR_SHELLNEWVALUE: ASSOCSTR = 6;
pub const ASSOCSTR_SUPPORTED_URI_PROTOCOLS: ASSOCSTR = 19;
pub const ASSOCSTR_TILEINFO: ASSOCSTR = 13;
pub const CTF_COINIT: i32 = 8;
pub const CTF_COINIT_MTA: i32 = 4096;
pub const CTF_COINIT_STA: i32 = 8;
pub const CTF_FREELIBANDEXIT: i32 = 16;
pub const CTF_INHERITWOW64: i32 = 256;
pub const CTF_INSIST: i32 = 1;
pub const CTF_KEYBOARD_LOCALE: i32 = 1024;
pub const CTF_NOADDREFLIB: i32 = 8192;
pub const CTF_OLEINITIALIZE: i32 = 2048;
pub const CTF_PROCESS_REF: i32 = 4;
pub const CTF_REF_COUNTED: i32 = 32;
pub const CTF_THREAD_REF: i32 = 2;
pub const CTF_UNUSED: i32 = 128;
pub const CTF_WAIT_ALLOWCOM: i32 = 64;
pub const CTF_WAIT_NO_REENTRANCY: i32 = 512;
pub type DLLGETVERSIONPROC = Option<unsafe extern "system" fn(param0: *mut DLLVERSIONINFO) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DLLVERSIONINFO {
    pub cbSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformID: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DLLVERSIONINFO2 {
    pub info1: DLLVERSIONINFO,
    pub dwFlags: u32,
    pub ullVersion: u64,
}
pub const DLLVER_BUILD_MASK: u32 = 4294901760;
pub const DLLVER_MAJOR_MASK: u64 = 18446462598732840960;
pub const DLLVER_MINOR_MASK: u64 = 281470681743360;
pub const DLLVER_PLATFORM_NT: u32 = 2;
pub const DLLVER_PLATFORM_WINDOWS: u32 = 1;
pub const DLLVER_QFE_MASK: u32 = 65535;
pub const FDTF_DEFAULT: u32 = 3;
pub const FDTF_LONGDATE: u32 = 4;
pub const FDTF_LONGTIME: u32 = 8;
pub const FDTF_LTRDATE: u32 = 256;
pub const FDTF_NOAUTOREADINGORDER: u32 = 1024;
pub const FDTF_RELATIVE: u32 = 16;
pub const FDTF_RTLDATE: u32 = 512;
pub const FDTF_SHORTDATE: u32 = 2;
pub const FDTF_SHORTTIME: u32 = 1;
pub type FILETYPEATTRIBUTEFLAGS = u32;
pub const FTA_AlwaysUnsafe: FILETYPEATTRIBUTEFLAGS = 131072;
pub const FTA_AlwaysUseDirectInvoke: FILETYPEATTRIBUTEFLAGS = 4194304;
pub const FTA_Exclude: FILETYPEATTRIBUTEFLAGS = 1;
pub const FTA_HasExtension: FILETYPEATTRIBUTEFLAGS = 4;
pub const FTA_NoDDE: FILETYPEATTRIBUTEFLAGS = 8192;
pub const FTA_NoEdit: FILETYPEATTRIBUTEFLAGS = 8;
pub const FTA_NoEditDesc: FILETYPEATTRIBUTEFLAGS = 256;
pub const FTA_NoEditDflt: FILETYPEATTRIBUTEFLAGS = 1024;
pub const FTA_NoEditIcon: FILETYPEATTRIBUTEFLAGS = 512;
pub const FTA_NoEditMIME: FILETYPEATTRIBUTEFLAGS = 32768;
pub const FTA_NoEditVerb: FILETYPEATTRIBUTEFLAGS = 64;
pub const FTA_NoEditVerbCmd: FILETYPEATTRIBUTEFLAGS = 2048;
pub const FTA_NoEditVerbExe: FILETYPEATTRIBUTEFLAGS = 4096;
pub const FTA_NoNewVerb: FILETYPEATTRIBUTEFLAGS = 32;
pub const FTA_NoRecentDocs: FILETYPEATTRIBUTEFLAGS = 1048576;
pub const FTA_NoRemove: FILETYPEATTRIBUTEFLAGS = 16;
pub const FTA_NoRemoveVerb: FILETYPEATTRIBUTEFLAGS = 128;
pub const FTA_None: FILETYPEATTRIBUTEFLAGS = 0;
pub const FTA_OpenIsSafe: FILETYPEATTRIBUTEFLAGS = 65536;
pub const FTA_SafeForElevation: FILETYPEATTRIBUTEFLAGS = 2097152;
pub const FTA_Show: FILETYPEATTRIBUTEFLAGS = 2;
pub const GCT_INVALID: u32 = 0;
pub const GCT_LFNCHAR: u32 = 1;
pub const GCT_SEPARATOR: u32 = 8;
pub const GCT_SHORTCHAR: u32 = 2;
pub const GCT_WILD: u32 = 4;
pub const GLOBALCOUNTER_APPLICATION_DESTINATIONS: SHGLOBALCOUNTER = 12;
pub const GLOBALCOUNTER_APPROVEDSITES: SHGLOBALCOUNTER = 4;
pub const GLOBALCOUNTER_APPSFOLDER_FILETYPEASSOCIATION_COUNTER: SHGLOBALCOUNTER = 55;
pub const GLOBALCOUNTER_APP_ITEMS_STATE_STORE_CACHE: SHGLOBALCOUNTER = 53;
pub const GLOBALCOUNTER_ASSOCCHANGED: SHGLOBALCOUNTER = 52;
pub const GLOBALCOUNTER_BANNERS_DATAMODEL_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 58;
pub const GLOBALCOUNTER_BITBUCKETNUMDELETERS: SHGLOBALCOUNTER = 14;
pub const GLOBALCOUNTER_COMMONPLACES_LIST_CACHE: SHGLOBALCOUNTER = 50;
pub const GLOBALCOUNTER_FOLDERDEFINITION_CACHE: SHGLOBALCOUNTER = 49;
pub const GLOBALCOUNTER_FOLDERSETTINGSCHANGE: SHGLOBALCOUNTER = 2;
pub const GLOBALCOUNTER_IEONLY_SESSIONS: SHGLOBALCOUNTER = 11;
pub const GLOBALCOUNTER_IESESSIONS: SHGLOBALCOUNTER = 10;
pub const GLOBALCOUNTER_INTERNETTOOLBAR_LAYOUT: SHGLOBALCOUNTER = 48;
pub const GLOBALCOUNTER_MAXIMUMVALUE: SHGLOBALCOUNTER = 59;
pub const GLOBALCOUNTER_OVERLAYMANAGER: SHGLOBALCOUNTER = 8;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE: SHGLOBALCOUNTER = 47;
pub const GLOBALCOUNTER_PRIVATE_PROFILE_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 51;
pub const GLOBALCOUNTER_QUERYASSOCIATIONS: SHGLOBALCOUNTER = 9;
pub const GLOBALCOUNTER_RATINGS: SHGLOBALCOUNTER = 3;
pub const GLOBALCOUNTER_RATINGS_STATECOUNTER: SHGLOBALCOUNTER = 46;
pub const GLOBALCOUNTER_RECYCLEBINCORRUPTED: SHGLOBALCOUNTER = 45;
pub const GLOBALCOUNTER_RECYCLEBINENUM: SHGLOBALCOUNTER = 44;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_A: SHGLOBALCOUNTER = 16;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_B: SHGLOBALCOUNTER = 17;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_C: SHGLOBALCOUNTER = 18;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_D: SHGLOBALCOUNTER = 19;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_E: SHGLOBALCOUNTER = 20;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_F: SHGLOBALCOUNTER = 21;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_G: SHGLOBALCOUNTER = 22;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_H: SHGLOBALCOUNTER = 23;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_I: SHGLOBALCOUNTER = 24;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_J: SHGLOBALCOUNTER = 25;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_K: SHGLOBALCOUNTER = 26;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_L: SHGLOBALCOUNTER = 27;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_M: SHGLOBALCOUNTER = 28;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_N: SHGLOBALCOUNTER = 29;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_O: SHGLOBALCOUNTER = 30;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_P: SHGLOBALCOUNTER = 31;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Q: SHGLOBALCOUNTER = 32;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_R: SHGLOBALCOUNTER = 33;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_S: SHGLOBALCOUNTER = 34;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_T: SHGLOBALCOUNTER = 35;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_U: SHGLOBALCOUNTER = 36;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_V: SHGLOBALCOUNTER = 37;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_W: SHGLOBALCOUNTER = 38;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_X: SHGLOBALCOUNTER = 39;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Y: SHGLOBALCOUNTER = 40;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Z: SHGLOBALCOUNTER = 41;
pub const GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SHARES: SHGLOBALCOUNTER = 15;
pub const GLOBALCOUNTER_RESTRICTIONS: SHGLOBALCOUNTER = 5;
pub const GLOBALCOUNTER_SEARCHMANAGER: SHGLOBALCOUNTER = 0;
pub const GLOBALCOUNTER_SEARCHOPTIONS: SHGLOBALCOUNTER = 1;
pub const GLOBALCOUNTER_SETTINGSYNC_ENABLED: SHGLOBALCOUNTER = 54;
pub const GLOBALCOUNTER_SHELLSETTINGSCHANGED: SHGLOBALCOUNTER = 6;
pub const GLOBALCOUNTER_SYNC_ENGINE_INFORMATION_CACHE_MACHINEWIDE: SHGLOBALCOUNTER = 57;
pub const GLOBALCOUNTER_SYSTEMPIDLCHANGE: SHGLOBALCOUNTER = 7;
pub const GLOBALCOUNTER_USERINFOCHANGED: SHGLOBALCOUNTER = 56;
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HUSKEY(pub super::winnt::HANDLE);
pub const ILMM_IE4: u32 = 0;
windows_core::imp::define_interface!(IQueryAssociations, IQueryAssociations_Vtbl, 0xc46ca590_3c3f_11d2_bee6_0000f805ca57);
windows_core::imp::interface_hierarchy!(IQueryAssociations, windows_core::IUnknown);
impl IQueryAssociations {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
    pub unsafe fn Init<P1>(&self, flags: ASSOCF, pszassoc: P1, hkprogid: Option<super::minwindef::HKEY>, hwnd: Option<super::windef::HWND>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), flags, pszassoc.param().abi(), hkprogid.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetString<P2>(&self, flags: ASSOCF, str: ASSOCSTR, pszextra: P2, pszout: Option<windows_core::PWSTR>, pcchout: *mut u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), flags, str, pszextra.param().abi(), pszout.unwrap_or(core::mem::zeroed()) as _, pcchout as _) }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetKey<P2>(&self, flags: ASSOCF, key: ASSOCKEY, pszextra: P2) -> windows_core::Result<super::minwindef::HKEY>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKey)(windows_core::Interface::as_raw(self), flags, key, pszextra.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetData<P2>(&self, flags: ASSOCF, data: ASSOCDATA, pszextra: P2, pvout: Option<*mut core::ffi::c_void>, pcbout: Option<*mut u32>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), flags, data, pszextra.param().abi(), pvout.unwrap_or(core::mem::zeroed()) as _, pcbout.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetEnum<P2>(&self, flags: ASSOCF, assocenum: ASSOCENUM, pszextra: P2, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetEnum)(windows_core::Interface::as_raw(self), flags, assocenum, pszextra.param().abi(), riid, ppvout as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryAssociations_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, ASSOCF, windows_core::PCWSTR, super::minwindef::HKEY, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_windef")))]
    Init: usize,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, ASSOCF, ASSOCSTR, windows_core::PCWSTR, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetKey: unsafe extern "system" fn(*mut core::ffi::c_void, ASSOCF, ASSOCKEY, windows_core::PCWSTR, *mut super::minwindef::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetKey: usize,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, ASSOCF, ASSOCDATA, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetEnum: unsafe extern "system" fn(*mut core::ffi::c_void, ASSOCF, ASSOCENUM, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub trait IQueryAssociations_Impl: windows_core::IUnknownImpl {
    fn Init(&self, flags: ASSOCF, pszassoc: &windows_core::PCWSTR, hkprogid: super::minwindef::HKEY, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn GetString(&self, flags: ASSOCF, str: ASSOCSTR, pszextra: &windows_core::PCWSTR, pszout: windows_core::PWSTR, pcchout: *mut u32) -> windows_core::Result<()>;
    fn GetKey(&self, flags: ASSOCF, key: ASSOCKEY, pszextra: &windows_core::PCWSTR) -> windows_core::Result<super::minwindef::HKEY>;
    fn GetData(&self, flags: ASSOCF, data: ASSOCDATA, pszextra: &windows_core::PCWSTR, pvout: *mut core::ffi::c_void, pcbout: *mut u32) -> windows_core::Result<()>;
    fn GetEnum(&self, flags: ASSOCF, assocenum: ASSOCENUM, pszextra: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl IQueryAssociations_Vtbl {
    pub const fn new<Identity: IQueryAssociations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IQueryAssociations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: ASSOCF, pszassoc: windows_core::PCWSTR, hkprogid: super::minwindef::HKEY, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryAssociations_Impl::Init(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pszassoc), core::mem::transmute_copy(&hkprogid), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetString<Identity: IQueryAssociations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: ASSOCF, str: ASSOCSTR, pszextra: windows_core::PCWSTR, pszout: windows_core::PWSTR, pcchout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryAssociations_Impl::GetString(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&str), core::mem::transmute(&pszextra), core::mem::transmute_copy(&pszout), core::mem::transmute_copy(&pcchout)).into()
            }
        }
        unsafe extern "system" fn GetKey<Identity: IQueryAssociations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: ASSOCF, key: ASSOCKEY, pszextra: windows_core::PCWSTR, phkeyout: *mut super::minwindef::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQueryAssociations_Impl::GetKey(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&key), core::mem::transmute(&pszextra)) {
                    Ok(ok__) => {
                        phkeyout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetData<Identity: IQueryAssociations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: ASSOCF, data: ASSOCDATA, pszextra: windows_core::PCWSTR, pvout: *mut core::ffi::c_void, pcbout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryAssociations_Impl::GetData(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&data), core::mem::transmute(&pszextra), core::mem::transmute_copy(&pvout), core::mem::transmute_copy(&pcbout)).into()
            }
        }
        unsafe extern "system" fn GetEnum<Identity: IQueryAssociations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: ASSOCF, assocenum: ASSOCENUM, pszextra: windows_core::PCWSTR, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryAssociations_Impl::GetEnum(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&assocenum), core::mem::transmute(&pszextra), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvout)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetKey: GetKey::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetEnum: GetEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryAssociations as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IQueryAssociations {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCQITAB(pub *const QITAB);
impl LPCQITAB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCQITAB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPQITAB(pub *mut QITAB);
impl LPQITAB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPQITAB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OS_ADVSERVER: u32 = 22;
pub const OS_ANYSERVER: u32 = 29;
pub const OS_APPLIANCE: u32 = 36;
pub const OS_DATACENTER: u32 = 21;
pub const OS_DOMAINMEMBER: u32 = 28;
pub const OS_EMBEDDED: u32 = 13;
pub const OS_FASTUSERSWITCHING: u32 = 26;
pub const OS_HOME: u32 = 19;
pub const OS_MEDIACENTER: u32 = 35;
pub const OS_MEORGREATER: u32 = 17;
pub const OS_NT: u32 = 1;
pub const OS_NT4ORGREATER: u32 = 3;
pub const OS_PERSONALTERMINALSERVER: u32 = 25;
pub const OS_PROFESSIONAL: u32 = 20;
pub const OS_SERVER: u32 = 23;
pub const OS_SERVERADMINUI: u32 = 34;
pub const OS_SMALLBUSINESSSERVER: u32 = 32;
pub const OS_TABLETPC: u32 = 33;
pub const OS_TERMINALCLIENT: u32 = 14;
pub const OS_TERMINALREMOTEADMIN: u32 = 15;
pub const OS_TERMINALSERVER: u32 = 24;
pub const OS_WEBSERVER: u32 = 31;
pub const OS_WELCOMELOGONUI: u32 = 27;
pub const OS_WIN2000ADVSERVER: u32 = 10;
pub const OS_WIN2000DATACENTER: u32 = 11;
pub const OS_WIN2000ORGREATER: u32 = 7;
pub const OS_WIN2000PRO: u32 = 8;
pub const OS_WIN2000SERVER: u32 = 9;
pub const OS_WIN2000TERMINAL: u32 = 12;
pub const OS_WIN95ORGREATER: u32 = 2;
pub const OS_WIN95_GOLD: u32 = 16;
pub const OS_WIN98ORGREATER: u32 = 5;
pub const OS_WIN98_GOLD: u32 = 6;
pub const OS_WINDOWS: u32 = 0;
pub const OS_WOW6432: u32 = 30;
pub const OS_XPORGREATER: u32 = 18;
pub type PARSEDURL = PARSEDURLA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PARSEDURLA {
    pub cbSize: u32,
    pub pszProtocol: windows_core::PCSTR,
    pub cchProtocol: u32,
    pub pszSuffix: windows_core::PCSTR,
    pub cchSuffix: u32,
    pub nScheme: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PARSEDURLW {
    pub cbSize: u32,
    pub pszProtocol: windows_core::PCWSTR,
    pub cchProtocol: u32,
    pub pszSuffix: windows_core::PCWSTR,
    pub cchSuffix: u32,
    pub nScheme: u32,
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHUSKEY(pub *mut HUSKEY);
#[cfg(feature = "Win32_winnt")]
impl PHUSKEY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PHUSKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PLATFORM_BROWSERONLY: u32 = 1;
pub const PLATFORM_IE3: u32 = 1;
pub const PLATFORM_INTEGRATED: u32 = 2;
pub const PLATFORM_UNKNOWN: u32 = 0;
pub const PMSF_DONT_STRIP_SPACES: u32 = 65536;
pub const PMSF_MULTIPLE: u32 = 1;
pub const PMSF_NORMAL: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PPARSEDURL(pub PPARSEDURLA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPARSEDURLA(pub *mut PARSEDURLA);
impl PPARSEDURLA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPARSEDURLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPARSEDURLW(pub *mut PARSEDURLW);
impl PPARSEDURLW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPARSEDURLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QITAB {
    pub piid: *const windows_core::GUID,
    pub dwOffset: u32,
}
impl Default for QITAB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SFBS_FLAGS(pub i32);
pub const SFBS_FLAGS_ROUND_TO_NEAREST_DISPLAYED_DIGIT: tagSFBS_FLAGS = 1;
pub const SFBS_FLAGS_TRUNCATE_UNDISPLAYED_DECIMAL_DIGITS: tagSFBS_FLAGS = 2;
pub const SHACF_AUTOAPPEND_FORCE_OFF: u32 = 2147483648;
pub const SHACF_AUTOAPPEND_FORCE_ON: u32 = 1073741824;
pub const SHACF_AUTOSUGGEST_FORCE_OFF: u32 = 536870912;
pub const SHACF_AUTOSUGGEST_FORCE_ON: u32 = 268435456;
pub const SHACF_DEFAULT: u32 = 0;
pub const SHACF_FILESYSTEM: u32 = 1;
pub const SHACF_FILESYS_DIRS: u32 = 32;
pub const SHACF_FILESYS_ONLY: u32 = 16;
pub const SHACF_URLALL: u32 = 6;
pub const SHACF_URLHISTORY: u32 = 2;
pub const SHACF_URLMRU: u32 = 4;
pub const SHACF_USETAB: u32 = 8;
pub const SHACF_VIRTUAL_NAMESPACE: u32 = 64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SHCT_FLAGS(pub u32);
pub type SHGLOBALCOUNTER = i32;
pub const SHGVSPB_ALLFOLDERS: u32 = 8;
pub const SHGVSPB_ALLUSERS: u32 = 2;
pub const SHGVSPB_FOLDER: u32 = 5;
pub const SHGVSPB_FOLDERNODEFAULTS: i32 = -2147483643;
pub const SHGVSPB_GLOBALDEFAULTS: u32 = 10;
pub const SHGVSPB_INHERIT: u32 = 16;
pub const SHGVSPB_NOAUTODEFAULTS: u32 = 2147483648;
pub const SHGVSPB_PERFOLDER: u32 = 4;
pub const SHGVSPB_PERUSER: u32 = 1;
pub const SHGVSPB_ROAM: u32 = 32;
pub const SHGVSPB_USERDEFAULTS: u32 = 9;
pub const SHREGDEL_BOTH: SHREGDEL_FLAGS = 17;
pub const SHREGDEL_DEFAULT: SHREGDEL_FLAGS = 0;
pub type SHREGDEL_FLAGS = i32;
pub const SHREGDEL_HKCU: SHREGDEL_FLAGS = 1;
pub const SHREGDEL_HKLM: SHREGDEL_FLAGS = 16;
pub const SHREGENUM_BOTH: SHREGENUM_FLAGS = 17;
pub const SHREGENUM_DEFAULT: SHREGENUM_FLAGS = 0;
pub type SHREGENUM_FLAGS = i32;
pub const SHREGENUM_HKCU: SHREGENUM_FLAGS = 1;
pub const SHREGENUM_HKLM: SHREGENUM_FLAGS = 16;
pub const SHREGSET_DEFAULT: u32 = 6;
pub const SHREGSET_FORCE_HKCU: u32 = 2;
pub const SHREGSET_FORCE_HKLM: u32 = 8;
pub const SHREGSET_HKCU: u32 = 1;
pub const SHREGSET_HKLM: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SRRF(pub i32);
pub const SRRF_NOEXPAND: u32 = 268435456;
pub const SRRF_NOVIRT: u32 = 1073741824;
pub const SRRF_RM_ANY: u32 = 0;
pub const SRRF_RM_NORMAL: u32 = 65536;
pub const SRRF_RM_SAFE: u32 = 131072;
pub const SRRF_RM_SAFENETWORK: u32 = 262144;
pub const SRRF_RT_ANY: u32 = 65535;
pub const SRRF_RT_DWORD: u32 = 24;
pub const SRRF_RT_QWORD: u32 = 72;
pub const SRRF_RT_REG_BINARY: u32 = 8;
pub const SRRF_RT_REG_DWORD: u32 = 16;
pub const SRRF_RT_REG_EXPAND_SZ: u32 = 4;
pub const SRRF_RT_REG_MULTI_SZ: u32 = 32;
pub const SRRF_RT_REG_NONE: u32 = 1;
pub const SRRF_RT_REG_QWORD: u32 = 64;
pub const SRRF_RT_REG_SZ: u32 = 2;
pub const SRRF_ZEROONFAILURE: u32 = 536870912;
pub const STIF_DEFAULT: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct STIF_FLAGS(pub i32);
pub const STIF_SUPPORT_HEX: u32 = 1;
pub const SZ_CONTENTTYPE_CDFA: windows_core::PCSTR = windows_core::s!("application/x-cdf");
pub const SZ_CONTENTTYPE_CDFW: windows_core::PCWSTR = windows_core::w!("application/x-cdf");
pub const SZ_CONTENTTYPE_HTMLA: windows_core::PCSTR = windows_core::s!("text/html");
pub const SZ_CONTENTTYPE_HTMLW: windows_core::PCWSTR = windows_core::w!("text/html");
pub type URLIS = i32;
pub const URLIS_APPLIABLE: URLIS = 4;
pub const URLIS_DIRECTORY: URLIS = 5;
pub const URLIS_FILEURL: URLIS = 3;
pub const URLIS_HASQUERY: URLIS = 6;
pub const URLIS_NOHISTORY: URLIS = 2;
pub const URLIS_OPAQUE: URLIS = 1;
pub const URLIS_URL: URLIS = 0;
pub const URL_APPLY_DEFAULT: u32 = 1;
pub const URL_APPLY_FORCEAPPLY: u32 = 8;
pub const URL_APPLY_GUESSFILE: u32 = 4;
pub const URL_APPLY_GUESSSCHEME: u32 = 2;
pub const URL_BROWSER_MODE: u32 = 33554432;
pub const URL_CONVERT_IF_DOSPATH: u32 = 2097152;
pub const URL_DONT_ESCAPE_EXTRA_INFO: u32 = 33554432;
pub const URL_DONT_SIMPLIFY: u32 = 134217728;
pub const URL_DONT_UNESCAPE: u32 = 131072;
pub const URL_DONT_UNESCAPE_EXTRA_INFO: u32 = 33554432;
pub const URL_ESCAPE_ASCII_URI_COMPONENT: u32 = 524288;
pub const URL_ESCAPE_AS_UTF8: u32 = 262144;
pub const URL_ESCAPE_PERCENT: u32 = 4096;
pub const URL_ESCAPE_SEGMENT_ONLY: u32 = 8192;
pub const URL_ESCAPE_SPACES_ONLY: u32 = 67108864;
pub const URL_ESCAPE_UNSAFE: u32 = 536870912;
pub const URL_ESCAPE_URI_COMPONENT: u32 = 786432;
pub const URL_FILE_USE_PATHURL: u32 = 65536;
pub const URL_INTERNAL_PATH: u32 = 8388608;
pub const URL_NO_META: u32 = 134217728;
pub type URL_PART = i32;
pub const URL_PARTFLAG_KEEPSCHEME: u32 = 1;
pub const URL_PART_HOSTNAME: URL_PART = 2;
pub const URL_PART_NONE: URL_PART = 0;
pub const URL_PART_PASSWORD: URL_PART = 4;
pub const URL_PART_PORT: URL_PART = 5;
pub const URL_PART_QUERY: URL_PART = 6;
pub const URL_PART_SCHEME: URL_PART = 1;
pub const URL_PART_USERNAME: URL_PART = 3;
pub const URL_PLUGGABLE_PROTOCOL: u32 = 1073741824;
pub type URL_SCHEME = i32;
pub const URL_SCHEME_ABOUT: URL_SCHEME = 17;
pub const URL_SCHEME_FILE: URL_SCHEME = 9;
pub const URL_SCHEME_FTP: URL_SCHEME = 1;
pub const URL_SCHEME_GOPHER: URL_SCHEME = 3;
pub const URL_SCHEME_HTTP: URL_SCHEME = 2;
pub const URL_SCHEME_HTTPS: URL_SCHEME = 11;
pub const URL_SCHEME_INVALID: URL_SCHEME = -1;
pub const URL_SCHEME_JAVASCRIPT: URL_SCHEME = 15;
pub const URL_SCHEME_KNOWNFOLDER: URL_SCHEME = 26;
pub const URL_SCHEME_LOCAL: URL_SCHEME = 14;
pub const URL_SCHEME_MAILTO: URL_SCHEME = 4;
pub const URL_SCHEME_MAXVALUE: URL_SCHEME = 27;
pub const URL_SCHEME_MK: URL_SCHEME = 10;
pub const URL_SCHEME_MSHELP: URL_SCHEME = 21;
pub const URL_SCHEME_MSSHELLDEVICE: URL_SCHEME = 22;
pub const URL_SCHEME_MSSHELLIDLIST: URL_SCHEME = 20;
pub const URL_SCHEME_MSSHELLROOTED: URL_SCHEME = 19;
pub const URL_SCHEME_NEWS: URL_SCHEME = 5;
pub const URL_SCHEME_NNTP: URL_SCHEME = 6;
pub const URL_SCHEME_RES: URL_SCHEME = 18;
pub const URL_SCHEME_SEARCH: URL_SCHEME = 25;
pub const URL_SCHEME_SEARCH_MS: URL_SCHEME = 24;
pub const URL_SCHEME_SHELL: URL_SCHEME = 12;
pub const URL_SCHEME_SNEWS: URL_SCHEME = 13;
pub const URL_SCHEME_TELNET: URL_SCHEME = 7;
pub const URL_SCHEME_UNKNOWN: URL_SCHEME = 0;
pub const URL_SCHEME_VBSCRIPT: URL_SCHEME = 16;
pub const URL_SCHEME_WAIS: URL_SCHEME = 8;
pub const URL_SCHEME_WILDCARD: URL_SCHEME = 23;
pub const URL_UNESCAPE: u32 = 268435456;
pub const URL_UNESCAPE_AS_UTF8: u32 = 262144;
pub const URL_UNESCAPE_HIGH_ANSI_ONLY: u32 = 4194304;
pub const URL_UNESCAPE_INPLACE: u32 = 1048576;
pub const URL_UNESCAPE_URI_COMPONENT: u32 = 262144;
pub const URL_WININET_COMPATIBILITY: u32 = 2147483648;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_CSCSYNCINPROGRESS: SHGLOBALCOUNTER = 13;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SERVERDRIVE: SHGLOBALCOUNTER = 42;
pub const __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEGLOBALDIRTYCOUNT: SHGLOBALCOUNTER = 43;
pub type tagSFBS_FLAGS = i32;
