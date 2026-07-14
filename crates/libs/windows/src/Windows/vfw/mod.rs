#[inline]
pub unsafe fn AVIBuildFilterA(lpszfilter: &mut [u8], fsaving: bool) -> windows_core::HRESULT {
    windows_core::link!("avifil32.dll" "system" fn AVIBuildFilterA(lpszfilter : windows_core::PSTR, cbfilter : i32, fsaving : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { AVIBuildFilterA(core::mem::transmute(lpszfilter.as_mut_ptr()), lpszfilter.len().try_into().unwrap(), fsaving.into()) }
}
#[inline]
pub unsafe fn AVIBuildFilterW(lpszfilter: &mut [u16], fsaving: bool) -> windows_core::HRESULT {
    windows_core::link!("avifil32.dll" "system" fn AVIBuildFilterW(lpszfilter : windows_core::PWSTR, cbfilter : i32, fsaving : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { AVIBuildFilterW(core::mem::transmute(lpszfilter.as_mut_ptr()), lpszfilter.len().try_into().unwrap(), fsaving.into()) }
}
#[inline]
pub unsafe fn AVIClearClipboard() -> windows_core::HRESULT {
    windows_core::link!("avifil32.dll" "system" fn AVIClearClipboard() -> windows_core::HRESULT);
    unsafe { AVIClearClipboard() }
}
#[inline]
pub unsafe fn AVIFileAddRef<P0>(pfile: P0) -> u32
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileAddRef(pfile : *mut core::ffi::c_void) -> u32);
    unsafe { AVIFileAddRef(pfile.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AVIFileCreateStreamA<P0>(pfile: P0, ppavi: *mut Option<IAVIStream>, psi: *const AVISTREAMINFOA) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileCreateStreamA(pfile : *mut core::ffi::c_void, ppavi : *mut *mut core::ffi::c_void, psi : *const AVISTREAMINFOA) -> windows_core::HRESULT);
    unsafe { AVIFileCreateStreamA(pfile.param().abi(), core::mem::transmute(ppavi), psi) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AVIFileCreateStreamW<P0>(pfile: P0, ppavi: *mut Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileCreateStreamW(pfile : *mut core::ffi::c_void, ppavi : *mut *mut core::ffi::c_void, psi : *const AVISTREAMINFOW) -> windows_core::HRESULT);
    unsafe { AVIFileCreateStreamW(pfile.param().abi(), core::mem::transmute(ppavi), psi) }
}
#[inline]
pub unsafe fn AVIFileEndRecord<P0>(pfile: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileEndRecord(pfile : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { AVIFileEndRecord(pfile.param().abi()) }
}
#[inline]
pub unsafe fn AVIFileExit() {
    windows_core::link!("avifil32.dll" "system" fn AVIFileExit());
    unsafe { AVIFileExit() }
}
#[inline]
pub unsafe fn AVIFileGetStream<P0>(pfile: P0, ppavi: *mut Option<IAVIStream>, fcctype: u32, lparam: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileGetStream(pfile : *mut core::ffi::c_void, ppavi : *mut *mut core::ffi::c_void, fcctype : u32, lparam : i32) -> windows_core::HRESULT);
    unsafe { AVIFileGetStream(pfile.param().abi(), core::mem::transmute(ppavi), fcctype, lparam) }
}
#[inline]
pub unsafe fn AVIFileInfoA<P0>(pfile: P0, pfi: *mut AVIFILEINFOA, lsize: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileInfoA(pfile : *mut core::ffi::c_void, pfi : *mut AVIFILEINFOA, lsize : i32) -> windows_core::HRESULT);
    unsafe { AVIFileInfoA(pfile.param().abi(), pfi as _, lsize) }
}
#[inline]
pub unsafe fn AVIFileInfoW<P0>(pfile: P0, pfi: *mut AVIFILEINFOW, lsize: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileInfoW(pfile : *mut core::ffi::c_void, pfi : *mut AVIFILEINFOW, lsize : i32) -> windows_core::HRESULT);
    unsafe { AVIFileInfoW(pfile.param().abi(), pfi as _, lsize) }
}
#[inline]
pub unsafe fn AVIFileInit() {
    windows_core::link!("avifil32.dll" "system" fn AVIFileInit());
    unsafe { AVIFileInit() }
}
#[inline]
pub unsafe fn AVIFileOpenA<P1>(ppfile: *mut Option<IAVIFile>, szfile: P1, umode: u32, lphandler: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileOpenA(ppfile : *mut *mut core::ffi::c_void, szfile : windows_core::PCSTR, umode : u32, lphandler : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { AVIFileOpenA(core::mem::transmute(ppfile), szfile.param().abi(), umode, lphandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIFileOpenW<P1>(ppfile: *mut Option<IAVIFile>, szfile: P1, umode: u32, lphandler: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileOpenW(ppfile : *mut *mut core::ffi::c_void, szfile : windows_core::PCWSTR, umode : u32, lphandler : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { AVIFileOpenW(core::mem::transmute(ppfile), szfile.param().abi(), umode, lphandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIFileReadData<P0>(pfile: P0, ckid: u32, lpdata: *mut core::ffi::c_void, lpcbdata: *mut i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileReadData(pfile : *mut core::ffi::c_void, ckid : u32, lpdata : *mut core::ffi::c_void, lpcbdata : *mut i32) -> windows_core::HRESULT);
    unsafe { AVIFileReadData(pfile.param().abi(), ckid, lpdata as _, lpcbdata as _) }
}
#[inline]
pub unsafe fn AVIFileRelease<P0>(pfile: P0) -> u32
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileRelease(pfile : *mut core::ffi::c_void) -> u32);
    unsafe { AVIFileRelease(pfile.param().abi()) }
}
#[inline]
pub unsafe fn AVIFileWriteData<P0>(pfile: P0, ckid: u32, lpdata: *const core::ffi::c_void, cbdata: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIFileWriteData(pfile : *mut core::ffi::c_void, ckid : u32, lpdata : *const core::ffi::c_void, cbdata : i32) -> windows_core::HRESULT);
    unsafe { AVIFileWriteData(pfile.param().abi(), ckid, lpdata, cbdata) }
}
#[inline]
pub unsafe fn AVIGetFromClipboard() -> windows_core::Result<IAVIFile> {
    windows_core::link!("avifil32.dll" "system" fn AVIGetFromClipboard(lppf : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AVIGetFromClipboard(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn AVIMakeCompressedStream<P1>(ppscompressed: *mut Option<IAVIStream>, ppssource: P1, lpoptions: *const AVICOMPRESSOPTIONS, pclsidhandler: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P1: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIMakeCompressedStream(ppscompressed : *mut *mut core::ffi::c_void, ppssource : *mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, pclsidhandler : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { AVIMakeCompressedStream(core::mem::transmute(ppscompressed), ppssource.param().abi(), lpoptions, pclsidhandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIMakeFileFromStreams(ppfile: *mut Option<IAVIFile>, papstreams: &[Option<IAVIStream>]) -> windows_core::HRESULT {
    windows_core::link!("avifil32.dll" "system" fn AVIMakeFileFromStreams(ppfile : *mut *mut core::ffi::c_void, nstreams : i32, papstreams : *const *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { AVIMakeFileFromStreams(core::mem::transmute(ppfile), papstreams.len().try_into().unwrap(), core::mem::transmute(papstreams.as_ptr())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AVIMakeStreamFromClipboard(cfformat: u32, hglobal: super::winnt::HANDLE) -> windows_core::Result<IAVIStream> {
    windows_core::link!("avifil32.dll" "system" fn AVIMakeStreamFromClipboard(cfformat : u32, hglobal : super::winnt::HANDLE, ppstream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        AVIMakeStreamFromClipboard(cfformat, hglobal, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn AVIPutFileOnClipboard<P0>(pf: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIFile>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIPutFileOnClipboard(pf : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { AVIPutFileOnClipboard(pf.param().abi()) }
}
#[inline]
pub unsafe fn AVISaveA<P0, P4>(szfile: P0, pclsidhandler: Option<*const windows_core::GUID>, lpfncallback: AVISAVECALLBACK, nstreams: i32, pfile: P4, lpoptions: *const AVICOMPRESSOPTIONS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "C" fn AVISaveA(szfile : windows_core::PCSTR, pclsidhandler : *const windows_core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : *mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS) -> windows_core::HRESULT);
    unsafe { AVISaveA(szfile.param().abi(), pclsidhandler.unwrap_or(core::mem::zeroed()) as _, lpfncallback, nstreams, pfile.param().abi(), lpoptions) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AVISaveOptions(hwnd: super::windef::HWND, uiflags: u32, nstreams: i32, ppavi: *const Option<IAVIStream>, plpoptions: *mut LPAVICOMPRESSOPTIONS) -> isize {
    windows_core::link!("avifil32.dll" "system" fn AVISaveOptions(hwnd : super::windef::HWND, uiflags : u32, nstreams : i32, ppavi : *const *mut core::ffi::c_void, plpoptions : *mut LPAVICOMPRESSOPTIONS) -> isize);
    unsafe { AVISaveOptions(hwnd, uiflags, nstreams, core::mem::transmute(ppavi), plpoptions as _) }
}
#[inline]
pub unsafe fn AVISaveOptionsFree(plpoptions: &[LPAVICOMPRESSOPTIONS]) -> windows_core::HRESULT {
    windows_core::link!("avifil32.dll" "system" fn AVISaveOptionsFree(nstreams : i32, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_core::HRESULT);
    unsafe { AVISaveOptionsFree(plpoptions.len().try_into().unwrap(), plpoptions.as_ptr()) }
}
#[inline]
pub unsafe fn AVISaveVA<P0>(szfile: P0, pclsidhandler: Option<*const windows_core::GUID>, lpfncallback: AVISAVECALLBACK, nstreams: i32, ppavi: *const Option<IAVIStream>, plpoptions: *const LPAVICOMPRESSOPTIONS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn AVISaveVA(szfile : windows_core::PCSTR, pclsidhandler : *const windows_core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const *mut core::ffi::c_void, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_core::HRESULT);
    unsafe { AVISaveVA(szfile.param().abi(), pclsidhandler.unwrap_or(core::mem::zeroed()) as _, lpfncallback, nstreams, core::mem::transmute(ppavi), plpoptions) }
}
#[inline]
pub unsafe fn AVISaveVW<P0>(szfile: P0, pclsidhandler: Option<*const windows_core::GUID>, lpfncallback: AVISAVECALLBACK, nstreams: i32, ppavi: *const Option<IAVIStream>, plpoptions: *const LPAVICOMPRESSOPTIONS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn AVISaveVW(szfile : windows_core::PCWSTR, pclsidhandler : *const windows_core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const *mut core::ffi::c_void, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_core::HRESULT);
    unsafe { AVISaveVW(szfile.param().abi(), pclsidhandler.unwrap_or(core::mem::zeroed()) as _, lpfncallback, nstreams, core::mem::transmute(ppavi), plpoptions) }
}
#[inline]
pub unsafe fn AVISaveW<P0, P4>(szfile: P0, pclsidhandler: Option<*const windows_core::GUID>, lpfncallback: AVISAVECALLBACK, nstreams: i32, pfile: P4, lpoptions: *const AVICOMPRESSOPTIONS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "C" fn AVISaveW(szfile : windows_core::PCWSTR, pclsidhandler : *const windows_core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : *mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS) -> windows_core::HRESULT);
    unsafe { AVISaveW(szfile.param().abi(), pclsidhandler.unwrap_or(core::mem::zeroed()) as _, lpfncallback, nstreams, pfile.param().abi(), lpoptions) }
}
#[inline]
pub unsafe fn AVIStreamAddRef<P0>(pavi: P0) -> u32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamAddRef(pavi : *mut core::ffi::c_void) -> u32);
    unsafe { AVIStreamAddRef(pavi.param().abi()) }
}
#[inline]
pub unsafe fn AVIStreamBeginStreaming<P0>(pavi: P0, lstart: i32, lend: i32, lrate: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamBeginStreaming(pavi : *mut core::ffi::c_void, lstart : i32, lend : i32, lrate : i32) -> windows_core::HRESULT);
    unsafe { AVIStreamBeginStreaming(pavi.param().abi(), lstart, lend, lrate) }
}
#[inline]
pub unsafe fn AVIStreamCreate(ppavi: *mut Option<IAVIStream>, lparam1: i32, lparam2: i32, pclsidhandler: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
    windows_core::link!("avifil32.dll" "system" fn AVIStreamCreate(ppavi : *mut *mut core::ffi::c_void, lparam1 : i32, lparam2 : i32, pclsidhandler : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { AVIStreamCreate(core::mem::transmute(ppavi), lparam1, lparam2, pclsidhandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIStreamEndStreaming<P0>(pavi: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamEndStreaming(pavi : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { AVIStreamEndStreaming(pavi.param().abi()) }
}
#[inline]
pub unsafe fn AVIStreamFindSample<P0>(pavi: P0, lpos: i32, lflags: i32) -> i32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamFindSample(pavi : *mut core::ffi::c_void, lpos : i32, lflags : i32) -> i32);
    unsafe { AVIStreamFindSample(pavi.param().abi(), lpos, lflags) }
}
#[inline]
pub unsafe fn AVIStreamGetFrame<P0>(pg: P0, lpos: i32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<IGetFrame>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamGetFrame(pg : *mut core::ffi::c_void, lpos : i32) -> *mut core::ffi::c_void);
    unsafe { AVIStreamGetFrame(pg.param().abi(), lpos) }
}
#[inline]
pub unsafe fn AVIStreamGetFrameClose<P0>(pg: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IGetFrame>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamGetFrameClose(pg : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { AVIStreamGetFrameClose(pg.param().abi()) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn AVIStreamGetFrameOpen<P0>(pavi: P0, lpbiwanted: Option<*const super::wingdi::BITMAPINFOHEADER>) -> Option<IGetFrame>
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamGetFrameOpen(pavi : *mut core::ffi::c_void, lpbiwanted : *const super::wingdi::BITMAPINFOHEADER) -> Option < IGetFrame >);
    unsafe { AVIStreamGetFrameOpen(pavi.param().abi(), lpbiwanted.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AVIStreamInfoA<P0>(pavi: P0, psi: *mut AVISTREAMINFOA, lsize: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamInfoA(pavi : *mut core::ffi::c_void, psi : *mut AVISTREAMINFOA, lsize : i32) -> windows_core::HRESULT);
    unsafe { AVIStreamInfoA(pavi.param().abi(), psi as _, lsize) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn AVIStreamInfoW<P0>(pavi: P0, psi: *mut AVISTREAMINFOW, lsize: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamInfoW(pavi : *mut core::ffi::c_void, psi : *mut AVISTREAMINFOW, lsize : i32) -> windows_core::HRESULT);
    unsafe { AVIStreamInfoW(pavi.param().abi(), psi as _, lsize) }
}
#[inline]
pub unsafe fn AVIStreamLength<P0>(pavi: P0) -> i32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamLength(pavi : *mut core::ffi::c_void) -> i32);
    unsafe { AVIStreamLength(pavi.param().abi()) }
}
#[inline]
pub unsafe fn AVIStreamOpenFromFileA<P1>(ppavi: *mut Option<IAVIStream>, szfile: P1, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileA(ppavi : *mut *mut core::ffi::c_void, szfile : windows_core::PCSTR, fcctype : u32, lparam : i32, mode : u32, pclsidhandler : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { AVIStreamOpenFromFileA(core::mem::transmute(ppavi), szfile.param().abi(), fcctype, lparam, mode, pclsidhandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIStreamOpenFromFileW<P1>(ppavi: *mut Option<IAVIStream>, szfile: P1, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: Option<*const windows_core::GUID>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileW(ppavi : *mut *mut core::ffi::c_void, szfile : windows_core::PCWSTR, fcctype : u32, lparam : i32, mode : u32, pclsidhandler : *const windows_core::GUID) -> windows_core::HRESULT);
    unsafe { AVIStreamOpenFromFileW(core::mem::transmute(ppavi), szfile.param().abi(), fcctype, lparam, mode, pclsidhandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIStreamRead<P0>(pavi: P0, lstart: i32, lsamples: i32, lpbuffer: Option<*mut core::ffi::c_void>, cbbuffer: i32, plbytes: Option<*mut i32>, plsamples: Option<*mut i32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamRead(pavi : *mut core::ffi::c_void, lstart : i32, lsamples : i32, lpbuffer : *mut core::ffi::c_void, cbbuffer : i32, plbytes : *mut i32, plsamples : *mut i32) -> windows_core::HRESULT);
    unsafe { AVIStreamRead(pavi.param().abi(), lstart, lsamples, lpbuffer.unwrap_or(core::mem::zeroed()) as _, cbbuffer, plbytes.unwrap_or(core::mem::zeroed()) as _, plsamples.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIStreamReadData<P0>(pavi: P0, fcc: u32, lp: Option<*mut core::ffi::c_void>, lpcb: *mut i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamReadData(pavi : *mut core::ffi::c_void, fcc : u32, lp : *mut core::ffi::c_void, lpcb : *mut i32) -> windows_core::HRESULT);
    unsafe { AVIStreamReadData(pavi.param().abi(), fcc, lp.unwrap_or(core::mem::zeroed()) as _, lpcb as _) }
}
#[inline]
pub unsafe fn AVIStreamReadFormat<P0>(pavi: P0, lpos: i32, lpformat: Option<*mut core::ffi::c_void>, lpcbformat: *mut i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamReadFormat(pavi : *mut core::ffi::c_void, lpos : i32, lpformat : *mut core::ffi::c_void, lpcbformat : *mut i32) -> windows_core::HRESULT);
    unsafe { AVIStreamReadFormat(pavi.param().abi(), lpos, lpformat.unwrap_or(core::mem::zeroed()) as _, lpcbformat as _) }
}
#[inline]
pub unsafe fn AVIStreamRelease<P0>(pavi: P0) -> u32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamRelease(pavi : *mut core::ffi::c_void) -> u32);
    unsafe { AVIStreamRelease(pavi.param().abi()) }
}
#[inline]
pub unsafe fn AVIStreamSampleToTime<P0>(pavi: P0, lsample: i32) -> i32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamSampleToTime(pavi : *mut core::ffi::c_void, lsample : i32) -> i32);
    unsafe { AVIStreamSampleToTime(pavi.param().abi(), lsample) }
}
#[inline]
pub unsafe fn AVIStreamSetFormat<P0>(pavi: P0, lpos: i32, lpformat: *const core::ffi::c_void, cbformat: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamSetFormat(pavi : *mut core::ffi::c_void, lpos : i32, lpformat : *const core::ffi::c_void, cbformat : i32) -> windows_core::HRESULT);
    unsafe { AVIStreamSetFormat(pavi.param().abi(), lpos, lpformat, cbformat) }
}
#[inline]
pub unsafe fn AVIStreamStart<P0>(pavi: P0) -> i32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamStart(pavi : *mut core::ffi::c_void) -> i32);
    unsafe { AVIStreamStart(pavi.param().abi()) }
}
#[inline]
pub unsafe fn AVIStreamTimeToSample<P0>(pavi: P0, ltime: i32) -> i32
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamTimeToSample(pavi : *mut core::ffi::c_void, ltime : i32) -> i32);
    unsafe { AVIStreamTimeToSample(pavi.param().abi(), ltime) }
}
#[inline]
pub unsafe fn AVIStreamWrite<P0>(pavi: P0, lstart: i32, lsamples: i32, lpbuffer: *const core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: Option<*mut i32>, plbyteswritten: Option<*mut i32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamWrite(pavi : *mut core::ffi::c_void, lstart : i32, lsamples : i32, lpbuffer : *const core::ffi::c_void, cbbuffer : i32, dwflags : u32, plsampwritten : *mut i32, plbyteswritten : *mut i32) -> windows_core::HRESULT);
    unsafe { AVIStreamWrite(pavi.param().abi(), lstart, lsamples, lpbuffer, cbbuffer, dwflags, plsampwritten.unwrap_or(core::mem::zeroed()) as _, plbyteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AVIStreamWriteData<P0>(pavi: P0, fcc: u32, lp: *const core::ffi::c_void, cb: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn AVIStreamWriteData(pavi : *mut core::ffi::c_void, fcc : u32, lp : *const core::ffi::c_void, cb : i32) -> windows_core::HRESULT);
    unsafe { AVIStreamWriteData(pavi.param().abi(), fcc, lp, cb) }
}
#[inline]
pub unsafe fn CreateEditableStream<P1>(ppseditable: *mut Option<IAVIStream>, pssource: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn CreateEditableStream(ppseditable : *mut *mut core::ffi::c_void, pssource : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateEditableStream(core::mem::transmute(ppseditable), pssource.param().abi()) }
}
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibBegin(hdd: HDRAWDIB, hdc: Option<super::windef::HDC>, dxdst: i32, dydst: i32, lpbi: *const super::wingdi::BITMAPINFOHEADER, dxsrc: i32, dysrc: i32, wflags: u32) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibBegin(hdd : HDRAWDIB, hdc : super::windef::HDC, dxdst : i32, dydst : i32, lpbi : *const super::wingdi::BITMAPINFOHEADER, dxsrc : i32, dysrc : i32, wflags : u32) -> windows_core::BOOL);
    unsafe { DrawDibBegin(hdd, hdc.unwrap_or(core::mem::zeroed()) as _, dxdst, dydst, lpbi, dxsrc, dysrc, wflags) }
}
#[cfg(all(feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibChangePalette(hdd: HDRAWDIB, istart: i32, lppe: &[super::wingdi::PALETTEENTRY]) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibChangePalette(hdd : HDRAWDIB, istart : i32, ilen : i32, lppe : *const super::wingdi::PALETTEENTRY) -> windows_core::BOOL);
    unsafe { DrawDibChangePalette(hdd, istart, lppe.len().try_into().unwrap(), lppe.as_ptr()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DrawDibClose(hdd: HDRAWDIB) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibClose(hdd : HDRAWDIB) -> windows_core::BOOL);
    unsafe { DrawDibClose(hdd) }
}
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibDraw(hdd: HDRAWDIB, hdc: super::windef::HDC, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: Option<*const super::wingdi::BITMAPINFOHEADER>, lpbits: Option<*const core::ffi::c_void>, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, wflags: u32) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibDraw(hdd : HDRAWDIB, hdc : super::windef::HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : *const super::wingdi::BITMAPINFOHEADER, lpbits : *const core::ffi::c_void, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, wflags : u32) -> windows_core::BOOL);
    unsafe { DrawDibDraw(hdd, hdc, xdst, ydst, dxdst, dydst, lpbi.unwrap_or(core::mem::zeroed()) as _, lpbits.unwrap_or(core::mem::zeroed()) as _, xsrc, ysrc, dxsrc, dysrc, wflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DrawDibEnd(hdd: HDRAWDIB) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibEnd(hdd : HDRAWDIB) -> windows_core::BOOL);
    unsafe { DrawDibEnd(hdd) }
}
#[cfg(all(feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibGetBuffer(hdd: HDRAWDIB, lpbi: *mut super::wingdi::BITMAPINFOHEADER, dwsize: u32, dwflags: u32) -> *mut core::ffi::c_void {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibGetBuffer(hdd : HDRAWDIB, lpbi : *mut super::wingdi::BITMAPINFOHEADER, dwsize : u32, dwflags : u32) -> *mut core::ffi::c_void);
    unsafe { DrawDibGetBuffer(hdd, lpbi as _, dwsize, dwflags) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibGetPalette(hdd: HDRAWDIB) -> super::windef::HPALETTE {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibGetPalette(hdd : HDRAWDIB) -> super::windef::HPALETTE);
    unsafe { DrawDibGetPalette(hdd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DrawDibOpen() -> HDRAWDIB {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibOpen() -> HDRAWDIB);
    unsafe { DrawDibOpen() }
}
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
#[inline]
pub unsafe fn DrawDibProfileDisplay(lpbi: *const super::wingdi::BITMAPINFOHEADER) -> super::minwindef::LRESULT {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibProfileDisplay(lpbi : *const super::wingdi::BITMAPINFOHEADER) -> super::minwindef::LRESULT);
    unsafe { DrawDibProfileDisplay(lpbi) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibRealize(hdd: HDRAWDIB, hdc: super::windef::HDC, fbackground: bool) -> u32 {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibRealize(hdd : HDRAWDIB, hdc : super::windef::HDC, fbackground : windows_core::BOOL) -> u32);
    unsafe { DrawDibRealize(hdd, hdc, fbackground.into()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn DrawDibSetPalette(hdd: HDRAWDIB, hpal: Option<super::windef::HPALETTE>) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibSetPalette(hdd : HDRAWDIB, hpal : super::windef::HPALETTE) -> windows_core::BOOL);
    unsafe { DrawDibSetPalette(hdd, hpal.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DrawDibStart(hdd: HDRAWDIB, rate: u32) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibStart(hdd : HDRAWDIB, rate : u32) -> windows_core::BOOL);
    unsafe { DrawDibStart(hdd, rate) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DrawDibStop(hdd: HDRAWDIB) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibStop(hdd : HDRAWDIB) -> windows_core::BOOL);
    unsafe { DrawDibStop(hdd) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DrawDibTime(hdd: HDRAWDIB, lpddtime: *mut DRAWDIBTIME) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn DrawDibTime(hdd : HDRAWDIB, lpddtime : *mut DRAWDIBTIME) -> windows_core::BOOL);
    unsafe { DrawDibTime(hdd, lpddtime as _) }
}
#[inline]
pub unsafe fn EditStreamClone<P0>(pavi: P0) -> windows_core::Result<IAVIStream>
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamClone(pavi : *mut core::ffi::c_void, ppresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        EditStreamClone(pavi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn EditStreamCopy<P0>(pavi: P0, plstart: *mut i32, pllength: *mut i32, ppresult: *mut Option<IAVIStream>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamCopy(pavi : *mut core::ffi::c_void, plstart : *mut i32, pllength : *mut i32, ppresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { EditStreamCopy(pavi.param().abi(), plstart as _, pllength as _, core::mem::transmute(ppresult)) }
}
#[inline]
pub unsafe fn EditStreamCut<P0>(pavi: P0, plstart: *mut i32, pllength: *mut i32, ppresult: *mut Option<IAVIStream>) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamCut(pavi : *mut core::ffi::c_void, plstart : *mut i32, pllength : *mut i32, ppresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { EditStreamCut(pavi.param().abi(), plstart as _, pllength as _, core::mem::transmute(ppresult)) }
}
#[inline]
pub unsafe fn EditStreamPaste<P0, P3>(pavi: P0, plpos: *mut i32, pllength: *mut i32, pstream: P3, lstart: i32, lend: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
    P3: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamPaste(pavi : *mut core::ffi::c_void, plpos : *mut i32, pllength : *mut i32, pstream : *mut core::ffi::c_void, lstart : i32, lend : i32) -> windows_core::HRESULT);
    unsafe { EditStreamPaste(pavi.param().abi(), plpos as _, pllength as _, pstream.param().abi(), lstart, lend) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EditStreamSetInfoA<P0>(pavi: P0, lpinfo: *const AVISTREAMINFOA, cbinfo: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamSetInfoA(pavi : *mut core::ffi::c_void, lpinfo : *const AVISTREAMINFOA, cbinfo : i32) -> windows_core::HRESULT);
    unsafe { EditStreamSetInfoA(pavi.param().abi(), lpinfo, cbinfo) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn EditStreamSetInfoW<P0>(pavi: P0, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamSetInfoW(pavi : *mut core::ffi::c_void, lpinfo : *const AVISTREAMINFOW, cbinfo : i32) -> windows_core::HRESULT);
    unsafe { EditStreamSetInfoW(pavi.param().abi(), lpinfo, cbinfo) }
}
#[inline]
pub unsafe fn EditStreamSetNameA<P0, P1>(pavi: P0, lpszname: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamSetNameA(pavi : *mut core::ffi::c_void, lpszname : windows_core::PCSTR) -> windows_core::HRESULT);
    unsafe { EditStreamSetNameA(pavi.param().abi(), lpszname.param().abi()) }
}
#[inline]
pub unsafe fn EditStreamSetNameW<P0, P1>(pavi: P0, lpszname: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<IAVIStream>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avifil32.dll" "system" fn EditStreamSetNameW(pavi : *mut core::ffi::c_void, lpszname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { EditStreamSetNameW(pavi.param().abi(), lpszname.param().abi()) }
}
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenFileNamePreviewA(lpofn: *mut super::commdlg::OPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewA(lpofn : *mut super::commdlg::OPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetOpenFileNamePreviewA(lpofn as _) }
}
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenFileNamePreviewW(lpofn: *mut super::commdlg::OPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewW(lpofn : *mut super::commdlg::OPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetOpenFileNamePreviewW(lpofn as _) }
}
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetSaveFileNamePreviewA(lpofn: *mut super::commdlg::OPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewA(lpofn : *mut super::commdlg::OPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetSaveFileNamePreviewA(lpofn as _) }
}
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetSaveFileNamePreviewW(lpofn: *mut super::commdlg::OPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewW(lpofn : *mut super::commdlg::OPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetSaveFileNamePreviewW(lpofn as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ICClose(hic: HIC) -> super::minwindef::LRESULT {
    windows_core::link!("msvfw32.dll" "system" fn ICClose(hic : HIC) -> super::minwindef::LRESULT);
    unsafe { ICClose(hic) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICCompress(hic: HIC, dwflags: u32, lpbioutput: *const super::wingdi::BITMAPINFOHEADER, lpdata: *mut core::ffi::c_void, lpbiinput: *const super::wingdi::BITMAPINFOHEADER, lpbits: *const core::ffi::c_void, lpckid: Option<*mut u32>, lpdwflags: Option<*mut u32>, lframenum: i32, dwframesize: u32, dwquality: u32, lpbiprev: Option<*const super::wingdi::BITMAPINFOHEADER>, lpprev: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("msvfw32.dll" "C" fn ICCompress(hic : HIC, dwflags : u32, lpbioutput : *const super::wingdi::BITMAPINFOHEADER, lpdata : *mut core::ffi::c_void, lpbiinput : *const super::wingdi::BITMAPINFOHEADER, lpbits : *const core::ffi::c_void, lpckid : *mut u32, lpdwflags : *mut u32, lframenum : i32, dwframesize : u32, dwquality : u32, lpbiprev : *const super::wingdi::BITMAPINFOHEADER, lpprev : *const core::ffi::c_void) -> u32);
    unsafe { ICCompress(hic, dwflags, lpbioutput, lpdata as _, lpbiinput, lpbits, lpckid.unwrap_or(core::mem::zeroed()) as _, lpdwflags.unwrap_or(core::mem::zeroed()) as _, lframenum, dwframesize, dwquality, lpbiprev.unwrap_or(core::mem::zeroed()) as _, lpprev.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ICCompressorChoose<P5>(hwnd: Option<super::windef::HWND>, uiflags: u32, pvin: Option<*const core::ffi::c_void>, lpdata: Option<*const core::ffi::c_void>, pc: *mut COMPVARS, lpsztitle: P5) -> windows_core::BOOL
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msvfw32.dll" "system" fn ICCompressorChoose(hwnd : super::windef::HWND, uiflags : u32, pvin : *const core::ffi::c_void, lpdata : *const core::ffi::c_void, pc : *mut COMPVARS, lpsztitle : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ICCompressorChoose(hwnd.unwrap_or(core::mem::zeroed()) as _, uiflags, pvin.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, pc as _, lpsztitle.param().abi()) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICCompressorFree(pc: *const COMPVARS) {
    windows_core::link!("msvfw32.dll" "system" fn ICCompressorFree(pc : *const COMPVARS));
    unsafe { ICCompressorFree(pc) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICDecompress(hic: HIC, dwflags: u32, lpbiformat: *const super::wingdi::BITMAPINFOHEADER, lpdata: *const core::ffi::c_void, lpbi: *const super::wingdi::BITMAPINFOHEADER, lpbits: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("msvfw32.dll" "C" fn ICDecompress(hic : HIC, dwflags : u32, lpbiformat : *const super::wingdi::BITMAPINFOHEADER, lpdata : *const core::ffi::c_void, lpbi : *const super::wingdi::BITMAPINFOHEADER, lpbits : *mut core::ffi::c_void) -> u32);
    unsafe { ICDecompress(hic, dwflags, lpbiformat, lpdata, lpbi, lpbits as _) }
}
#[inline]
pub unsafe fn ICDraw(hic: HIC, dwflags: u32, lpformat: *const core::ffi::c_void, lpdata: Option<*const core::ffi::c_void>, cbdata: u32, ltime: i32) -> u32 {
    windows_core::link!("msvfw32.dll" "C" fn ICDraw(hic : HIC, dwflags : u32, lpformat : *const core::ffi::c_void, lpdata : *const core::ffi::c_void, cbdata : u32, ltime : i32) -> u32);
    unsafe { ICDraw(hic, dwflags, lpformat, lpdata.unwrap_or(core::mem::zeroed()) as _, cbdata, ltime) }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ICDrawBegin(hic: HIC, dwflags: u32, hpal: Option<super::windef::HPALETTE>, hwnd: Option<super::windef::HWND>, hdc: Option<super::windef::HDC>, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::wingdi::BITMAPINFOHEADER, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, dwrate: u32, dwscale: u32) -> u32 {
    windows_core::link!("msvfw32.dll" "C" fn ICDrawBegin(hic : HIC, dwflags : u32, hpal : super::windef::HPALETTE, hwnd : super::windef::HWND, hdc : super::windef::HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : *const super::wingdi::BITMAPINFOHEADER, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, dwrate : u32, dwscale : u32) -> u32);
    unsafe { ICDrawBegin(hic, dwflags, hpal.unwrap_or(core::mem::zeroed()) as _, hwnd.unwrap_or(core::mem::zeroed()) as _, hdc.unwrap_or(core::mem::zeroed()) as _, xdst, ydst, dxdst, dydst, lpbi, xsrc, ysrc, dxsrc, dysrc, dwrate, dwscale) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICGetDisplayFormat(hic: Option<HIC>, lpbiin: *const super::wingdi::BITMAPINFOHEADER, lpbiout: *mut super::wingdi::BITMAPINFOHEADER, bitdepth: i32, dx: i32, dy: i32) -> HIC {
    windows_core::link!("msvfw32.dll" "system" fn ICGetDisplayFormat(hic : HIC, lpbiin : *const super::wingdi::BITMAPINFOHEADER, lpbiout : *mut super::wingdi::BITMAPINFOHEADER, bitdepth : i32, dx : i32, dy : i32) -> HIC);
    unsafe { ICGetDisplayFormat(hic.unwrap_or(core::mem::zeroed()) as _, lpbiin, lpbiout as _, bitdepth, dx, dy) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ICGetInfo(hic: HIC, picinfo: *mut ICINFO, cb: u32) -> super::minwindef::LRESULT {
    windows_core::link!("msvfw32.dll" "system" fn ICGetInfo(hic : HIC, picinfo : *mut ICINFO, cb : u32) -> super::minwindef::LRESULT);
    unsafe { ICGetInfo(hic, picinfo as _, cb) }
}
#[cfg(all(feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn ICImageCompress(hic: HIC, uiflags: u32, lpbiin: *const super::wingdi::BITMAPINFO, lpbits: *const core::ffi::c_void, lpbiout: Option<*const super::wingdi::BITMAPINFO>, lquality: i32, plsize: Option<*mut i32>) -> super::winnt::HANDLE {
    windows_core::link!("msvfw32.dll" "system" fn ICImageCompress(hic : HIC, uiflags : u32, lpbiin : *const super::wingdi::BITMAPINFO, lpbits : *const core::ffi::c_void, lpbiout : *const super::wingdi::BITMAPINFO, lquality : i32, plsize : *mut i32) -> super::winnt::HANDLE);
    unsafe { ICImageCompress(hic, uiflags, lpbiin, lpbits, lpbiout.unwrap_or(core::mem::zeroed()) as _, lquality, plsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn ICImageDecompress(hic: Option<HIC>, uiflags: u32, lpbiin: *const super::wingdi::BITMAPINFO, lpbits: *const core::ffi::c_void, lpbiout: Option<*const super::wingdi::BITMAPINFO>) -> super::winnt::HANDLE {
    windows_core::link!("msvfw32.dll" "system" fn ICImageDecompress(hic : HIC, uiflags : u32, lpbiin : *const super::wingdi::BITMAPINFO, lpbits : *const core::ffi::c_void, lpbiout : *const super::wingdi::BITMAPINFO) -> super::winnt::HANDLE);
    unsafe { ICImageDecompress(hic.unwrap_or(core::mem::zeroed()) as _, uiflags, lpbiin, lpbits, lpbiout.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ICInfo(fcctype: u32, fcchandler: u32, lpicinfo: *mut ICINFO) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn ICInfo(fcctype : u32, fcchandler : u32, lpicinfo : *mut ICINFO) -> windows_core::BOOL);
    unsafe { ICInfo(fcctype, fcchandler, lpicinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ICInstall<P3>(fcctype: u32, fcchandler: u32, lparam: super::minwindef::LPARAM, szdesc: P3, wflags: u32) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msvfw32.dll" "system" fn ICInstall(fcctype : u32, fcchandler : u32, lparam : super::minwindef::LPARAM, szdesc : windows_core::PCSTR, wflags : u32) -> windows_core::BOOL);
    unsafe { ICInstall(fcctype, fcchandler, lparam, szdesc.param().abi(), wflags) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICLocate(fcctype: u32, fcchandler: u32, lpbiin: *const super::wingdi::BITMAPINFOHEADER, lpbiout: Option<*const super::wingdi::BITMAPINFOHEADER>, wflags: u16) -> HIC {
    windows_core::link!("msvfw32.dll" "system" fn ICLocate(fcctype : u32, fcchandler : u32, lpbiin : *const super::wingdi::BITMAPINFOHEADER, lpbiout : *const super::wingdi::BITMAPINFOHEADER, wflags : u16) -> HIC);
    unsafe { ICLocate(fcctype, fcchandler, lpbiin, lpbiout.unwrap_or(core::mem::zeroed()) as _, wflags) }
}
#[inline]
pub unsafe fn ICOpen(fcctype: u32, fcchandler: u32, wmode: u32) -> HIC {
    windows_core::link!("msvfw32.dll" "system" fn ICOpen(fcctype : u32, fcchandler : u32, wmode : u32) -> HIC);
    unsafe { ICOpen(fcctype, fcchandler, wmode) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ICOpenFunction(fcctype: u32, fcchandler: u32, wmode: u32, lpfnhandler: super::minwindef::FARPROC) -> HIC {
    windows_core::link!("msvfw32.dll" "system" fn ICOpenFunction(fcctype : u32, fcchandler : u32, wmode : u32, lpfnhandler : super::minwindef::FARPROC) -> HIC);
    unsafe { ICOpenFunction(fcctype, fcchandler, wmode, lpfnhandler) }
}
#[inline]
pub unsafe fn ICRemove(fcctype: u32, fcchandler: u32, wflags: u32) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn ICRemove(fcctype : u32, fcchandler : u32, wflags : u32) -> windows_core::BOOL);
    unsafe { ICRemove(fcctype, fcchandler, wflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ICSendMessage(hic: HIC, msg: u32, dw1: usize, dw2: usize) -> super::minwindef::LRESULT {
    windows_core::link!("msvfw32.dll" "system" fn ICSendMessage(hic : HIC, msg : u32, dw1 : usize, dw2 : usize) -> super::minwindef::LRESULT);
    unsafe { ICSendMessage(hic, msg, dw1, dw2) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICSeqCompressFrame(pc: *const COMPVARS, uiflags: Option<u32>, lpbits: *const core::ffi::c_void, pfkey: *mut windows_core::BOOL, plsize: Option<*mut i32>) -> *mut core::ffi::c_void {
    windows_core::link!("msvfw32.dll" "system" fn ICSeqCompressFrame(pc : *const COMPVARS, uiflags : u32, lpbits : *const core::ffi::c_void, pfkey : *mut windows_core::BOOL, plsize : *mut i32) -> *mut core::ffi::c_void);
    unsafe { ICSeqCompressFrame(pc, uiflags.unwrap_or(core::mem::zeroed()) as _, lpbits, pfkey as _, plsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICSeqCompressFrameEnd(pc: *const COMPVARS) {
    windows_core::link!("msvfw32.dll" "system" fn ICSeqCompressFrameEnd(pc : *const COMPVARS));
    unsafe { ICSeqCompressFrameEnd(pc) }
}
#[cfg(feature = "wingdi")]
#[inline]
pub unsafe fn ICSeqCompressFrameStart(pc: *const COMPVARS, lpbiin: *const super::wingdi::BITMAPINFO) -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "system" fn ICSeqCompressFrameStart(pc : *const COMPVARS, lpbiin : *const super::wingdi::BITMAPINFO) -> windows_core::BOOL);
    unsafe { ICSeqCompressFrameStart(pc, lpbiin) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn MCIWndCreateA<P3>(hwndparent: Option<super::windef::HWND>, hinstance: Option<super::minwindef::HINSTANCE>, dwstyle: u32, szfile: P3) -> super::windef::HWND
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msvfw32.dll" "C" fn MCIWndCreateA(hwndparent : super::windef::HWND, hinstance : super::minwindef::HINSTANCE, dwstyle : u32, szfile : windows_core::PCSTR) -> super::windef::HWND);
    unsafe { MCIWndCreateA(hwndparent.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, dwstyle, szfile.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn MCIWndCreateW<P3>(hwndparent: Option<super::windef::HWND>, hinstance: Option<super::minwindef::HINSTANCE>, dwstyle: u32, szfile: P3) -> super::windef::HWND
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msvfw32.dll" "C" fn MCIWndCreateW(hwndparent : super::windef::HWND, hinstance : super::minwindef::HINSTANCE, dwstyle : u32, szfile : windows_core::PCWSTR) -> super::windef::HWND);
    unsafe { MCIWndCreateW(hwndparent.unwrap_or(core::mem::zeroed()) as _, hinstance.unwrap_or(core::mem::zeroed()) as _, dwstyle, szfile.param().abi()) }
}
#[inline]
pub unsafe fn MCIWndRegisterClass() -> windows_core::BOOL {
    windows_core::link!("msvfw32.dll" "C" fn MCIWndRegisterClass() -> windows_core::BOOL);
    unsafe { MCIWndRegisterClass() }
}
#[inline]
pub unsafe fn VideoForWindowsVersion() -> u32 {
    windows_core::link!("msvfw32.dll" "system" fn VideoForWindowsVersion() -> u32);
    unsafe { VideoForWindowsVersion() }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn capCreateCaptureWindowA<P0>(lpszwindowname: P0, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<super::windef::HWND>, nid: i32) -> super::windef::HWND
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("avicap32.dll" "system" fn capCreateCaptureWindowA(lpszwindowname : windows_core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::windef::HWND, nid : i32) -> super::windef::HWND);
    unsafe { capCreateCaptureWindowA(lpszwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.unwrap_or(core::mem::zeroed()) as _, nid) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn capCreateCaptureWindowW<P0>(lpszwindowname: P0, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Option<super::windef::HWND>, nid: i32) -> super::windef::HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("avicap32.dll" "system" fn capCreateCaptureWindowW(lpszwindowname : windows_core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::windef::HWND, nid : i32) -> super::windef::HWND);
    unsafe { capCreateCaptureWindowW(lpszwindowname.param().abi(), dwstyle, x, y, nwidth, nheight, hwndparent.unwrap_or(core::mem::zeroed()) as _, nid) }
}
#[inline]
pub unsafe fn capGetDriverDescriptionA(wdriverindex: u32, lpszname: &mut [u8], lpszver: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("avicap32.dll" "system" fn capGetDriverDescriptionA(wdriverindex : u32, lpszname : windows_core::PSTR, cbname : i32, lpszver : windows_core::PSTR, cbver : i32) -> windows_core::BOOL);
    unsafe { capGetDriverDescriptionA(wdriverindex, core::mem::transmute(lpszname.as_mut_ptr()), lpszname.len().try_into().unwrap(), core::mem::transmute(lpszver.as_mut_ptr()), lpszver.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn capGetDriverDescriptionW(wdriverindex: u32, lpszname: &mut [u16], lpszver: &mut [u16]) -> windows_core::BOOL {
    windows_core::link!("avicap32.dll" "system" fn capGetDriverDescriptionW(wdriverindex : u32, lpszname : windows_core::PWSTR, cbname : i32, lpszver : windows_core::PWSTR, cbver : i32) -> windows_core::BOOL);
    unsafe { capGetDriverDescriptionW(wdriverindex, core::mem::transmute(lpszname.as_mut_ptr()), lpszname.len().try_into().unwrap(), core::mem::transmute(lpszver.as_mut_ptr()), lpszver.len().try_into().unwrap()) }
}
pub const AVICOMPRESSF_DATARATE: u32 = 2;
pub const AVICOMPRESSF_INTERLEAVE: u32 = 1;
pub const AVICOMPRESSF_KEYFRAMES: u32 = 4;
pub const AVICOMPRESSF_VALID: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVICOMPRESSOPTIONS {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwKeyFrameEvery: u32,
    pub dwQuality: u32,
    pub dwBytesPerSecond: u32,
    pub dwFlags: u32,
    pub lpFormat: *mut core::ffi::c_void,
    pub cbFormat: u32,
    pub lpParms: *mut core::ffi::c_void,
    pub cbParms: u32,
    pub dwInterleaveEvery: u32,
}
impl Default for AVICOMPRESSOPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AVIERR_BADFLAGS: i32 = -2147205015;
pub const AVIERR_BADFORMAT: i32 = -2147205018;
pub const AVIERR_BADHANDLE: i32 = -2147205012;
pub const AVIERR_BADPARAM: i32 = -2147205014;
pub const AVIERR_BADSIZE: i32 = -2147205013;
pub const AVIERR_BUFFERTOOSMALL: i32 = -2147205004;
pub const AVIERR_CANTCOMPRESS: i32 = -2147205003;
pub const AVIERR_COMPRESSOR: i32 = -2147205008;
pub const AVIERR_ERROR: i32 = -2147204921;
pub const AVIERR_FILEOPEN: i32 = -2147205009;
pub const AVIERR_FILEREAD: i32 = -2147205011;
pub const AVIERR_FILEWRITE: i32 = -2147205010;
pub const AVIERR_INTERNAL: i32 = -2147205016;
pub const AVIERR_MEMORY: i32 = -2147205017;
pub const AVIERR_NOCOMPRESSOR: i32 = -2147205007;
pub const AVIERR_NODATA: i32 = -2147205005;
pub const AVIERR_OK: u32 = 0;
pub const AVIERR_READONLY: i32 = -2147205006;
pub const AVIERR_UNSUPPORTED: i32 = -2147205019;
pub const AVIERR_USERABORT: i32 = -2147204922;
pub const AVIFILECAPS_ALLKEYFRAMES: u32 = 16;
pub const AVIFILECAPS_CANREAD: u32 = 1;
pub const AVIFILECAPS_CANWRITE: u32 = 2;
pub const AVIFILECAPS_NOCOMPRESSION: u32 = 32;
pub const AVIFILEHANDLER_CANACCEPTNONRGB: u32 = 4;
pub const AVIFILEHANDLER_CANREAD: u32 = 1;
pub const AVIFILEHANDLER_CANWRITE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIFILEINFOA {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [i8; 64],
}
impl Default for AVIFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIFILEINFOW {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [u16; 64],
}
impl Default for AVIFILEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AVIFILEINFO_COPYRIGHTED: u32 = 131072;
pub const AVIFILEINFO_HASINDEX: u32 = 16;
pub const AVIFILEINFO_ISINTERLEAVED: u32 = 256;
pub const AVIFILEINFO_MUSTUSEINDEX: u32 = 32;
pub const AVIFILEINFO_WASCAPTUREFILE: u32 = 65536;
pub const AVIF_COPYRIGHTED: u32 = 131072;
pub const AVIF_HASINDEX: u32 = 16;
pub const AVIF_ISINTERLEAVED: u32 = 256;
pub const AVIF_MUSTUSEINDEX: u32 = 32;
pub const AVIF_WASCAPTUREFILE: u32 = 65536;
pub const AVIGETFRAMEF_BESTDISPLAYFMT: u32 = 1;
pub const AVIIF_COMPUSE: u32 = 268369920;
pub const AVIIF_FIRSTPART: u32 = 32;
pub const AVIIF_KEYFRAME: u32 = 16;
pub const AVIIF_LASTPART: u32 = 64;
pub const AVIIF_LIST: u32 = 1;
pub const AVIIF_MIDPART: u32 = 96;
pub const AVIIF_NOTIME: u32 = 256;
pub const AVIIF_TWOCC: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AVIINDEXENTRY {
    pub ckid: u32,
    pub dwFlags: u32,
    pub dwChunkOffset: u32,
    pub dwChunkLength: u32,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVIPALCHANGE {
    pub bFirstEntry: u8,
    pub bNumEntries: u8,
    pub wFlags: u16,
    pub peNew: [super::wingdi::PALETTEENTRY; 0],
}
#[cfg(feature = "wingdi")]
impl Default for AVIPALCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AVISAVECALLBACK = Option<unsafe extern "system" fn(param0: i32) -> windows_core::BOOL>;
pub const AVISF_DISABLED: u32 = 1;
pub const AVISF_VIDEO_PALCHANGES: u32 = 65536;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISTREAMINFOA {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::windef::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [i8; 64],
}
#[cfg(feature = "windef")]
impl Default for AVISTREAMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AVISTREAMINFOW {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::windef::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [u16; 64],
}
#[cfg(feature = "windef")]
impl Default for AVISTREAMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AVISTREAMINFO_DISABLED: u32 = 1;
pub const AVISTREAMINFO_FORMATCHANGES: u32 = 65536;
pub const AVISTREAMREAD_CONVENIENT: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AVIStreamHeader {
    pub fccType: super::mmiscapi::FOURCC,
    pub fccHandler: super::mmiscapi::FOURCC,
    pub dwFlags: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwInitialFrames: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::windef::RECT,
}
pub const AVI_HEADERSIZE: u32 = 2048;
pub const AVSTREAMMASTER_AUDIO: u32 = 0;
pub const AVSTREAMMASTER_NONE: u32 = 1;
pub const BI_1632: u32 = 842217009;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPCONTROLCALLBACK = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, nstate: i32) -> super::minwindef::LRESULT>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: u32,
    pub fHasOverlay: windows_core::BOOL,
    pub fHasDlgVideoSource: windows_core::BOOL,
    pub fHasDlgVideoFormat: windows_core::BOOL,
    pub fHasDlgVideoDisplay: windows_core::BOOL,
    pub fCaptureInitialized: windows_core::BOOL,
    pub fDriverSuppliesPalettes: windows_core::BOOL,
    pub hVideoIn: super::winnt::HANDLE,
    pub hVideoOut: super::winnt::HANDLE,
    pub hVideoExtIn: super::winnt::HANDLE,
    pub hVideoExtOut: super::winnt::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPERRORCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, nid: i32, lpsz: windows_core::PCSTR) -> super::minwindef::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPERRORCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, nid: i32, lpsz: windows_core::PCWSTR) -> super::minwindef::LRESULT>;
#[repr(C)]
#[cfg(feature = "mmiscapi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAPINFOCHUNK {
    pub fccInfoID: super::mmiscapi::FOURCC,
    pub lpData: *mut core::ffi::c_void,
    pub cbData: i32,
}
#[cfg(feature = "mmiscapi")]
impl Default for CAPINFOCHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CAPSTATUS {
    pub uiImageWidth: u32,
    pub uiImageHeight: u32,
    pub fLiveWindow: windows_core::BOOL,
    pub fOverlayWindow: windows_core::BOOL,
    pub fScale: windows_core::BOOL,
    pub ptScroll: super::windef::POINT,
    pub fUsingDefaultPalette: windows_core::BOOL,
    pub fAudioHardware: windows_core::BOOL,
    pub fCapFileExists: windows_core::BOOL,
    pub dwCurrentVideoFrame: u32,
    pub dwCurrentVideoFramesDropped: u32,
    pub dwCurrentWaveSamples: u32,
    pub dwCurrentTimeElapsedMS: u32,
    pub hPalCurrent: super::windef::HPALETTE,
    pub fCapturingNow: windows_core::BOOL,
    pub dwReturn: u32,
    pub wNumVideoAllocated: u32,
    pub wNumAudioAllocated: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPSTATUSCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, nid: i32, lpsz: windows_core::PCSTR) -> super::minwindef::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPSTATUSCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, nid: i32, lpsz: windows_core::PCWSTR) -> super::minwindef::LRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: u32,
    pub fMakeUserHitOKToCapture: windows_core::BOOL,
    pub wPercentDropForError: u32,
    pub fYield: windows_core::BOOL,
    pub dwIndexSize: u32,
    pub wChunkGranularity: u32,
    pub fUsingDOSMemory: windows_core::BOOL,
    pub wNumVideoRequested: u32,
    pub fCaptureAudio: windows_core::BOOL,
    pub wNumAudioRequested: u32,
    pub vKeyAbort: u32,
    pub fAbortLeftMouse: windows_core::BOOL,
    pub fAbortRightMouse: windows_core::BOOL,
    pub fLimitEnabled: windows_core::BOOL,
    pub wTimeLimit: u32,
    pub fMCIControl: windows_core::BOOL,
    pub fStepMCIDevice: windows_core::BOOL,
    pub dwMCIStartTime: u32,
    pub dwMCIStopTime: u32,
    pub fStepCaptureAt2x: windows_core::BOOL,
    pub wStepCaptureAverageFrames: u32,
    pub dwAudioBufferSize: u32,
    pub fDisableWriteCache: windows_core::BOOL,
    pub AVStreamMaster: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPVIDEOCALLBACK = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, lpvhdr: *const VIDEOHDR) -> super::minwindef::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
pub type CAPWAVECALLBACK = Option<unsafe extern "system" fn(hwnd: super::windef::HWND, lpwhdr: *const super::mmeapi::WAVEHDR) -> super::minwindef::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPYIELDCALLBACK = Option<unsafe extern "system" fn(hwnd: super::windef::HWND) -> super::minwindef::LRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHANNEL_CAPS {
    pub dwFlags: u32,
    pub dwSrcRectXMod: u32,
    pub dwSrcRectYMod: u32,
    pub dwSrcRectWidthMod: u32,
    pub dwSrcRectHeightMod: u32,
    pub dwDstRectXMod: u32,
    pub dwDstRectYMod: u32,
    pub dwDstRectWidthMod: u32,
    pub dwDstRectHeightMod: u32,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMPVARS {
    pub cbSize: i32,
    pub dwFlags: u32,
    pub hic: HIC,
    pub fccType: u32,
    pub fccHandler: u32,
    pub lpbiIn: super::wingdi::LPBITMAPINFO,
    pub lpbiOut: super::wingdi::LPBITMAPINFO,
    pub lpBitsOut: *mut core::ffi::c_void,
    pub lpBitsPrev: *mut core::ffi::c_void,
    pub lFrame: i32,
    pub lKey: i32,
    pub lDataRate: i32,
    pub lQ: i32,
    pub lKeyCount: i32,
    pub lpState: *mut core::ffi::c_void,
    pub cbState: i32,
}
#[cfg(feature = "wingdi")]
impl Default for COMPVARS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CONTROLCALLBACK_CAPTURING: u32 = 2;
pub const CONTROLCALLBACK_PREROLL: u32 = 1;
pub const DDF_0001: u32 = 1;
pub const DDF_2000: u32 = 8192;
pub const DDF_ANIMATE: u32 = 32;
pub const DDF_BACKGROUNDPAL: u32 = 512;
pub const DDF_BUFFER: u32 = 64;
pub const DDF_DONTDRAW: u32 = 16;
pub const DDF_FULLSCREEN: u32 = 256;
pub const DDF_HALFTONE: u32 = 4096;
pub const DDF_HURRYUP: u32 = 2048;
pub const DDF_JUSTDRAWIT: u32 = 128;
pub const DDF_NOTKEYFRAME: u32 = 1024;
pub const DDF_PREROLL: u32 = 16;
pub const DDF_SAME_DIB: u32 = 8;
pub const DDF_SAME_DRAW: u32 = 8;
pub const DDF_SAME_HDC: u32 = 4;
pub const DDF_SAME_SIZE: u32 = 8;
pub const DDF_UPDATE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRAWDIBTIME {
    pub timeCount: i32,
    pub timeDraw: i32,
    pub timeDecompress: i32,
    pub timeDither: i32,
    pub timeStretch: i32,
    pub timeBlt: i32,
    pub timeSetDIBits: i32,
}
pub const DVM_CONFIGURE_END: u32 = 8191;
pub const DVM_CONFIGURE_START: u32 = 4096;
pub const DVM_DST_RECT: u32 = 4101;
pub const DVM_FORMAT: u32 = 4098;
pub const DVM_PALETTE: u32 = 4097;
pub const DVM_PALETTERGB555: u32 = 4099;
pub const DVM_SRC_RECT: u32 = 4100;
pub const DVM_USER: u32 = 16384;
pub const DV_ERR_13: u32 = 16;
pub const DV_ERR_ALLOCATED: u32 = 19;
pub const DV_ERR_BADDEVICEID: u32 = 20;
pub const DV_ERR_BADERRNUM: u32 = 22;
pub const DV_ERR_BADFORMAT: u32 = 2;
pub const DV_ERR_BADINSTALL: u32 = 8;
pub const DV_ERR_BASE: u32 = 1;
pub const DV_ERR_CONFIG1: u32 = 13;
pub const DV_ERR_CONFIG2: u32 = 14;
pub const DV_ERR_CREATEPALETTE: u32 = 9;
pub const DV_ERR_DMA_CONFLICT: u32 = 26;
pub const DV_ERR_FLAGS: u32 = 15;
pub const DV_ERR_INT_CONFLICT: u32 = 27;
pub const DV_ERR_INVALHANDLE: u32 = 21;
pub const DV_ERR_IO_CONFLICT: u32 = 25;
pub const DV_ERR_LASTERROR: u32 = 28;
pub const DV_ERR_MEM_CONFLICT: u32 = 24;
pub const DV_ERR_NOMEM: u32 = 18;
pub const DV_ERR_NONSPECIFIC: u32 = 1;
pub const DV_ERR_NOTDETECTED: u32 = 7;
pub const DV_ERR_NOTSUPPORTED: u32 = 17;
pub const DV_ERR_NO_BUFFERS: u32 = 23;
pub const DV_ERR_OK: u32 = 0;
pub const DV_ERR_PARAM1: u32 = 11;
pub const DV_ERR_PARAM2: u32 = 12;
pub const DV_ERR_PROTECT_ONLY: u32 = 28;
pub const DV_ERR_SIZEFIELD: u32 = 10;
pub const DV_ERR_STILLPLAYING: u32 = 3;
pub const DV_ERR_SYNC: u32 = 5;
pub const DV_ERR_TOOMANYCHANNELS: u32 = 6;
pub const DV_ERR_UNPREPARED: u32 = 4;
pub const DV_ERR_USER_MSG: u32 = 1001;
pub const DV_VM_CLOSE: u32 = 977;
pub const DV_VM_DATA: u32 = 978;
pub const DV_VM_ERROR: u32 = 979;
pub const DV_VM_OPEN: u32 = 976;
pub const FIND_ANY: u32 = 32;
pub const FIND_DIR: u32 = 15;
pub const FIND_FORMAT: u32 = 64;
pub const FIND_FROM_START: u32 = 8;
pub const FIND_INDEX: u32 = 16384;
pub const FIND_KEY: u32 = 16;
pub const FIND_LENGTH: u32 = 4096;
pub const FIND_NEXT: u32 = 1;
pub const FIND_OFFSET: u32 = 8192;
pub const FIND_POS: u32 = 0;
pub const FIND_PREV: u32 = 4;
pub const FIND_RET: u32 = 61440;
pub const FIND_SIZE: u32 = 12288;
pub const FIND_TYPE: u32 = 240;
#[cfg(feature = "winnt")]
pub type HDRAWDIB = super::winnt::HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIC(pub *mut core::ffi::c_void);
impl Default for HIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HVIDEO(pub *mut core::ffi::c_void);
impl Default for HVIDEO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IAVIEditStream, IAVIEditStream_Vtbl, 0x00020024_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAVIEditStream, windows_core::IUnknown);
impl IAVIEditStream {
    pub unsafe fn Cut(&self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut Option<IAVIStream>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cut)(windows_core::Interface::as_raw(self), plstart as _, pllength as _, core::mem::transmute(ppresult)) }
    }
    pub unsafe fn Copy(&self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut Option<IAVIStream>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), plstart as _, pllength as _, core::mem::transmute(ppresult)) }
    }
    pub unsafe fn Paste(&self, plpos: *mut i32, pllength: *mut i32, pstream: &Option<IAVIStream>, lstart: i32, lend: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Paste)(windows_core::Interface::as_raw(self), plpos as _, pllength as _, core::mem::transmute_copy(pstream), lstart, lend) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IAVIStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInfo)(windows_core::Interface::as_raw(self), lpinfo, cbinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIEditStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Paste: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const AVISTREAMINFOW, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetInfo: usize,
}
#[cfg(feature = "windef")]
pub trait IAVIEditStream_Impl: windows_core::IUnknownImpl {
    fn Cut(&self, plstart: *mut i32, pllength: *mut i32, ppresult: windows_core::OutRef<IAVIStream>) -> windows_core::Result<()>;
    fn Copy(&self, plstart: *mut i32, pllength: *mut i32, ppresult: windows_core::OutRef<IAVIStream>) -> windows_core::Result<()>;
    fn Paste(&self, plpos: *mut i32, pllength: *mut i32, pstream: windows_core::OutRef<IAVIStream>, lstart: i32, lend: i32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IAVIStream>;
    fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAVIEditStream_Vtbl {
    pub const fn new<Identity: IAVIEditStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cut<Identity: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIEditStream_Impl::Cut(this, core::mem::transmute_copy(&plstart), core::mem::transmute_copy(&pllength), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIEditStream_Impl::Copy(this, core::mem::transmute_copy(&plstart), core::mem::transmute_copy(&pllength), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn Paste<Identity: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: *mut core::ffi::c_void, lstart: i32, lend: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIEditStream_Impl::Paste(this, core::mem::transmute_copy(&plpos), core::mem::transmute_copy(&pllength), core::mem::transmute(&pstream), core::mem::transmute_copy(&lstart), core::mem::transmute_copy(&lend)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAVIEditStream_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInfo<Identity: IAVIEditStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIEditStream_Impl::SetInfo(this, core::mem::transmute_copy(&lpinfo), core::mem::transmute_copy(&cbinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cut: Cut::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            SetInfo: SetInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAVIEditStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAVIEditStream {}
windows_core::imp::define_interface!(IAVIFile, IAVIFile_Vtbl, 0x00020020_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAVIFile, windows_core::IUnknown);
impl IAVIFile {
    pub unsafe fn Info(&self, pfi: *mut AVIFILEINFOW, lsize: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Info)(windows_core::Interface::as_raw(self), pfi as _, lsize) }
    }
    pub unsafe fn GetStream(&self, ppstream: *mut Option<IAVIStream>, fcctype: u32, lparam: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStream)(windows_core::Interface::as_raw(self), core::mem::transmute(ppstream), fcctype, lparam) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CreateStream(&self, ppstream: *mut Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), core::mem::transmute(ppstream), psi) }
    }
    pub unsafe fn WriteData(&self, ckid: u32, lpdata: *const core::ffi::c_void, cbdata: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteData)(windows_core::Interface::as_raw(self), ckid, lpdata, cbdata) }
    }
    pub unsafe fn ReadData(&self, ckid: u32, lpdata: *mut core::ffi::c_void, lpcbdata: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadData)(windows_core::Interface::as_raw(self), ckid, lpdata as _, lpcbdata as _) }
    }
    pub unsafe fn EndRecord(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndRecord)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DeleteStream(&self, fcctype: u32, lparam: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteStream)(windows_core::Interface::as_raw(self), fcctype, lparam) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AVIFILEINFOW, i32) -> windows_core::HRESULT,
    pub GetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *const AVISTREAMINFOW) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateStream: usize,
    pub WriteData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReadData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EndRecord: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IAVIFile_Impl: windows_core::IUnknownImpl {
    fn Info(&self, pfi: *mut AVIFILEINFOW, lsize: i32) -> windows_core::Result<()>;
    fn GetStream(&self, ppstream: windows_core::OutRef<IAVIStream>, fcctype: u32, lparam: i32) -> windows_core::Result<()>;
    fn CreateStream(&self, ppstream: windows_core::OutRef<IAVIStream>, psi: *const AVISTREAMINFOW) -> windows_core::Result<()>;
    fn WriteData(&self, ckid: u32, lpdata: *const core::ffi::c_void, cbdata: i32) -> windows_core::Result<()>;
    fn ReadData(&self, ckid: u32, lpdata: *mut core::ffi::c_void, lpcbdata: *mut i32) -> windows_core::Result<()>;
    fn EndRecord(&self) -> windows_core::Result<()>;
    fn DeleteStream(&self, fcctype: u32, lparam: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAVIFile_Vtbl {
    pub const fn new<Identity: IAVIFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Info<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::Info(this, core::mem::transmute_copy(&pfi), core::mem::transmute_copy(&lsize)).into()
            }
        }
        unsafe extern "system" fn GetStream<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void, fcctype: u32, lparam: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::GetStream(this, core::mem::transmute_copy(&ppstream), core::mem::transmute_copy(&fcctype), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn CreateStream<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void, psi: *const AVISTREAMINFOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::CreateStream(this, core::mem::transmute_copy(&ppstream), core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn WriteData<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ckid: u32, lpdata: *const core::ffi::c_void, cbdata: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::WriteData(this, core::mem::transmute_copy(&ckid), core::mem::transmute_copy(&lpdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn ReadData<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ckid: u32, lpdata: *mut core::ffi::c_void, lpcbdata: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::ReadData(this, core::mem::transmute_copy(&ckid), core::mem::transmute_copy(&lpdata), core::mem::transmute_copy(&lpcbdata)).into()
            }
        }
        unsafe extern "system" fn EndRecord<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::EndRecord(this).into()
            }
        }
        unsafe extern "system" fn DeleteStream<Identity: IAVIFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcctype: u32, lparam: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIFile_Impl::DeleteStream(this, core::mem::transmute_copy(&fcctype), core::mem::transmute_copy(&lparam)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Info: Info::<Identity, OFFSET>,
            GetStream: GetStream::<Identity, OFFSET>,
            CreateStream: CreateStream::<Identity, OFFSET>,
            WriteData: WriteData::<Identity, OFFSET>,
            ReadData: ReadData::<Identity, OFFSET>,
            EndRecord: EndRecord::<Identity, OFFSET>,
            DeleteStream: DeleteStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAVIFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAVIFile {}
#[cfg(feature = "objidl")]
windows_core::imp::define_interface!(IAVIPersistFile, IAVIPersistFile_Vtbl, 0x00020025_0000_0000_c000_000000000046);
#[cfg(feature = "objidl")]
impl core::ops::Deref for IAVIPersistFile {
    type Target = super::objidl::IPersistFile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidl")]
windows_core::imp::interface_hierarchy!(IAVIPersistFile, windows_core::IUnknown, super::objidl::IPersist, super::objidl::IPersistFile);
#[cfg(feature = "objidl")]
impl IAVIPersistFile {
    pub unsafe fn Reserved1(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reserved1)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "objidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAVIPersistFile_Vtbl {
    pub base__: super::objidl::IPersistFile_Vtbl,
    pub Reserved1: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidl")]
pub trait IAVIPersistFile_Impl: super::objidl::IPersistFile_Impl {
    fn Reserved1(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IAVIPersistFile_Vtbl {
    pub const fn new<Identity: IAVIPersistFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reserved1<Identity: IAVIPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIPersistFile_Impl::Reserved1(this).into()
            }
        }
        Self { base__: super::objidl::IPersistFile_Vtbl::new::<Identity, OFFSET>(), Reserved1: Reserved1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAVIPersistFile as windows_core::Interface>::IID || iid == &<super::objidl::IPersist as windows_core::Interface>::IID || iid == &<super::objidl::IPersistFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IAVIPersistFile {}
windows_core::imp::define_interface!(IAVIStream, IAVIStream_Vtbl, 0x00020021_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAVIStream, windows_core::IUnknown);
impl IAVIStream {
    #[cfg(feature = "minwindef")]
    pub unsafe fn Create(&self, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), lparam1, lparam2) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Info(&self, psi: *mut AVISTREAMINFOW, lsize: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Info)(windows_core::Interface::as_raw(self), psi as _, lsize) }
    }
    pub unsafe fn FindSample(&self, lpos: i32, lflags: i32) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).FindSample)(windows_core::Interface::as_raw(self), lpos, lflags) }
    }
    pub unsafe fn ReadFormat(&self, lpos: i32, lpformat: Option<*mut core::ffi::c_void>, lpcbformat: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadFormat)(windows_core::Interface::as_raw(self), lpos, lpformat.unwrap_or(core::mem::zeroed()) as _, lpcbformat as _) }
    }
    pub unsafe fn SetFormat(&self, lpos: i32, lpformat: *const core::ffi::c_void, cbformat: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), lpos, lpformat, cbformat) }
    }
    pub unsafe fn Read(&self, lstart: i32, lsamples: i32, lpbuffer: Option<*mut core::ffi::c_void>, cbbuffer: i32, plbytes: Option<*mut i32>, plsamples: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), lstart, lsamples, lpbuffer.unwrap_or(core::mem::zeroed()) as _, cbbuffer, plbytes.unwrap_or(core::mem::zeroed()) as _, plsamples.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Write(&self, lstart: i32, lsamples: i32, lpbuffer: *const core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: Option<*mut i32>, plbyteswritten: Option<*mut i32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), lstart, lsamples, lpbuffer, cbbuffer, dwflags, plsampwritten.unwrap_or(core::mem::zeroed()) as _, plbyteswritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Delete(&self, lstart: i32, lsamples: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), lstart, lsamples) }
    }
    pub unsafe fn ReadData(&self, fcc: u32, lp: Option<*mut core::ffi::c_void>, lpcb: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadData)(windows_core::Interface::as_raw(self), fcc, lp.unwrap_or(core::mem::zeroed()) as _, lpcb as _) }
    }
    pub unsafe fn WriteData(&self, fcc: u32, lp: *const core::ffi::c_void, cb: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteData)(windows_core::Interface::as_raw(self), fcc, lp, cb) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInfo)(windows_core::Interface::as_raw(self), lpinfo, cbinfo) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::LPARAM, super::minwindef::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Create: usize,
    #[cfg(feature = "windef")]
    pub Info: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AVISTREAMINFOW, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Info: usize,
    pub FindSample: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> i32,
    pub ReadFormat: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const core::ffi::c_void, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub ReadData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WriteData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const AVISTREAMINFOW, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetInfo: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub trait IAVIStream_Impl: windows_core::IUnknownImpl {
    fn Create(&self, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> windows_core::Result<()>;
    fn Info(&self, psi: *mut AVISTREAMINFOW, lsize: i32) -> windows_core::Result<()>;
    fn FindSample(&self, lpos: i32, lflags: i32) -> i32;
    fn ReadFormat(&self, lpos: i32, lpformat: *mut core::ffi::c_void, lpcbformat: *mut i32) -> windows_core::Result<()>;
    fn SetFormat(&self, lpos: i32, lpformat: *const core::ffi::c_void, cbformat: i32) -> windows_core::Result<()>;
    fn Read(&self, lstart: i32, lsamples: i32, lpbuffer: *mut core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> windows_core::Result<()>;
    fn Write(&self, lstart: i32, lsamples: i32, lpbuffer: *const core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> windows_core::Result<()>;
    fn Delete(&self, lstart: i32, lsamples: i32) -> windows_core::Result<()>;
    fn ReadData(&self, fcc: u32, lp: *mut core::ffi::c_void, lpcb: *mut i32) -> windows_core::Result<()>;
    fn WriteData(&self, fcc: u32, lp: *const core::ffi::c_void, cb: i32) -> windows_core::Result<()>;
    fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl IAVIStream_Vtbl {
    pub const fn new<Identity: IAVIStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::Create(this, core::mem::transmute_copy(&lparam1), core::mem::transmute_copy(&lparam2)).into()
            }
        }
        unsafe extern "system" fn Info<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::Info(this, core::mem::transmute_copy(&psi), core::mem::transmute_copy(&lsize)).into()
            }
        }
        unsafe extern "system" fn FindSample<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpos: i32, lflags: i32) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::FindSample(this, core::mem::transmute_copy(&lpos), core::mem::transmute_copy(&lflags))
            }
        }
        unsafe extern "system" fn ReadFormat<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpos: i32, lpformat: *mut core::ffi::c_void, lpcbformat: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::ReadFormat(this, core::mem::transmute_copy(&lpos), core::mem::transmute_copy(&lpformat), core::mem::transmute_copy(&lpcbformat)).into()
            }
        }
        unsafe extern "system" fn SetFormat<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpos: i32, lpformat: *const core::ffi::c_void, cbformat: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::SetFormat(this, core::mem::transmute_copy(&lpos), core::mem::transmute_copy(&lpformat), core::mem::transmute_copy(&cbformat)).into()
            }
        }
        unsafe extern "system" fn Read<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::Read(this, core::mem::transmute_copy(&lstart), core::mem::transmute_copy(&lsamples), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&plbytes), core::mem::transmute_copy(&plsamples)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::Write(this, core::mem::transmute_copy(&lstart), core::mem::transmute_copy(&lsamples), core::mem::transmute_copy(&lpbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&plsampwritten), core::mem::transmute_copy(&plbyteswritten)).into()
            }
        }
        unsafe extern "system" fn Delete<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstart: i32, lsamples: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::Delete(this, core::mem::transmute_copy(&lstart), core::mem::transmute_copy(&lsamples)).into()
            }
        }
        unsafe extern "system" fn ReadData<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, lpcb: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::ReadData(this, core::mem::transmute_copy(&fcc), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&lpcb)).into()
            }
        }
        unsafe extern "system" fn WriteData<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcc: u32, lp: *const core::ffi::c_void, cb: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::WriteData(this, core::mem::transmute_copy(&fcc), core::mem::transmute_copy(&lp), core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn SetInfo<Identity: IAVIStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStream_Impl::SetInfo(this, core::mem::transmute_copy(&lpinfo), core::mem::transmute_copy(&cbinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            Info: Info::<Identity, OFFSET>,
            FindSample: FindSample::<Identity, OFFSET>,
            ReadFormat: ReadFormat::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            ReadData: ReadData::<Identity, OFFSET>,
            WriteData: WriteData::<Identity, OFFSET>,
            SetInfo: SetInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAVIStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl windows_core::RuntimeName for IAVIStream {}
windows_core::imp::define_interface!(IAVIStreaming, IAVIStreaming_Vtbl, 0x00020022_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAVIStreaming, windows_core::IUnknown);
impl IAVIStreaming {
    pub unsafe fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), lstart, lend, lrate) }
    }
    pub unsafe fn End(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIStreaming_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAVIStreaming_Impl: windows_core::IUnknownImpl {
    fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
}
impl IAVIStreaming_Vtbl {
    pub const fn new<Identity: IAVIStreaming_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin<Identity: IAVIStreaming_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStreaming_Impl::Begin(this, core::mem::transmute_copy(&lstart), core::mem::transmute_copy(&lend), core::mem::transmute_copy(&lrate)).into()
            }
        }
        unsafe extern "system" fn End<Identity: IAVIStreaming_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAVIStreaming_Impl::End(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Begin: Begin::<Identity, OFFSET>, End: End::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAVIStreaming as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAVIStreaming {}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICCOMPRESS {
    pub dwFlags: u32,
    pub lpbiOutput: super::wingdi::LPBITMAPINFOHEADER,
    pub lpOutput: *mut core::ffi::c_void,
    pub lpbiInput: super::wingdi::LPBITMAPINFOHEADER,
    pub lpInput: *mut core::ffi::c_void,
    pub lpckid: super::minwindef::LPDWORD,
    pub lpdwFlags: super::minwindef::LPDWORD,
    pub lFrameNum: i32,
    pub dwFrameSize: u32,
    pub dwQuality: u32,
    pub lpbiPrev: super::wingdi::LPBITMAPINFOHEADER,
    pub lpPrev: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
impl Default for ICCOMPRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICCOMPRESSFRAMES {
    pub dwFlags: u32,
    pub lpbiOutput: super::wingdi::LPBITMAPINFOHEADER,
    pub lOutput: super::minwindef::LPARAM,
    pub lpbiInput: super::wingdi::LPBITMAPINFOHEADER,
    pub lInput: super::minwindef::LPARAM,
    pub lStartFrame: i32,
    pub lFrameCount: i32,
    pub lQuality: i32,
    pub lDataRate: i32,
    pub lKeyRate: i32,
    pub dwRate: u32,
    pub dwScale: u32,
    pub dwOverheadPerFrame: u32,
    pub dwReserved2: u32,
    pub GetData: *mut u8,
    pub PutData: *mut u8,
}
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
impl Default for ICCOMPRESSFRAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICCOMPRESSFRAMES_PADDING: u32 = 1;
pub const ICCOMPRESS_KEYFRAME: u32 = 1;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICDECOMPRESS {
    pub dwFlags: u32,
    pub lpbiInput: super::wingdi::LPBITMAPINFOHEADER,
    pub lpInput: *mut core::ffi::c_void,
    pub lpbiOutput: super::wingdi::LPBITMAPINFOHEADER,
    pub lpOutput: *mut core::ffi::c_void,
    pub ckid: u32,
}
#[cfg(feature = "wingdi")]
impl Default for ICDECOMPRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICDECOMPRESSEX {
    pub dwFlags: u32,
    pub lpbiSrc: super::wingdi::LPBITMAPINFOHEADER,
    pub lpSrc: *mut core::ffi::c_void,
    pub lpbiDst: super::wingdi::LPBITMAPINFOHEADER,
    pub lpDst: *mut core::ffi::c_void,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
}
#[cfg(feature = "wingdi")]
impl Default for ICDECOMPRESSEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICDECOMPRESS_HURRYUP: u32 = 2147483648;
pub const ICDECOMPRESS_NOTKEYFRAME: u32 = 134217728;
pub const ICDECOMPRESS_NULLFRAME: u32 = 268435456;
pub const ICDECOMPRESS_PREROLL: u32 = 536870912;
pub const ICDECOMPRESS_UPDATE: u32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICDRAW {
    pub dwFlags: u32,
    pub lpFormat: *mut core::ffi::c_void,
    pub lpData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub lTime: i32,
}
impl Default for ICDRAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ICDRAWBEGIN {
    pub dwFlags: u32,
    pub hpal: super::windef::HPALETTE,
    pub hwnd: super::windef::HWND,
    pub hdc: super::windef::HDC,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub lpbi: super::wingdi::LPBITMAPINFOHEADER,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dwRate: u32,
    pub dwScale: u32,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ICDRAWSUGGEST {
    pub lpbiIn: super::wingdi::LPBITMAPINFOHEADER,
    pub lpbiSuggest: super::wingdi::LPBITMAPINFOHEADER,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
}
pub const ICDRAW_ANIMATE: u32 = 8;
pub const ICDRAW_BUFFER: u32 = 256;
pub const ICDRAW_CONTINUE: u32 = 16;
pub const ICDRAW_FULLSCREEN: u32 = 2;
pub const ICDRAW_HDC: u32 = 4;
pub const ICDRAW_HURRYUP: u32 = 2147483648;
pub const ICDRAW_MEMORYDC: u32 = 32;
pub const ICDRAW_NOTKEYFRAME: u32 = 134217728;
pub const ICDRAW_NULLFRAME: u32 = 268435456;
pub const ICDRAW_PREROLL: u32 = 536870912;
pub const ICDRAW_QUERY: u32 = 1;
pub const ICDRAW_RENDER: u32 = 128;
pub const ICDRAW_UPDATE: u32 = 1073741824;
pub const ICDRAW_UPDATING: u32 = 64;
pub const ICERR_ABORT: i32 = -10;
pub const ICERR_BADBITDEPTH: i32 = -200;
pub const ICERR_BADFLAGS: i32 = -5;
pub const ICERR_BADFORMAT: i32 = -2;
pub const ICERR_BADHANDLE: i32 = -8;
pub const ICERR_BADIMAGESIZE: i32 = -201;
pub const ICERR_BADPARAM: i32 = -6;
pub const ICERR_BADSIZE: i32 = -7;
pub const ICERR_CANTUPDATE: i32 = -9;
pub const ICERR_CUSTOM: i32 = -400;
pub const ICERR_DONTDRAW: u32 = 1;
pub const ICERR_ERROR: i32 = -100;
pub const ICERR_GOTOKEYFRAME: u32 = 3;
pub const ICERR_INTERNAL: i32 = -4;
pub const ICERR_MEMORY: i32 = -3;
pub const ICERR_NEWPALETTE: u32 = 2;
pub const ICERR_OK: u32 = 0;
pub const ICERR_STOPDRAWING: u32 = 4;
pub const ICERR_UNSUPPORTED: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICINFO {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwVersion: u32,
    pub dwVersionICM: u32,
    pub szName: [u16; 16],
    pub szDescription: [u16; 128],
    pub szDriver: [u16; 128],
}
impl Default for ICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICINSTALL_DRIVER: u32 = 2;
pub const ICINSTALL_DRIVERW: u32 = 32770;
pub const ICINSTALL_FUNCTION: u32 = 1;
pub const ICINSTALL_HDRV: u32 = 4;
pub const ICINSTALL_UNICODE: u32 = 32768;
pub const ICMF_ABOUT_QUERY: u32 = 1;
pub const ICMF_CHOOSE_ALLCOMPRESSORS: u32 = 8;
pub const ICMF_CHOOSE_DATARATE: u32 = 2;
pub const ICMF_CHOOSE_KEYFRAME: u32 = 1;
pub const ICMF_CHOOSE_PREVIEW: u32 = 4;
pub const ICMF_COMPVARS_VALID: u32 = 1;
pub const ICMF_CONFIGURE_QUERY: u32 = 1;
pub const ICMODE_COMPRESS: u32 = 1;
pub const ICMODE_DECOMPRESS: u32 = 2;
pub const ICMODE_DRAW: u32 = 8;
pub const ICMODE_FASTCOMPRESS: u32 = 5;
pub const ICMODE_FASTDECOMPRESS: u32 = 3;
pub const ICMODE_QUERY: u32 = 4;
pub const ICM_ABOUT: u32 = 20491;
pub const ICM_COMPRESS: u32 = 16392;
pub const ICM_COMPRESS_BEGIN: u32 = 16391;
pub const ICM_COMPRESS_END: u32 = 16393;
pub const ICM_COMPRESS_FRAMES: u32 = 16455;
pub const ICM_COMPRESS_FRAMES_INFO: u32 = 16454;
pub const ICM_COMPRESS_GET_FORMAT: u32 = 16388;
pub const ICM_COMPRESS_GET_SIZE: u32 = 16389;
pub const ICM_COMPRESS_QUERY: u32 = 16390;
pub const ICM_CONFIGURE: u32 = 20490;
pub const ICM_DECOMPRESS: u32 = 16397;
pub const ICM_DECOMPRESSEX: u32 = 16446;
pub const ICM_DECOMPRESSEX_BEGIN: u32 = 16444;
pub const ICM_DECOMPRESSEX_END: u32 = 16447;
pub const ICM_DECOMPRESSEX_QUERY: u32 = 16445;
pub const ICM_DECOMPRESS_BEGIN: u32 = 16396;
pub const ICM_DECOMPRESS_END: u32 = 16398;
pub const ICM_DECOMPRESS_GET_FORMAT: u32 = 16394;
pub const ICM_DECOMPRESS_GET_PALETTE: u32 = 16414;
pub const ICM_DECOMPRESS_QUERY: u32 = 16395;
pub const ICM_DECOMPRESS_SET_PALETTE: u32 = 16413;
pub const ICM_DRAW: u32 = 16417;
pub const ICM_DRAW_BEGIN: u32 = 16399;
pub const ICM_DRAW_BITS: u32 = 16404;
pub const ICM_DRAW_CHANGEPALETTE: u32 = 16435;
pub const ICM_DRAW_END: u32 = 16405;
pub const ICM_DRAW_FLUSH: u32 = 16421;
pub const ICM_DRAW_GETTIME: u32 = 16416;
pub const ICM_DRAW_GET_PALETTE: u32 = 16400;
pub const ICM_DRAW_IDLE: u32 = 16436;
pub const ICM_DRAW_QUERY: u32 = 16415;
pub const ICM_DRAW_REALIZE: u32 = 16420;
pub const ICM_DRAW_RENDERBUFFER: u32 = 16422;
pub const ICM_DRAW_SETTIME: u32 = 16419;
pub const ICM_DRAW_START: u32 = 16402;
pub const ICM_DRAW_START_PLAY: u32 = 16423;
pub const ICM_DRAW_STOP: u32 = 16403;
pub const ICM_DRAW_STOP_PLAY: u32 = 16424;
pub const ICM_DRAW_SUGGESTFORMAT: u32 = 16434;
pub const ICM_DRAW_UPDATE: u32 = 16401;
pub const ICM_DRAW_WINDOW: u32 = 16418;
pub const ICM_ENUMFORMATS: u32 = 20501;
pub const ICM_FRAMERATE: u32 = 1382904390;
pub const ICM_GET: u32 = 20521;
pub const ICM_GETBUFFERSWANTED: u32 = 16425;
pub const ICM_GETDEFAULTKEYFRAMERATE: u32 = 16426;
pub const ICM_GETDEFAULTQUALITY: u32 = 20510;
pub const ICM_GETERRORTEXT: u32 = 20492;
pub const ICM_GETFORMATNAME: u32 = 20500;
pub const ICM_GETINFO: u32 = 20482;
pub const ICM_GETQUALITY: u32 = 20511;
pub const ICM_GETSTATE: u32 = 20480;
pub const ICM_KEYFRAMERATE: u32 = 1383687499;
pub const ICM_RESERVED: u32 = 20480;
pub const ICM_RESERVED_HIGH: u32 = 24576;
pub const ICM_RESERVED_LOW: u32 = 20480;
pub const ICM_SET: u32 = 20520;
pub const ICM_SETQUALITY: u32 = 20512;
pub const ICM_SETSTATE: u32 = 20481;
pub const ICM_SET_STATUS_PROC: u32 = 16456;
pub const ICM_USER: u32 = 16384;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICOPEN {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: super::minwindef::LRESULT,
    pub pV1Reserved: *mut core::ffi::c_void,
    pub pV2Reserved: *mut core::ffi::c_void,
    pub dnDevNode: u32,
}
#[cfg(feature = "minwindef")]
impl Default for ICOPEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ICPALETTE {
    pub dwFlags: u32,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: super::wingdi::LPPALETTEENTRY,
}
pub const ICQUALITY_DEFAULT: i32 = -1;
pub const ICQUALITY_HIGH: u32 = 10000;
pub const ICQUALITY_LOW: u32 = 0;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ICSETSTATUSPROC {
    pub dwFlags: u32,
    pub lParam: super::minwindef::LPARAM,
    pub Status: *mut u8,
}
#[cfg(feature = "minwindef")]
impl Default for ICSETSTATUSPROC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICSTATUS_END: u32 = 2;
pub const ICSTATUS_ERROR: u32 = 3;
pub const ICSTATUS_START: u32 = 0;
pub const ICSTATUS_STATUS: u32 = 1;
pub const ICSTATUS_YIELD: u32 = 4;
pub const ICVERSION: u32 = 260;
pub const IDS_CAP_AUDIO_DROP_COMPERROR: u32 = 442;
pub const IDS_CAP_AUDIO_DROP_ERROR: u32 = 441;
pub const IDS_CAP_AVI_DRAWDIB_ERROR: u32 = 439;
pub const IDS_CAP_AVI_INIT_ERROR: u32 = 433;
pub const IDS_CAP_BEGIN: u32 = 300;
pub const IDS_CAP_CANTOPEN: u32 = 409;
pub const IDS_CAP_COMPRESSOR_ERROR: u32 = 440;
pub const IDS_CAP_DEFAVIEXT: u32 = 407;
pub const IDS_CAP_DEFPALEXT: u32 = 408;
pub const IDS_CAP_DRIVER_ERROR: u32 = 418;
pub const IDS_CAP_END: u32 = 301;
pub const IDS_CAP_ERRORDIBSAVE: u32 = 406;
pub const IDS_CAP_ERRORPALOPEN: u32 = 404;
pub const IDS_CAP_ERRORPALSAVE: u32 = 405;
pub const IDS_CAP_FILEEXISTS: u32 = 403;
pub const IDS_CAP_FILE_OPEN_ERROR: u32 = 429;
pub const IDS_CAP_FILE_WRITE_ERROR: u32 = 430;
pub const IDS_CAP_INFO: u32 = 401;
pub const IDS_CAP_MCI_CANT_STEP_ERROR: u32 = 437;
pub const IDS_CAP_MCI_CONTROL_ERROR: u32 = 436;
pub const IDS_CAP_NODISKSPACE: u32 = 415;
pub const IDS_CAP_NO_AUDIO_CAP_ERROR: u32 = 438;
pub const IDS_CAP_NO_FRAME_CAP_ERROR: u32 = 434;
pub const IDS_CAP_NO_PALETTE_WARN: u32 = 435;
pub const IDS_CAP_OUTOFMEM: u32 = 402;
pub const IDS_CAP_READONLYFILE: u32 = 413;
pub const IDS_CAP_RECORDING_ERROR: u32 = 431;
pub const IDS_CAP_RECORDING_ERROR2: u32 = 432;
pub const IDS_CAP_SAVEASPERCENT: u32 = 417;
pub const IDS_CAP_SEQ_MSGSTART: u32 = 410;
pub const IDS_CAP_SEQ_MSGSTOP: u32 = 411;
pub const IDS_CAP_SETFILESIZE: u32 = 416;
pub const IDS_CAP_STAT_CAP_AUDIO: u32 = 509;
pub const IDS_CAP_STAT_CAP_FINI: u32 = 503;
pub const IDS_CAP_STAT_CAP_INIT: u32 = 502;
pub const IDS_CAP_STAT_CAP_L_FRAMES: u32 = 508;
pub const IDS_CAP_STAT_FRAMESDROPPED: u32 = 513;
pub const IDS_CAP_STAT_I_FRAMES: u32 = 506;
pub const IDS_CAP_STAT_LIVE_MODE: u32 = 500;
pub const IDS_CAP_STAT_L_FRAMES: u32 = 507;
pub const IDS_CAP_STAT_OPTPAL_BUILD: u32 = 505;
pub const IDS_CAP_STAT_OVERLAY_MODE: u32 = 501;
pub const IDS_CAP_STAT_PALETTE_BUILD: u32 = 504;
pub const IDS_CAP_STAT_VIDEOAUDIO: u32 = 511;
pub const IDS_CAP_STAT_VIDEOCURRENT: u32 = 510;
pub const IDS_CAP_STAT_VIDEOONLY: u32 = 512;
pub const IDS_CAP_VIDEDITERR: u32 = 412;
pub const IDS_CAP_VIDEO_ADD_ERROR: u32 = 427;
pub const IDS_CAP_VIDEO_ALLOC_ERROR: u32 = 425;
pub const IDS_CAP_VIDEO_OPEN_ERROR: u32 = 424;
pub const IDS_CAP_VIDEO_PREPARE_ERROR: u32 = 426;
pub const IDS_CAP_VIDEO_SIZE_ERROR: u32 = 428;
pub const IDS_CAP_WAVE_ADD_ERROR: u32 = 422;
pub const IDS_CAP_WAVE_ALLOC_ERROR: u32 = 420;
pub const IDS_CAP_WAVE_OPEN_ERROR: u32 = 419;
pub const IDS_CAP_WAVE_PREPARE_ERROR: u32 = 421;
pub const IDS_CAP_WAVE_SIZE_ERROR: u32 = 423;
pub const IDS_CAP_WRITEERROR: u32 = 414;
windows_core::imp::define_interface!(IGetFrame, IGetFrame_Vtbl, 0x00020023_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IGetFrame, windows_core::IUnknown);
impl IGetFrame {
    pub unsafe fn GetFrame(&self, lpos: i32) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).GetFrame)(windows_core::Interface::as_raw(self), lpos) }
    }
    pub unsafe fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), lstart, lend, lrate) }
    }
    pub unsafe fn End(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn SetFormat(&self, lpbi: *const super::wingdi::BITMAPINFOHEADER, lpbits: Option<*const core::ffi::c_void>, x: i32, y: i32, dx: i32, dy: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFormat)(windows_core::Interface::as_raw(self), lpbi, lpbits.unwrap_or(core::mem::zeroed()) as _, x, y, dx, dy) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> *mut core::ffi::c_void,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub SetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wingdi::BITMAPINFOHEADER, *const core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    SetFormat: usize,
}
#[cfg(feature = "wingdi")]
pub trait IGetFrame_Impl: windows_core::IUnknownImpl {
    fn GetFrame(&self, lpos: i32) -> *mut core::ffi::c_void;
    fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
    fn SetFormat(&self, lpbi: *const super::wingdi::BITMAPINFOHEADER, lpbits: *const core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "wingdi")]
impl IGetFrame_Vtbl {
    pub const fn new<Identity: IGetFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrame<Identity: IGetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpos: i32) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetFrame_Impl::GetFrame(this, core::mem::transmute_copy(&lpos))
            }
        }
        unsafe extern "system" fn Begin<Identity: IGetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetFrame_Impl::Begin(this, core::mem::transmute_copy(&lstart), core::mem::transmute_copy(&lend), core::mem::transmute_copy(&lrate)).into()
            }
        }
        unsafe extern "system" fn End<Identity: IGetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetFrame_Impl::End(this).into()
            }
        }
        unsafe extern "system" fn SetFormat<Identity: IGetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpbi: *const super::wingdi::BITMAPINFOHEADER, lpbits: *const core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGetFrame_Impl::SetFormat(this, core::mem::transmute_copy(&lpbi), core::mem::transmute_copy(&lpbits), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrame: GetFrame::<Identity, OFFSET>,
            Begin: Begin::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetFrame as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wingdi")]
impl windows_core::RuntimeName for IGetFrame {}
pub type LPAVICOMPRESSOPTIONS = *mut AVICOMPRESSOPTIONS;
pub type LPAVIFILEINFOA = *mut AVIFILEINFOA;
pub type LPAVIFILEINFOW = *mut AVIFILEINFOW;
#[cfg(feature = "windef")]
pub type LPAVISTREAMINFOA = *mut AVISTREAMINFOA;
#[cfg(feature = "windef")]
pub type LPAVISTREAMINFOW = *mut AVISTREAMINFOW;
#[cfg(feature = "winnt")]
pub type LPCAPDRIVERCAPS = *mut CAPDRIVERCAPS;
#[cfg(feature = "mmiscapi")]
pub type LPCAPINFOCHUNK = *mut CAPINFOCHUNK;
#[cfg(feature = "windef")]
pub type LPCAPSTATUS = *mut CAPSTATUS;
pub type LPCAPTUREPARMS = *mut CAPTUREPARMS;
pub type LPCHANNEL_CAPS = *mut CHANNEL_CAPS;
pub type LPDRAWDIBTIME = *mut DRAWDIBTIME;
pub type LPHVIDEO = *mut HVIDEO;
#[cfg(feature = "minwindef")]
pub type LPVIDEOHDR = *mut VIDEOHDR;
pub const MCIWNDF_NOAUTOSIZEMOVIE: u32 = 4;
pub const MCIWNDF_NOAUTOSIZEWINDOW: u32 = 1;
pub const MCIWNDF_NOERRORDLG: u32 = 16384;
pub const MCIWNDF_NOMENU: u32 = 8;
pub const MCIWNDF_NOOPEN: u32 = 32768;
pub const MCIWNDF_NOPLAYBAR: u32 = 2;
pub const MCIWNDF_NOTIFYALL: u32 = 7936;
pub const MCIWNDF_NOTIFYANSI: u32 = 128;
pub const MCIWNDF_NOTIFYERROR: u32 = 4096;
pub const MCIWNDF_NOTIFYMEDIA: u32 = 2176;
pub const MCIWNDF_NOTIFYMEDIAA: u32 = 2176;
pub const MCIWNDF_NOTIFYMEDIAW: u32 = 2048;
pub const MCIWNDF_NOTIFYMODE: u32 = 256;
pub const MCIWNDF_NOTIFYPOS: u32 = 512;
pub const MCIWNDF_NOTIFYSIZE: u32 = 1024;
pub const MCIWNDF_RECORD: u32 = 8192;
pub const MCIWNDF_SHOWALL: u32 = 112;
pub const MCIWNDF_SHOWMODE: u32 = 64;
pub const MCIWNDF_SHOWNAME: u32 = 16;
pub const MCIWNDF_SHOWPOS: u32 = 32;
pub const MCIWNDM_CAN_CONFIG: u32 = 1173;
pub const MCIWNDM_CAN_EJECT: u32 = 1172;
pub const MCIWNDM_CAN_PLAY: u32 = 1168;
pub const MCIWNDM_CAN_RECORD: u32 = 1170;
pub const MCIWNDM_CAN_SAVE: u32 = 1171;
pub const MCIWNDM_CAN_WINDOW: u32 = 1169;
pub const MCIWNDM_CHANGESTYLES: u32 = 1159;
pub const MCIWNDM_EJECT: u32 = 1131;
pub const MCIWNDM_GETACTIVETIMER: u32 = 1156;
pub const MCIWNDM_GETALIAS: u32 = 1161;
pub const MCIWNDM_GETDEVICE: u32 = 1149;
pub const MCIWNDM_GETDEVICEA: u32 = 1149;
pub const MCIWNDM_GETDEVICEID: u32 = 1124;
pub const MCIWNDM_GETDEVICEW: u32 = 1249;
pub const MCIWNDM_GETEND: u32 = 1129;
pub const MCIWNDM_GETERROR: u32 = 1152;
pub const MCIWNDM_GETERRORA: u32 = 1152;
pub const MCIWNDM_GETERRORW: u32 = 1252;
pub const MCIWNDM_GETFILENAME: u32 = 1148;
pub const MCIWNDM_GETFILENAMEA: u32 = 1148;
pub const MCIWNDM_GETFILENAMEW: u32 = 1248;
pub const MCIWNDM_GETINACTIVETIMER: u32 = 1157;
pub const MCIWNDM_GETLENGTH: u32 = 1128;
pub const MCIWNDM_GETMODE: u32 = 1130;
pub const MCIWNDM_GETMODEA: u32 = 1130;
pub const MCIWNDM_GETMODEW: u32 = 1230;
pub const MCIWNDM_GETPALETTE: u32 = 1150;
pub const MCIWNDM_GETPOSITION: u32 = 1126;
pub const MCIWNDM_GETPOSITIONA: u32 = 1126;
pub const MCIWNDM_GETPOSITIONW: u32 = 1226;
pub const MCIWNDM_GETREPEAT: u32 = 1139;
pub const MCIWNDM_GETSPEED: u32 = 1137;
pub const MCIWNDM_GETSTART: u32 = 1127;
pub const MCIWNDM_GETSTYLES: u32 = 1160;
pub const MCIWNDM_GETTIMEFORMAT: u32 = 1144;
pub const MCIWNDM_GETTIMEFORMATA: u32 = 1144;
pub const MCIWNDM_GETTIMEFORMATW: u32 = 1244;
pub const MCIWNDM_GETVOLUME: u32 = 1135;
pub const MCIWNDM_GETZOOM: u32 = 1133;
pub const MCIWNDM_GET_DEST: u32 = 1166;
pub const MCIWNDM_GET_SOURCE: u32 = 1164;
pub const MCIWNDM_NEW: u32 = 1158;
pub const MCIWNDM_NEWA: u32 = 1158;
pub const MCIWNDM_NEWW: u32 = 1258;
pub const MCIWNDM_NOTIFYERROR: u32 = 1229;
pub const MCIWNDM_NOTIFYMEDIA: u32 = 1227;
pub const MCIWNDM_NOTIFYMODE: u32 = 1224;
pub const MCIWNDM_NOTIFYPOS: u32 = 1225;
pub const MCIWNDM_NOTIFYSIZE: u32 = 1226;
pub const MCIWNDM_OPEN: u32 = 1177;
pub const MCIWNDM_OPENA: u32 = 1177;
pub const MCIWNDM_OPENINTERFACE: u32 = 1175;
pub const MCIWNDM_OPENW: u32 = 1276;
pub const MCIWNDM_PALETTEKICK: u32 = 1174;
pub const MCIWNDM_PLAYFROM: u32 = 1146;
pub const MCIWNDM_PLAYREVERSE: u32 = 1163;
pub const MCIWNDM_PLAYTO: u32 = 1147;
pub const MCIWNDM_PUT_DEST: u32 = 1167;
pub const MCIWNDM_PUT_SOURCE: u32 = 1165;
pub const MCIWNDM_REALIZE: u32 = 1142;
pub const MCIWNDM_RETURNSTRING: u32 = 1162;
pub const MCIWNDM_RETURNSTRINGA: u32 = 1162;
pub const MCIWNDM_RETURNSTRINGW: u32 = 1262;
pub const MCIWNDM_SENDSTRING: u32 = 1125;
pub const MCIWNDM_SENDSTRINGA: u32 = 1125;
pub const MCIWNDM_SENDSTRINGW: u32 = 1225;
pub const MCIWNDM_SETACTIVETIMER: u32 = 1154;
pub const MCIWNDM_SETINACTIVETIMER: u32 = 1155;
pub const MCIWNDM_SETOWNER: u32 = 1176;
pub const MCIWNDM_SETPALETTE: u32 = 1151;
pub const MCIWNDM_SETREPEAT: u32 = 1138;
pub const MCIWNDM_SETSPEED: u32 = 1136;
pub const MCIWNDM_SETTIMEFORMAT: u32 = 1143;
pub const MCIWNDM_SETTIMEFORMATA: u32 = 1143;
pub const MCIWNDM_SETTIMEFORMATW: u32 = 1243;
pub const MCIWNDM_SETTIMERS: u32 = 1153;
pub const MCIWNDM_SETVOLUME: u32 = 1134;
pub const MCIWNDM_SETZOOM: u32 = 1132;
pub const MCIWNDM_VALIDATEMEDIA: u32 = 1145;
pub const MCIWNDOPENF_NEW: u32 = 1;
pub const MCIWND_END: i32 = -2;
pub const MCIWND_START: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MainAVIHeader {
    pub dwMicroSecPerFrame: u32,
    pub dwMaxBytesPerSec: u32,
    pub dwPaddingGranularity: u32,
    pub dwFlags: u32,
    pub dwTotalFrames: u32,
    pub dwInitialFrames: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwReserved: [u32; 4],
}
impl Default for MainAVIHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type PCAPDRIVERCAPS = *mut CAPDRIVERCAPS;
#[cfg(feature = "mmiscapi")]
pub type PCAPINFOCHUNK = *mut CAPINFOCHUNK;
#[cfg(feature = "windef")]
pub type PCAPSTATUS = *mut CAPSTATUS;
pub type PCAPTUREPARMS = *mut CAPTUREPARMS;
pub type PCHANNEL_CAPS = *mut CHANNEL_CAPS;
#[cfg(feature = "wingdi")]
pub type PCOMPVARS = *mut COMPVARS;
pub const PD_CAN_DRAW_DIB: u32 = 1;
pub const PD_CAN_STRETCHDIB: u32 = 2;
pub const PD_STRETCHDIB_1_1_OK: u32 = 4;
pub const PD_STRETCHDIB_1_2_OK: u32 = 8;
pub const PD_STRETCHDIB_1_N_OK: u32 = 16;
#[cfg(feature = "minwindef")]
pub type PVIDEOHDR = *mut VIDEOHDR;
pub const SEARCH_ANY: u32 = 32;
pub const SEARCH_BACKWARD: u32 = 4;
pub const SEARCH_FORWARD: u32 = 1;
pub const SEARCH_KEY: u32 = 16;
pub const SEARCH_NEAREST: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TWOCC(pub u16);
pub const VCAPS_CAN_SCALE: u32 = 8;
pub const VCAPS_DST_CAN_CLIP: u32 = 4;
pub const VCAPS_OVERLAY: u32 = 1;
pub const VCAPS_SRC_CAN_CLIP: u32 = 2;
pub const VHDR_DONE: u32 = 1;
pub const VHDR_INQUEUE: u32 = 4;
pub const VHDR_KEYFRAME: u32 = 8;
pub const VHDR_PREPARED: u32 = 2;
pub const VHDR_VALID: u32 = 15;
pub const VIDCF_COMPRESSFRAMES: u32 = 8;
pub const VIDCF_CRUNCH: u32 = 2;
pub const VIDCF_DRAW: u32 = 16;
pub const VIDCF_FASTTEMPORALC: u32 = 32;
pub const VIDCF_FASTTEMPORALD: u32 = 128;
pub const VIDCF_QUALITY: u32 = 1;
pub const VIDCF_TEMPORAL: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEOHDR {
    pub lpData: super::minwindef::LPBYTE,
    pub dwBufferLength: u32,
    pub dwBytesUsed: u32,
    pub dwTimeCaptured: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwReserved: [usize; 4],
}
#[cfg(feature = "minwindef")]
impl Default for VIDEOHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VIDEO_CONFIGURE_CURRENT: u32 = 16;
pub const VIDEO_CONFIGURE_GET: u32 = 8192;
pub const VIDEO_CONFIGURE_MAX: u32 = 128;
pub const VIDEO_CONFIGURE_MIN: u32 = 64;
pub const VIDEO_CONFIGURE_NOMINAL: u32 = 32;
pub const VIDEO_CONFIGURE_QUERY: u32 = 32768;
pub const VIDEO_CONFIGURE_QUERYSIZE: u32 = 1;
pub const VIDEO_CONFIGURE_SET: u32 = 4096;
pub const VIDEO_DLG_QUERY: u32 = 16;
pub const VIDEO_EXTERNALIN: u32 = 1;
pub const VIDEO_EXTERNALOUT: u32 = 2;
pub const VIDEO_IN: u32 = 4;
pub const VIDEO_OUT: u32 = 8;
pub const WM_CAP_ABORT: u32 = 1093;
pub const WM_CAP_DLG_VIDEOCOMPRESSION: u32 = 1070;
pub const WM_CAP_DLG_VIDEODISPLAY: u32 = 1067;
pub const WM_CAP_DLG_VIDEOFORMAT: u32 = 1065;
pub const WM_CAP_DLG_VIDEOSOURCE: u32 = 1066;
pub const WM_CAP_DRIVER_CONNECT: u32 = 1034;
pub const WM_CAP_DRIVER_DISCONNECT: u32 = 1035;
pub const WM_CAP_DRIVER_GET_CAPS: u32 = 1038;
pub const WM_CAP_DRIVER_GET_NAME: u32 = 1036;
pub const WM_CAP_DRIVER_GET_NAMEA: u32 = 1036;
pub const WM_CAP_DRIVER_GET_NAMEW: u32 = 1136;
pub const WM_CAP_DRIVER_GET_VERSION: u32 = 1037;
pub const WM_CAP_DRIVER_GET_VERSIONA: u32 = 1037;
pub const WM_CAP_DRIVER_GET_VERSIONW: u32 = 1137;
pub const WM_CAP_EDIT_COPY: u32 = 1054;
pub const WM_CAP_END: u32 = 1205;
pub const WM_CAP_FILE_ALLOCATE: u32 = 1046;
pub const WM_CAP_FILE_GET_CAPTURE_FILE: u32 = 1045;
pub const WM_CAP_FILE_GET_CAPTURE_FILEA: u32 = 1045;
pub const WM_CAP_FILE_GET_CAPTURE_FILEW: u32 = 1145;
pub const WM_CAP_FILE_SAVEAS: u32 = 1047;
pub const WM_CAP_FILE_SAVEASA: u32 = 1047;
pub const WM_CAP_FILE_SAVEASW: u32 = 1147;
pub const WM_CAP_FILE_SAVEDIB: u32 = 1049;
pub const WM_CAP_FILE_SAVEDIBA: u32 = 1049;
pub const WM_CAP_FILE_SAVEDIBW: u32 = 1149;
pub const WM_CAP_FILE_SET_CAPTURE_FILE: u32 = 1044;
pub const WM_CAP_FILE_SET_CAPTURE_FILEA: u32 = 1044;
pub const WM_CAP_FILE_SET_CAPTURE_FILEW: u32 = 1144;
pub const WM_CAP_FILE_SET_INFOCHUNK: u32 = 1048;
pub const WM_CAP_GET_AUDIOFORMAT: u32 = 1060;
pub const WM_CAP_GET_CAPSTREAMPTR: u32 = 1025;
pub const WM_CAP_GET_MCI_DEVICE: u32 = 1091;
pub const WM_CAP_GET_MCI_DEVICEA: u32 = 1091;
pub const WM_CAP_GET_MCI_DEVICEW: u32 = 1191;
pub const WM_CAP_GET_SEQUENCE_SETUP: u32 = 1089;
pub const WM_CAP_GET_STATUS: u32 = 1078;
pub const WM_CAP_GET_USER_DATA: u32 = 1032;
pub const WM_CAP_GET_VIDEOFORMAT: u32 = 1068;
pub const WM_CAP_GRAB_FRAME: u32 = 1084;
pub const WM_CAP_GRAB_FRAME_NOSTOP: u32 = 1085;
pub const WM_CAP_PAL_AUTOCREATE: u32 = 1107;
pub const WM_CAP_PAL_MANUALCREATE: u32 = 1108;
pub const WM_CAP_PAL_OPEN: u32 = 1104;
pub const WM_CAP_PAL_OPENA: u32 = 1104;
pub const WM_CAP_PAL_OPENW: u32 = 1204;
pub const WM_CAP_PAL_PASTE: u32 = 1106;
pub const WM_CAP_PAL_SAVE: u32 = 1105;
pub const WM_CAP_PAL_SAVEA: u32 = 1105;
pub const WM_CAP_PAL_SAVEW: u32 = 1205;
pub const WM_CAP_SEQUENCE: u32 = 1086;
pub const WM_CAP_SEQUENCE_NOFILE: u32 = 1087;
pub const WM_CAP_SET_AUDIOFORMAT: u32 = 1059;
pub const WM_CAP_SET_CALLBACK_CAPCONTROL: u32 = 1109;
pub const WM_CAP_SET_CALLBACK_ERROR: u32 = 1026;
pub const WM_CAP_SET_CALLBACK_ERRORA: u32 = 1026;
pub const WM_CAP_SET_CALLBACK_ERRORW: u32 = 1126;
pub const WM_CAP_SET_CALLBACK_FRAME: u32 = 1029;
pub const WM_CAP_SET_CALLBACK_STATUS: u32 = 1027;
pub const WM_CAP_SET_CALLBACK_STATUSA: u32 = 1027;
pub const WM_CAP_SET_CALLBACK_STATUSW: u32 = 1127;
pub const WM_CAP_SET_CALLBACK_VIDEOSTREAM: u32 = 1030;
pub const WM_CAP_SET_CALLBACK_WAVESTREAM: u32 = 1031;
pub const WM_CAP_SET_CALLBACK_YIELD: u32 = 1028;
pub const WM_CAP_SET_MCI_DEVICE: u32 = 1090;
pub const WM_CAP_SET_MCI_DEVICEA: u32 = 1090;
pub const WM_CAP_SET_MCI_DEVICEW: u32 = 1190;
pub const WM_CAP_SET_OVERLAY: u32 = 1075;
pub const WM_CAP_SET_PREVIEW: u32 = 1074;
pub const WM_CAP_SET_PREVIEWRATE: u32 = 1076;
pub const WM_CAP_SET_SCALE: u32 = 1077;
pub const WM_CAP_SET_SCROLL: u32 = 1079;
pub const WM_CAP_SET_SEQUENCE_SETUP: u32 = 1088;
pub const WM_CAP_SET_USER_DATA: u32 = 1033;
pub const WM_CAP_SET_VIDEOFORMAT: u32 = 1069;
pub const WM_CAP_SINGLE_FRAME: u32 = 1096;
pub const WM_CAP_SINGLE_FRAME_CLOSE: u32 = 1095;
pub const WM_CAP_SINGLE_FRAME_OPEN: u32 = 1094;
pub const WM_CAP_START: u32 = 1024;
pub const WM_CAP_STOP: u32 = 1092;
pub const WM_CAP_UNICODE_END: u32 = 1205;
pub const WM_CAP_UNICODE_START: u32 = 1124;
pub const ckidAVIMAINHDR: u32 = 1751742049;
pub const ckidAVINEWINDEX: u32 = 829973609;
pub const ckidAVIPADDING: u32 = 1263424842;
pub const ckidSTREAMFORMAT: u32 = 1718776947;
pub const ckidSTREAMHANDLERDATA: u32 = 1685222515;
pub const ckidSTREAMHEADER: u32 = 1752331379;
pub const ckidSTREAMNAME: u32 = 1852994675;
pub const cktypeDIBbits: u32 = 25188;
pub const cktypeDIBcompressed: u32 = 25444;
pub const cktypePALchange: u32 = 25456;
pub const cktypeWAVEbytes: u32 = 25207;
pub const comptypeDIB: u32 = 541215044;
pub const formtypeAVI: u32 = 541677121;
pub const infotypeDIGITIZATION_TIME: u32 = 1414087753;
pub const infotypeSMPTE_TIME: u32 = 1347244873;
pub const listtypeAVIHEADER: u32 = 1819436136;
pub const listtypeAVIMOVIE: u32 = 1769369453;
pub const listtypeAVIRECORD: u32 = 543384946;
pub const listtypeSTREAMHEADER: u32 = 1819440243;
pub const streamtypeAUDIO: u32 = 1935963489;
pub const streamtypeMIDI: u32 = 1935960429;
pub const streamtypeTEXT: u32 = 1937012852;
pub const streamtypeVIDEO: u32 = 1935960438;
