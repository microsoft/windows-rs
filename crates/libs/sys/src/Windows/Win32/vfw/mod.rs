windows_link::link!("avifil32.dll" "system" fn AVIBuildFilterA(lpszfilter : windows_sys::core::PSTR, cbfilter : i32, fsaving : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIBuildFilterW(lpszfilter : windows_sys::core::PWSTR, cbfilter : i32, fsaving : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIClearClipboard() -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileAddRef(pfile : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn AVIFileCreateStreamA(pfile : *mut core::ffi::c_void, ppavi : *mut *mut core::ffi::c_void, psi : *const AVISTREAMINFOA) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn AVIFileCreateStreamW(pfile : *mut core::ffi::c_void, ppavi : *mut *mut core::ffi::c_void, psi : *const AVISTREAMINFOW) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileEndRecord(pfile : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileExit());
windows_link::link!("avifil32.dll" "system" fn AVIFileGetStream(pfile : *mut core::ffi::c_void, ppavi : *mut *mut core::ffi::c_void, fcctype : u32, lparam : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileInfoA(pfile : *mut core::ffi::c_void, pfi : *mut AVIFILEINFOA, lsize : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileInfoW(pfile : *mut core::ffi::c_void, pfi : *mut AVIFILEINFOW, lsize : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileInit());
windows_link::link!("avifil32.dll" "system" fn AVIFileOpenA(ppfile : *mut *mut core::ffi::c_void, szfile : windows_sys::core::PCSTR, umode : u32, lphandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileOpenW(ppfile : *mut *mut core::ffi::c_void, szfile : windows_sys::core::PCWSTR, umode : u32, lphandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileReadData(pfile : *mut core::ffi::c_void, ckid : u32, lpdata : *mut core::ffi::c_void, lpcbdata : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileRelease(pfile : *mut core::ffi::c_void) -> u32);
windows_link::link!("avifil32.dll" "system" fn AVIFileWriteData(pfile : *mut core::ffi::c_void, ckid : u32, lpdata : *const core::ffi::c_void, cbdata : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIGetFromClipboard(lppf : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIMakeCompressedStream(ppscompressed : *mut *mut core::ffi::c_void, ppssource : *mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIMakeFileFromStreams(ppfile : *mut *mut core::ffi::c_void, nstreams : i32, papstreams : *const *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("avifil32.dll" "system" fn AVIMakeStreamFromClipboard(cfformat : u32, hglobal : super::HANDLE, ppstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIPutFileOnClipboard(pf : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "C" fn AVISaveA(szfile : windows_sys::core::PCSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : *mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, ...) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn AVISaveOptions(hwnd : super::HWND, uiflags : u32, nstreams : i32, ppavi : *const *mut core::ffi::c_void, plpoptions : *mut LPAVICOMPRESSOPTIONS) -> isize);
windows_link::link!("avifil32.dll" "system" fn AVISaveOptionsFree(nstreams : i32, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVISaveVA(szfile : windows_sys::core::PCSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const *mut core::ffi::c_void, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVISaveVW(szfile : windows_sys::core::PCWSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const *mut core::ffi::c_void, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "C" fn AVISaveW(szfile : windows_sys::core::PCWSTR, pclsidhandler : *const windows_sys::core::GUID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : *mut core::ffi::c_void, lpoptions : *const AVICOMPRESSOPTIONS, ...) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamAddRef(pavi : *mut core::ffi::c_void) -> u32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamBeginStreaming(pavi : *mut core::ffi::c_void, lstart : i32, lend : i32, lrate : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamCreate(ppavi : *mut *mut core::ffi::c_void, lparam1 : i32, lparam2 : i32, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamEndStreaming(pavi : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamFindSample(pavi : *mut core::ffi::c_void, lpos : i32, lflags : i32) -> i32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamGetFrame(pg : *mut core::ffi::c_void, lpos : i32) -> *mut core::ffi::c_void);
windows_link::link!("avifil32.dll" "system" fn AVIStreamGetFrameClose(pg : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "wingdi")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamGetFrameOpen(pavi : *mut core::ffi::c_void, lpbiwanted : *const super::BITMAPINFOHEADER) -> *mut core::ffi::c_void);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamInfoA(pavi : *mut core::ffi::c_void, psi : *mut AVISTREAMINFOA, lsize : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamInfoW(pavi : *mut core::ffi::c_void, psi : *mut AVISTREAMINFOW, lsize : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamLength(pavi : *mut core::ffi::c_void) -> i32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileA(ppavi : *mut *mut core::ffi::c_void, szfile : windows_sys::core::PCSTR, fcctype : u32, lparam : i32, mode : u32, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileW(ppavi : *mut *mut core::ffi::c_void, szfile : windows_sys::core::PCWSTR, fcctype : u32, lparam : i32, mode : u32, pclsidhandler : *const windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamRead(pavi : *mut core::ffi::c_void, lstart : i32, lsamples : i32, lpbuffer : *mut core::ffi::c_void, cbbuffer : i32, plbytes : *mut i32, plsamples : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamReadData(pavi : *mut core::ffi::c_void, fcc : u32, lp : *mut core::ffi::c_void, lpcb : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamReadFormat(pavi : *mut core::ffi::c_void, lpos : i32, lpformat : *mut core::ffi::c_void, lpcbformat : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamRelease(pavi : *mut core::ffi::c_void) -> u32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamSampleToTime(pavi : *mut core::ffi::c_void, lsample : i32) -> i32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamSetFormat(pavi : *mut core::ffi::c_void, lpos : i32, lpformat : *const core::ffi::c_void, cbformat : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamStart(pavi : *mut core::ffi::c_void) -> i32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamTimeToSample(pavi : *mut core::ffi::c_void, ltime : i32) -> i32);
windows_link::link!("avifil32.dll" "system" fn AVIStreamWrite(pavi : *mut core::ffi::c_void, lstart : i32, lsamples : i32, lpbuffer : *const core::ffi::c_void, cbbuffer : i32, dwflags : u32, plsampwritten : *mut i32, plbyteswritten : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamWriteData(pavi : *mut core::ffi::c_void, fcc : u32, lp : *const core::ffi::c_void, cb : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn CreateEditableStream(ppseditable : *mut *mut core::ffi::c_void, pssource : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibBegin(hdd : HDRAWDIB, hdc : super::HDC, dxdst : i32, dydst : i32, lpbi : *const super::BITMAPINFOHEADER, dxsrc : i32, dysrc : i32, wflags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibChangePalette(hdd : HDRAWDIB, istart : i32, ilen : i32, lppe : *const super::PALETTEENTRY) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibClose(hdd : HDRAWDIB) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibDraw(hdd : HDRAWDIB, hdc : super::HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : *const super::BITMAPINFOHEADER, lpbits : *const core::ffi::c_void, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, wflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibEnd(hdd : HDRAWDIB) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibGetBuffer(hdd : HDRAWDIB, lpbi : *mut super::BITMAPINFOHEADER, dwsize : u32, dwflags : u32) -> *mut core::ffi::c_void);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibGetPalette(hdd : HDRAWDIB) -> super::HPALETTE);
#[cfg(feature = "winnt")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibOpen() -> HDRAWDIB);
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibProfileDisplay(lpbi : *const super::BITMAPINFOHEADER) -> super::LRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibRealize(hdd : HDRAWDIB, hdc : super::HDC, fbackground : windows_sys::core::BOOL) -> u32);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibSetPalette(hdd : HDRAWDIB, hpal : super::HPALETTE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibStart(hdd : HDRAWDIB, rate : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibStop(hdd : HDRAWDIB) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibTime(hdd : HDRAWDIB, lpddtime : *mut DRAWDIBTIME) -> windows_sys::core::BOOL);
windows_link::link!("avifil32.dll" "system" fn EditStreamClone(pavi : *mut core::ffi::c_void, ppresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamCopy(pavi : *mut core::ffi::c_void, plstart : *mut i32, pllength : *mut i32, ppresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamCut(pavi : *mut core::ffi::c_void, plstart : *mut i32, pllength : *mut i32, ppresult : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamPaste(pavi : *mut core::ffi::c_void, plpos : *mut i32, pllength : *mut i32, pstream : *mut core::ffi::c_void, lstart : i32, lend : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn EditStreamSetInfoA(pavi : *mut core::ffi::c_void, lpinfo : *const AVISTREAMINFOA, cbinfo : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("avifil32.dll" "system" fn EditStreamSetInfoW(pavi : *mut core::ffi::c_void, lpinfo : *const AVISTREAMINFOW, cbinfo : i32) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamSetNameA(pavi : *mut core::ffi::c_void, lpszname : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamSetNameW(pavi : *mut core::ffi::c_void, lpszname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewA(lpofn : *mut super::OPENFILENAMEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewW(lpofn : *mut super::OPENFILENAMEW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewA(lpofn : *mut super::OPENFILENAMEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewW(lpofn : *mut super::OPENFILENAMEW) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("msvfw32.dll" "system" fn ICClose(hic : HIC) -> super::LRESULT);
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "C" fn ICCompress(hic : HIC, dwflags : u32, lpbioutput : *const super::BITMAPINFOHEADER, lpdata : *mut core::ffi::c_void, lpbiinput : *const super::BITMAPINFOHEADER, lpbits : *const core::ffi::c_void, lpckid : *mut u32, lpdwflags : *mut u32, lframenum : i32, dwframesize : u32, dwquality : u32, lpbiprev : *const super::BITMAPINFOHEADER, lpprev : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICCompressorChoose(hwnd : super::HWND, uiflags : u32, pvin : *const core::ffi::c_void, lpdata : *const core::ffi::c_void, pc : *mut COMPVARS, lpsztitle : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "system" fn ICCompressorFree(pc : *const COMPVARS));
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "C" fn ICDecompress(hic : HIC, dwflags : u32, lpbiformat : *const super::BITMAPINFOHEADER, lpdata : *const core::ffi::c_void, lpbi : *const super::BITMAPINFOHEADER, lpbits : *mut core::ffi::c_void) -> u32);
windows_link::link!("msvfw32.dll" "C" fn ICDraw(hic : HIC, dwflags : u32, lpformat : *const core::ffi::c_void, lpdata : *const core::ffi::c_void, cbdata : u32, ltime : i32) -> u32);
#[cfg(all(feature = "windef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "C" fn ICDrawBegin(hic : HIC, dwflags : u32, hpal : super::HPALETTE, hwnd : super::HWND, hdc : super::HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : *const super::BITMAPINFOHEADER, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, dwrate : u32, dwscale : u32) -> u32);
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "system" fn ICGetDisplayFormat(hic : HIC, lpbiin : *const super::BITMAPINFOHEADER, lpbiout : *mut super::BITMAPINFOHEADER, bitdepth : i32, dx : i32, dy : i32) -> HIC);
#[cfg(feature = "minwindef")]
windows_link::link!("msvfw32.dll" "system" fn ICGetInfo(hic : HIC, picinfo : *mut ICINFO, cb : u32) -> super::LRESULT);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn ICImageCompress(hic : HIC, uiflags : u32, lpbiin : *const super::BITMAPINFO, lpbits : *const core::ffi::c_void, lpbiout : *const super::BITMAPINFO, lquality : i32, plsize : *mut i32) -> super::HANDLE);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn ICImageDecompress(hic : HIC, uiflags : u32, lpbiin : *const super::BITMAPINFO, lpbits : *const core::ffi::c_void, lpbiout : *const super::BITMAPINFO) -> super::HANDLE);
windows_link::link!("msvfw32.dll" "system" fn ICInfo(fcctype : u32, fcchandler : u32, lpicinfo : *mut ICINFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("msvfw32.dll" "system" fn ICInstall(fcctype : u32, fcchandler : u32, lparam : super::LPARAM, szdesc : windows_sys::core::PCSTR, wflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "system" fn ICLocate(fcctype : u32, fcchandler : u32, lpbiin : *const super::BITMAPINFOHEADER, lpbiout : *const super::BITMAPINFOHEADER, wflags : u16) -> HIC);
windows_link::link!("msvfw32.dll" "system" fn ICOpen(fcctype : u32, fcchandler : u32, wmode : u32) -> HIC);
#[cfg(feature = "minwindef")]
windows_link::link!("msvfw32.dll" "system" fn ICOpenFunction(fcctype : u32, fcchandler : u32, wmode : u32, lpfnhandler : super::FARPROC) -> HIC);
windows_link::link!("msvfw32.dll" "system" fn ICRemove(fcctype : u32, fcchandler : u32, wflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("msvfw32.dll" "system" fn ICSendMessage(hic : HIC, msg : u32, dw1 : usize, dw2 : usize) -> super::LRESULT);
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "system" fn ICSeqCompressFrame(pc : *const COMPVARS, uiflags : u32, lpbits : *const core::ffi::c_void, pfkey : *mut windows_sys::core::BOOL, plsize : *mut i32) -> *mut core::ffi::c_void);
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "system" fn ICSeqCompressFrameEnd(pc : *const COMPVARS));
#[cfg(feature = "wingdi")]
windows_link::link!("msvfw32.dll" "system" fn ICSeqCompressFrameStart(pc : *const COMPVARS, lpbiin : *const super::BITMAPINFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("msvfw32.dll" "C" fn MCIWndCreateA(hwndparent : super::HWND, hinstance : super::HINSTANCE, dwstyle : u32, szfile : windows_sys::core::PCSTR) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("msvfw32.dll" "C" fn MCIWndCreateW(hwndparent : super::HWND, hinstance : super::HINSTANCE, dwstyle : u32, szfile : windows_sys::core::PCWSTR) -> super::HWND);
windows_link::link!("msvfw32.dll" "C" fn MCIWndRegisterClass() -> windows_sys::core::BOOL);
windows_link::link!("msvfw32.dll" "system" fn VideoForWindowsVersion() -> u32);
#[cfg(feature = "windef")]
windows_link::link!("avicap32.dll" "system" fn capCreateCaptureWindowA(lpszwindowname : windows_sys::core::PCSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, nid : i32) -> super::HWND);
#[cfg(feature = "windef")]
windows_link::link!("avicap32.dll" "system" fn capCreateCaptureWindowW(lpszwindowname : windows_sys::core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, nid : i32) -> super::HWND);
windows_link::link!("avicap32.dll" "system" fn capGetDriverDescriptionA(wdriverindex : u32, lpszname : windows_sys::core::PSTR, cbname : i32, lpszver : windows_sys::core::PSTR, cbver : i32) -> windows_sys::core::BOOL);
windows_link::link!("avicap32.dll" "system" fn capGetDriverDescriptionW(wdriverindex : u32, lpszname : windows_sys::core::PWSTR, cbname : i32, lpszver : windows_sys::core::PWSTR, cbver : i32) -> windows_sys::core::BOOL);
pub const AVICOMPRESSF_DATARATE: i32 = 2;
pub const AVICOMPRESSF_INTERLEAVE: i32 = 1;
pub const AVICOMPRESSF_KEYFRAMES: i32 = 4;
pub const AVICOMPRESSF_VALID: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const AVIERR_OK: i32 = 0;
pub const AVIERR_READONLY: i32 = -2147205006;
pub const AVIERR_UNSUPPORTED: i32 = -2147205019;
pub const AVIERR_USERABORT: i32 = -2147204922;
pub const AVIFILECAPS_ALLKEYFRAMES: i32 = 16;
pub const AVIFILECAPS_CANREAD: i32 = 1;
pub const AVIFILECAPS_CANWRITE: i32 = 2;
pub const AVIFILECAPS_NOCOMPRESSION: i32 = 32;
pub const AVIFILEHANDLER_CANACCEPTNONRGB: i32 = 4;
pub const AVIFILEHANDLER_CANREAD: i32 = 1;
pub const AVIFILEHANDLER_CANWRITE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
pub const AVIFILEINFO_COPYRIGHTED: i32 = 131072;
pub const AVIFILEINFO_HASINDEX: i32 = 16;
pub const AVIFILEINFO_ISINTERLEAVED: i32 = 256;
pub const AVIFILEINFO_MUSTUSEINDEX: i32 = 32;
pub const AVIFILEINFO_WASCAPTUREFILE: i32 = 65536;
pub const AVIF_COPYRIGHTED: i32 = 131072;
pub const AVIF_HASINDEX: i32 = 16;
pub const AVIF_ISINTERLEAVED: i32 = 256;
pub const AVIF_MUSTUSEINDEX: i32 = 32;
pub const AVIF_WASCAPTUREFILE: i32 = 65536;
pub const AVIGETFRAMEF_BESTDISPLAYFMT: i32 = 1;
pub const AVIIF_COMPUSE: i32 = 268369920;
pub const AVIIF_FIRSTPART: i32 = 32;
pub const AVIIF_KEYFRAME: i32 = 16;
pub const AVIIF_LASTPART: i32 = 64;
pub const AVIIF_LIST: i32 = 1;
pub const AVIIF_MIDPART: i32 = 96;
pub const AVIIF_NOTIME: i32 = 256;
pub const AVIIF_TWOCC: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AVIINDEXENTRY {
    pub ckid: u32,
    pub dwFlags: u32,
    pub dwChunkOffset: u32,
    pub dwChunkLength: u32,
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy)]
pub struct AVIPALCHANGE {
    pub bFirstEntry: u8,
    pub bNumEntries: u8,
    pub wFlags: u16,
    pub peNew: [super::PALETTEENTRY; 0],
}
#[cfg(feature = "wingdi")]
impl Default for AVIPALCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AVISAVECALLBACK = Option<unsafe extern "system" fn(param0: i32) -> windows_sys::core::BOOL>;
pub const AVISF_DISABLED: i32 = 1;
pub const AVISF_VIDEO_PALCHANGES: i32 = 65536;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
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
    pub rcFrame: super::RECT,
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
#[derive(Clone, Copy)]
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
    pub rcFrame: super::RECT,
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
pub const AVISTREAMINFO_DISABLED: i32 = 1;
pub const AVISTREAMINFO_FORMATCHANGES: i32 = 65536;
pub const AVISTREAMREAD_CONVENIENT: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct AVIStreamHeader {
    pub fccType: super::FOURCC,
    pub fccHandler: super::FOURCC,
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
    pub rcFrame: super::RECT,
}
pub const AVI_HEADERSIZE: i32 = 2048;
pub const AVSTREAMMASTER_AUDIO: i32 = 0;
pub const AVSTREAMMASTER_NONE: i32 = 1;
pub const BI_1632: i32 = 842217009;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPCONTROLCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, nstate: i32) -> super::LRESULT>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: u32,
    pub fHasOverlay: windows_sys::core::BOOL,
    pub fHasDlgVideoSource: windows_sys::core::BOOL,
    pub fHasDlgVideoFormat: windows_sys::core::BOOL,
    pub fHasDlgVideoDisplay: windows_sys::core::BOOL,
    pub fCaptureInitialized: windows_sys::core::BOOL,
    pub fDriverSuppliesPalettes: windows_sys::core::BOOL,
    pub hVideoIn: super::HANDLE,
    pub hVideoOut: super::HANDLE,
    pub hVideoExtIn: super::HANDLE,
    pub hVideoExtOut: super::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for CAPDRIVERCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPERRORCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCSTR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPERRORCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCWSTR) -> super::LRESULT>;
#[repr(C)]
#[cfg(feature = "mmiscapi")]
#[derive(Clone, Copy)]
pub struct CAPINFOCHUNK {
    pub fccInfoID: super::FOURCC,
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
#[derive(Clone, Copy)]
pub struct CAPSTATUS {
    pub uiImageWidth: u32,
    pub uiImageHeight: u32,
    pub fLiveWindow: windows_sys::core::BOOL,
    pub fOverlayWindow: windows_sys::core::BOOL,
    pub fScale: windows_sys::core::BOOL,
    pub ptScroll: super::POINT,
    pub fUsingDefaultPalette: windows_sys::core::BOOL,
    pub fAudioHardware: windows_sys::core::BOOL,
    pub fCapFileExists: windows_sys::core::BOOL,
    pub dwCurrentVideoFrame: u32,
    pub dwCurrentVideoFramesDropped: u32,
    pub dwCurrentWaveSamples: u32,
    pub dwCurrentTimeElapsedMS: u32,
    pub hPalCurrent: super::HPALETTE,
    pub fCapturingNow: windows_sys::core::BOOL,
    pub dwReturn: u32,
    pub wNumVideoAllocated: u32,
    pub wNumAudioAllocated: u32,
}
#[cfg(feature = "windef")]
impl Default for CAPSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPSTATUSCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCSTR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPSTATUSCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCWSTR) -> super::LRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: u32,
    pub fMakeUserHitOKToCapture: windows_sys::core::BOOL,
    pub wPercentDropForError: u32,
    pub fYield: windows_sys::core::BOOL,
    pub dwIndexSize: u32,
    pub wChunkGranularity: u32,
    pub fUsingDOSMemory: windows_sys::core::BOOL,
    pub wNumVideoRequested: u32,
    pub fCaptureAudio: windows_sys::core::BOOL,
    pub wNumAudioRequested: u32,
    pub vKeyAbort: u32,
    pub fAbortLeftMouse: windows_sys::core::BOOL,
    pub fAbortRightMouse: windows_sys::core::BOOL,
    pub fLimitEnabled: windows_sys::core::BOOL,
    pub wTimeLimit: u32,
    pub fMCIControl: windows_sys::core::BOOL,
    pub fStepMCIDevice: windows_sys::core::BOOL,
    pub dwMCIStartTime: u32,
    pub dwMCIStopTime: u32,
    pub fStepCaptureAt2x: windows_sys::core::BOOL,
    pub wStepCaptureAverageFrames: u32,
    pub dwAudioBufferSize: u32,
    pub fDisableWriteCache: windows_sys::core::BOOL,
    pub AVStreamMaster: u32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPVIDEOCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, lpvhdr: *const VIDEOHDR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
pub type CAPWAVECALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, lpwhdr: *const super::WAVEHDR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPYIELDCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND) -> super::LRESULT>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy)]
pub struct COMPVARS {
    pub cbSize: i32,
    pub dwFlags: u32,
    pub hic: HIC,
    pub fccType: u32,
    pub fccHandler: u32,
    pub lpbiIn: super::LPBITMAPINFO,
    pub lpbiOut: super::LPBITMAPINFO,
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
pub const CONTROLCALLBACK_CAPTURING: i32 = 2;
pub const CONTROLCALLBACK_PREROLL: i32 = 1;
pub const DDF_0001: i32 = 1;
pub const DDF_2000: i32 = 8192;
pub const DDF_ANIMATE: i32 = 32;
pub const DDF_BACKGROUNDPAL: i32 = 512;
pub const DDF_BUFFER: i32 = 64;
pub const DDF_DONTDRAW: i32 = 16;
pub const DDF_FULLSCREEN: i32 = 256;
pub const DDF_HALFTONE: i32 = 4096;
pub const DDF_HURRYUP: i32 = 2048;
pub const DDF_JUSTDRAWIT: i32 = 128;
pub const DDF_NOTKEYFRAME: i32 = 1024;
pub const DDF_PREROLL: i32 = 16;
pub const DDF_SAME_DIB: i32 = 8;
pub const DDF_SAME_DRAW: i32 = 8;
pub const DDF_SAME_HDC: i32 = 4;
pub const DDF_SAME_SIZE: i32 = 8;
pub const DDF_UPDATE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRAWDIBTIME {
    pub timeCount: i32,
    pub timeDraw: i32,
    pub timeDecompress: i32,
    pub timeDither: i32,
    pub timeStretch: i32,
    pub timeBlt: i32,
    pub timeSetDIBits: i32,
}
pub const DVM_CONFIGURE_END: i32 = 8191;
pub const DVM_CONFIGURE_START: i32 = 4096;
pub const DVM_DST_RECT: i32 = 4101;
pub const DVM_FORMAT: i32 = 4098;
pub const DVM_PALETTE: i32 = 4097;
pub const DVM_PALETTERGB555: i32 = 4099;
pub const DVM_SRC_RECT: i32 = 4100;
pub const DVM_USER: i32 = 16384;
pub const DV_ERR_13: i32 = 16;
pub const DV_ERR_ALLOCATED: i32 = 19;
pub const DV_ERR_BADDEVICEID: i32 = 20;
pub const DV_ERR_BADERRNUM: i32 = 22;
pub const DV_ERR_BADFORMAT: i32 = 2;
pub const DV_ERR_BADINSTALL: i32 = 8;
pub const DV_ERR_BASE: i32 = 1;
pub const DV_ERR_CONFIG1: i32 = 13;
pub const DV_ERR_CONFIG2: i32 = 14;
pub const DV_ERR_CREATEPALETTE: i32 = 9;
pub const DV_ERR_DMA_CONFLICT: i32 = 26;
pub const DV_ERR_FLAGS: i32 = 15;
pub const DV_ERR_INT_CONFLICT: i32 = 27;
pub const DV_ERR_INVALHANDLE: i32 = 21;
pub const DV_ERR_IO_CONFLICT: i32 = 25;
pub const DV_ERR_LASTERROR: i32 = 28;
pub const DV_ERR_MEM_CONFLICT: i32 = 24;
pub const DV_ERR_NOMEM: i32 = 18;
pub const DV_ERR_NONSPECIFIC: i32 = 1;
pub const DV_ERR_NOTDETECTED: i32 = 7;
pub const DV_ERR_NOTSUPPORTED: i32 = 17;
pub const DV_ERR_NO_BUFFERS: i32 = 23;
pub const DV_ERR_OK: i32 = 0;
pub const DV_ERR_PARAM1: i32 = 11;
pub const DV_ERR_PARAM2: i32 = 12;
pub const DV_ERR_PROTECT_ONLY: i32 = 28;
pub const DV_ERR_SIZEFIELD: i32 = 10;
pub const DV_ERR_STILLPLAYING: i32 = 3;
pub const DV_ERR_SYNC: i32 = 5;
pub const DV_ERR_TOOMANYCHANNELS: i32 = 6;
pub const DV_ERR_UNPREPARED: i32 = 4;
pub const DV_ERR_USER_MSG: i32 = 1001;
pub const DV_VM_CLOSE: i32 = 977;
pub const DV_VM_DATA: i32 = 978;
pub const DV_VM_ERROR: i32 = 979;
pub const DV_VM_OPEN: i32 = 976;
pub const FIND_ANY: i32 = 32;
pub const FIND_DIR: i32 = 15;
pub const FIND_FORMAT: i32 = 64;
pub const FIND_FROM_START: i32 = 8;
pub const FIND_INDEX: i32 = 16384;
pub const FIND_KEY: i32 = 16;
pub const FIND_LENGTH: i32 = 4096;
pub const FIND_NEXT: i32 = 1;
pub const FIND_OFFSET: i32 = 8192;
pub const FIND_POS: i32 = 0;
pub const FIND_PREV: i32 = 4;
pub const FIND_RET: i32 = 61440;
pub const FIND_SIZE: i32 = 12288;
pub const FIND_TYPE: i32 = 240;
#[cfg(feature = "winnt")]
pub type HDRAWDIB = super::HANDLE;
pub type HIC = *mut core::ffi::c_void;
pub type HVIDEO = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct ICCOMPRESS {
    pub dwFlags: u32,
    pub lpbiOutput: super::LPBITMAPINFOHEADER,
    pub lpOutput: *mut core::ffi::c_void,
    pub lpbiInput: super::LPBITMAPINFOHEADER,
    pub lpInput: *mut core::ffi::c_void,
    pub lpckid: super::LPDWORD,
    pub lpdwFlags: super::LPDWORD,
    pub lFrameNum: i32,
    pub dwFrameSize: u32,
    pub dwQuality: u32,
    pub lpbiPrev: super::LPBITMAPINFOHEADER,
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
#[derive(Clone, Copy)]
pub struct ICCOMPRESSFRAMES {
    pub dwFlags: u32,
    pub lpbiOutput: super::LPBITMAPINFOHEADER,
    pub lOutput: super::LPARAM,
    pub lpbiInput: super::LPBITMAPINFOHEADER,
    pub lInput: super::LPARAM,
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
pub const ICCOMPRESSFRAMES_PADDING: i32 = 1;
pub const ICCOMPRESS_KEYFRAME: i32 = 1;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy)]
pub struct ICDECOMPRESS {
    pub dwFlags: u32,
    pub lpbiInput: super::LPBITMAPINFOHEADER,
    pub lpInput: *mut core::ffi::c_void,
    pub lpbiOutput: super::LPBITMAPINFOHEADER,
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
#[derive(Clone, Copy)]
pub struct ICDECOMPRESSEX {
    pub dwFlags: u32,
    pub lpbiSrc: super::LPBITMAPINFOHEADER,
    pub lpSrc: *mut core::ffi::c_void,
    pub lpbiDst: super::LPBITMAPINFOHEADER,
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
pub const ICDECOMPRESS_NOTKEYFRAME: i32 = 134217728;
pub const ICDECOMPRESS_NULLFRAME: i32 = 268435456;
pub const ICDECOMPRESS_PREROLL: i32 = 536870912;
pub const ICDECOMPRESS_UPDATE: i32 = 1073741824;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct ICDRAWBEGIN {
    pub dwFlags: u32,
    pub hpal: super::HPALETTE,
    pub hwnd: super::HWND,
    pub hdc: super::HDC,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub lpbi: super::LPBITMAPINFOHEADER,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dwRate: u32,
    pub dwScale: u32,
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl Default for ICDRAWBEGIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy)]
pub struct ICDRAWSUGGEST {
    pub lpbiIn: super::LPBITMAPINFOHEADER,
    pub lpbiSuggest: super::LPBITMAPINFOHEADER,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
}
#[cfg(feature = "wingdi")]
impl Default for ICDRAWSUGGEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICDRAW_ANIMATE: i32 = 8;
pub const ICDRAW_BUFFER: i32 = 256;
pub const ICDRAW_CONTINUE: i32 = 16;
pub const ICDRAW_FULLSCREEN: i32 = 2;
pub const ICDRAW_HDC: i32 = 4;
pub const ICDRAW_HURRYUP: u32 = 2147483648;
pub const ICDRAW_MEMORYDC: i32 = 32;
pub const ICDRAW_NOTKEYFRAME: i32 = 134217728;
pub const ICDRAW_NULLFRAME: i32 = 268435456;
pub const ICDRAW_PREROLL: i32 = 536870912;
pub const ICDRAW_QUERY: i32 = 1;
pub const ICDRAW_RENDER: i32 = 128;
pub const ICDRAW_UPDATE: i32 = 1073741824;
pub const ICDRAW_UPDATING: i32 = 64;
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
pub const ICERR_DONTDRAW: i32 = 1;
pub const ICERR_ERROR: i32 = -100;
pub const ICERR_GOTOKEYFRAME: i32 = 3;
pub const ICERR_INTERNAL: i32 = -4;
pub const ICERR_MEMORY: i32 = -3;
pub const ICERR_NEWPALETTE: i32 = 2;
pub const ICERR_OK: i32 = 0;
pub const ICERR_STOPDRAWING: i32 = 4;
pub const ICERR_UNSUPPORTED: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const ICINSTALL_DRIVER: i32 = 2;
pub const ICINSTALL_DRIVERW: i32 = 32770;
pub const ICINSTALL_FUNCTION: i32 = 1;
pub const ICINSTALL_HDRV: i32 = 4;
pub const ICINSTALL_UNICODE: i32 = 32768;
pub const ICMF_ABOUT_QUERY: i32 = 1;
pub const ICMF_CHOOSE_ALLCOMPRESSORS: i32 = 8;
pub const ICMF_CHOOSE_DATARATE: i32 = 2;
pub const ICMF_CHOOSE_KEYFRAME: i32 = 1;
pub const ICMF_CHOOSE_PREVIEW: i32 = 4;
pub const ICMF_COMPVARS_VALID: i32 = 1;
pub const ICMF_CONFIGURE_QUERY: i32 = 1;
pub const ICMODE_COMPRESS: i32 = 1;
pub const ICMODE_DECOMPRESS: i32 = 2;
pub const ICMODE_DRAW: i32 = 8;
pub const ICMODE_FASTCOMPRESS: i32 = 5;
pub const ICMODE_FASTDECOMPRESS: i32 = 3;
pub const ICMODE_QUERY: i32 = 4;
pub const ICM_ABOUT: i32 = 20491;
pub const ICM_COMPRESS: i32 = 16392;
pub const ICM_COMPRESS_BEGIN: i32 = 16391;
pub const ICM_COMPRESS_END: i32 = 16393;
pub const ICM_COMPRESS_FRAMES: i32 = 16455;
pub const ICM_COMPRESS_FRAMES_INFO: i32 = 16454;
pub const ICM_COMPRESS_GET_FORMAT: i32 = 16388;
pub const ICM_COMPRESS_GET_SIZE: i32 = 16389;
pub const ICM_COMPRESS_QUERY: i32 = 16390;
pub const ICM_CONFIGURE: i32 = 20490;
pub const ICM_DECOMPRESS: i32 = 16397;
pub const ICM_DECOMPRESSEX: i32 = 16446;
pub const ICM_DECOMPRESSEX_BEGIN: i32 = 16444;
pub const ICM_DECOMPRESSEX_END: i32 = 16447;
pub const ICM_DECOMPRESSEX_QUERY: i32 = 16445;
pub const ICM_DECOMPRESS_BEGIN: i32 = 16396;
pub const ICM_DECOMPRESS_END: i32 = 16398;
pub const ICM_DECOMPRESS_GET_FORMAT: i32 = 16394;
pub const ICM_DECOMPRESS_GET_PALETTE: i32 = 16414;
pub const ICM_DECOMPRESS_QUERY: i32 = 16395;
pub const ICM_DECOMPRESS_SET_PALETTE: i32 = 16413;
pub const ICM_DRAW: i32 = 16417;
pub const ICM_DRAW_BEGIN: i32 = 16399;
pub const ICM_DRAW_BITS: i32 = 16404;
pub const ICM_DRAW_CHANGEPALETTE: i32 = 16435;
pub const ICM_DRAW_END: i32 = 16405;
pub const ICM_DRAW_FLUSH: i32 = 16421;
pub const ICM_DRAW_GETTIME: i32 = 16416;
pub const ICM_DRAW_GET_PALETTE: i32 = 16400;
pub const ICM_DRAW_IDLE: i32 = 16436;
pub const ICM_DRAW_QUERY: i32 = 16415;
pub const ICM_DRAW_REALIZE: i32 = 16420;
pub const ICM_DRAW_RENDERBUFFER: i32 = 16422;
pub const ICM_DRAW_SETTIME: i32 = 16419;
pub const ICM_DRAW_START: i32 = 16402;
pub const ICM_DRAW_START_PLAY: i32 = 16423;
pub const ICM_DRAW_STOP: i32 = 16403;
pub const ICM_DRAW_STOP_PLAY: i32 = 16424;
pub const ICM_DRAW_SUGGESTFORMAT: i32 = 16434;
pub const ICM_DRAW_UPDATE: i32 = 16401;
pub const ICM_DRAW_WINDOW: i32 = 16418;
pub const ICM_ENUMFORMATS: i32 = 20501;
pub const ICM_FRAMERATE: u32 = 1382904390;
pub const ICM_GET: i32 = 20521;
pub const ICM_GETBUFFERSWANTED: i32 = 16425;
pub const ICM_GETDEFAULTKEYFRAMERATE: i32 = 16426;
pub const ICM_GETDEFAULTQUALITY: i32 = 20510;
pub const ICM_GETERRORTEXT: i32 = 20492;
pub const ICM_GETFORMATNAME: i32 = 20500;
pub const ICM_GETINFO: i32 = 20482;
pub const ICM_GETQUALITY: i32 = 20511;
pub const ICM_GETSTATE: i32 = 20480;
pub const ICM_KEYFRAMERATE: u32 = 1383687499;
pub const ICM_RESERVED: i32 = 20480;
pub const ICM_RESERVED_HIGH: i32 = 24576;
pub const ICM_RESERVED_LOW: i32 = 20480;
pub const ICM_SET: i32 = 20520;
pub const ICM_SETQUALITY: i32 = 20512;
pub const ICM_SETSTATE: i32 = 20481;
pub const ICM_SET_STATUS_PROC: i32 = 16456;
pub const ICM_USER: i32 = 16384;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ICOPEN {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: super::LRESULT,
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
#[derive(Clone, Copy)]
pub struct ICPALETTE {
    pub dwFlags: u32,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: super::LPPALETTEENTRY,
}
#[cfg(feature = "wingdi")]
impl Default for ICPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICQUALITY_DEFAULT: i32 = -1;
pub const ICQUALITY_HIGH: i32 = 10000;
pub const ICQUALITY_LOW: i32 = 0;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ICSETSTATUSPROC {
    pub dwFlags: u32,
    pub lParam: super::LPARAM,
    pub Status: *mut u8,
}
#[cfg(feature = "minwindef")]
impl Default for ICSETSTATUSPROC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ICSTATUS_END: i32 = 2;
pub const ICSTATUS_ERROR: i32 = 3;
pub const ICSTATUS_START: i32 = 0;
pub const ICSTATUS_STATUS: i32 = 1;
pub const ICSTATUS_YIELD: i32 = 4;
pub const ICVERSION: i32 = 260;
pub const IDS_CAP_AUDIO_DROP_COMPERROR: i32 = 442;
pub const IDS_CAP_AUDIO_DROP_ERROR: i32 = 441;
pub const IDS_CAP_AVI_DRAWDIB_ERROR: i32 = 439;
pub const IDS_CAP_AVI_INIT_ERROR: i32 = 433;
pub const IDS_CAP_BEGIN: i32 = 300;
pub const IDS_CAP_CANTOPEN: i32 = 409;
pub const IDS_CAP_COMPRESSOR_ERROR: i32 = 440;
pub const IDS_CAP_DEFAVIEXT: i32 = 407;
pub const IDS_CAP_DEFPALEXT: i32 = 408;
pub const IDS_CAP_DRIVER_ERROR: i32 = 418;
pub const IDS_CAP_END: i32 = 301;
pub const IDS_CAP_ERRORDIBSAVE: i32 = 406;
pub const IDS_CAP_ERRORPALOPEN: i32 = 404;
pub const IDS_CAP_ERRORPALSAVE: i32 = 405;
pub const IDS_CAP_FILEEXISTS: i32 = 403;
pub const IDS_CAP_FILE_OPEN_ERROR: i32 = 429;
pub const IDS_CAP_FILE_WRITE_ERROR: i32 = 430;
pub const IDS_CAP_INFO: i32 = 401;
pub const IDS_CAP_MCI_CANT_STEP_ERROR: i32 = 437;
pub const IDS_CAP_MCI_CONTROL_ERROR: i32 = 436;
pub const IDS_CAP_NODISKSPACE: i32 = 415;
pub const IDS_CAP_NO_AUDIO_CAP_ERROR: i32 = 438;
pub const IDS_CAP_NO_FRAME_CAP_ERROR: i32 = 434;
pub const IDS_CAP_NO_PALETTE_WARN: i32 = 435;
pub const IDS_CAP_OUTOFMEM: i32 = 402;
pub const IDS_CAP_READONLYFILE: i32 = 413;
pub const IDS_CAP_RECORDING_ERROR: i32 = 431;
pub const IDS_CAP_RECORDING_ERROR2: i32 = 432;
pub const IDS_CAP_SAVEASPERCENT: i32 = 417;
pub const IDS_CAP_SEQ_MSGSTART: i32 = 410;
pub const IDS_CAP_SEQ_MSGSTOP: i32 = 411;
pub const IDS_CAP_SETFILESIZE: i32 = 416;
pub const IDS_CAP_STAT_CAP_AUDIO: i32 = 509;
pub const IDS_CAP_STAT_CAP_FINI: i32 = 503;
pub const IDS_CAP_STAT_CAP_INIT: i32 = 502;
pub const IDS_CAP_STAT_CAP_L_FRAMES: i32 = 508;
pub const IDS_CAP_STAT_FRAMESDROPPED: i32 = 513;
pub const IDS_CAP_STAT_I_FRAMES: i32 = 506;
pub const IDS_CAP_STAT_LIVE_MODE: i32 = 500;
pub const IDS_CAP_STAT_L_FRAMES: i32 = 507;
pub const IDS_CAP_STAT_OPTPAL_BUILD: i32 = 505;
pub const IDS_CAP_STAT_OVERLAY_MODE: i32 = 501;
pub const IDS_CAP_STAT_PALETTE_BUILD: i32 = 504;
pub const IDS_CAP_STAT_VIDEOAUDIO: i32 = 511;
pub const IDS_CAP_STAT_VIDEOCURRENT: i32 = 510;
pub const IDS_CAP_STAT_VIDEOONLY: i32 = 512;
pub const IDS_CAP_VIDEDITERR: i32 = 412;
pub const IDS_CAP_VIDEO_ADD_ERROR: i32 = 427;
pub const IDS_CAP_VIDEO_ALLOC_ERROR: i32 = 425;
pub const IDS_CAP_VIDEO_OPEN_ERROR: i32 = 424;
pub const IDS_CAP_VIDEO_PREPARE_ERROR: i32 = 426;
pub const IDS_CAP_VIDEO_SIZE_ERROR: i32 = 428;
pub const IDS_CAP_WAVE_ADD_ERROR: i32 = 422;
pub const IDS_CAP_WAVE_ALLOC_ERROR: i32 = 420;
pub const IDS_CAP_WAVE_OPEN_ERROR: i32 = 419;
pub const IDS_CAP_WAVE_PREPARE_ERROR: i32 = 421;
pub const IDS_CAP_WAVE_SIZE_ERROR: i32 = 423;
pub const IDS_CAP_WRITEERROR: i32 = 414;
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
pub const MCIWNDF_NOAUTOSIZEMOVIE: i32 = 4;
pub const MCIWNDF_NOAUTOSIZEWINDOW: i32 = 1;
pub const MCIWNDF_NOERRORDLG: i32 = 16384;
pub const MCIWNDF_NOMENU: i32 = 8;
pub const MCIWNDF_NOOPEN: i32 = 32768;
pub const MCIWNDF_NOPLAYBAR: i32 = 2;
pub const MCIWNDF_NOTIFYALL: i32 = 7936;
pub const MCIWNDF_NOTIFYANSI: i32 = 128;
pub const MCIWNDF_NOTIFYERROR: i32 = 4096;
pub const MCIWNDF_NOTIFYMEDIA: i32 = 2176;
pub const MCIWNDF_NOTIFYMEDIAA: i32 = 2176;
pub const MCIWNDF_NOTIFYMEDIAW: i32 = 2048;
pub const MCIWNDF_NOTIFYMODE: i32 = 256;
pub const MCIWNDF_NOTIFYPOS: i32 = 512;
pub const MCIWNDF_NOTIFYSIZE: i32 = 1024;
pub const MCIWNDF_RECORD: i32 = 8192;
pub const MCIWNDF_SHOWALL: i32 = 112;
pub const MCIWNDF_SHOWMODE: i32 = 64;
pub const MCIWNDF_SHOWNAME: i32 = 16;
pub const MCIWNDF_SHOWPOS: i32 = 32;
pub const MCIWNDM_CAN_CONFIG: i32 = 1173;
pub const MCIWNDM_CAN_EJECT: i32 = 1172;
pub const MCIWNDM_CAN_PLAY: i32 = 1168;
pub const MCIWNDM_CAN_RECORD: i32 = 1170;
pub const MCIWNDM_CAN_SAVE: i32 = 1171;
pub const MCIWNDM_CAN_WINDOW: i32 = 1169;
pub const MCIWNDM_CHANGESTYLES: i32 = 1159;
pub const MCIWNDM_EJECT: i32 = 1131;
pub const MCIWNDM_GETACTIVETIMER: i32 = 1156;
pub const MCIWNDM_GETALIAS: i32 = 1161;
pub const MCIWNDM_GETDEVICE: i32 = 1149;
pub const MCIWNDM_GETDEVICEA: i32 = 1149;
pub const MCIWNDM_GETDEVICEID: i32 = 1124;
pub const MCIWNDM_GETDEVICEW: i32 = 1249;
pub const MCIWNDM_GETEND: i32 = 1129;
pub const MCIWNDM_GETERROR: i32 = 1152;
pub const MCIWNDM_GETERRORA: i32 = 1152;
pub const MCIWNDM_GETERRORW: i32 = 1252;
pub const MCIWNDM_GETFILENAME: i32 = 1148;
pub const MCIWNDM_GETFILENAMEA: i32 = 1148;
pub const MCIWNDM_GETFILENAMEW: i32 = 1248;
pub const MCIWNDM_GETINACTIVETIMER: i32 = 1157;
pub const MCIWNDM_GETLENGTH: i32 = 1128;
pub const MCIWNDM_GETMODE: i32 = 1130;
pub const MCIWNDM_GETMODEA: i32 = 1130;
pub const MCIWNDM_GETMODEW: i32 = 1230;
pub const MCIWNDM_GETPALETTE: i32 = 1150;
pub const MCIWNDM_GETPOSITION: i32 = 1126;
pub const MCIWNDM_GETPOSITIONA: i32 = 1126;
pub const MCIWNDM_GETPOSITIONW: i32 = 1226;
pub const MCIWNDM_GETREPEAT: i32 = 1139;
pub const MCIWNDM_GETSPEED: i32 = 1137;
pub const MCIWNDM_GETSTART: i32 = 1127;
pub const MCIWNDM_GETSTYLES: i32 = 1160;
pub const MCIWNDM_GETTIMEFORMAT: i32 = 1144;
pub const MCIWNDM_GETTIMEFORMATA: i32 = 1144;
pub const MCIWNDM_GETTIMEFORMATW: i32 = 1244;
pub const MCIWNDM_GETVOLUME: i32 = 1135;
pub const MCIWNDM_GETZOOM: i32 = 1133;
pub const MCIWNDM_GET_DEST: i32 = 1166;
pub const MCIWNDM_GET_SOURCE: i32 = 1164;
pub const MCIWNDM_NEW: i32 = 1158;
pub const MCIWNDM_NEWA: i32 = 1158;
pub const MCIWNDM_NEWW: i32 = 1258;
pub const MCIWNDM_NOTIFYERROR: i32 = 1229;
pub const MCIWNDM_NOTIFYMEDIA: i32 = 1227;
pub const MCIWNDM_NOTIFYMODE: i32 = 1224;
pub const MCIWNDM_NOTIFYPOS: i32 = 1225;
pub const MCIWNDM_NOTIFYSIZE: i32 = 1226;
pub const MCIWNDM_OPEN: i32 = 1177;
pub const MCIWNDM_OPENA: i32 = 1177;
pub const MCIWNDM_OPENINTERFACE: i32 = 1175;
pub const MCIWNDM_OPENW: i32 = 1276;
pub const MCIWNDM_PALETTEKICK: i32 = 1174;
pub const MCIWNDM_PLAYFROM: i32 = 1146;
pub const MCIWNDM_PLAYREVERSE: i32 = 1163;
pub const MCIWNDM_PLAYTO: i32 = 1147;
pub const MCIWNDM_PUT_DEST: i32 = 1167;
pub const MCIWNDM_PUT_SOURCE: i32 = 1165;
pub const MCIWNDM_REALIZE: i32 = 1142;
pub const MCIWNDM_RETURNSTRING: i32 = 1162;
pub const MCIWNDM_RETURNSTRINGA: i32 = 1162;
pub const MCIWNDM_RETURNSTRINGW: i32 = 1262;
pub const MCIWNDM_SENDSTRING: i32 = 1125;
pub const MCIWNDM_SENDSTRINGA: i32 = 1125;
pub const MCIWNDM_SENDSTRINGW: i32 = 1225;
pub const MCIWNDM_SETACTIVETIMER: i32 = 1154;
pub const MCIWNDM_SETINACTIVETIMER: i32 = 1155;
pub const MCIWNDM_SETOWNER: i32 = 1176;
pub const MCIWNDM_SETPALETTE: i32 = 1151;
pub const MCIWNDM_SETREPEAT: i32 = 1138;
pub const MCIWNDM_SETSPEED: i32 = 1136;
pub const MCIWNDM_SETTIMEFORMAT: i32 = 1143;
pub const MCIWNDM_SETTIMEFORMATA: i32 = 1143;
pub const MCIWNDM_SETTIMEFORMATW: i32 = 1243;
pub const MCIWNDM_SETTIMERS: i32 = 1153;
pub const MCIWNDM_SETVOLUME: i32 = 1134;
pub const MCIWNDM_SETZOOM: i32 = 1132;
pub const MCIWNDM_VALIDATEMEDIA: i32 = 1145;
pub const MCIWNDOPENF_NEW: i32 = 1;
pub const MCIWND_END: i32 = -2;
pub const MCIWND_START: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub const PD_CAN_DRAW_DIB: i32 = 1;
pub const PD_CAN_STRETCHDIB: i32 = 2;
pub const PD_STRETCHDIB_1_1_OK: i32 = 4;
pub const PD_STRETCHDIB_1_2_OK: i32 = 8;
pub const PD_STRETCHDIB_1_N_OK: i32 = 16;
#[cfg(feature = "minwindef")]
pub type PVIDEOHDR = *mut VIDEOHDR;
pub const SEARCH_ANY: i32 = 32;
pub const SEARCH_BACKWARD: i32 = 4;
pub const SEARCH_FORWARD: i32 = 1;
pub const SEARCH_KEY: i32 = 16;
pub const SEARCH_NEAREST: i32 = 4;
pub type TWOCC = u16;
pub const VCAPS_CAN_SCALE: i32 = 8;
pub const VCAPS_DST_CAN_CLIP: i32 = 4;
pub const VCAPS_OVERLAY: i32 = 1;
pub const VCAPS_SRC_CAN_CLIP: i32 = 2;
pub const VHDR_DONE: i32 = 1;
pub const VHDR_INQUEUE: i32 = 4;
pub const VHDR_KEYFRAME: i32 = 8;
pub const VHDR_PREPARED: i32 = 2;
pub const VHDR_VALID: i32 = 15;
pub const VIDCF_COMPRESSFRAMES: i32 = 8;
pub const VIDCF_CRUNCH: i32 = 2;
pub const VIDCF_DRAW: i32 = 16;
pub const VIDCF_FASTTEMPORALC: i32 = 32;
pub const VIDCF_FASTTEMPORALD: i32 = 128;
pub const VIDCF_QUALITY: i32 = 1;
pub const VIDCF_TEMPORAL: i32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct VIDEOHDR {
    pub lpData: super::LPBYTE,
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
pub const VIDEO_CONFIGURE_CURRENT: i32 = 16;
pub const VIDEO_CONFIGURE_GET: i32 = 8192;
pub const VIDEO_CONFIGURE_MAX: i32 = 128;
pub const VIDEO_CONFIGURE_MIN: i32 = 64;
pub const VIDEO_CONFIGURE_NOMINAL: i32 = 32;
pub const VIDEO_CONFIGURE_QUERY: i32 = 32768;
pub const VIDEO_CONFIGURE_QUERYSIZE: i32 = 1;
pub const VIDEO_CONFIGURE_SET: i32 = 4096;
pub const VIDEO_DLG_QUERY: i32 = 16;
pub const VIDEO_EXTERNALIN: i32 = 1;
pub const VIDEO_EXTERNALOUT: i32 = 2;
pub const VIDEO_IN: i32 = 4;
pub const VIDEO_OUT: i32 = 8;
pub const WM_CAP_ABORT: i32 = 1093;
pub const WM_CAP_DLG_VIDEOCOMPRESSION: i32 = 1070;
pub const WM_CAP_DLG_VIDEODISPLAY: i32 = 1067;
pub const WM_CAP_DLG_VIDEOFORMAT: i32 = 1065;
pub const WM_CAP_DLG_VIDEOSOURCE: i32 = 1066;
pub const WM_CAP_DRIVER_CONNECT: i32 = 1034;
pub const WM_CAP_DRIVER_DISCONNECT: i32 = 1035;
pub const WM_CAP_DRIVER_GET_CAPS: i32 = 1038;
pub const WM_CAP_DRIVER_GET_NAME: i32 = 1036;
pub const WM_CAP_DRIVER_GET_NAMEA: i32 = 1036;
pub const WM_CAP_DRIVER_GET_NAMEW: i32 = 1136;
pub const WM_CAP_DRIVER_GET_VERSION: i32 = 1037;
pub const WM_CAP_DRIVER_GET_VERSIONA: i32 = 1037;
pub const WM_CAP_DRIVER_GET_VERSIONW: i32 = 1137;
pub const WM_CAP_EDIT_COPY: i32 = 1054;
pub const WM_CAP_END: i32 = 1205;
pub const WM_CAP_FILE_ALLOCATE: i32 = 1046;
pub const WM_CAP_FILE_GET_CAPTURE_FILE: i32 = 1045;
pub const WM_CAP_FILE_GET_CAPTURE_FILEA: i32 = 1045;
pub const WM_CAP_FILE_GET_CAPTURE_FILEW: i32 = 1145;
pub const WM_CAP_FILE_SAVEAS: i32 = 1047;
pub const WM_CAP_FILE_SAVEASA: i32 = 1047;
pub const WM_CAP_FILE_SAVEASW: i32 = 1147;
pub const WM_CAP_FILE_SAVEDIB: i32 = 1049;
pub const WM_CAP_FILE_SAVEDIBA: i32 = 1049;
pub const WM_CAP_FILE_SAVEDIBW: i32 = 1149;
pub const WM_CAP_FILE_SET_CAPTURE_FILE: i32 = 1044;
pub const WM_CAP_FILE_SET_CAPTURE_FILEA: i32 = 1044;
pub const WM_CAP_FILE_SET_CAPTURE_FILEW: i32 = 1144;
pub const WM_CAP_FILE_SET_INFOCHUNK: i32 = 1048;
pub const WM_CAP_GET_AUDIOFORMAT: i32 = 1060;
pub const WM_CAP_GET_CAPSTREAMPTR: i32 = 1025;
pub const WM_CAP_GET_MCI_DEVICE: i32 = 1091;
pub const WM_CAP_GET_MCI_DEVICEA: i32 = 1091;
pub const WM_CAP_GET_MCI_DEVICEW: i32 = 1191;
pub const WM_CAP_GET_SEQUENCE_SETUP: i32 = 1089;
pub const WM_CAP_GET_STATUS: i32 = 1078;
pub const WM_CAP_GET_USER_DATA: i32 = 1032;
pub const WM_CAP_GET_VIDEOFORMAT: i32 = 1068;
pub const WM_CAP_GRAB_FRAME: i32 = 1084;
pub const WM_CAP_GRAB_FRAME_NOSTOP: i32 = 1085;
pub const WM_CAP_PAL_AUTOCREATE: i32 = 1107;
pub const WM_CAP_PAL_MANUALCREATE: i32 = 1108;
pub const WM_CAP_PAL_OPEN: i32 = 1104;
pub const WM_CAP_PAL_OPENA: i32 = 1104;
pub const WM_CAP_PAL_OPENW: i32 = 1204;
pub const WM_CAP_PAL_PASTE: i32 = 1106;
pub const WM_CAP_PAL_SAVE: i32 = 1105;
pub const WM_CAP_PAL_SAVEA: i32 = 1105;
pub const WM_CAP_PAL_SAVEW: i32 = 1205;
pub const WM_CAP_SEQUENCE: i32 = 1086;
pub const WM_CAP_SEQUENCE_NOFILE: i32 = 1087;
pub const WM_CAP_SET_AUDIOFORMAT: i32 = 1059;
pub const WM_CAP_SET_CALLBACK_CAPCONTROL: i32 = 1109;
pub const WM_CAP_SET_CALLBACK_ERROR: i32 = 1026;
pub const WM_CAP_SET_CALLBACK_ERRORA: i32 = 1026;
pub const WM_CAP_SET_CALLBACK_ERRORW: i32 = 1126;
pub const WM_CAP_SET_CALLBACK_FRAME: i32 = 1029;
pub const WM_CAP_SET_CALLBACK_STATUS: i32 = 1027;
pub const WM_CAP_SET_CALLBACK_STATUSA: i32 = 1027;
pub const WM_CAP_SET_CALLBACK_STATUSW: i32 = 1127;
pub const WM_CAP_SET_CALLBACK_VIDEOSTREAM: i32 = 1030;
pub const WM_CAP_SET_CALLBACK_WAVESTREAM: i32 = 1031;
pub const WM_CAP_SET_CALLBACK_YIELD: i32 = 1028;
pub const WM_CAP_SET_MCI_DEVICE: i32 = 1090;
pub const WM_CAP_SET_MCI_DEVICEA: i32 = 1090;
pub const WM_CAP_SET_MCI_DEVICEW: i32 = 1190;
pub const WM_CAP_SET_OVERLAY: i32 = 1075;
pub const WM_CAP_SET_PREVIEW: i32 = 1074;
pub const WM_CAP_SET_PREVIEWRATE: i32 = 1076;
pub const WM_CAP_SET_SCALE: i32 = 1077;
pub const WM_CAP_SET_SCROLL: i32 = 1079;
pub const WM_CAP_SET_SEQUENCE_SETUP: i32 = 1088;
pub const WM_CAP_SET_USER_DATA: i32 = 1033;
pub const WM_CAP_SET_VIDEOFORMAT: i32 = 1069;
pub const WM_CAP_SINGLE_FRAME: i32 = 1096;
pub const WM_CAP_SINGLE_FRAME_CLOSE: i32 = 1095;
pub const WM_CAP_SINGLE_FRAME_OPEN: i32 = 1094;
pub const WM_CAP_START: i32 = 1024;
pub const WM_CAP_STOP: i32 = 1092;
pub const WM_CAP_UNICODE_END: i32 = 1205;
pub const WM_CAP_UNICODE_START: i32 = 1124;
pub const ckidAVIMAINHDR: u32 = 1751742049;
pub const ckidAVINEWINDEX: u32 = 829973609;
pub const ckidAVIPADDING: u32 = 1263424842;
pub const ckidSTREAMFORMAT: u32 = 1718776947;
pub const ckidSTREAMHANDLERDATA: u32 = 1685222515;
pub const ckidSTREAMHEADER: u32 = 1752331379;
pub const ckidSTREAMNAME: u32 = 1852994675;
pub const cktypeDIBbits: i32 = 25188;
pub const cktypeDIBcompressed: i32 = 25444;
pub const cktypePALchange: i32 = 25456;
pub const cktypeWAVEbytes: i32 = 25207;
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
