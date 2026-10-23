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
pub unsafe fn DoDragDrop(pdataobj: &Option<super::LPDATAOBJECT>, pdropsource: &Option<super::LPDROPSOURCE>, dwokeffects: super::DWORD, pdweffect: super::LPDWORD) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : *mut core::ffi::c_void, pdropsource : *mut core::ffi::c_void, dwokeffects : super::DWORD, pdweffect : super::LPDWORD) -> windows_core::HRESULT);
    unsafe { DoDragDrop(core::mem::transmute_copy(pdataobj), core::mem::transmute_copy(pdropsource), dwokeffects, pdweffect as _) }
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
pub unsafe fn OleConvertIStorageToOLESTREAM(pstg: &Option<super::LPSTORAGE>, lpolestream: LPOLESTREAM) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAM(pstg : *mut core::ffi::c_void, lpolestream : LPOLESTREAM) -> windows_core::HRESULT);
    unsafe { OleConvertIStorageToOLESTREAM(core::mem::transmute_copy(pstg), lpolestream as _) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertIStorageToOLESTREAMEx(pstg: &Option<super::LPSTORAGE>, cfformat: super::CLIPFORMAT, lwidth: super::LONG, lheight: super::LONG, dwsize: super::DWORD, pmedium: super::LPSTGMEDIUM, polestm: LPOLESTREAM) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertIStorageToOLESTREAMEx(pstg : *mut core::ffi::c_void, cfformat : super::CLIPFORMAT, lwidth : super::LONG, lheight : super::LONG, dwsize : super::DWORD, pmedium : super::LPSTGMEDIUM, polestm : LPOLESTREAM) -> windows_core::HRESULT);
    unsafe { OleConvertIStorageToOLESTREAMEx(core::mem::transmute_copy(pstg), cfformat, lwidth, lheight, dwsize, pmedium as _, polestm as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage(lpolestream: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, ptd: *const super::DVTARGETDEVICE) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage(lpolestream : LPOLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage(lpolestream as _, core::mem::transmute_copy(pstg), ptd as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage2(lpolestream: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, ptd: Option<*const super::DVTARGETDEVICE>, opt: Option<super::DWORD>, pvcallbackcontext: Option<super::PVOID>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : LPOLESTREAM, pstg : *mut core::ffi::c_void, ptd : *const super::DVTARGETDEVICE, opt : super::DWORD, pvcallbackcontext : super::PVOID, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorage2(lpolestream, core::mem::transmute_copy(pstg), ptd.unwrap_or(core::mem::zeroed()) as _, opt.unwrap_or(core::mem::zeroed()) as _, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pqueryconvertolelinkcallback) }
}
#[cfg(all(feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx(polestm: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, pcfformat: *mut super::CLIPFORMAT, plwwidth: *mut super::LONG, plheight: *mut super::LONG, pdwsize: *mut super::DWORD, pmedium: super::LPSTGMEDIUM) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx(polestm : LPOLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut super::LONG, plheight : *mut super::LONG, pdwsize : *mut super::DWORD, pmedium : super::LPSTGMEDIUM) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx(polestm as _, core::mem::transmute_copy(pstg), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, pmedium as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "objidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx2(polestm: LPOLESTREAM, pstg: &Option<super::LPSTORAGE>, pcfformat: *mut super::CLIPFORMAT, plwwidth: *mut super::LONG, plheight: *mut super::LONG, pdwsize: *mut super::DWORD, pmedium: super::LPSTGMEDIUM, opt: Option<super::DWORD>, pvcallbackcontext: Option<super::PVOID>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : LPOLESTREAM, pstg : *mut core::ffi::c_void, pcfformat : *mut super::CLIPFORMAT, plwwidth : *mut super::LONG, plheight : *mut super::LONG, pdwsize : *mut super::DWORD, pmedium : super::LPSTGMEDIUM, opt : super::DWORD, pvcallbackcontext : super::PVOID, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    unsafe { OleConvertOLESTREAMToIStorageEx2(polestm, core::mem::transmute_copy(pstg), pcfformat as _, plwwidth as _, plheight as _, pdwsize as _, pmedium, opt.unwrap_or(core::mem::zeroed()) as _, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pqueryconvertolelinkcallback) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreate(rclsid: *const super::IID, riid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreate(rclsid : *const super::IID, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreate(rclsid as _, riid as _, renderopt, pformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "unknwnbase"))]
#[inline]
pub unsafe fn OleCreateDefaultHandler(clsid: *const super::IID, punkouter: &Option<super::LPUNKNOWN>, riid: *const super::IID, lplpobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const super::IID, punkouter : *mut core::ffi::c_void, riid : *const super::IID, lplpobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateDefaultHandler(clsid as _, core::mem::transmute_copy(punkouter), riid as _, lplpobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "unknwnbase"))]
#[inline]
pub unsafe fn OleCreateEmbeddingHelper(clsid: *const super::IID, punkouter: &Option<super::LPUNKNOWN>, flags: super::DWORD, pcf: &Option<super::LPCLASSFACTORY>, riid: *const super::IID, lplpobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const super::IID, punkouter : *mut core::ffi::c_void, flags : super::DWORD, pcf : *mut core::ffi::c_void, riid : *const super::IID, lplpobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateEmbeddingHelper(clsid as _, core::mem::transmute_copy(punkouter), flags, core::mem::transmute_copy(pcf), riid as _, lplpobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateEx(rclsid: *const super::IID, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: &Option<super::IAdviseSink>, rgdwconnection: *mut super::DWORD, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const super::IID, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateEx(rclsid as _, riid as _, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, core::mem::transmute_copy(lpadvisesink), rgdwconnection as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateFromData(psrcdataobj: &Option<super::LPDATAOBJECT>, riid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromData(core::mem::transmute_copy(psrcdataobj), riid as _, renderopt, pformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateFromDataEx(psrcdataobj: &Option<super::LPDATAOBJECT>, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: &Option<super::IAdviseSink>, rgdwconnection: *mut super::DWORD, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromDataEx(core::mem::transmute_copy(psrcdataobj), riid as _, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, core::mem::transmute_copy(lpadvisesink), rgdwconnection as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateFromFile(rclsid: *const super::IID, lpszfilename: super::LPCOLESTR, riid: *const super::IID, renderopt: super::DWORD, lpformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const super::IID, lpszfilename : super::LPCOLESTR, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromFile(rclsid as _, lpszfilename as _, riid as _, renderopt, lpformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateFromFileEx(rclsid: *const super::IID, lpszfilename: super::LPCOLESTR, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: &Option<super::IAdviseSink>, rgdwconnection: *mut super::DWORD, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const super::IID, lpszfilename : super::LPCOLESTR, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateFromFileEx(rclsid as _, lpszfilename as _, riid as _, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, core::mem::transmute_copy(lpadvisesink), rgdwconnection as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLink(pmklinksrc: &Option<super::LPMONIKER>, riid: *const super::IID, renderopt: super::DWORD, lpformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : *mut core::ffi::c_void, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLink(core::mem::transmute_copy(pmklinksrc), riid as _, renderopt, lpformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkEx(pmklinksrc: &Option<super::LPMONIKER>, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: &Option<super::IAdviseSink>, rgdwconnection: *mut super::DWORD, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : *mut core::ffi::c_void, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkEx(core::mem::transmute_copy(pmklinksrc), riid as _, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, core::mem::transmute_copy(lpadvisesink), rgdwconnection as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkFromData(psrcdataobj: &Option<super::LPDATAOBJECT>, riid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkFromData(core::mem::transmute_copy(psrcdataobj), riid as _, renderopt, pformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateLinkFromDataEx(psrcdataobj: &Option<super::LPDATAOBJECT>, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: &Option<super::IAdviseSink>, rgdwconnection: *mut super::DWORD, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : *mut core::ffi::c_void, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkFromDataEx(core::mem::transmute_copy(psrcdataobj), riid as _, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, core::mem::transmute_copy(lpadvisesink), rgdwconnection as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateLinkToFile(lpszfilename: super::LPCOLESTR, riid: *const super::IID, renderopt: super::DWORD, lpformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : super::LPCOLESTR, riid : *const super::IID, renderopt : super::DWORD, lpformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkToFile(lpszfilename as _, riid as _, renderopt, lpformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleCreateLinkToFileEx(lpszfilename: super::LPCOLESTR, riid: *const super::IID, dwflags: super::DWORD, renderopt: super::DWORD, cformats: super::ULONG, rgadvf: *mut super::DWORD, rgformatetc: super::LPFORMATETC, lpadvisesink: &Option<super::IAdviseSink>, rgdwconnection: *mut super::DWORD, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : super::LPCOLESTR, riid : *const super::IID, dwflags : super::DWORD, renderopt : super::DWORD, cformats : super::ULONG, rgadvf : *mut super::DWORD, rgformatetc : super::LPFORMATETC, lpadvisesink : *mut core::ffi::c_void, rgdwconnection : *mut super::DWORD, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateLinkToFileEx(lpszfilename as _, riid as _, dwflags, renderopt, cformats, rgadvf as _, rgformatetc as _, core::mem::transmute_copy(lpadvisesink), rgdwconnection as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(all(feature = "oleidl", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn OleCreateMenuDescriptor(hmenucombined: super::HMENU, lpmenuwidths: super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU {
    windows_core::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::HMENU, lpmenuwidths : super::LPOLEMENUGROUPWIDTHS) -> super::HOLEMENU);
    unsafe { OleCreateMenuDescriptor(hmenucombined as _, lpmenuwidths as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl", feature = "shared", feature = "wtypes"))]
#[inline]
pub unsafe fn OleCreateStaticFromData(psrcdataobj: &Option<super::LPDATAOBJECT>, iid: *const super::IID, renderopt: super::DWORD, pformatetc: super::LPFORMATETC, pclientsite: &Option<super::LPOLECLIENTSITE>, pstg: &Option<super::LPSTORAGE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : *mut core::ffi::c_void, iid : *const super::IID, renderopt : super::DWORD, pformatetc : super::LPFORMATETC, pclientsite : *mut core::ffi::c_void, pstg : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleCreateStaticFromData(core::mem::transmute_copy(psrcdataobj), iid as _, renderopt, pformatetc as _, core::mem::transmute_copy(pclientsite), core::mem::transmute_copy(pstg), ppvobj as _) }
}
#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn OleDestroyMenuDescriptor(holemenu: super::HOLEMENU) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : super::HOLEMENU) -> windows_core::HRESULT);
    unsafe { OleDestroyMenuDescriptor(holemenu as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl"))]
#[inline]
pub unsafe fn OleDoAutoConvert(pstg: &Option<super::LPSTORAGE>, pclsidnew: super::LPCLSID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : *mut core::ffi::c_void, pclsidnew : super::LPCLSID) -> windows_core::HRESULT);
    unsafe { OleDoAutoConvert(core::mem::transmute_copy(pstg), pclsidnew as _) }
}
#[cfg(all(feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[inline]
pub unsafe fn OleDraw(punknown: &Option<super::LPUNKNOWN>, dwaspect: super::DWORD, hdcdraw: super::HDC, lprcbounds: super::LPCRECT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleDraw(punknown : *mut core::ffi::c_void, dwaspect : super::DWORD, hdcdraw : super::HDC, lprcbounds : super::LPCRECT) -> windows_core::HRESULT);
    unsafe { OleDraw(core::mem::transmute_copy(punknown), dwaspect, hdcdraw as _, lprcbounds as _) }
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
    unsafe { OleGetAutoConvert(clsidold as _, pclsidnew as _) }
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
pub unsafe fn OleGetIconOfClass(rclsid: *const super::IID, lpszlabel: Option<super::LPOLESTR>, fusetypeaslabel: windows_core::BOOL) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const super::IID, lpszlabel : super::LPOLESTR, fusetypeaslabel : windows_core::BOOL) -> super::HGLOBAL);
    unsafe { OleGetIconOfClass(rclsid as _, lpszlabel.unwrap_or(core::mem::zeroed()) as _, fusetypeaslabel) }
}
#[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleGetIconOfFile(lpszpath: super::LPOLESTR, fusefileaslabel: windows_core::BOOL) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : super::LPOLESTR, fusefileaslabel : windows_core::BOOL) -> super::HGLOBAL);
    unsafe { OleGetIconOfFile(lpszpath, fusefileaslabel) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn OleInitialize(pvreserved: super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleInitialize(pvreserved : super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleInitialize(pvreserved as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleIsCurrentClipboard(pdataobj: &Option<super::LPDATAOBJECT>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleIsCurrentClipboard(core::mem::transmute_copy(pdataobj)) }
}
#[cfg(feature = "oleidl")]
#[inline]
pub unsafe fn OleIsRunning(pobject: &Option<super::LPOLEOBJECT>) -> windows_core::BOOL {
    windows_core::link!("ole32.dll" "system" fn OleIsRunning(pobject : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { OleIsRunning(core::mem::transmute_copy(pobject)) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidl", feature = "oleidl"))]
#[inline]
pub unsafe fn OleLoad(pstg: &Option<super::LPSTORAGE>, riid: *const super::IID, pclientsite: &Option<super::LPOLECLIENTSITE>, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleLoad(pstg : *mut core::ffi::c_void, riid : *const super::IID, pclientsite : *mut core::ffi::c_void, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleLoad(core::mem::transmute_copy(pstg), riid as _, core::mem::transmute_copy(pclientsite), ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase"))]
#[inline]
pub unsafe fn OleLoadFromStream(pstm: &Option<super::LPSTREAM>, iidinterface: *const super::IID, ppvobj: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : *mut core::ffi::c_void, iidinterface : *const super::IID, ppvobj : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { OleLoadFromStream(core::mem::transmute_copy(pstm), iidinterface as _, ppvobj as _) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleLockRunning(punknown: &Option<super::LPUNKNOWN>, flock: windows_core::BOOL, flastunlockcloses: windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleLockRunning(punknown : *mut core::ffi::c_void, flock : windows_core::BOOL, flastunlockcloses : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleLockRunning(core::mem::transmute_copy(punknown), flock, flastunlockcloses) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleMetafilePictFromIconAndLabel(hicon: super::HICON, lpszlabel: super::LPOLESTR, lpszsourcefile: super::LPOLESTR, iiconindex: super::UINT) -> super::HGLOBAL {
    windows_core::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::HICON, lpszlabel : super::LPOLESTR, lpszsourcefile : super::LPOLESTR, iiconindex : super::UINT) -> super::HGLOBAL);
    unsafe { OleMetafilePictFromIconAndLabel(hicon as _, lpszlabel, lpszsourcefile, iiconindex) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleNoteObjectVisible(punknown: &Option<super::LPUNKNOWN>, fvisible: windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : *mut core::ffi::c_void, fvisible : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleNoteObjectVisible(core::mem::transmute_copy(punknown), fvisible) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleQueryCreateFromData(psrcdataobject: &Option<super::LPDATAOBJECT>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleQueryCreateFromData(core::mem::transmute_copy(psrcdataobject)) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleQueryLinkFromData(psrcdataobject: &Option<super::LPDATAOBJECT>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleQueryLinkFromData(core::mem::transmute_copy(psrcdataobject)) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn OleRegEnumFormatEtc(clsid: *const super::IID, dwdirection: super::DWORD, ppenum: *mut Option<super::LPENUMFORMATETC>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const super::IID, dwdirection : super::DWORD, ppenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleRegEnumFormatEtc(clsid as _, dwdirection, core::mem::transmute(ppenum)) }
}
#[cfg(all(feature = "guiddef", feature = "oleidl"))]
#[inline]
pub unsafe fn OleRegEnumVerbs(clsid: *const super::IID, ppenum: *mut Option<super::LPENUMOLEVERB>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const super::IID, ppenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleRegEnumVerbs(clsid as _, core::mem::transmute(ppenum)) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn OleRegGetMiscStatus(clsid: *const super::IID, dwaspect: super::DWORD, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const super::IID, dwaspect : super::DWORD, pdwstatus : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { OleRegGetMiscStatus(clsid as _, dwaspect, pdwstatus as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn OleRegGetUserType(clsid: *const super::IID, dwformoftype: super::DWORD, pszusertype: *mut super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const super::IID, dwformoftype : super::DWORD, pszusertype : *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { OleRegGetUserType(clsid as _, dwformoftype, pszusertype as _) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleRun(punknown: &Option<super::LPUNKNOWN>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleRun(punknown : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleRun(core::mem::transmute_copy(punknown)) }
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
pub unsafe fn OleSaveToStream(ppstm: &Option<super::LPPERSISTSTREAM>, pstm: &Option<super::LPSTREAM>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : *mut core::ffi::c_void, pstm : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSaveToStream(core::mem::transmute_copy(ppstm), core::mem::transmute_copy(pstm)) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn OleSetAutoConvert(clsidold: *const super::IID, clsidnew: *const super::IID) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const super::IID, clsidnew : *const super::IID) -> windows_core::HRESULT);
    unsafe { OleSetAutoConvert(clsidold as _, clsidnew as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn OleSetClipboard(pdataobj: &Option<super::LPDATAOBJECT>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSetClipboard(core::mem::transmute_copy(pdataobj)) }
}
#[cfg(feature = "unknwnbase")]
#[inline]
pub unsafe fn OleSetContainedObject(punknown: &Option<super::LPUNKNOWN>, fcontained: windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : *mut core::ffi::c_void, fcontained : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleSetContainedObject(core::mem::transmute_copy(punknown), fcontained) }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
#[inline]
pub unsafe fn OleSetMenuDescriptor(holemenu: super::HOLEMENU, hwndframe: super::HWND, hwndactiveobject: super::HWND, lpframe: &Option<super::LPOLEINPLACEFRAME>, lpactiveobj: &Option<super::LPOLEINPLACEACTIVEOBJECT>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : super::HOLEMENU, hwndframe : super::HWND, hwndactiveobject : super::HWND, lpframe : *mut core::ffi::c_void, lpactiveobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleSetMenuDescriptor(holemenu as _, hwndframe as _, hwndactiveobject as _, core::mem::transmute_copy(lpframe), core::mem::transmute_copy(lpactiveobj)) }
}
#[cfg(all(feature = "minwindef", feature = "oleidl", feature = "shared", feature = "windef", feature = "winuser"))]
#[inline]
pub unsafe fn OleTranslateAccelerator(lpframe: &Option<super::LPOLEINPLACEFRAME>, lpframeinfo: super::LPOLEINPLACEFRAMEINFO, lpmsg: super::LPMSG) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : *mut core::ffi::c_void, lpframeinfo : super::LPOLEINPLACEFRAMEINFO, lpmsg : super::LPMSG) -> windows_core::HRESULT);
    unsafe { OleTranslateAccelerator(core::mem::transmute_copy(lpframe), lpframeinfo as _, lpmsg as _) }
}
#[inline]
pub unsafe fn OleUninitialize() {
    windows_core::link!("ole32.dll" "system" fn OleUninitialize());
    unsafe { OleUninitialize() }
}
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ReadFmtUserTypeStg(pstg: &Option<super::LPSTORAGE>, pcf: *mut super::CLIPFORMAT, lplpszusertype: Option<*mut super::LPOLESTR>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn ReadFmtUserTypeStg(pstg : *mut core::ffi::c_void, pcf : *mut super::CLIPFORMAT, lplpszusertype : *mut super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { ReadFmtUserTypeStg(core::mem::transmute_copy(pstg), pcf as _, lplpszusertype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "oleidl", feature = "windef"))]
#[inline]
pub unsafe fn RegisterDragDrop(hwnd: super::HWND, pdroptarget: &Option<super::LPDROPTARGET>) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::HWND, pdroptarget : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RegisterDragDrop(hwnd as _, core::mem::transmute_copy(pdroptarget)) }
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
pub unsafe fn SetConvertStg(pstg: &Option<super::LPSTORAGE>, fconvert: windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn SetConvertStg(pstg : *mut core::ffi::c_void, fconvert : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { SetConvertStg(core::mem::transmute_copy(pstg), fconvert) }
}
#[cfg(all(feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WriteFmtUserTypeStg(pstg: &Option<super::LPSTORAGE>, cf: super::CLIPFORMAT, lpszusertype: super::LPOLESTR) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn WriteFmtUserTypeStg(pstg : *mut core::ffi::c_void, cf : super::CLIPFORMAT, lpszusertype : super::LPOLESTR) -> windows_core::HRESULT);
    unsafe { WriteFmtUserTypeStg(core::mem::transmute_copy(pstg), cf, lpszusertype) }
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
