#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIBuildFilterA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIBuildFilterW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIClearClipboard();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileAddRef();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileCreateStreamA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileCreateStreamW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileEndRecord();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileExit();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileGetStream();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileInfoA();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileInfoW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileInit();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileOpenA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIFileOpenW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileReadData();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileRelease();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIFileWriteData();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIGetFromClipboard();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIMakeCompressedStream();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIMakeFileFromStreams();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIMakeStreamFromClipboard();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIPutFileOnClipboard();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveOptions();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVISaveOptionsFree();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveVA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveVW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVISaveW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamAddRef();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamBeginStreaming();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamCreate();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamEndStreaming();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamFindSample();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamGetFrame();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamGetFrameClose();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AVIStreamGetFrameOpen();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamInfoA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamInfoW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamLength();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamOpenFromFileA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AVIStreamOpenFromFileW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamRead();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamReadData();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamReadFormat();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamRelease();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamSampleToTime();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamSetFormat();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamStart();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamTimeToSample();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamWrite();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn AVIStreamWriteData();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDriver();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn CreateEditableStream();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefDriverProc();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibBegin();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibChangePalette();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibClose();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibDraw();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibEnd();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DrawDibGetBuffer();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DrawDibGetPalette();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn DrawDibOpen();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibProfileDisplay();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibRealize();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawDibSetPalette();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibStart();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibStop();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawDibTime();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DriverCallback();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrvGetModuleHandle();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamClone();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamCopy();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamCut();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn EditStreamPaste();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetInfoA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetInfoW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetNameA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditStreamSetNameW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDriverModuleHandle();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetOpenFileNamePreviewA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetOpenFileNamePreviewW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetSaveFileNamePreviewA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn GetSaveFileNamePreviewW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICClose();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICCompress();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICCompressorChoose();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICCompressorFree();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICDecompress();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn ICDraw();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICDrawBegin();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICGetDisplayFormat();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICGetInfo();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICImageCompress();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICImageDecompress();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICInfo();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICInstall();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICLocate();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn ICOpen();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICOpenFunction();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICRemove();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ICSendMessage();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICSeqCompressFrame();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ICSeqCompressFrameEnd();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ICSeqCompressFrameStart();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MCIWndCreateA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MCIWndCreateW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MCIWndRegisterClass();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDriver();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendDriverMessage();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn VideoForWindowsVersion();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capCreateCaptureWindowA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capCreateCaptureWindowW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capGetDriverDescriptionA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn capGetDriverDescriptionW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn joyGetDevCapsA();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetDevCapsW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetNumDevs();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetPos();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetPosEx();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyGetThreshold();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joyReleaseCapture();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn joySetCapture();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn joySetThreshold();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciDriverNotify();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciDriverYield();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciFreeCommandResource();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciGetCreatorTask();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDFromElementIDA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDFromElementIDW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetDeviceIDW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciGetDriverData();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetErrorStringA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciGetErrorStringW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciGetYieldProc();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciLoadCommandResource();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciSendCommandA();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mciSendCommandW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSendStringA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSendStringW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSetDriverData();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mciSetYieldProc();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmDrvInstall();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmGetCurrentTask();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmTaskBlock();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmTaskCreate();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmTaskSignal();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmTaskYield();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioAdvance();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioAscend();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioClose();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioCreateChunk();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioDescend();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioFlush();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioGetInfo();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioInstallIOProcA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioInstallIOProcW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioOpenA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioOpenW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioRead();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioRenameA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioRenameW();
    #[doc = "*Required features: `Win32_Media_Multimedia`*"]
    pub fn mmioSeek();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioSendMessage();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioSetBuffer();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioSetInfo();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioStringToFOURCCA();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioStringToFOURCCW();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mmioWrite();
    #[doc = "*Required features: `Win32_Media_Multimedia`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndOpenSound();
}
