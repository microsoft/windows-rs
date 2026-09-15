#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn CreateOleAdviseHolder() -> windows_core::Result<super::LPOLEADVISEHOLDER> {
    windows_core::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateOleAdviseHolder(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl"))]
#[inline]
pub unsafe fn DoDragDrop<P0, P1>(pdataobj: P0, pdropsource: P1, dwokeffects: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P1: windows_core::Param<super::LPDROPSOURCE>,
{
    windows_core::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : *mut core::ffi::c_void, pdropsource : *mut core::ffi::c_void, dwokeffects : u32, pdweffect : super::LPDWORD) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DoDragDrop(pdataobj.param().abi(), pdropsource.param().abi(), dwokeffects, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn IsAccelerator(haccel: super::HACCEL, caccelentries: i32, lpmsg: super::LPMSG, lpwcmd: *mut u16) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::HACCEL, caccelentries : i32, lpmsg : super::LPMSG, lpwcmd : *mut u16) -> windows_core::BOOL);
    unsafe { IsAccelerator(haccel, caccelentries, lpmsg, lpwcmd as _) }
}
#[inline]
pub unsafe fn OleBuildVersion() -> u32 {
    windows_core::link!("ole32.dll" "system" fn OleBuildVersion() -> u32);
    unsafe { OleBuildVersion() }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<P0>(pstg: P0) -> windows_core::Result<OLESTREAM>
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : *mut core::ffi::c_void, lpolestream : LPOLESTREAM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleConvertIStorageToOLESTREAM(pstg.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<P0>(pstg: P0, cfformat: super::CLIPFORMAT, lwidth: i32, lheight: i32, dwsize: u32, pmedium: super::LPSTGMEDIUM) -> windows_core::Result<OLESTREAM>
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : *mut core::ffi::c_void, cfformat : super::CLIPFORMAT, lwidth : i32, lheight : i32, dwsize : u32, pmedium : super::LPSTGMEDIUM, polestm : LPOLESTREAM) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleConvertIStorageToOLESTREAMEx(pstg.param().abi(), cfformat, lwidth, lheight, dwsize, pmedium, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage(lpolestream: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, ptd: *const super::DVTARGETDEVICE) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : LPOLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage(lpolestream, core::mem::transmute_copy(pstg), ptd) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage2(lpolestream: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, ptd: Option<*const super::DVTARGETDEVICE>, opt: Option<u32>, pvcallbackcontext: Option<*const core::ffi::c_void>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : LPOLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage2(lpolestream, core::mem::transmute_copy(pstg), ptd.unwrap_or(core::mem::zeroed()) as _, opt.unwrap_or(core::mem::zeroed()) as _, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pqueryconvertolelinkcallback) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx(polestm: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, pcfformat: *mut super::CLIPFORMAT, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: super::LPSTGMEDIUM) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : LPOLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : super::LPSTGMEDIUM) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx(polestm, core::mem::transmute_copy(pstg), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, pmedium) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx2(polestm: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, pcfformat: *mut super::CLIPFORMAT, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: super::LPSTGMEDIUM, opt: Option<u32>, pvcallbackcontext: Option<*const core::ffi::c_void>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : LPOLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : super::LPSTGMEDIUM, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx2(polestm, core::mem::transmute_copy(pstg), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, pmedium, opt.unwrap_or(core::mem::zeroed()) as _, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pqueryconvertolelinkcallback) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreate<P4, P5>(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, renderopt: u32, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreate(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreate(rclsid, riid, renderopt, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[inline]
pub unsafe fn OleCreateDefaultHandler<P1>(clsid: *const windows_core::GUID, punkouter: P1, riid: *const windows_core::GUID, lplpobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateDefaultHandler(clsid, punkouter.param().abi(), riid, lplpobj as _) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleCreateEmbeddingHelper<P1, P3>(clsid: *const windows_core::GUID, punkouter: P1, flags: u32, pcf: P3, riid: *const windows_core::GUID, lplpobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::IUnknown>,
    P3: windows_core::Param<super::LPCLASSFACTORY>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, flags : u32, pcf : *mut core::ffi::c_void, riid : *const windows_core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateEmbeddingHelper(clsid, punkouter.param().abi(), flags, pcf.param().abi(), riid, lplpobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateEx<P7, P9, P10>(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *const u32, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut u32, pclientsite: P9, pstg: P10, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateEx(rclsid, riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateFromData<P0, P4, P5>(psrcdataobj: P0, riid: *const windows_core::GUID, renderopt: u32, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateFromData(psrcdataobj.param().abi(), riid, renderopt, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateFromDataEx<P0, P7, P9, P10>(psrcdataobj: P0, riid: *const windows_core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *const u32, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut u32, pclientsite: P9, pstg: P10, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateFromDataEx(psrcdataobj.param().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateFromFile<P5, P6>(rclsid: *const windows_core::GUID, lpszfilename: super::LPCOLESTR, riid: *const windows_core::GUID, renderopt: u32, lpformatetc: super::LPFORMATETC, pclientsite: P5, pstg: P6, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P5: windows_core::Param<super::LPOLECLIENTSITE>,
    P6: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const windows_core::GUID, lpszfilename : super::LPCOLESTR, riid : *const windows_core::GUID, renderopt : u32, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateFromFile(rclsid, lpszfilename, riid, renderopt, lpformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateFromFileEx<P8, P10, P11>(rclsid: *const windows_core::GUID, lpszfilename: super::LPCOLESTR, riid: *const windows_core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *const u32, rgformatetc: super::LPFORMATETC, lpadvisesink: P8, rgdwconnection: *mut u32, pclientsite: P10, pstg: P11, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P8: windows_core::Param<super::IAdviseSink>,
    P10: windows_core::Param<super::LPOLECLIENTSITE>,
    P11: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const windows_core::GUID, lpszfilename : super::LPCOLESTR, riid : *const windows_core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateFromFileEx(rclsid, lpszfilename, riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLink<P0, P4, P5>(pmklinksrc: P0, riid: *const windows_core::GUID, renderopt: u32, lpformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : *mut core::ffi::c_void, riid : *const windows_core::GUID, renderopt : u32, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateLink(pmklinksrc.param().abi(), riid, renderopt, lpformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkEx<P0, P7, P9, P10>(pmklinksrc: P0, riid: *const windows_core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *const u32, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut u32, pclientsite: P9, pstg: P10, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : *mut core::ffi::c_void, riid : *const windows_core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateLinkEx(pmklinksrc.param().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkFromData<P0, P4, P5>(psrcdataobj: P0, riid: *const windows_core::GUID, renderopt: u32, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateLinkFromData(psrcdataobj.param().abi(), riid, renderopt, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkFromDataEx<P0, P7, P9, P10>(psrcdataobj: P0, riid: *const windows_core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *const u32, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut u32, pclientsite: P9, pstg: P10, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const windows_core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateLinkFromDataEx(psrcdataobj.param().abi(), riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateLinkToFile<P4, P5>(lpszfilename: super::LPCOLESTR, riid: *const windows_core::GUID, renderopt: u32, lpformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : super::LPCOLESTR, riid : *const windows_core::GUID, renderopt : u32, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateLinkToFile(lpszfilename, riid, renderopt, lpformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateLinkToFileEx<P7, P9, P10>(lpszfilename: super::LPCOLESTR, riid: *const windows_core::GUID, dwflags: u32, renderopt: u32, cformats: u32, rgadvf: *const u32, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut u32, pclientsite: P9, pstg: P10, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : super::LPCOLESTR, riid : *const windows_core::GUID, dwflags : u32, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateLinkToFileEx(lpszfilename, riid, dwflags, renderopt, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn OleCreateMenuDescriptor(hmenucombined: super::HMENU, lpmenuwidths: super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU {
    windows_core::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::HMENU, lpmenuwidths : super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU);
    unsafe { OleCreateMenuDescriptor(hmenucombined, lpmenuwidths) }
}
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateStaticFromData<P0, P4, P5>(psrcdataobj: P0, iid: *const windows_core::GUID, renderopt: u32, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : *mut core::ffi::c_void, iid : *const windows_core::GUID, renderopt : u32, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleCreateStaticFromData(psrcdataobj.param().abi(), iid, renderopt, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "winnt"))]
#[inline]
pub unsafe fn OleDestroyMenuDescriptor(holemenu: super::HOLEMENU) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : super::HOLEMENU) -> windows_core::HRESULT);
    unsafe { OleDestroyMenuDescriptor(holemenu) }
}
#[cfg(all(feature = "guiddef", feature = "objidl"))]
#[inline]
pub unsafe fn OleDoAutoConvert<P0>(pstg: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : *mut core::ffi::c_void, pclsidnew : super::LPCLSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleDoAutoConvert(pstg.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn OleDraw<P0>(punknown: P0, dwaspect: u32, hdcdraw: super::HDC, lprcbounds: super::LPCRECT) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn OleDraw(punknown : *mut core::ffi::c_void, dwaspect : u32, hdcdraw : super::HDC, lprcbounds : super::LPCRECT) -> windows_core::HRESULT);
    unsafe { OleDraw(punknown.param().abi(), dwaspect, hdcdraw, lprcbounds) }
}
#[cfg(all(feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn OleDuplicateData(hsrc: super::HANDLE, cfformat: super::CLIPFORMAT, uiflags: u32) -> super::HANDLE {
    windows_core::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::HANDLE, cfformat : super::CLIPFORMAT, uiflags : u32) -> super::HANDLE);
    unsafe { OleDuplicateData(hsrc, cfformat, uiflags) }
}
#[inline]
pub unsafe fn OleFlushClipboard() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_core::HRESULT);
    unsafe { OleFlushClipboard() }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn OleGetAutoConvert(clsidold: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_core::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const windows_core::GUID, pclsidnew : super::LPCLSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleGetAutoConvert(clsidold, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleGetClipboard() -> windows_core::Result<super::LPDATAOBJECT> {
    windows_core::link!("ole32.dll" "system" fn OleGetClipboard(ppdataobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleGetClipboard(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut Option<super::IDataObject>, dataenterpriseid: *mut windows_core::PWSTR, sourcedescription: *mut windows_core::PWSTR, targetdescription: *mut windows_core::PWSTR, datadescription: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleGetClipboardWithEnterpriseInfo(dataobject : *mut *mut core::ffi::c_void, dataenterpriseid : *mut windows_core::PWSTR, sourcedescription : *mut windows_core::PWSTR, targetdescription : *mut windows_core::PWSTR, datadescription : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { OleGetClipboardWithEnterpriseInfo(core::mem::transmute(dataobject), dataenterpriseid as _, sourcedescription as _, targetdescription as _, datadescription as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleGetIconOfClass(rclsid: *const windows_core::GUID, lpszlabel: Option<super::LPOLESTR>, fusetypeaslabel: bool) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const windows_core::GUID, lpszlabel : super::LPOLESTR, fusetypeaslabel : windows_core::BOOL) -> super::HGLOBAL);
    unsafe { OleGetIconOfClass(rclsid, lpszlabel.unwrap_or(core::mem::zeroed()) as _, fusetypeaslabel.into()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleGetIconOfFile(lpszpath: super::LPOLESTR, fusefileaslabel: bool) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : super::LPOLESTR, fusefileaslabel : windows_core::BOOL) -> super::HGLOBAL);
    unsafe { OleGetIconOfFile(lpszpath, fusefileaslabel.into()) }
}
#[inline]
pub unsafe fn OleInitialize(pvreserved: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleInitialize(pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleInitialize(pvreserved) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleIsCurrentClipboard<P0>(pdataobj: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleIsCurrentClipboard(pdataobj.param().abi()) }
}
#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn OleIsRunning<P0>(pobject: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<super::LPOLEOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleIsRunning(pobject : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { OleIsRunning(pobject.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "oleidl"))]
#[inline]
pub unsafe fn OleLoad<P0, P2>(pstg: P0, riid: *const windows_core::GUID, pclientsite: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
    P2: windows_core::Param<super::LPOLECLIENTSITE>,
{
    windows_core::link!("ole32.dll" "system" fn OleLoad(pstg : *mut core::ffi::c_void, riid : *const windows_core::GUID, pclientsite : *mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleLoad(pstg.param().abi(), riid, pclientsite.param().abi(), ppvobj as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn OleLoadFromStream<P0>(pstm: P0, iidinterface: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTREAM>,
{
    windows_core::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : *mut core::ffi::c_void, iidinterface : *const windows_core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleLoadFromStream(pstm.param().abi(), iidinterface, ppvobj as _) }
}
#[inline]
pub unsafe fn OleLockRunning<P0>(punknown: P0, flock: bool, flastunlockcloses: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn OleLockRunning(punknown : *mut core::ffi::c_void, flock : windows_core::BOOL, flastunlockcloses : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleLockRunning(punknown.param().abi(), flock.into(), flastunlockcloses.into()) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleMetafilePictFromIconAndLabel(hicon: super::HICON, lpszlabel: super::LPOLESTR, lpszsourcefile: super::LPOLESTR, iiconindex: u32) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::HICON, lpszlabel : super::LPOLESTR, lpszsourcefile : super::LPOLESTR, iiconindex : u32) -> super::HGLOBAL);
    unsafe { OleMetafilePictFromIconAndLabel(hicon, lpszlabel, lpszsourcefile, iiconindex) }
}
#[inline]
pub unsafe fn OleNoteObjectVisible<P0>(punknown: P0, fvisible: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : *mut core::ffi::c_void, fvisible : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleNoteObjectVisible(punknown.param().abi(), fvisible.into()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleQueryCreateFromData<P0>(psrcdataobject: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleQueryCreateFromData(psrcdataobject.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleQueryLinkFromData<P0>(psrcdataobject: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleQueryLinkFromData(psrcdataobject.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleRegEnumFormatEtc(clsid: *const windows_core::GUID, dwdirection: u32) -> windows_core::Result<super::LPENUMFORMATETC> {
    windows_core::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const windows_core::GUID, dwdirection : u32, ppenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegEnumFormatEtc(clsid, dwdirection, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn OleRegEnumVerbs(clsid: *const windows_core::GUID) -> windows_core::Result<super::LPENUMOLEVERB> {
    windows_core::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const windows_core::GUID, ppenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegEnumVerbs(clsid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn OleRegGetMiscStatus(clsid: *const windows_core::GUID, dwaspect: u32) -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const windows_core::GUID, dwaspect : u32, pdwstatus : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegGetMiscStatus(clsid, dwaspect, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn OleRegGetUserType(clsid: *const windows_core::GUID, dwformoftype: u32) -> windows_core::Result<super::LPOLESTR> {
    windows_core::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const windows_core::GUID, dwformoftype : u32, pszusertype : *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegGetUserType(clsid, dwformoftype, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn OleRun<P0>(punknown: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn OleRun(punknown : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleRun(punknown.param().abi()) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleSave<P0, P1>(pps: P0, pstg: P1, fsameasload: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPPERSISTSTORAGE>,
    P1: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleSave(pps : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, fsameasload : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleSave(pps.param().abi(), pstg.param().abi(), fsameasload.into()) }
}
#[cfg(all(feature = "objidl", feature = "objidlbase"))]
#[inline]
pub unsafe fn OleSaveToStream<P0, P1>(ppstm: P0, pstm: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPPERSISTSTREAM>,
    P1: windows_core::Param<super::LPSTREAM>,
{
    windows_core::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : *mut core::ffi::c_void, pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSaveToStream(ppstm.param().abi(), pstm.param().abi()) }
}
#[inline]
pub unsafe fn OleSetAutoConvert(clsidold: *const windows_core::GUID, clsidnew: *const windows_core::GUID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const windows_core::GUID, clsidnew : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { OleSetAutoConvert(clsidold, clsidnew) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleSetClipboard<P0>(pdataobj: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSetClipboard(pdataobj.param().abi()) }
}
#[inline]
pub unsafe fn OleSetContainedObject<P0>(punknown: P0, fcontained: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : *mut core::ffi::c_void, fcontained : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleSetContainedObject(punknown.param().abi(), fcontained.into()) }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn OleSetMenuDescriptor<P3, P4>(holemenu: super::HOLEMENU, hwndframe: super::HWND, hwndactiveobject: super::HWND, lpframe: P3, lpactiveobj: P4) -> windows_core::HRESULT
where
    P3: windows_core::Param<super::LPOLEINPLACEFRAME>,
    P4: windows_core::Param<super::LPOLEINPLACEACTIVEOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : super::HOLEMENU, hwndframe : super::HWND, hwndactiveobject : super::HWND, lpframe : *mut core::ffi::c_void, lpactiveobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSetMenuDescriptor(holemenu, hwndframe, hwndactiveobject, lpframe.param().abi(), lpactiveobj.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn OleTranslateAccelerator<P0>(lpframe: P0, lpframeinfo: super::LPOLEINPLACEFRAMEINFO, lpmsg: super::LPMSG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPOLEINPLACEFRAME>,
{
    windows_core::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : *mut core::ffi::c_void, lpframeinfo : super::LPOLEINPLACEFRAMEINFO, lpmsg : super::LPMSG) -> windows_core::HRESULT);
    unsafe { OleTranslateAccelerator(lpframe.param().abi(), lpframeinfo, lpmsg) }
}
#[inline]
pub unsafe fn OleUninitialize() {
    windows_core::link!("ole32.dll" "system" fn OleUninitialize());
    unsafe { OleUninitialize() }
}
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ReadFmtUserTypeStg<P0>(pstg: P0, pcf: *mut super::CLIPFORMAT, lplpszusertype: *mut super::LPOLESTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : *mut core::ffi::c_void, pcf : *mut super::CLIPFORMAT, lplpszusertype : *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { ReadFmtUserTypeStg(pstg.param().abi(), pcf as _, lplpszusertype as _) }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
#[inline]
pub unsafe fn RegisterDragDrop<P1>(hwnd: super::HWND, pdroptarget: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPDROPTARGET>,
{
    windows_core::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::HWND, pdroptarget : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RegisterDragDrop(hwnd, pdroptarget.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ReleaseStgMedium(param0: super::LPSTGMEDIUM) {
    windows_core::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : super::LPSTGMEDIUM));
    unsafe { ReleaseStgMedium(param0) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RevokeDragDrop(hwnd: super::HWND) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::HWND) -> windows_core::HRESULT);
    unsafe { RevokeDragDrop(hwnd) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn SetConvertStg<P0>(pstg: P0, fconvert: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn SetConvertStg(pstg : *mut core::ffi::c_void, fconvert : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { SetConvertStg(pstg.param().abi(), fconvert.into()) }
}
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WriteFmtUserTypeStg<P0>(pstg: P0, cf: super::CLIPFORMAT, lpszusertype: super::LPOLESTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn WriteFmtUserTypeStg(pstg : *mut core::ffi::c_void, cf : super::CLIPFORMAT, lpszusertype : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { WriteFmtUserTypeStg(pstg.param().abi(), cf, lpszusertype) }
}
pub const DATA_E_FORMATETC: windows_core::HRESULT = windows_core::HRESULT(0x80040064_u32 as _);
pub const EMBDHLP_CREATENOW: i32 = 0;
pub const EMBDHLP_DELAYCREATE: i32 = 65536;
pub const EMBDHLP_INPROC_HANDLER: i32 = 0;
pub const EMBDHLP_INPROC_SERVER: i32 = 1;
pub const E_DRAW: windows_core::HRESULT = windows_core::HRESULT(0x80040140_u32 as _);
pub type LPOLESTREAM = *mut OLESTREAM;
pub type LPOLESTREAMVTBL = *mut OLESTREAMVTBL;
pub const OLECREATE_LEAVERUNNING: i32 = 1;
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6;
pub const OLEIVERB_HIDE: i32 = -3;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5;
pub const OLEIVERB_OPEN: i32 = -2;
pub const OLEIVERB_PRIMARY: i32 = 0;
pub const OLEIVERB_SHOW: i32 = -1;
pub const OLEIVERB_UIACTIVATE: i32 = -4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OLESTREAM {
    pub lpstbl: LPOLESTREAMVTBL,
}
#[cfg(all(feature = "guiddef", feature = "wtypesbase"))]
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: super::LPCLSID, szclass: super::LPOLESTR, sztopicname: super::LPOLESTR, szitemname: super::LPOLESTR, szuncname: super::LPOLESTR, linkupdatingoption: u32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OLESTREAMVTBL {
    pub Get: *mut u8,
    pub Put: *mut u8,
}
pub const OLESTREAM_CONVERSION_DEFAULT: i32 = 0;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: i32 = 1;
