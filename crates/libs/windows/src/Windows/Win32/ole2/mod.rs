#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn CreateOleAdviseHolder() -> windows_core::Result<super::LPOLEADVISEHOLDER> {
    windows_core::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateOleAdviseHolder(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared"))]
#[inline]
pub unsafe fn DoDragDrop<P0, P1>(pdataobj: P0, pdropsource: P1, dwokeffects: super::DWORD, pdweffect: super::LPDWORD) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P1: windows_core::Param<super::LPDROPSOURCE>,
{
    windows_core::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : *mut core::ffi::c_void, pdropsource : *mut core::ffi::c_void, dwokeffects : super::DWORD, pdweffect : super::LPDWORD) -> windows_core::HRESULT);
    unsafe { DoDragDrop(pdataobj.param().abi(), pdropsource.param().abi(), dwokeffects, pdweffect as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn IsAccelerator(haccel: super::HACCEL, caccelentries: i32, lpmsg: super::LPMSG, lpwcmd: *mut super::WORD) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::HACCEL, caccelentries : i32, lpmsg : super::LPMSG, lpwcmd : *mut super::WORD) -> windows_core::BOOL);
    unsafe { IsAccelerator(haccel as _, caccelentries, lpmsg as _, lpwcmd as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn OleBuildVersion() -> super::DWORD {
    windows_core::link!("ole32.dll" "system" fn OleBuildVersion() -> super::DWORD);
    unsafe { OleBuildVersion() }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAM<P0>(pstg: P0, lpolestream: LPOLESTREAM) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : *mut core::ffi::c_void, lpolestream : LPOLESTREAM) -> windows_core::HRESULT);
    unsafe { OleConvertIStorageToOLESTREAM(pstg.param().abi(), lpolestream as _) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx<P0>(pstg: P0, cfformat: super::CLIPFORMAT, lwidth: super::LONG, lheight: super::LONG, dwsize: super::DWORD, pmedium: super::LPSTGMEDIUM, polestm: LPOLESTREAM) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : *mut core::ffi::c_void, cfformat : super::CLIPFORMAT, lwidth : super::LONG, lheight : super::LONG, dwsize : super::DWORD, pmedium : super::LPSTGMEDIUM, polestm : LPOLESTREAM) -> windows_core::HRESULT);
    unsafe { OleConvertIStorageToOLESTREAMEx(pstg.param().abi(), cfformat, lwidth, lheight, dwsize, pmedium as _, polestm as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage<P1>(lpolestream: LPOLESTREAM, pstg: P1, ptd: *const super::DVTARGETDEVICE) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : LPOLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage(lpolestream as _, pstg.param().abi(), ptd) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage2<P1>(lpolestream: LPOLESTREAM, pstg: P1, ptd: Option<*const super::DVTARGETDEVICE>, opt: Option<super::DWORD>, pvcallbackcontext: Option<super::PVOID>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : LPOLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE, opt : super::DWORD, pvcallbackcontext : super::PVOID, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage2(lpolestream, pstg.param().abi(), ptd.unwrap_or(core::mem::zeroed()) as _, opt.unwrap_or(core::mem::zeroed()) as _, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pqueryconvertolelinkcallback) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx<P1>(polestm: LPOLESTREAM, pstg: P1, pcfformat: *mut super::CLIPFORMAT, plwwidth: *mut super::LONG, plheight: *mut super::LONG, pdwsize: *mut super::DWORD, pmedium: super::LPSTGMEDIUM) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : LPOLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut super::LONG, plheight : *mut super::LONG, pdwsize : *mut super::DWORD, pmedium : super::LPSTGMEDIUM) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx(polestm as _, pstg.param().abi(), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, pmedium as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx2<P1>(polestm: LPOLESTREAM, pstg: P1, pcfformat: *mut super::CLIPFORMAT, plwwidth: *mut super::LONG, plheight: *mut super::LONG, pdwsize: *mut super::DWORD, pmedium: super::LPSTGMEDIUM, opt: Option<super::DWORD>, pvcallbackcontext: Option<super::PVOID>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : LPOLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut super::LONG, plheight : *mut super::LONG, pdwsize : *mut super::DWORD, pmedium : super::LPSTGMEDIUM, opt : super::DWORD, pvcallbackcontext : super::PVOID, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx2(polestm, pstg.param().abi(), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, pmedium, opt.unwrap_or(core::mem::zeroed()) as _, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pqueryconvertolelinkcallback) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreate<P4, P5>(rclsid: *const super::IID, riid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreate(rclsid : *const super::IID, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreate(rclsid, riid, renderopt, pformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn OleCreateDefaultHandler<P1>(clsid: *const super::IID, punkouter: P1, riid: *const super::IID, lplpobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const super::IID, punkouter : *mut core::ffi::c_void, riid : *const super::IID, lplpobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateDefaultHandler(clsid, punkouter.param().abi(), riid, lplpobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
#[inline]
pub unsafe fn OleCreateEmbeddingHelper<P1, P3>(clsid: *const super::IID, punkouter: P1, flags: super::DWORD, pcf: P3, riid: *const super::IID, lplpobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPUNKNOWN>,
    P3: windows_core::Param<super::LPCLASSFACTORY>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const super::IID, punkouter : *mut core::ffi::c_void, flags : super::DWORD, pcf : *mut core::ffi::c_void, riid : *const super::IID, lplpobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateEmbeddingHelper(clsid, punkouter.param().abi(), flags, pcf.param().abi(), riid, lplpobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateEx<P7, P9, P10>(rclsid: *const super::IID, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut super::DWORD, pclientsite: P9, pstg: P10, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const super::IID, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateEx(rclsid, riid, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateFromData<P0, P4, P5>(psrcdataobj: P0, riid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromData(psrcdataobj.param().abi(), riid, renderopt, pformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateFromDataEx<P0, P7, P9, P10>(psrcdataobj: P0, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut super::DWORD, pclientsite: P9, pstg: P10, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromDataEx(psrcdataobj.param().abi(), riid, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateFromFile<P5, P6>(rclsid: *const super::IID, lpszfilename: super::LPCOLESTR, riid: *const super::IID, renderopt: super::DWORD, lpformatetc: super::LPFORMATETC, pclientsite: P5, pstg: P6, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P5: windows_core::Param<super::LPOLECLIENTSITE>,
    P6: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const super::IID, lpszfilename : super::LPCOLESTR, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromFile(rclsid, lpszfilename, riid, renderopt, lpformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateFromFileEx<P8, P10, P11>(rclsid: *const super::IID, lpszfilename: super::LPCOLESTR, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: P8, rgdwconnection: *mut super::DWORD, pclientsite: P10, pstg: P11, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P8: windows_core::Param<super::IAdviseSink>,
    P10: windows_core::Param<super::LPOLECLIENTSITE>,
    P11: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const super::IID, lpszfilename : super::LPCOLESTR, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromFileEx(rclsid, lpszfilename, riid, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLink<P0, P4, P5>(pmklinksrc: P0, riid: *const super::IID, renderopt: super::DWORD, lpformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : *mut core::ffi::c_void, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLink(pmklinksrc.param().abi(), riid, renderopt, lpformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkEx<P0, P7, P9, P10>(pmklinksrc: P0, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut super::DWORD, pclientsite: P9, pstg: P10, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : *mut core::ffi::c_void, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkEx(pmklinksrc.param().abi(), riid, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkFromData<P0, P4, P5>(psrcdataobj: P0, riid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkFromData(psrcdataobj.param().abi(), riid, renderopt, pformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkFromDataEx<P0, P7, P9, P10>(psrcdataobj: P0, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut super::DWORD, pclientsite: P9, pstg: P10, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkFromDataEx(psrcdataobj.param().abi(), riid, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateLinkToFile<P4, P5>(lpszfilename: super::LPCOLESTR, riid: *const super::IID, renderopt: super::DWORD, lpformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : super::LPCOLESTR, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkToFile(lpszfilename, riid, renderopt, lpformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateLinkToFileEx<P7, P9, P10>(lpszfilename: super::LPCOLESTR, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: P7, rgdwconnection: *mut super::DWORD, pclientsite: P9, pstg: P10, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P7: windows_core::Param<super::IAdviseSink>,
    P9: windows_core::Param<super::LPOLECLIENTSITE>,
    P10: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : super::LPCOLESTR, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkToFileEx(lpszfilename, riid, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, lpadvisesink.param().abi(), rgdwconnection as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn OleCreateMenuDescriptor(hmenucombined: super::HMENU, lpmenuwidths: super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU {
    windows_core::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::HMENU, lpmenuwidths : super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU);
    unsafe { OleCreateMenuDescriptor(hmenucombined as _, lpmenuwidths as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateStaticFromData<P0, P4, P5>(psrcdataobj: P0, iid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: P4, pstg: P5, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPDATAOBJECT>,
    P4: windows_core::Param<super::LPOLECLIENTSITE>,
    P5: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : *mut core::ffi::c_void, iid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateStaticFromData(psrcdataobj.param().abi(), iid, renderopt, pformatetc as _, pclientsite.param().abi(), pstg.param().abi(), ppvobj as _) }
}
#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn OleDestroyMenuDescriptor(holemenu: super::HOLEMENU) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : super::HOLEMENU) -> windows_core::HRESULT);
    unsafe { OleDestroyMenuDescriptor(holemenu as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl"))]
#[inline]
pub unsafe fn OleDoAutoConvert<P0>(pstg: P0, pclsidnew: super::LPCLSID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : *mut core::ffi::c_void, pclsidnew : super::LPCLSID) -> windows_core::HRESULT);
    unsafe { OleDoAutoConvert(pstg.param().abi(), pclsidnew as _) }
}
#[cfg(all(feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[inline]
pub unsafe fn OleDraw<P0>(punknown: P0, dwaspect: super::DWORD, hdcdraw: super::HDC, lprcbounds: super::LPCRECT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn OleDraw(punknown : *mut core::ffi::c_void, dwaspect : super::DWORD, hdcdraw : super::HDC, lprcbounds : super::LPCRECT) -> windows_core::HRESULT);
    unsafe { OleDraw(punknown.param().abi(), dwaspect, hdcdraw as _, lprcbounds) }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "wtypes"))]
#[inline]
pub unsafe fn OleDuplicateData(hsrc: super::HANDLE, cfformat: super::CLIPFORMAT, uiflags: super::UINT) -> super::HANDLE {
    windows_core::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::HANDLE, cfformat : super::CLIPFORMAT, uiflags : super::UINT) -> super::HANDLE);
    unsafe { OleDuplicateData(hsrc as _, cfformat, uiflags) }
}
#[inline]
pub unsafe fn OleFlushClipboard() -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_core::HRESULT);
    unsafe { OleFlushClipboard() }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn OleGetAutoConvert(clsidold: *const super::IID, pclsidnew: super::LPCLSID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const super::IID, pclsidnew : super::LPCLSID) -> windows_core::HRESULT);
    unsafe { OleGetAutoConvert(clsidold, pclsidnew as _) }
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleGetIconOfClass(rclsid: *const super::IID, lpszlabel: Option<super::LPOLESTR>, fusetypeaslabel: bool) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const super::IID, lpszlabel : super::LPOLESTR, fusetypeaslabel : windows_core::BOOL) -> super::HGLOBAL);
    unsafe { OleGetIconOfClass(rclsid, lpszlabel.unwrap_or(core::mem::zeroed()) as _, fusetypeaslabel.into()) }
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleGetIconOfFile(lpszpath: super::LPOLESTR, fusefileaslabel: bool) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : super::LPOLESTR, fusefileaslabel : windows_core::BOOL) -> super::HGLOBAL);
    unsafe { OleGetIconOfFile(lpszpath, fusefileaslabel.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn OleInitialize(pvreserved: super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleInitialize(pvreserved : super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleInitialize(pvreserved as _) }
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl"))]
#[inline]
pub unsafe fn OleLoad<P0, P2>(pstg: P0, riid: *const super::IID, pclientsite: P2, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
    P2: windows_core::Param<super::LPOLECLIENTSITE>,
{
    windows_core::link!("ole32.dll" "system" fn OleLoad(pstg : *mut core::ffi::c_void, riid : *const super::IID, pclientsite : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleLoad(pstg.param().abi(), riid, pclientsite.param().abi(), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
#[inline]
pub unsafe fn OleLoadFromStream<P0>(pstm: P0, iidinterface: *const super::IID, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTREAM>,
{
    windows_core::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : *mut core::ffi::c_void, iidinterface : *const super::IID, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleLoadFromStream(pstm.param().abi(), iidinterface, ppvobj as _) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleLockRunning<P0>(punknown: P0, flock: bool, flastunlockcloses: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn OleLockRunning(punknown : *mut core::ffi::c_void, flock : windows_core::BOOL, flastunlockcloses : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleLockRunning(punknown.param().abi(), flock.into(), flastunlockcloses.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleMetafilePictFromIconAndLabel(hicon: super::HICON, lpszlabel: super::LPOLESTR, lpszsourcefile: super::LPOLESTR, iiconindex: super::UINT) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::HICON, lpszlabel : super::LPOLESTR, lpszsourcefile : super::LPOLESTR, iiconindex : super::UINT) -> super::HGLOBAL);
    unsafe { OleMetafilePictFromIconAndLabel(hicon as _, lpszlabel, lpszsourcefile, iiconindex) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleNoteObjectVisible<P0>(punknown: P0, fvisible: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
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
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn OleRegEnumFormatEtc(clsid: *const super::IID, dwdirection: super::DWORD) -> windows_core::Result<super::LPENUMFORMATETC> {
    windows_core::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const super::IID, dwdirection : super::DWORD, ppenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegEnumFormatEtc(clsid, dwdirection, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "guiddef", feature = "oleidl"))]
#[inline]
pub unsafe fn OleRegEnumVerbs(clsid: *const super::IID) -> windows_core::Result<super::LPENUMOLEVERB> {
    windows_core::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const super::IID, ppenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegEnumVerbs(clsid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn OleRegGetMiscStatus(clsid: *const super::IID, dwaspect: super::DWORD, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const super::IID, dwaspect : super::DWORD, pdwstatus : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { OleRegGetMiscStatus(clsid, dwaspect, pdwstatus as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleRegGetUserType(clsid: *const super::IID, dwformoftype: super::DWORD) -> windows_core::Result<super::LPOLESTR> {
    windows_core::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const super::IID, dwformoftype : super::DWORD, pszusertype : *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        OleRegGetUserType(clsid, dwformoftype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleRun<P0>(punknown: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
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
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn OleSetAutoConvert(clsidold: *const super::IID, clsidnew: *const super::IID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const super::IID, clsidnew : *const super::IID) -> windows_core::HRESULT);
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
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleSetContainedObject<P0>(punknown: P0, fcontained: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPUNKNOWN>,
{
    windows_core::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : *mut core::ffi::c_void, fcontained : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleSetContainedObject(punknown.param().abi(), fcontained.into()) }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
#[inline]
pub unsafe fn OleSetMenuDescriptor<P3, P4>(holemenu: super::HOLEMENU, hwndframe: super::HWND, hwndactiveobject: super::HWND, lpframe: P3, lpactiveobj: P4) -> windows_core::HRESULT
where
    P3: windows_core::Param<super::LPOLEINPLACEFRAME>,
    P4: windows_core::Param<super::LPOLEINPLACEACTIVEOBJECT>,
{
    windows_core::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : super::HOLEMENU, hwndframe : super::HWND, hwndactiveobject : super::HWND, lpframe : *mut core::ffi::c_void, lpactiveobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSetMenuDescriptor(holemenu as _, hwndframe as _, hwndactiveobject as _, lpframe.param().abi(), lpactiveobj.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn OleTranslateAccelerator<P0>(lpframe: P0, lpframeinfo: super::LPOLEINPLACEFRAMEINFO, lpmsg: super::LPMSG) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPOLEINPLACEFRAME>,
{
    windows_core::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : *mut core::ffi::c_void, lpframeinfo : super::LPOLEINPLACEFRAMEINFO, lpmsg : super::LPMSG) -> windows_core::HRESULT);
    unsafe { OleTranslateAccelerator(lpframe.param().abi(), lpframeinfo as _, lpmsg as _) }
}
#[inline]
pub unsafe fn OleUninitialize() {
    windows_core::link!("ole32.dll" "system" fn OleUninitialize());
    unsafe { OleUninitialize() }
}
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ReadFmtUserTypeStg<P0>(pstg: P0, pcf: *mut super::CLIPFORMAT, lplpszusertype: Option<*mut super::LPOLESTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPSTORAGE>,
{
    windows_core::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : *mut core::ffi::c_void, pcf : *mut super::CLIPFORMAT, lplpszusertype : *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { ReadFmtUserTypeStg(pstg.param().abi(), pcf as _, lplpszusertype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
#[inline]
pub unsafe fn RegisterDragDrop<P1>(hwnd: super::HWND, pdroptarget: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::LPDROPTARGET>,
{
    windows_core::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::HWND, pdroptarget : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RegisterDragDrop(hwnd as _, pdroptarget.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ReleaseStgMedium(param0: super::LPSTGMEDIUM) {
    windows_core::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : super::LPSTGMEDIUM));
    unsafe { ReleaseStgMedium(param0 as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn RevokeDragDrop(hwnd: super::HWND) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::HWND) -> windows_core::HRESULT);
    unsafe { RevokeDragDrop(hwnd as _) }
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
#[cfg(feature = "shared")]
pub type LPOLESTREAM = *mut OLESTREAM;
#[cfg(feature = "shared")]
pub type LPOLESTREAMVTBL = *mut OLESTREAMVTBL;
pub const OLECREATE_LEAVERUNNING: i32 = 1;
pub const OLEIVERB_DISCARDUNDOSTATE: i32 = -6;
pub const OLEIVERB_HIDE: i32 = -3;
pub const OLEIVERB_INPLACEACTIVATE: i32 = -5;
pub const OLEIVERB_OPEN: i32 = -2;
pub const OLEIVERB_PRIMARY: i32 = 0;
pub const OLEIVERB_SHOW: i32 = -1;
pub const OLEIVERB_UIACTIVATE: i32 = -4;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct OLESTREAM {
    pub lpstbl: LPOLESTREAMVTBL,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: super::LPCLSID, szclass: super::LPOLESTR, sztopicname: super::LPOLESTR, szitemname: super::LPOLESTR, szuncname: super::LPOLESTR, linkupdatingoption: super::ULONG, pvcontext: super::PVOID) -> windows_core::HRESULT>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct OLESTREAMVTBL {
    pub Get: Option<unsafe extern "system" fn(LPOLESTREAM, *mut core::ffi::c_void, super::DWORD) -> super::DWORD>,
    pub Put: Option<unsafe extern "system" fn(LPOLESTREAM, *const core::ffi::c_void, super::DWORD) -> super::DWORD>,
}
pub const OLESTREAM_CONVERSION_DEFAULT: i32 = 0;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: i32 = 1;
