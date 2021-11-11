#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIBuildFilterA(lpszfilter: super::super::Foundation::PSTR, cbfilter: i32, fsaving: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIBuildFilterW(lpszfilter: super::super::Foundation::PWSTR, cbfilter: i32, fsaving: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIClearClipboard() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileAddRef(pfile: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileCreateStreamA(pfile: ::windows::runtime::RawPtr, ppavi: *mut ::windows::runtime::RawPtr, psi: *const AVISTREAMINFOA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileCreateStreamW(pfile: ::windows::runtime::RawPtr, ppavi: *mut ::windows::runtime::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileEndRecord(pfile: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileExit();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileGetStream(pfile: ::windows::runtime::RawPtr, ppavi: *mut ::windows::runtime::RawPtr, fcctype: u32, lparam: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileInfoA(pfile: ::windows::runtime::RawPtr, pfi: *mut AVIFILEINFOA, lsize: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileInfoW(pfile: ::windows::runtime::RawPtr, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileInit();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileOpenA(ppfile: *mut ::windows::runtime::RawPtr, szfile: super::super::Foundation::PSTR, umode: u32, lphandler: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileOpenW(ppfile: *mut ::windows::runtime::RawPtr, szfile: super::super::Foundation::PWSTR, umode: u32, lphandler: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileReadData(pfile: ::windows::runtime::RawPtr, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileRelease(pfile: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileWriteData(pfile: ::windows::runtime::RawPtr, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIGetFromClipboard(lppf: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIMakeCompressedStream(ppscompressed: *mut ::windows::runtime::RawPtr, ppssource: ::windows::runtime::RawPtr, lpoptions: *const AVICOMPRESSOPTIONS, pclsidhandler: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIMakeFileFromStreams(ppfile: *mut ::windows::runtime::RawPtr, nstreams: i32, papstreams: *const ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIMakeStreamFromClipboard(cfformat: u32, hglobal: super::super::Foundation::HANDLE, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIPutFileOnClipboard(pf: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveA(szfile: super::super::Foundation::PSTR, pclsidhandler: *const ::windows::runtime::GUID, lpfncallback: ::windows::runtime::RawPtr, nstreams: i32, pfile: ::windows::runtime::RawPtr, lpoptions: *const AVICOMPRESSOPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveOptions(hwnd: super::super::Foundation::HWND, uiflags: u32, nstreams: i32, ppavi: *const ::windows::runtime::RawPtr, plpoptions: *mut *mut AVICOMPRESSOPTIONS) -> isize;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVISaveOptionsFree(nstreams: i32, plpoptions: *const *const AVICOMPRESSOPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveVA(szfile: super::super::Foundation::PSTR, pclsidhandler: *const ::windows::runtime::GUID, lpfncallback: ::windows::runtime::RawPtr, nstreams: i32, ppavi: *const ::windows::runtime::RawPtr, plpoptions: *const *const AVICOMPRESSOPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveVW(szfile: super::super::Foundation::PWSTR, pclsidhandler: *const ::windows::runtime::GUID, lpfncallback: ::windows::runtime::RawPtr, nstreams: i32, ppavi: *const ::windows::runtime::RawPtr, plpoptions: *const *const AVICOMPRESSOPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveW(szfile: super::super::Foundation::PWSTR, pclsidhandler: *const ::windows::runtime::GUID, lpfncallback: ::windows::runtime::RawPtr, nstreams: i32, pfile: ::windows::runtime::RawPtr, lpoptions: *const AVICOMPRESSOPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamAddRef(pavi: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamBeginStreaming(pavi: ::windows::runtime::RawPtr, lstart: i32, lend: i32, lrate: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamCreate(ppavi: *mut ::windows::runtime::RawPtr, lparam1: i32, lparam2: i32, pclsidhandler: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamEndStreaming(pavi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamFindSample(pavi: ::windows::runtime::RawPtr, lpos: i32, lflags: i32) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamGetFrame(pg: ::windows::runtime::RawPtr, lpos: i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamGetFrameClose(pg: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AVIStreamGetFrameOpen(pavi: ::windows::runtime::RawPtr, lpbiwanted: *const super::super::Graphics::Gdi::BITMAPINFOHEADER) -> ::core::option::Option<IGetFrame>;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamInfoA(pavi: ::windows::runtime::RawPtr, psi: *mut AVISTREAMINFOA, lsize: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamInfoW(pavi: ::windows::runtime::RawPtr, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamLength(pavi: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamOpenFromFileA(ppavi: *mut ::windows::runtime::RawPtr, szfile: super::super::Foundation::PSTR, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamOpenFromFileW(ppavi: *mut ::windows::runtime::RawPtr, szfile: super::super::Foundation::PWSTR, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamRead(pavi: ::windows::runtime::RawPtr, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamReadData(pavi: ::windows::runtime::RawPtr, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamReadFormat(pavi: ::windows::runtime::RawPtr, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamRelease(pavi: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamSampleToTime(pavi: ::windows::runtime::RawPtr, lsample: i32) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamSetFormat(pavi: ::windows::runtime::RawPtr, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamStart(pavi: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamTimeToSample(pavi: ::windows::runtime::RawPtr, ltime: i32) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamWrite(pavi: ::windows::runtime::RawPtr, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamWriteData(pavi: ::windows::runtime::RawPtr, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDriver(hdriver: HDRVR, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn CreateEditableStream(ppseditable: *mut ::windows::runtime::RawPtr, pssource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefDriverProc(dwdriveridentifier: usize, hdrvr: HDRVR, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibBegin(hdd: isize, hdc: super::super::Graphics::Gdi::HDC, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, dxsrc: i32, dysrc: i32, wflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibChangePalette(hdd: isize, istart: i32, ilen: i32, lppe: *const super::super::Graphics::Gdi::PALETTEENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibClose(hdd: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibDraw(hdd: isize, hdc: super::super::Graphics::Gdi::HDC, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, wflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibEnd(hdd: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DrawDibGetBuffer(hdd: isize, lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, dwsize: u32, dwflags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DrawDibGetPalette(hdd: isize) -> super::super::Graphics::Gdi::HPALETTE;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn DrawDibOpen() -> isize;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibProfileDisplay(lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibRealize(hdd: isize, hdc: super::super::Graphics::Gdi::HDC, fbackground: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibSetPalette(hdd: isize, hpal: super::super::Graphics::Gdi::HPALETTE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibStart(hdd: isize, rate: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibStop(hdd: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibTime(hdd: isize, lpddtime: *mut DRAWDIBTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DriverCallback(dwcallback: usize, dwflags: u32, hdevice: HDRVR, dwmsg: u32, dwuser: usize, dwparam1: usize, dwparam2: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrvGetModuleHandle(hdriver: HDRVR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamClone(pavi: ::windows::runtime::RawPtr, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamCopy(pavi: ::windows::runtime::RawPtr, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamCut(pavi: ::windows::runtime::RawPtr, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamPaste(pavi: ::windows::runtime::RawPtr, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::runtime::RawPtr, lstart: i32, lend: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetInfoA(pavi: ::windows::runtime::RawPtr, lpinfo: *const AVISTREAMINFOA, cbinfo: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetInfoW(pavi: ::windows::runtime::RawPtr, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetNameA(pavi: ::windows::runtime::RawPtr, lpszname: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetNameW(pavi: ::windows::runtime::RawPtr, lpszname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDriverModuleHandle(hdriver: HDRVR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetOpenFileNamePreviewA(lpofn: *mut ::core::mem::ManuallyDrop<super::super::UI::Controls::Dialogs::OPENFILENAMEA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetOpenFileNamePreviewW(lpofn: *mut ::core::mem::ManuallyDrop<super::super::UI::Controls::Dialogs::OPENFILENAMEW>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetSaveFileNamePreviewA(lpofn: *mut ::core::mem::ManuallyDrop<super::super::UI::Controls::Dialogs::OPENFILENAMEA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetSaveFileNamePreviewW(lpofn: *mut ::core::mem::ManuallyDrop<super::super::UI::Controls::Dialogs::OPENFILENAMEW>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICClose(hic: HIC) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICCompress(hic: HIC, dwflags: u32, lpbioutput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpdata: *mut ::core::ffi::c_void, lpbiinput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, lpckid: *mut u32, lpdwflags: *mut u32, lframenum: i32, dwframesize: u32, dwquality: u32, lpbiprev: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpprev: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICCompressorChoose(hwnd: super::super::Foundation::HWND, uiflags: u32, pvin: *const ::core::ffi::c_void, lpdata: *const ::core::ffi::c_void, pc: *mut COMPVARS, lpsztitle: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICCompressorFree(pc: *const COMPVARS);
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICDecompress(hic: HIC, dwflags: u32, lpbiformat: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpdata: *const ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn ICDraw(hic: HIC, dwflags: u32, lpformat: *const ::core::ffi::c_void, lpdata: *const ::core::ffi::c_void, cbdata: u32, ltime: i32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICDrawBegin(hic: HIC, dwflags: u32, hpal: super::super::Graphics::Gdi::HPALETTE, hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, dwrate: u32, dwscale: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICGetDisplayFormat(hic: HIC, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbiout: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, bitdepth: i32, dx: i32, dy: i32) -> HIC;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICGetInfo(hic: HIC, picinfo: *mut ICINFO, cb: u32) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICImageCompress(hic: HIC, uiflags: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO, lpbits: *const ::core::ffi::c_void, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO, lquality: i32, plsize: *mut i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICImageDecompress(hic: HIC, uiflags: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO, lpbits: *const ::core::ffi::c_void, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICInfo(fcctype: u32, fcchandler: u32, lpicinfo: *mut ICINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICInstall(fcctype: u32, fcchandler: u32, lparam: super::super::Foundation::LPARAM, szdesc: super::super::Foundation::PSTR, wflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICLocate(fcctype: u32, fcchandler: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, wflags: u16) -> HIC;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn ICOpen(fcctype: u32, fcchandler: u32, wmode: u32) -> HIC;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICOpenFunction(fcctype: u32, fcchandler: u32, wmode: u32, lpfnhandler: ::windows::runtime::RawPtr) -> HIC;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICRemove(fcctype: u32, fcchandler: u32, wflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICSendMessage(hic: HIC, msg: u32, dw1: usize, dw2: usize) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICSeqCompressFrame(pc: *const COMPVARS, uiflags: u32, lpbits: *const ::core::ffi::c_void, pfkey: *mut super::super::Foundation::BOOL, plsize: *mut i32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICSeqCompressFrameEnd(pc: *const COMPVARS);
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICSeqCompressFrameStart(pc: *const COMPVARS, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MCIWndCreateA(hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, dwstyle: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MCIWndCreateW(hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, dwstyle: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MCIWndRegisterClass() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDriver(szdrivername: super::super::Foundation::PWSTR, szsectionname: super::super::Foundation::PWSTR, lparam2: super::super::Foundation::LPARAM) -> HDRVR;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendDriverMessage(hdriver: HDRVR, message: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn VideoForWindowsVersion() -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capCreateCaptureWindowA(lpszwindowname: super::super::Foundation::PSTR, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, nid: i32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capCreateCaptureWindowW(lpszwindowname: super::super::Foundation::PWSTR, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, nid: i32) -> super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capGetDriverDescriptionA(wdriverindex: u32, lpszname: super::super::Foundation::PSTR, cbname: i32, lpszver: super::super::Foundation::PSTR, cbver: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capGetDriverDescriptionW(wdriverindex: u32, lpszname: super::super::Foundation::PWSTR, cbname: i32, lpszver: super::super::Foundation::PWSTR, cbver: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn joyGetDevCapsA(ujoyid: usize, pjc: *mut JOYCAPSA, cbjc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetDevCapsW(ujoyid: usize, pjc: *mut JOYCAPSW, cbjc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetPos(ujoyid: u32, pji: *mut JOYINFO) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetPosEx(ujoyid: u32, pji: *mut JOYINFOEX) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetThreshold(ujoyid: u32, puthreshold: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyReleaseCapture(ujoyid: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn joySetCapture(hwnd: super::super::Foundation::HWND, ujoyid: u32, uperiod: u32, fchanged: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joySetThreshold(ujoyid: u32, uthreshold: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciDriverNotify(hwndcallback: super::super::Foundation::HANDLE, wdeviceid: u32, ustatus: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciDriverYield(wdeviceid: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciFreeCommandResource(wtable: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciGetCreatorTask(mciid: u32) -> super::HTASK;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDA(pszdevice: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDFromElementIDA(dwelementid: u32, lpstrtype: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDFromElementIDW(dwelementid: u32, lpstrtype: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDW(pszdevice: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciGetDriverData(wdeviceid: u32) -> usize;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetErrorStringA(mcierr: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetErrorStringW(mcierr: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciGetYieldProc(mciid: u32, pdwyielddata: *const u32) -> ::core::option::Option<YIELDPROC>;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciLoadCommandResource(hinstance: super::super::Foundation::HANDLE, lpresname: super::super::Foundation::PWSTR, wtype: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciSendCommandA(mciid: u32, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciSendCommandW(mciid: u32, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSendStringA(lpstrcommand: super::super::Foundation::PSTR, lpstrreturnstring: super::super::Foundation::PSTR, ureturnlength: u32, hwndcallback: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSendStringW(lpstrcommand: super::super::Foundation::PWSTR, lpstrreturnstring: super::super::Foundation::PWSTR, ureturnlength: u32, hwndcallback: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSetDriverData(wdeviceid: u32, dwdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSetYieldProc(mciid: u32, fpyieldproc: ::windows::runtime::RawPtr, dwyielddata: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmDrvInstall(hdriver: HDRVR, wszdrventry: super::super::Foundation::PWSTR, drvmessage: ::windows::runtime::RawPtr, wflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmGetCurrentTask() -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmTaskBlock(h: u32);
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmTaskCreate(lpfn: ::windows::runtime::RawPtr, lph: *mut super::super::Foundation::HANDLE, dwinst: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmTaskSignal(h: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmTaskYield();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioAdvance(hmmio: HMMIO, pmmioinfo: *const ::core::mem::ManuallyDrop<MMIOINFO>, fuadvance: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioAscend(hmmio: HMMIO, pmmcki: *const MMCKINFO, fuascend: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioClose(hmmio: HMMIO, fuclose: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioCreateChunk(hmmio: HMMIO, pmmcki: *const MMCKINFO, fucreate: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioDescend(hmmio: HMMIO, pmmcki: *mut MMCKINFO, pmmckiparent: *const MMCKINFO, fudescend: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioFlush(hmmio: HMMIO, fuflush: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioGetInfo(hmmio: HMMIO, pmmioinfo: *mut ::core::mem::ManuallyDrop<MMIOINFO>, fuinfo: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioInstallIOProcA(fccioproc: u32, pioproc: ::windows::runtime::RawPtr, dwflags: u32) -> ::core::option::Option<LPMMIOPROC>;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioInstallIOProcW(fccioproc: u32, pioproc: ::windows::runtime::RawPtr, dwflags: u32) -> ::core::option::Option<LPMMIOPROC>;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioOpenA(pszfilename: super::super::Foundation::PSTR, pmmioinfo: *mut ::core::mem::ManuallyDrop<MMIOINFO>, fdwopen: u32) -> HMMIO;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioOpenW(pszfilename: super::super::Foundation::PWSTR, pmmioinfo: *mut ::core::mem::ManuallyDrop<MMIOINFO>, fdwopen: u32) -> HMMIO;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioRead(hmmio: HMMIO, pch: *mut i8, cch: i32) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioRenameA(pszfilename: super::super::Foundation::PSTR, psznewfilename: super::super::Foundation::PSTR, pmmioinfo: *const ::core::mem::ManuallyDrop<MMIOINFO>, fdwrename: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioRenameW(pszfilename: super::super::Foundation::PWSTR, psznewfilename: super::super::Foundation::PWSTR, pmmioinfo: *const ::core::mem::ManuallyDrop<MMIOINFO>, fdwrename: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioSeek(hmmio: HMMIO, loffset: i32, iorigin: i32) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioSendMessage(hmmio: HMMIO, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioSetBuffer(hmmio: HMMIO, pchbuffer: super::super::Foundation::PSTR, cchbuffer: i32, fubuffer: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioSetInfo(hmmio: HMMIO, pmmioinfo: *const ::core::mem::ManuallyDrop<MMIOINFO>, fuinfo: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioStringToFOURCCA(sz: super::super::Foundation::PSTR, uflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioStringToFOURCCW(sz: super::super::Foundation::PWSTR, uflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioWrite(hmmio: HMMIO, pch: super::super::Foundation::PSTR, cch: i32) -> i32;
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndOpenSound(eventname: super::super::Foundation::PWSTR, appname: super::super::Foundation::PWSTR, flags: i32, filehandle: *mut super::super::Foundation::HANDLE) -> i32;
}
