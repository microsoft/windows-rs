impl ::core::default::Default for ADPCMCOEFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for ADPCMEWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for APTXWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for AUDIOFILE_AF10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for AUDIOFILE_AF36WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for AVICOMPRESSOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AVICOMPRESSOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType && self.fccHandler == other.fccHandler && self.dwKeyFrameEvery == other.dwKeyFrameEvery && self.dwQuality == other.dwQuality && self.dwBytesPerSecond == other.dwBytesPerSecond && self.dwFlags == other.dwFlags && self.lpFormat == other.lpFormat && self.cbFormat == other.cbFormat && self.lpParms == other.lpParms && self.cbParms == other.cbParms && self.dwInterleaveEvery == other.dwInterleaveEvery
    }
}
impl ::core::cmp::Eq for AVICOMPRESSOPTIONS {}
impl ::core::fmt::Debug for AVICOMPRESSOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVICOMPRESSOPTIONS").field("fccType", &self.fccType).field("fccHandler", &self.fccHandler).field("dwKeyFrameEvery", &self.dwKeyFrameEvery).field("dwQuality", &self.dwQuality).field("dwBytesPerSecond", &self.dwBytesPerSecond).field("dwFlags", &self.dwFlags).field("lpFormat", &self.lpFormat).field("cbFormat", &self.cbFormat).field("lpParms", &self.lpParms).field("cbParms", &self.cbParms).field("dwInterleaveEvery", &self.dwInterleaveEvery).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AVIFILEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AVIFILEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxBytesPerSec == other.dwMaxBytesPerSec && self.dwFlags == other.dwFlags && self.dwCaps == other.dwCaps && self.dwStreams == other.dwStreams && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwScale == other.dwScale && self.dwRate == other.dwRate && self.dwLength == other.dwLength && self.dwEditCount == other.dwEditCount && self.szFileType == other.szFileType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AVIFILEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AVIFILEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVIFILEINFOA")
            .field("dwMaxBytesPerSec", &self.dwMaxBytesPerSec)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("dwStreams", &self.dwStreams)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwLength", &self.dwLength)
            .field("dwEditCount", &self.dwEditCount)
            .field("szFileType", &self.szFileType)
            .finish()
    }
}
impl ::core::default::Default for AVIFILEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AVIFILEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxBytesPerSec == other.dwMaxBytesPerSec && self.dwFlags == other.dwFlags && self.dwCaps == other.dwCaps && self.dwStreams == other.dwStreams && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwScale == other.dwScale && self.dwRate == other.dwRate && self.dwLength == other.dwLength && self.dwEditCount == other.dwEditCount && self.szFileType == other.szFileType
    }
}
impl ::core::cmp::Eq for AVIFILEINFOW {}
impl ::core::fmt::Debug for AVIFILEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVIFILEINFOW")
            .field("dwMaxBytesPerSec", &self.dwMaxBytesPerSec)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("dwStreams", &self.dwStreams)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwLength", &self.dwLength)
            .field("dwEditCount", &self.dwEditCount)
            .field("szFileType", &self.szFileType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AVISTREAMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AVISTREAMINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType && self.fccHandler == other.fccHandler && self.dwFlags == other.dwFlags && self.dwCaps == other.dwCaps && self.wPriority == other.wPriority && self.wLanguage == other.wLanguage && self.dwScale == other.dwScale && self.dwRate == other.dwRate && self.dwStart == other.dwStart && self.dwLength == other.dwLength && self.dwInitialFrames == other.dwInitialFrames && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize && self.dwQuality == other.dwQuality && self.dwSampleSize == other.dwSampleSize && self.rcFrame == other.rcFrame && self.dwEditCount == other.dwEditCount && self.dwFormatChangeCount == other.dwFormatChangeCount && self.szName == other.szName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AVISTREAMINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AVISTREAMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVISTREAMINFOA")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("wPriority", &self.wPriority)
            .field("wLanguage", &self.wLanguage)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwStart", &self.dwStart)
            .field("dwLength", &self.dwLength)
            .field("dwInitialFrames", &self.dwInitialFrames)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwQuality", &self.dwQuality)
            .field("dwSampleSize", &self.dwSampleSize)
            .field("rcFrame", &self.rcFrame)
            .field("dwEditCount", &self.dwEditCount)
            .field("dwFormatChangeCount", &self.dwFormatChangeCount)
            .field("szName", &self.szName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AVISTREAMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AVISTREAMINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType && self.fccHandler == other.fccHandler && self.dwFlags == other.dwFlags && self.dwCaps == other.dwCaps && self.wPriority == other.wPriority && self.wLanguage == other.wLanguage && self.dwScale == other.dwScale && self.dwRate == other.dwRate && self.dwStart == other.dwStart && self.dwLength == other.dwLength && self.dwInitialFrames == other.dwInitialFrames && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize && self.dwQuality == other.dwQuality && self.dwSampleSize == other.dwSampleSize && self.rcFrame == other.rcFrame && self.dwEditCount == other.dwEditCount && self.dwFormatChangeCount == other.dwFormatChangeCount && self.szName == other.szName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AVISTREAMINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AVISTREAMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVISTREAMINFOW")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("wPriority", &self.wPriority)
            .field("wLanguage", &self.wLanguage)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwStart", &self.dwStart)
            .field("dwLength", &self.dwLength)
            .field("dwInitialFrames", &self.dwInitialFrames)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwQuality", &self.dwQuality)
            .field("dwSampleSize", &self.dwSampleSize)
            .field("rcFrame", &self.rcFrame)
            .field("dwEditCount", &self.dwEditCount)
            .field("dwFormatChangeCount", &self.dwFormatChangeCount)
            .field("szName", &self.szName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAPDRIVERCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAPDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wDeviceIndex == other.wDeviceIndex && self.fHasOverlay == other.fHasOverlay && self.fHasDlgVideoSource == other.fHasDlgVideoSource && self.fHasDlgVideoFormat == other.fHasDlgVideoFormat && self.fHasDlgVideoDisplay == other.fHasDlgVideoDisplay && self.fCaptureInitialized == other.fCaptureInitialized && self.fDriverSuppliesPalettes == other.fDriverSuppliesPalettes && self.hVideoIn == other.hVideoIn && self.hVideoOut == other.hVideoOut && self.hVideoExtIn == other.hVideoExtIn && self.hVideoExtOut == other.hVideoExtOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAPDRIVERCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CAPDRIVERCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPDRIVERCAPS")
            .field("wDeviceIndex", &self.wDeviceIndex)
            .field("fHasOverlay", &self.fHasOverlay)
            .field("fHasDlgVideoSource", &self.fHasDlgVideoSource)
            .field("fHasDlgVideoFormat", &self.fHasDlgVideoFormat)
            .field("fHasDlgVideoDisplay", &self.fHasDlgVideoDisplay)
            .field("fCaptureInitialized", &self.fCaptureInitialized)
            .field("fDriverSuppliesPalettes", &self.fDriverSuppliesPalettes)
            .field("hVideoIn", &self.hVideoIn)
            .field("hVideoOut", &self.hVideoOut)
            .field("hVideoExtIn", &self.hVideoExtIn)
            .field("hVideoExtOut", &self.hVideoExtOut)
            .finish()
    }
}
impl ::core::default::Default for CAPINFOCHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CAPINFOCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.fccInfoID == other.fccInfoID && self.lpData == other.lpData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for CAPINFOCHUNK {}
impl ::core::fmt::Debug for CAPINFOCHUNK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPINFOCHUNK").field("fccInfoID", &self.fccInfoID).field("lpData", &self.lpData).field("cbData", &self.cbData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CAPSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CAPSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.uiImageWidth == other.uiImageWidth
            && self.uiImageHeight == other.uiImageHeight
            && self.fLiveWindow == other.fLiveWindow
            && self.fOverlayWindow == other.fOverlayWindow
            && self.fScale == other.fScale
            && self.ptScroll == other.ptScroll
            && self.fUsingDefaultPalette == other.fUsingDefaultPalette
            && self.fAudioHardware == other.fAudioHardware
            && self.fCapFileExists == other.fCapFileExists
            && self.dwCurrentVideoFrame == other.dwCurrentVideoFrame
            && self.dwCurrentVideoFramesDropped == other.dwCurrentVideoFramesDropped
            && self.dwCurrentWaveSamples == other.dwCurrentWaveSamples
            && self.dwCurrentTimeElapsedMS == other.dwCurrentTimeElapsedMS
            && self.hPalCurrent == other.hPalCurrent
            && self.fCapturingNow == other.fCapturingNow
            && self.dwReturn == other.dwReturn
            && self.wNumVideoAllocated == other.wNumVideoAllocated
            && self.wNumAudioAllocated == other.wNumAudioAllocated
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CAPSTATUS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CAPSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPSTATUS")
            .field("uiImageWidth", &self.uiImageWidth)
            .field("uiImageHeight", &self.uiImageHeight)
            .field("fLiveWindow", &self.fLiveWindow)
            .field("fOverlayWindow", &self.fOverlayWindow)
            .field("fScale", &self.fScale)
            .field("ptScroll", &self.ptScroll)
            .field("fUsingDefaultPalette", &self.fUsingDefaultPalette)
            .field("fAudioHardware", &self.fAudioHardware)
            .field("fCapFileExists", &self.fCapFileExists)
            .field("dwCurrentVideoFrame", &self.dwCurrentVideoFrame)
            .field("dwCurrentVideoFramesDropped", &self.dwCurrentVideoFramesDropped)
            .field("dwCurrentWaveSamples", &self.dwCurrentWaveSamples)
            .field("dwCurrentTimeElapsedMS", &self.dwCurrentTimeElapsedMS)
            .field("hPalCurrent", &self.hPalCurrent)
            .field("fCapturingNow", &self.fCapturingNow)
            .field("dwReturn", &self.dwReturn)
            .field("wNumVideoAllocated", &self.wNumVideoAllocated)
            .field("wNumAudioAllocated", &self.wNumAudioAllocated)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAPTUREPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAPTUREPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRequestMicroSecPerFrame == other.dwRequestMicroSecPerFrame
            && self.fMakeUserHitOKToCapture == other.fMakeUserHitOKToCapture
            && self.wPercentDropForError == other.wPercentDropForError
            && self.fYield == other.fYield
            && self.dwIndexSize == other.dwIndexSize
            && self.wChunkGranularity == other.wChunkGranularity
            && self.fUsingDOSMemory == other.fUsingDOSMemory
            && self.wNumVideoRequested == other.wNumVideoRequested
            && self.fCaptureAudio == other.fCaptureAudio
            && self.wNumAudioRequested == other.wNumAudioRequested
            && self.vKeyAbort == other.vKeyAbort
            && self.fAbortLeftMouse == other.fAbortLeftMouse
            && self.fAbortRightMouse == other.fAbortRightMouse
            && self.fLimitEnabled == other.fLimitEnabled
            && self.wTimeLimit == other.wTimeLimit
            && self.fMCIControl == other.fMCIControl
            && self.fStepMCIDevice == other.fStepMCIDevice
            && self.dwMCIStartTime == other.dwMCIStartTime
            && self.dwMCIStopTime == other.dwMCIStopTime
            && self.fStepCaptureAt2x == other.fStepCaptureAt2x
            && self.wStepCaptureAverageFrames == other.wStepCaptureAverageFrames
            && self.dwAudioBufferSize == other.dwAudioBufferSize
            && self.fDisableWriteCache == other.fDisableWriteCache
            && self.AVStreamMaster == other.AVStreamMaster
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAPTUREPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CAPTUREPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPTUREPARMS")
            .field("dwRequestMicroSecPerFrame", &self.dwRequestMicroSecPerFrame)
            .field("fMakeUserHitOKToCapture", &self.fMakeUserHitOKToCapture)
            .field("wPercentDropForError", &self.wPercentDropForError)
            .field("fYield", &self.fYield)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("wChunkGranularity", &self.wChunkGranularity)
            .field("fUsingDOSMemory", &self.fUsingDOSMemory)
            .field("wNumVideoRequested", &self.wNumVideoRequested)
            .field("fCaptureAudio", &self.fCaptureAudio)
            .field("wNumAudioRequested", &self.wNumAudioRequested)
            .field("vKeyAbort", &self.vKeyAbort)
            .field("fAbortLeftMouse", &self.fAbortLeftMouse)
            .field("fAbortRightMouse", &self.fAbortRightMouse)
            .field("fLimitEnabled", &self.fLimitEnabled)
            .field("wTimeLimit", &self.wTimeLimit)
            .field("fMCIControl", &self.fMCIControl)
            .field("fStepMCIDevice", &self.fStepMCIDevice)
            .field("dwMCIStartTime", &self.dwMCIStartTime)
            .field("dwMCIStopTime", &self.dwMCIStopTime)
            .field("fStepCaptureAt2x", &self.fStepCaptureAt2x)
            .field("wStepCaptureAverageFrames", &self.wStepCaptureAverageFrames)
            .field("dwAudioBufferSize", &self.dwAudioBufferSize)
            .field("fDisableWriteCache", &self.fDisableWriteCache)
            .field("AVStreamMaster", &self.AVStreamMaster)
            .finish()
    }
}
impl ::core::default::Default for CHANNEL_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANNEL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwSrcRectXMod == other.dwSrcRectXMod && self.dwSrcRectYMod == other.dwSrcRectYMod && self.dwSrcRectWidthMod == other.dwSrcRectWidthMod && self.dwSrcRectHeightMod == other.dwSrcRectHeightMod && self.dwDstRectXMod == other.dwDstRectXMod && self.dwDstRectYMod == other.dwDstRectYMod && self.dwDstRectWidthMod == other.dwDstRectWidthMod && self.dwDstRectHeightMod == other.dwDstRectHeightMod
    }
}
impl ::core::cmp::Eq for CHANNEL_CAPS {}
impl ::core::fmt::Debug for CHANNEL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_CAPS").field("dwFlags", &self.dwFlags).field("dwSrcRectXMod", &self.dwSrcRectXMod).field("dwSrcRectYMod", &self.dwSrcRectYMod).field("dwSrcRectWidthMod", &self.dwSrcRectWidthMod).field("dwSrcRectHeightMod", &self.dwSrcRectHeightMod).field("dwDstRectXMod", &self.dwDstRectXMod).field("dwDstRectYMod", &self.dwDstRectYMod).field("dwDstRectWidthMod", &self.dwDstRectWidthMod).field("dwDstRectHeightMod", &self.dwDstRectHeightMod).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for COMPVARS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for COMPVARS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.hic == other.hic && self.fccType == other.fccType && self.fccHandler == other.fccHandler && self.lpbiIn == other.lpbiIn && self.lpbiOut == other.lpbiOut && self.lpBitsOut == other.lpBitsOut && self.lpBitsPrev == other.lpBitsPrev && self.lFrame == other.lFrame && self.lKey == other.lKey && self.lDataRate == other.lDataRate && self.lQ == other.lQ && self.lKeyCount == other.lKeyCount && self.lpState == other.lpState && self.cbState == other.cbState
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for COMPVARS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for COMPVARS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPVARS")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("hic", &self.hic)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("lpbiIn", &self.lpbiIn)
            .field("lpbiOut", &self.lpbiOut)
            .field("lpBitsOut", &self.lpBitsOut)
            .field("lpBitsPrev", &self.lpBitsPrev)
            .field("lFrame", &self.lFrame)
            .field("lKey", &self.lKey)
            .field("lDataRate", &self.lDataRate)
            .field("lQ", &self.lQ)
            .field("lKeyCount", &self.lKeyCount)
            .field("lpState", &self.lpState)
            .field("cbState", &self.cbState)
            .finish()
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CONTRESCR10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CONTRESVQLPCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CREATIVEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CSIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIALOGICOKIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGIFIXWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGIREALWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGISTDWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DOLBYAC2WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRAWDIBTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRAWDIBTIME {
    fn eq(&self, other: &Self) -> bool {
        self.timeCount == other.timeCount && self.timeDraw == other.timeDraw && self.timeDecompress == other.timeDecompress && self.timeDither == other.timeDither && self.timeStretch == other.timeStretch && self.timeBlt == other.timeBlt && self.timeSetDIBits == other.timeSetDIBits
    }
}
impl ::core::cmp::Eq for DRAWDIBTIME {}
impl ::core::fmt::Debug for DRAWDIBTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWDIBTIME").field("timeCount", &self.timeCount).field("timeDraw", &self.timeDraw).field("timeDecompress", &self.timeDecompress).field("timeDither", &self.timeDither).field("timeStretch", &self.timeStretch).field("timeBlt", &self.timeBlt).field("timeSetDIBits", &self.timeSetDIBits).finish()
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DRMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRVCONFIGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRVM_IOCTL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DVIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for ECHOSC1WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EXBMINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for FMTOWNS_SND_WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for G721_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for G723_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for GSM610WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IAVIEditStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIEditStream {}
impl ::core::fmt::Debug for IAVIEditStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIEditStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAVIFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIFile {}
impl ::core::fmt::Debug for IAVIFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIFile").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAVIPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAVIPersistFile {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAVIPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIPersistFile").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IAVIPersistFile {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<P0>(&self, pszfilename: P0, dwmode: super::super::System::Com::STGM) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self), pszfilename.into().abi(), dwmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<P0, P1>(&self, pszfilename: P0, fremember: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pszfilename.into().abi(), fremember.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveCompleted)(::windows::core::Vtable::as_raw(self), pszfilename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCurFile(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurFile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IAVIStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIStream {}
impl ::core::fmt::Debug for IAVIStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAVIStreaming {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAVIStreaming {}
impl ::core::fmt::Debug for IAVIStreaming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAVIStreaming").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICCOMPRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICCOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpbiOutput == other.lpbiOutput && self.lpOutput == other.lpOutput && self.lpbiInput == other.lpbiInput && self.lpInput == other.lpInput && self.lpckid == other.lpckid && self.lpdwFlags == other.lpdwFlags && self.lFrameNum == other.lFrameNum && self.dwFrameSize == other.dwFrameSize && self.dwQuality == other.dwQuality && self.lpbiPrev == other.lpbiPrev && self.lpPrev == other.lpPrev
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICCOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICCOMPRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICCOMPRESS").field("dwFlags", &self.dwFlags).field("lpbiOutput", &self.lpbiOutput).field("lpOutput", &self.lpOutput).field("lpbiInput", &self.lpbiInput).field("lpInput", &self.lpInput).field("lpckid", &self.lpckid).field("lpdwFlags", &self.lpdwFlags).field("lFrameNum", &self.lFrameNum).field("dwFrameSize", &self.dwFrameSize).field("dwQuality", &self.dwQuality).field("lpbiPrev", &self.lpbiPrev).field("lpPrev", &self.lpPrev).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICCOMPRESSFRAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICCOMPRESSFRAMES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpbiOutput == other.lpbiOutput && self.lOutput == other.lOutput && self.lpbiInput == other.lpbiInput && self.lInput == other.lInput && self.lStartFrame == other.lStartFrame && self.lFrameCount == other.lFrameCount && self.lQuality == other.lQuality && self.lDataRate == other.lDataRate && self.lKeyRate == other.lKeyRate && self.dwRate == other.dwRate && self.dwScale == other.dwScale && self.dwOverheadPerFrame == other.dwOverheadPerFrame && self.dwReserved2 == other.dwReserved2 && self.GetData == other.GetData && self.PutData == other.PutData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICCOMPRESSFRAMES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICCOMPRESSFRAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICCOMPRESSFRAMES")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lOutput", &self.lOutput)
            .field("lpbiInput", &self.lpbiInput)
            .field("lInput", &self.lInput)
            .field("lStartFrame", &self.lStartFrame)
            .field("lFrameCount", &self.lFrameCount)
            .field("lQuality", &self.lQuality)
            .field("lDataRate", &self.lDataRate)
            .field("lKeyRate", &self.lKeyRate)
            .field("dwRate", &self.dwRate)
            .field("dwScale", &self.dwScale)
            .field("dwOverheadPerFrame", &self.dwOverheadPerFrame)
            .field("dwReserved2", &self.dwReserved2)
            .field("GetData", &self.GetData)
            .field("PutData", &self.PutData)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICDECOMPRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICDECOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpbiInput == other.lpbiInput && self.lpInput == other.lpInput && self.lpbiOutput == other.lpbiOutput && self.lpOutput == other.lpOutput && self.ckid == other.ckid
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICDECOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICDECOMPRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDECOMPRESS").field("dwFlags", &self.dwFlags).field("lpbiInput", &self.lpbiInput).field("lpInput", &self.lpInput).field("lpbiOutput", &self.lpbiOutput).field("lpOutput", &self.lpOutput).field("ckid", &self.ckid).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICDECOMPRESSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICDECOMPRESSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpbiSrc == other.lpbiSrc && self.lpSrc == other.lpSrc && self.lpbiDst == other.lpbiDst && self.lpDst == other.lpDst && self.xDst == other.xDst && self.yDst == other.yDst && self.dxDst == other.dxDst && self.dyDst == other.dyDst && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.dxSrc == other.dxSrc && self.dySrc == other.dySrc
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICDECOMPRESSEX {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICDECOMPRESSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDECOMPRESSEX").field("dwFlags", &self.dwFlags).field("lpbiSrc", &self.lpbiSrc).field("lpSrc", &self.lpSrc).field("lpbiDst", &self.lpbiDst).field("lpDst", &self.lpDst).field("xDst", &self.xDst).field("yDst", &self.yDst).field("dxDst", &self.dxDst).field("dyDst", &self.dyDst).field("xSrc", &self.xSrc).field("ySrc", &self.ySrc).field("dxSrc", &self.dxSrc).field("dySrc", &self.dySrc).finish()
    }
}
impl ::core::default::Default for ICDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ICDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpFormat == other.lpFormat && self.lpData == other.lpData && self.cbData == other.cbData && self.lTime == other.lTime
    }
}
impl ::core::cmp::Eq for ICDRAW {}
impl ::core::fmt::Debug for ICDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAW").field("dwFlags", &self.dwFlags).field("lpFormat", &self.lpFormat).field("lpData", &self.lpData).field("cbData", &self.cbData).field("lTime", &self.lTime).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICDRAWBEGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICDRAWBEGIN {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.hpal == other.hpal && self.hwnd == other.hwnd && self.hdc == other.hdc && self.xDst == other.xDst && self.yDst == other.yDst && self.dxDst == other.dxDst && self.dyDst == other.dyDst && self.lpbi == other.lpbi && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.dxSrc == other.dxSrc && self.dySrc == other.dySrc && self.dwRate == other.dwRate && self.dwScale == other.dwScale
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICDRAWBEGIN {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICDRAWBEGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAWBEGIN").field("dwFlags", &self.dwFlags).field("hpal", &self.hpal).field("hwnd", &self.hwnd).field("hdc", &self.hdc).field("xDst", &self.xDst).field("yDst", &self.yDst).field("dxDst", &self.dxDst).field("dyDst", &self.dyDst).field("lpbi", &self.lpbi).field("xSrc", &self.xSrc).field("ySrc", &self.ySrc).field("dxSrc", &self.dxSrc).field("dySrc", &self.dySrc).field("dwRate", &self.dwRate).field("dwScale", &self.dwScale).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICDRAWSUGGEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICDRAWSUGGEST {
    fn eq(&self, other: &Self) -> bool {
        self.lpbiIn == other.lpbiIn && self.lpbiSuggest == other.lpbiSuggest && self.dxSrc == other.dxSrc && self.dySrc == other.dySrc && self.dxDst == other.dxDst && self.dyDst == other.dyDst && self.hicDecompressor == other.hicDecompressor
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICDRAWSUGGEST {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICDRAWSUGGEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAWSUGGEST").field("lpbiIn", &self.lpbiIn).field("lpbiSuggest", &self.lpbiSuggest).field("dxSrc", &self.dxSrc).field("dySrc", &self.dySrc).field("dxDst", &self.dxDst).field("dyDst", &self.dyDst).field("hicDecompressor", &self.hicDecompressor).finish()
    }
}
impl ::core::default::Default for ICINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ICINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fccType == other.fccType && self.fccHandler == other.fccHandler && self.dwFlags == other.dwFlags && self.dwVersion == other.dwVersion && self.dwVersionICM == other.dwVersionICM && self.szName == other.szName && self.szDescription == other.szDescription && self.szDriver == other.szDriver
    }
}
impl ::core::cmp::Eq for ICINFO {}
impl ::core::fmt::Debug for ICINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICINFO").field("dwSize", &self.dwSize).field("fccType", &self.fccType).field("fccHandler", &self.fccHandler).field("dwFlags", &self.dwFlags).field("dwVersion", &self.dwVersion).field("dwVersionICM", &self.dwVersionICM).field("szName", &self.szName).field("szDescription", &self.szDescription).field("szDriver", &self.szDriver).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ICOPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ICOPEN {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fccType == other.fccType && self.fccHandler == other.fccHandler && self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags && self.dwError == other.dwError && self.pV1Reserved == other.pV1Reserved && self.pV2Reserved == other.pV2Reserved && self.dnDevNode == other.dnDevNode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ICOPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ICOPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICOPEN").field("dwSize", &self.dwSize).field("fccType", &self.fccType).field("fccHandler", &self.fccHandler).field("dwVersion", &self.dwVersion).field("dwFlags", &self.dwFlags).field("dwError", &self.dwError).field("pV1Reserved", &self.pV1Reserved).field("pV2Reserved", &self.pV2Reserved).field("dnDevNode", &self.dnDevNode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.iStart == other.iStart && self.iLen == other.iLen && self.lppe == other.lppe
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICPALETTE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICPALETTE").field("dwFlags", &self.dwFlags).field("iStart", &self.iStart).field("iLen", &self.iLen).field("lppe", &self.lppe).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ICSETSTATUSPROC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ICSETSTATUSPROC {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lParam == other.lParam && self.Status == other.Status
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ICSETSTATUSPROC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ICSETSTATUSPROC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICSETSTATUSPROC").field("dwFlags", &self.dwFlags).field("lParam", &self.lParam).field("Status", &self.Status).finish()
    }
}
impl ::core::cmp::PartialEq for IGetFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetFrame {}
impl ::core::fmt::Debug for IGetFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetFrame").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for IMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOYCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOYCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOYCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOYCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOYINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JPEGINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_ANIM_PLAY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_ANIM_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MCI_ANIM_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_BREAK_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_CAPTURE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_CAPTURE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_COPY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_CUE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_CUT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_DELETE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_INFO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_INFO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_LIST_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_LIST_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_MONITOR_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_PASTE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_QUALITY_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_QUALITY_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RECORD_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_RESERVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_RESERVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RESTORE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RESTORE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_SETAUDIO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_SETAUDIO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_SETVIDEO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_SETVIDEO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_SIGNAL_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_STATUS_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_STATUS_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_DGV_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MCI_DGV_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_GENERIC_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_GETDEVCAPS_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_INFO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_INFO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_LOAD_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_OPEN_DRIVER_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_LOAD_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_PLAY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_RECORD_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SEEK_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SEQ_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_STATUS_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SYSINFO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_SYSINFO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_VD_ESCAPE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_VD_ESCAPE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_VD_PLAY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_VD_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_WAVE_DELETE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_WAVE_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_WAVE_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MCI_WAVE_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for MEDIASPACEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIDIOPENSTRMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for MIXEROPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MMCKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMIOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for MSAUDIO1WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for NMS_VBXADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLICELPWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLIGSMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLIOPRWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLISBCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for SIERRAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for SONARCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TIMEREVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for TRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VIDEOHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEOHDR {
    fn eq(&self, other: &Self) -> bool {
        self.lpData == other.lpData && self.dwBufferLength == other.dwBufferLength && self.dwBytesUsed == other.dwBytesUsed && self.dwTimeCaptured == other.dwTimeCaptured && self.dwUser == other.dwUser && self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for VIDEOHDR {}
impl ::core::fmt::Debug for VIDEOHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEOHDR").field("lpData", &self.lpData).field("dwBufferLength", &self.dwBufferLength).field("dwBytesUsed", &self.dwBytesUsed).field("dwTimeCaptured", &self.dwTimeCaptured).field("dwUser", &self.dwUser).field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for WAVEOPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for WMAUDIO2WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for WMAUDIO3WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for YAMAHA_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for s_RIFFWAVE_inst {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for s_RIFFWAVE_inst {
    fn eq(&self, other: &Self) -> bool {
        self.bUnshiftedNote == other.bUnshiftedNote && self.chFineTune == other.chFineTune && self.chGain == other.chGain && self.bLowNote == other.bLowNote && self.bHighNote == other.bHighNote && self.bLowVelocity == other.bLowVelocity && self.bHighVelocity == other.bHighVelocity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for s_RIFFWAVE_inst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for s_RIFFWAVE_inst {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("s_RIFFWAVE_inst").field("bUnshiftedNote", &self.bUnshiftedNote).field("chFineTune", &self.chFineTune).field("chGain", &self.chGain).field("bLowNote", &self.bLowNote).field("bHighNote", &self.bHighNote).field("bLowVelocity", &self.bLowVelocity).field("bHighVelocity", &self.bHighVelocity).finish()
    }
}
