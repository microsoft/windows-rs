#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIBuildFilterA(lpszfilter : windows_sys::core::PSTR, cbfilter : super::LONG, fsaving : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIBuildFilterW(lpszfilter : windows_sys::core::PWSTR, cbfilter : super::LONG, fsaving : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIClearClipboard() -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIFileAddRef(pfile : PAVIFILE) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileCreateStreamA(pfile : PAVIFILE, ppavi : *mut PAVISTREAM, psi : *const AVISTREAMINFOA) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileCreateStreamW(pfile : PAVIFILE, ppavi : *mut PAVISTREAM, psi : *const AVISTREAMINFOW) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileEndRecord(pfile : PAVIFILE) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileExit());
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIFileGetStream(pfile : PAVIFILE, ppavi : *mut PAVISTREAM, fcctype : super::DWORD, lparam : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIFileInfoA(pfile : PAVIFILE, pfi : LPAVIFILEINFOA, lsize : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileInfoW(pfile : PAVIFILE, pfi : LPAVIFILEINFOW, lsize : super::LONG) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIFileInit());
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileOpenA(ppfile : *mut PAVIFILE, szfile : windows_sys::core::PCSTR, umode : super::UINT, lphandler : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileOpenW(ppfile : *mut PAVIFILE, szfile : windows_sys::core::PCWSTR, umode : super::UINT, lphandler : super::LPCLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileReadData(pfile : PAVIFILE, ckid : super::DWORD, lpdata : super::LPVOID, lpcbdata : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIFileRelease(pfile : PAVIFILE) -> super::ULONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIFileWriteData(pfile : PAVIFILE, ckid : super::DWORD, lpdata : super::LPVOID, cbdata : super::LONG) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIGetFromClipboard(lppf : *mut PAVIFILE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIMakeCompressedStream(ppscompressed : *mut PAVISTREAM, ppssource : PAVISTREAM, lpoptions : *const AVICOMPRESSOPTIONS, pclsidhandler : *const super::CLSID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIMakeFileFromStreams(ppfile : *mut PAVIFILE, nstreams : i32, papstreams : *const PAVISTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("avifil32.dll" "system" fn AVIMakeStreamFromClipboard(cfformat : super::UINT, hglobal : super::HANDLE, ppstream : *mut PAVISTREAM) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIPutFileOnClipboard(pf : PAVIFILE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "C" fn AVISaveA(szfile : windows_sys::core::PCSTR, pclsidhandler : *const super::CLSID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : PAVISTREAM, lpoptions : LPAVICOMPRESSOPTIONS, ...) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("avifil32.dll" "system" fn AVISaveOptions(hwnd : super::HWND, uiflags : super::UINT, nstreams : i32, ppavi : *const PAVISTREAM, plpoptions : *mut LPAVICOMPRESSOPTIONS) -> super::INT_PTR);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVISaveOptionsFree(nstreams : i32, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVISaveVA(szfile : windows_sys::core::PCSTR, pclsidhandler : *const super::CLSID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const PAVISTREAM, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVISaveVW(szfile : windows_sys::core::PCWSTR, pclsidhandler : *const super::CLSID, lpfncallback : AVISAVECALLBACK, nstreams : i32, ppavi : *const PAVISTREAM, plpoptions : *const LPAVICOMPRESSOPTIONS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "C" fn AVISaveW(szfile : windows_sys::core::PCWSTR, pclsidhandler : *const super::CLSID, lpfncallback : AVISAVECALLBACK, nstreams : i32, pfile : PAVISTREAM, lpoptions : LPAVICOMPRESSOPTIONS, ...) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamAddRef(pavi : PAVISTREAM) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamBeginStreaming(pavi : PAVISTREAM, lstart : super::LONG, lend : super::LONG, lrate : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamCreate(ppavi : *mut PAVISTREAM, lparam1 : super::LONG, lparam2 : super::LONG, pclsidhandler : *const super::CLSID) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn AVIStreamEndStreaming(pavi : PAVISTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamFindSample(pavi : PAVISTREAM, lpos : super::LONG, lflags : super::LONG) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamGetFrame(pg : PGETFRAME, lpos : super::LONG) -> super::LPVOID);
windows_link::link!("avifil32.dll" "system" fn AVIStreamGetFrameClose(pg : PGETFRAME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamGetFrameOpen(pavi : PAVISTREAM, lpbiwanted : super::LPBITMAPINFOHEADER) -> PGETFRAME);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamInfoA(pavi : PAVISTREAM, psi : LPAVISTREAMINFOA, lsize : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamInfoW(pavi : PAVISTREAM, psi : LPAVISTREAMINFOW, lsize : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamLength(pavi : PAVISTREAM) -> super::LONG);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileA(ppavi : *mut PAVISTREAM, szfile : windows_sys::core::PCSTR, fcctype : super::DWORD, lparam : super::LONG, mode : super::UINT, pclsidhandler : *const super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamOpenFromFileW(ppavi : *mut PAVISTREAM, szfile : windows_sys::core::PCWSTR, fcctype : super::DWORD, lparam : super::LONG, mode : super::UINT, pclsidhandler : *const super::CLSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamRead(pavi : PAVISTREAM, lstart : super::LONG, lsamples : super::LONG, lpbuffer : super::LPVOID, cbbuffer : super::LONG, plbytes : *mut super::LONG, plsamples : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamReadData(pavi : PAVISTREAM, fcc : super::DWORD, lp : super::LPVOID, lpcb : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamReadFormat(pavi : PAVISTREAM, lpos : super::LONG, lpformat : super::LPVOID, lpcbformat : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamRelease(pavi : PAVISTREAM) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamSampleToTime(pavi : PAVISTREAM, lsample : super::LONG) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamSetFormat(pavi : PAVISTREAM, lpos : super::LONG, lpformat : super::LPVOID, cbformat : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamStart(pavi : PAVISTREAM) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn AVIStreamTimeToSample(pavi : PAVISTREAM, ltime : super::LONG) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamWrite(pavi : PAVISTREAM, lstart : super::LONG, lsamples : super::LONG, lpbuffer : super::LPVOID, cbbuffer : super::LONG, dwflags : super::DWORD, plsampwritten : *mut super::LONG, plbyteswritten : *mut super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("avifil32.dll" "system" fn AVIStreamWriteData(pavi : PAVISTREAM, fcc : super::DWORD, lp : super::LPVOID, cb : super::LONG) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn CreateEditableStream(ppseditable : *mut PAVISTREAM, pssource : PAVISTREAM) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibBegin(hdd : HDRAWDIB, hdc : super::HDC, dxdst : i32, dydst : i32, lpbi : super::LPBITMAPINFOHEADER, dxsrc : i32, dysrc : i32, wflags : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibChangePalette(hdd : HDRAWDIB, istart : i32, ilen : i32, lppe : super::LPPALETTEENTRY) -> windows_sys::core::BOOL);
windows_link::link!("msvfw32.dll" "system" fn DrawDibClose(hdd : HDRAWDIB) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibDraw(hdd : HDRAWDIB, hdc : super::HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : super::LPBITMAPINFOHEADER, lpbits : super::LPVOID, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, wflags : super::UINT) -> windows_sys::core::BOOL);
windows_link::link!("msvfw32.dll" "system" fn DrawDibEnd(hdd : HDRAWDIB) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibGetBuffer(hdd : HDRAWDIB, lpbi : super::LPBITMAPINFOHEADER, dwsize : super::DWORD, dwflags : super::DWORD) -> super::LPVOID);
#[cfg(feature = "windef")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibGetPalette(hdd : HDRAWDIB) -> super::HPALETTE);
windows_link::link!("msvfw32.dll" "system" fn DrawDibOpen() -> HDRAWDIB);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibProfileDisplay(lpbi : super::LPBITMAPINFOHEADER) -> super::LRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn DrawDibRealize(hdd : HDRAWDIB, hdc : super::HDC, fbackground : windows_sys::core::BOOL) -> super::UINT);
#[cfg(feature = "windef")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibSetPalette(hdd : HDRAWDIB, hpal : super::HPALETTE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibStart(hdd : HDRAWDIB, rate : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("msvfw32.dll" "system" fn DrawDibStop(hdd : HDRAWDIB) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("msvfw32.dll" "system" fn DrawDibTime(hdd : HDRAWDIB, lpddtime : LPDRAWDIBTIME) -> windows_sys::core::BOOL);
windows_link::link!("avifil32.dll" "system" fn EditStreamClone(pavi : PAVISTREAM, ppresult : *mut PAVISTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn EditStreamCopy(pavi : PAVISTREAM, plstart : *mut super::LONG, pllength : *mut super::LONG, ppresult : *mut PAVISTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn EditStreamCut(pavi : PAVISTREAM, plstart : *mut super::LONG, pllength : *mut super::LONG, ppresult : *mut PAVISTREAM) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("avifil32.dll" "system" fn EditStreamPaste(pavi : PAVISTREAM, plpos : *mut super::LONG, pllength : *mut super::LONG, pstream : PAVISTREAM, lstart : super::LONG, lend : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("avifil32.dll" "system" fn EditStreamSetInfoA(pavi : PAVISTREAM, lpinfo : LPAVISTREAMINFOA, cbinfo : super::LONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("avifil32.dll" "system" fn EditStreamSetInfoW(pavi : PAVISTREAM, lpinfo : LPAVISTREAMINFOW, cbinfo : super::LONG) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamSetNameA(pavi : PAVISTREAM, lpszname : windows_sys::core::PCSTR) -> windows_sys::core::HRESULT);
windows_link::link!("avifil32.dll" "system" fn EditStreamSetNameW(pavi : PAVISTREAM, lpszname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewA(lpofn : super::LPOPENFILENAMEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetOpenFileNamePreviewW(lpofn : super::LPOPENFILENAMEW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewA(lpofn : super::LPOPENFILENAMEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "commdlg", feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "system" fn GetSaveFileNamePreviewW(lpofn : super::LPOPENFILENAMEW) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("msvfw32.dll" "system" fn ICClose(hic : HIC) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "C" fn ICCompress(hic : HIC, dwflags : super::DWORD, lpbioutput : super::LPBITMAPINFOHEADER, lpdata : super::LPVOID, lpbiinput : super::LPBITMAPINFOHEADER, lpbits : super::LPVOID, lpckid : super::LPDWORD, lpdwflags : super::LPDWORD, lframenum : super::LONG, dwframesize : super::DWORD, dwquality : super::DWORD, lpbiprev : super::LPBITMAPINFOHEADER, lpprev : super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICCompressorChoose(hwnd : super::HWND, uiflags : super::UINT, pvin : super::LPVOID, lpdata : super::LPVOID, pc : PCOMPVARS, lpsztitle : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICCompressorFree(pc : PCOMPVARS));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "C" fn ICDecompress(hic : HIC, dwflags : super::DWORD, lpbiformat : super::LPBITMAPINFOHEADER, lpdata : super::LPVOID, lpbi : super::LPBITMAPINFOHEADER, lpbits : super::LPVOID) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("msvfw32.dll" "C" fn ICDraw(hic : HIC, dwflags : super::DWORD, lpformat : super::LPVOID, lpdata : super::LPVOID, cbdata : super::DWORD, ltime : super::LONG) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "C" fn ICDrawBegin(hic : HIC, dwflags : super::DWORD, hpal : super::HPALETTE, hwnd : super::HWND, hdc : super::HDC, xdst : i32, ydst : i32, dxdst : i32, dydst : i32, lpbi : super::LPBITMAPINFOHEADER, xsrc : i32, ysrc : i32, dxsrc : i32, dysrc : i32, dwrate : super::DWORD, dwscale : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICGetDisplayFormat(hic : HIC, lpbiin : super::LPBITMAPINFOHEADER, lpbiout : super::LPBITMAPINFOHEADER, bitdepth : i32, dx : i32, dy : i32) -> HIC);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn ICGetInfo(hic : HIC, picinfo : *mut ICINFO, cb : super::DWORD) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn ICImageCompress(hic : HIC, uiflags : super::UINT, lpbiin : super::LPBITMAPINFO, lpbits : super::LPVOID, lpbiout : super::LPBITMAPINFO, lquality : super::LONG, plsize : *mut super::LONG) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn ICImageDecompress(hic : HIC, uiflags : super::UINT, lpbiin : super::LPBITMAPINFO, lpbits : super::LPVOID, lpbiout : super::LPBITMAPINFO) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("msvfw32.dll" "system" fn ICInfo(fcctype : super::DWORD, fcchandler : super::DWORD, lpicinfo : *mut ICINFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("msvfw32.dll" "system" fn ICInstall(fcctype : super::DWORD, fcchandler : super::DWORD, lparam : super::LPARAM, szdesc : windows_sys::core::PCSTR, wflags : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICLocate(fcctype : super::DWORD, fcchandler : super::DWORD, lpbiin : super::LPBITMAPINFOHEADER, lpbiout : super::LPBITMAPINFOHEADER, wflags : super::WORD) -> HIC);
#[cfg(feature = "shared")]
windows_link::link!("msvfw32.dll" "system" fn ICOpen(fcctype : super::DWORD, fcchandler : super::DWORD, wmode : super::UINT) -> HIC);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("msvfw32.dll" "system" fn ICOpenFunction(fcctype : super::DWORD, fcchandler : super::DWORD, wmode : super::UINT, lpfnhandler : super::FARPROC) -> HIC);
#[cfg(feature = "shared")]
windows_link::link!("msvfw32.dll" "system" fn ICRemove(fcctype : super::DWORD, fcchandler : super::DWORD, wflags : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("msvfw32.dll" "system" fn ICSendMessage(hic : HIC, msg : super::UINT, dw1 : super::DWORD_PTR, dw2 : super::DWORD_PTR) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICSeqCompressFrame(pc : PCOMPVARS, uiflags : super::UINT, lpbits : super::LPVOID, pfkey : *mut windows_sys::core::BOOL, plsize : *mut super::LONG) -> super::LPVOID);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICSeqCompressFrameEnd(pc : PCOMPVARS));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
windows_link::link!("msvfw32.dll" "system" fn ICSeqCompressFrameStart(pc : PCOMPVARS, lpbiin : super::LPBITMAPINFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "C" fn MCIWndCreateA(hwndparent : super::HWND, hinstance : super::HINSTANCE, dwstyle : super::DWORD, szfile : windows_sys::core::PCSTR) -> super::HWND);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
windows_link::link!("msvfw32.dll" "C" fn MCIWndCreateW(hwndparent : super::HWND, hinstance : super::HINSTANCE, dwstyle : super::DWORD, szfile : windows_sys::core::PCWSTR) -> super::HWND);
windows_link::link!("msvfw32.dll" "C" fn MCIWndRegisterClass() -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("msvfw32.dll" "system" fn VideoForWindowsVersion() -> super::DWORD);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("avicap32.dll" "system" fn capCreateCaptureWindowA(lpszwindowname : windows_sys::core::PCSTR, dwstyle : super::DWORD, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, nid : i32) -> super::HWND);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("avicap32.dll" "system" fn capCreateCaptureWindowW(lpszwindowname : windows_sys::core::PCWSTR, dwstyle : super::DWORD, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : super::HWND, nid : i32) -> super::HWND);
#[cfg(feature = "shared")]
windows_link::link!("avicap32.dll" "system" fn capGetDriverDescriptionA(wdriverindex : super::UINT, lpszname : windows_sys::core::PSTR, cbname : i32, lpszver : windows_sys::core::PSTR, cbver : i32) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("avicap32.dll" "system" fn capGetDriverDescriptionW(wdriverindex : super::UINT, lpszname : windows_sys::core::PWSTR, cbname : i32, lpszver : windows_sys::core::PWSTR, cbver : i32) -> windows_sys::core::BOOL);
pub const AVICOMPRESSF_DATARATE: i32 = 2;
pub const AVICOMPRESSF_INTERLEAVE: i32 = 1;
pub const AVICOMPRESSF_KEYFRAMES: i32 = 4;
pub const AVICOMPRESSF_VALID: i32 = 8;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct AVICOMPRESSOPTIONS {
    pub fccType: super::DWORD,
    pub fccHandler: super::DWORD,
    pub dwKeyFrameEvery: super::DWORD,
    pub dwQuality: super::DWORD,
    pub dwBytesPerSecond: super::DWORD,
    pub dwFlags: super::DWORD,
    pub lpFormat: super::LPVOID,
    pub cbFormat: super::DWORD,
    pub lpParms: super::LPVOID,
    pub cbParms: super::DWORD,
    pub dwInterleaveEvery: super::DWORD,
}
#[cfg(feature = "wtypesbase")]
pub const AVIERR_BADFLAGS: super::SCODE = 0x80044069_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_BADFORMAT: super::SCODE = 0x80044066_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_BADHANDLE: super::SCODE = 0x8004406C_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_BADPARAM: super::SCODE = 0x8004406A_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_BADSIZE: super::SCODE = 0x8004406B_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_BUFFERTOOSMALL: super::SCODE = 0x80044074_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_CANTCOMPRESS: super::SCODE = 0x80044075_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_COMPRESSOR: super::SCODE = 0x80044070_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_ERROR: super::SCODE = 0x800440C7_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_FILEOPEN: super::SCODE = 0x8004406F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_FILEREAD: super::SCODE = 0x8004406D_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_FILEWRITE: super::SCODE = 0x8004406E_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_INTERNAL: super::SCODE = 0x80044068_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_MEMORY: super::SCODE = 0x80044067_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_NOCOMPRESSOR: super::SCODE = 0x80044071_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_NODATA: super::SCODE = 0x80044073_u32 as _;
pub const AVIERR_OK: i32 = 0;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_READONLY: super::SCODE = 0x80044072_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_UNSUPPORTED: super::SCODE = 0x80044065_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const AVIERR_USERABORT: super::SCODE = 0x800440C6_u32 as _;
pub const AVIFILECAPS_ALLKEYFRAMES: i32 = 16;
pub const AVIFILECAPS_CANREAD: i32 = 1;
pub const AVIFILECAPS_CANWRITE: i32 = 2;
pub const AVIFILECAPS_NOCOMPRESSION: i32 = 32;
pub const AVIFILEHANDLER_CANACCEPTNONRGB: i32 = 4;
pub const AVIFILEHANDLER_CANREAD: i32 = 1;
pub const AVIFILEHANDLER_CANWRITE: i32 = 2;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct AVIFILEINFOA {
    pub dwMaxBytesPerSec: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwStreams: super::DWORD,
    pub dwSuggestedBufferSize: super::DWORD,
    pub dwWidth: super::DWORD,
    pub dwHeight: super::DWORD,
    pub dwScale: super::DWORD,
    pub dwRate: super::DWORD,
    pub dwLength: super::DWORD,
    pub dwEditCount: super::DWORD,
    pub szFileType: [i8; 64],
}
#[cfg(feature = "shared")]
impl Default for AVIFILEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AVIFILEINFOW {
    pub dwMaxBytesPerSec: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCaps: super::DWORD,
    pub dwStreams: super::DWORD,
    pub dwSuggestedBufferSize: super::DWORD,
    pub dwWidth: super::DWORD,
    pub dwHeight: super::DWORD,
    pub dwScale: super::DWORD,
    pub dwRate: super::DWORD,
    pub dwLength: super::DWORD,
    pub dwEditCount: super::DWORD,
    pub szFileType: [super::WCHAR; 64],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AVIINDEXENTRY {
    pub ckid: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwChunkOffset: super::DWORD,
    pub dwChunkLength: super::DWORD,
}
pub type AVISAVECALLBACK = Option<unsafe extern "system" fn(param0: i32) -> windows_sys::core::BOOL>;
pub const AVISF_DISABLED: i32 = 1;
pub const AVISF_VIDEO_PALCHANGES: i32 = 65536;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct AVISTREAMINFOA {
    pub fccType: super::DWORD,
    pub fccHandler: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCaps: super::DWORD,
    pub wPriority: super::WORD,
    pub wLanguage: super::WORD,
    pub dwScale: super::DWORD,
    pub dwRate: super::DWORD,
    pub dwStart: super::DWORD,
    pub dwLength: super::DWORD,
    pub dwInitialFrames: super::DWORD,
    pub dwSuggestedBufferSize: super::DWORD,
    pub dwQuality: super::DWORD,
    pub dwSampleSize: super::DWORD,
    pub rcFrame: super::RECT,
    pub dwEditCount: super::DWORD,
    pub dwFormatChangeCount: super::DWORD,
    pub szName: [i8; 64],
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl Default for AVISTREAMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct AVISTREAMINFOW {
    pub fccType: super::DWORD,
    pub fccHandler: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwCaps: super::DWORD,
    pub wPriority: super::WORD,
    pub wLanguage: super::WORD,
    pub dwScale: super::DWORD,
    pub dwRate: super::DWORD,
    pub dwStart: super::DWORD,
    pub dwLength: super::DWORD,
    pub dwInitialFrames: super::DWORD,
    pub dwSuggestedBufferSize: super::DWORD,
    pub dwQuality: super::DWORD,
    pub dwSampleSize: super::DWORD,
    pub rcFrame: super::RECT,
    pub dwEditCount: super::DWORD,
    pub dwFormatChangeCount: super::DWORD,
    pub szName: [super::WCHAR; 64],
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for AVISTREAMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AVISTREAMINFO_DISABLED: i32 = 1;
pub const AVISTREAMINFO_FORMATCHANGES: i32 = 65536;
pub const AVISTREAMREAD_CONVENIENT: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct AVIStreamHeader {
    pub fccType: super::FOURCC,
    pub fccHandler: super::FOURCC,
    pub dwFlags: super::DWORD,
    pub wPriority: super::WORD,
    pub wLanguage: super::WORD,
    pub dwInitialFrames: super::DWORD,
    pub dwScale: super::DWORD,
    pub dwRate: super::DWORD,
    pub dwStart: super::DWORD,
    pub dwLength: super::DWORD,
    pub dwSuggestedBufferSize: super::DWORD,
    pub dwQuality: super::DWORD,
    pub dwSampleSize: super::DWORD,
    pub rcFrame: super::RECT,
}
pub const AVI_HEADERSIZE: i32 = 2048;
pub const AVSTREAMMASTER_AUDIO: i32 = 0;
pub const AVSTREAMMASTER_NONE: i32 = 1;
pub const BI_1632: i32 = 842217009;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPCONTROLCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, nstate: i32) -> super::LRESULT>;
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: super::UINT,
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
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPERRORCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCSTR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPERRORCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCWSTR) -> super::LRESULT>;
#[repr(C, packed(8))]
#[cfg(all(feature = "minwindef", feature = "mmiscapi", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CAPINFOCHUNK {
    pub fccInfoID: super::FOURCC,
    pub lpData: super::LPVOID,
    pub cbData: super::LONG,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct CAPSTATUS {
    pub uiImageWidth: super::UINT,
    pub uiImageHeight: super::UINT,
    pub fLiveWindow: windows_sys::core::BOOL,
    pub fOverlayWindow: windows_sys::core::BOOL,
    pub fScale: windows_sys::core::BOOL,
    pub ptScroll: super::POINT,
    pub fUsingDefaultPalette: windows_sys::core::BOOL,
    pub fAudioHardware: windows_sys::core::BOOL,
    pub fCapFileExists: windows_sys::core::BOOL,
    pub dwCurrentVideoFrame: super::DWORD,
    pub dwCurrentVideoFramesDropped: super::DWORD,
    pub dwCurrentWaveSamples: super::DWORD,
    pub dwCurrentTimeElapsedMS: super::DWORD,
    pub hPalCurrent: super::HPALETTE,
    pub fCapturingNow: windows_sys::core::BOOL,
    pub dwReturn: super::DWORD,
    pub wNumVideoAllocated: super::UINT,
    pub wNumAudioAllocated: super::UINT,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPSTATUSCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCSTR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPSTATUSCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::HWND, nid: i32, lpsz: windows_sys::core::PCWSTR) -> super::LRESULT>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: super::DWORD,
    pub fMakeUserHitOKToCapture: windows_sys::core::BOOL,
    pub wPercentDropForError: super::UINT,
    pub fYield: windows_sys::core::BOOL,
    pub dwIndexSize: super::DWORD,
    pub wChunkGranularity: super::UINT,
    pub fUsingDOSMemory: windows_sys::core::BOOL,
    pub wNumVideoRequested: super::UINT,
    pub fCaptureAudio: windows_sys::core::BOOL,
    pub wNumAudioRequested: super::UINT,
    pub vKeyAbort: super::UINT,
    pub fAbortLeftMouse: windows_sys::core::BOOL,
    pub fAbortRightMouse: windows_sys::core::BOOL,
    pub fLimitEnabled: windows_sys::core::BOOL,
    pub wTimeLimit: super::UINT,
    pub fMCIControl: windows_sys::core::BOOL,
    pub fStepMCIDevice: windows_sys::core::BOOL,
    pub dwMCIStartTime: super::DWORD,
    pub dwMCIStopTime: super::DWORD,
    pub fStepCaptureAt2x: windows_sys::core::BOOL,
    pub wStepCaptureAverageFrames: super::UINT,
    pub dwAudioBufferSize: super::DWORD,
    pub fDisableWriteCache: windows_sys::core::BOOL,
    pub AVStreamMaster: super::UINT,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type CAPVIDEOCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, lpvhdr: LPVIDEOHDR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef"))]
pub type CAPWAVECALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND, lpwhdr: super::LPWAVEHDR) -> super::LRESULT>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CAPYIELDCALLBACK = Option<unsafe extern "system" fn(hwnd: super::HWND) -> super::LRESULT>;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CHANNEL_CAPS {
    pub dwFlags: super::DWORD,
    pub dwSrcRectXMod: super::DWORD,
    pub dwSrcRectYMod: super::DWORD,
    pub dwSrcRectWidthMod: super::DWORD,
    pub dwSrcRectHeightMod: super::DWORD,
    pub dwDstRectXMod: super::DWORD,
    pub dwDstRectYMod: super::DWORD,
    pub dwDstRectWidthMod: super::DWORD,
    pub dwDstRectHeightMod: super::DWORD,
}
pub const CLSID_AVIFile: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020000_0000_0000_c000_000000000046);
pub const CLSID_AVISimpleUnMarshal: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020009_0000_0000_c000_000000000046);
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct COMPVARS {
    pub cbSize: super::LONG,
    pub dwFlags: super::DWORD,
    pub hic: HIC,
    pub fccType: super::DWORD,
    pub fccHandler: super::DWORD,
    pub lpbiIn: super::LPBITMAPINFO,
    pub lpbiOut: super::LPBITMAPINFO,
    pub lpBitsOut: super::LPVOID,
    pub lpBitsPrev: super::LPVOID,
    pub lFrame: super::LONG,
    pub lKey: super::LONG,
    pub lDataRate: super::LONG,
    pub lQ: super::LONG,
    pub lKeyCount: super::LONG,
    pub lpState: super::LPVOID,
    pub cbState: super::LONG,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRAWDIBTIME {
    pub timeCount: super::LONG,
    pub timeDraw: super::LONG,
    pub timeDecompress: super::LONG,
    pub timeDither: super::LONG,
    pub timeStretch: super::LONG,
    pub timeBlt: super::LONG,
    pub timeSetDIBits: super::LONG,
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
pub type HDRAWDIB = *mut core::ffi::c_void;
pub type HIC = *mut HIC__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIC__ {
    pub unused: i32,
}
pub type HVIDEO = *mut HVIDEO__;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct HVIDEO__ {
    pub unused: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct ICCOMPRESS {
    pub dwFlags: super::DWORD,
    pub lpbiOutput: super::LPBITMAPINFOHEADER,
    pub lpOutput: super::LPVOID,
    pub lpbiInput: super::LPBITMAPINFOHEADER,
    pub lpInput: super::LPVOID,
    pub lpckid: super::LPDWORD,
    pub lpdwFlags: super::LPDWORD,
    pub lFrameNum: super::LONG,
    pub dwFrameSize: super::DWORD,
    pub dwQuality: super::DWORD,
    pub lpbiPrev: super::LPBITMAPINFOHEADER,
    pub lpPrev: super::LPVOID,
}
pub const ICCOMPRESSFRAMES_PADDING: i32 = 1;
pub const ICCOMPRESS_KEYFRAME: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct ICDECOMPRESS {
    pub dwFlags: super::DWORD,
    pub lpbiInput: super::LPBITMAPINFOHEADER,
    pub lpInput: super::LPVOID,
    pub lpbiOutput: super::LPBITMAPINFOHEADER,
    pub lpOutput: super::LPVOID,
    pub ckid: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct ICDECOMPRESSEX {
    pub dwFlags: super::DWORD,
    pub lpbiSrc: super::LPBITMAPINFOHEADER,
    pub lpSrc: super::LPVOID,
    pub lpbiDst: super::LPBITMAPINFOHEADER,
    pub lpDst: super::LPVOID,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
}
pub const ICDECOMPRESS_HURRYUP: u32 = 2147483648;
pub const ICDECOMPRESS_NOTKEYFRAME: i32 = 134217728;
pub const ICDECOMPRESS_NULLFRAME: i32 = 268435456;
pub const ICDECOMPRESS_PREROLL: i32 = 536870912;
pub const ICDECOMPRESS_UPDATE: i32 = 1073741824;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct ICDRAW {
    pub dwFlags: super::DWORD,
    pub lpFormat: super::LPVOID,
    pub lpData: super::LPVOID,
    pub cbData: super::DWORD,
    pub lTime: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct ICDRAWBEGIN {
    pub dwFlags: super::DWORD,
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
    pub dwRate: super::DWORD,
    pub dwScale: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct ICDRAWSUGGEST {
    pub lpbiIn: super::LPBITMAPINFOHEADER,
    pub lpbiSuggest: super::LPBITMAPINFOHEADER,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ICINFO {
    pub dwSize: super::DWORD,
    pub fccType: super::DWORD,
    pub fccHandler: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwVersionICM: super::DWORD,
    pub szName: [super::WCHAR; 16],
    pub szDescription: [super::WCHAR; 128],
    pub szDriver: [super::WCHAR; 128],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(feature = "shared")]
pub const ICM_FRAMERATE: super::DWORD = 1382904390;
pub const ICM_GET: i32 = 20521;
pub const ICM_GETBUFFERSWANTED: i32 = 16425;
pub const ICM_GETDEFAULTKEYFRAMERATE: i32 = 16426;
pub const ICM_GETDEFAULTQUALITY: i32 = 20510;
pub const ICM_GETERRORTEXT: i32 = 20492;
pub const ICM_GETFORMATNAME: i32 = 20500;
pub const ICM_GETINFO: i32 = 20482;
pub const ICM_GETQUALITY: i32 = 20511;
pub const ICM_GETSTATE: i32 = 20480;
#[cfg(feature = "shared")]
pub const ICM_KEYFRAMERATE: super::DWORD = 1383687499;
pub const ICM_RESERVED: i32 = 20480;
pub const ICM_RESERVED_HIGH: i32 = 24576;
pub const ICM_RESERVED_LOW: i32 = 20480;
pub const ICM_SET: i32 = 20520;
pub const ICM_SETQUALITY: i32 = 20512;
pub const ICM_SETSTATE: i32 = 20481;
pub const ICM_SET_STATUS_PROC: i32 = 16456;
pub const ICM_USER: i32 = 16384;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct ICOPEN {
    pub dwSize: super::DWORD,
    pub fccType: super::DWORD,
    pub fccHandler: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwError: super::LRESULT,
    pub pV1Reserved: super::LPVOID,
    pub pV2Reserved: super::LPVOID,
    pub dnDevNode: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct ICPALETTE {
    pub dwFlags: super::DWORD,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: super::LPPALETTEENTRY,
}
pub const ICQUALITY_DEFAULT: i32 = -1;
pub const ICQUALITY_HIGH: i32 = 10000;
pub const ICQUALITY_LOW: i32 = 0;
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
pub const IID_IAVIEditStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020024_0000_0000_c000_000000000046);
pub const IID_IAVIFile: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020020_0000_0000_c000_000000000046);
pub const IID_IAVIPersistFile: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020025_0000_0000_c000_000000000046);
pub const IID_IAVIStream: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020021_0000_0000_c000_000000000046);
pub const IID_IAVIStreaming: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020022_0000_0000_c000_000000000046);
pub const IID_IGetFrame: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00020023_0000_0000_c000_000000000046);
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPAVICOMPRESSOPTIONS = *mut AVICOMPRESSOPTIONS;
#[cfg(feature = "shared")]
pub type LPAVIFILEINFOA = *mut AVIFILEINFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPAVIFILEINFOW = *mut AVIFILEINFOW;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPAVISTREAMINFOA = *mut AVISTREAMINFOA;
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPAVISTREAMINFOW = *mut AVISTREAMINFOW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCAPDRIVERCAPS = *mut CAPDRIVERCAPS;
#[cfg(all(feature = "minwindef", feature = "mmiscapi", feature = "shared"))]
pub type LPCAPINFOCHUNK = *mut CAPINFOCHUNK;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPCAPSTATUS = *mut CAPSTATUS;
#[cfg(feature = "shared")]
pub type LPCAPTUREPARMS = *mut CAPTUREPARMS;
#[cfg(feature = "shared")]
pub type LPCHANNEL_CAPS = *mut CHANNEL_CAPS;
#[cfg(feature = "shared")]
pub type LPDRAWDIBTIME = *mut DRAWDIBTIME;
pub type LPHVIDEO = *mut *mut HVIDEO__;
#[cfg(feature = "shared")]
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
pub const MCIWND_WINDOW_CLASS: windows_sys::core::PCSTR = windows_sys::core::s!("MCIWndClass");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MainAVIHeader {
    pub dwMicroSecPerFrame: super::DWORD,
    pub dwMaxBytesPerSec: super::DWORD,
    pub dwPaddingGranularity: super::DWORD,
    pub dwFlags: super::DWORD,
    pub dwTotalFrames: super::DWORD,
    pub dwInitialFrames: super::DWORD,
    pub dwStreams: super::DWORD,
    pub dwSuggestedBufferSize: super::DWORD,
    pub dwWidth: super::DWORD,
    pub dwHeight: super::DWORD,
    pub dwReserved: [super::DWORD; 4],
}
#[cfg(feature = "shared")]
impl Default for MainAVIHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PAVIEDITSTREAM = *mut core::ffi::c_void;
pub type PAVIFILE = *mut core::ffi::c_void;
#[cfg(feature = "objidl")]
pub type PAVIPERSISTFILE = *mut core::ffi::c_void;
pub type PAVISTREAM = *mut core::ffi::c_void;
pub type PAVISTREAMING = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCAPDRIVERCAPS = *mut CAPDRIVERCAPS;
#[cfg(all(feature = "minwindef", feature = "mmiscapi", feature = "shared"))]
pub type PCAPINFOCHUNK = *mut CAPINFOCHUNK;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type PCAPSTATUS = *mut CAPSTATUS;
#[cfg(feature = "shared")]
pub type PCAPTUREPARMS = *mut CAPTUREPARMS;
#[cfg(feature = "shared")]
pub type PCHANNEL_CAPS = *mut CHANNEL_CAPS;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wingdi"))]
pub type PCOMPVARS = *mut COMPVARS;
pub const PD_CAN_DRAW_DIB: i32 = 1;
pub const PD_CAN_STRETCHDIB: i32 = 2;
pub const PD_STRETCHDIB_1_1_OK: i32 = 4;
pub const PD_STRETCHDIB_1_2_OK: i32 = 8;
pub const PD_STRETCHDIB_1_N_OK: i32 = 16;
pub type PGETFRAME = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
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
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VIDEOHDR {
    pub lpData: super::LPBYTE,
    pub dwBufferLength: super::DWORD,
    pub dwBytesUsed: super::DWORD,
    pub dwTimeCaptured: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub dwFlags: super::DWORD,
    pub dwReserved: [super::DWORD_PTR; 4],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
pub const ckidAVIMAINHDR: super::DWORD = 1751742049;
#[cfg(feature = "shared")]
pub const ckidAVINEWINDEX: super::DWORD = 829973609;
#[cfg(feature = "shared")]
pub const ckidAVIPADDING: super::DWORD = 1263424842;
#[cfg(feature = "shared")]
pub const ckidSTREAMFORMAT: super::DWORD = 1718776947;
#[cfg(feature = "shared")]
pub const ckidSTREAMHANDLERDATA: super::DWORD = 1685222515;
#[cfg(feature = "shared")]
pub const ckidSTREAMHEADER: super::DWORD = 1752331379;
#[cfg(feature = "shared")]
pub const ckidSTREAMNAME: super::DWORD = 1852994675;
pub const cktypeDIBbits: i32 = 25188;
pub const cktypeDIBcompressed: i32 = 25444;
pub const cktypePALchange: i32 = 25456;
pub const cktypeWAVEbytes: i32 = 25207;
#[cfg(feature = "shared")]
pub const comptypeDIB: super::DWORD = 541215044;
#[cfg(feature = "shared")]
pub const formtypeAVI: super::DWORD = 541677121;
#[cfg(feature = "shared")]
pub const infotypeDIGITIZATION_TIME: super::DWORD = 1414087753;
#[cfg(feature = "shared")]
pub const infotypeSMPTE_TIME: super::DWORD = 1347244873;
#[cfg(feature = "shared")]
pub const listtypeAVIHEADER: super::DWORD = 1819436136;
#[cfg(feature = "shared")]
pub const listtypeAVIMOVIE: super::DWORD = 1769369453;
#[cfg(feature = "shared")]
pub const listtypeAVIRECORD: super::DWORD = 543384946;
#[cfg(feature = "shared")]
pub const listtypeSTREAMHEADER: super::DWORD = 1819440243;
#[cfg(feature = "shared")]
pub const streamtypeAUDIO: super::DWORD = 1935963489;
#[cfg(feature = "shared")]
pub const streamtypeMIDI: super::DWORD = 1935960429;
#[cfg(feature = "shared")]
pub const streamtypeTEXT: super::DWORD = 1937012852;
#[cfg(feature = "shared")]
pub const streamtypeVIDEO: super::DWORD = 1935960438;
