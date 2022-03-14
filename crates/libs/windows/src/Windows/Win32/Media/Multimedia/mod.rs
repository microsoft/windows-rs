#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACMDM_BASE: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_COPYRIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_DUALCHANNEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_ID_MPEG1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_JOINTSTEREO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_LAYER1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_LAYER2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_LAYER3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_ORIGINALHOME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_PRIVATEBIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_PROTECTIONBIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_SINGLECHANNEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ACM_MPEG_STEREO: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct ADPCMCOEFSET {
    pub iCoef1: i16,
    pub iCoef2: i16,
}
impl ::core::marker::Copy for ADPCMCOEFSET {}
impl ::core::clone::Clone for ADPCMCOEFSET {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ADPCMCOEFSET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ADPCMCOEFSET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADPCMCOEFSET>()) == 0 }
    }
}
impl ::core::cmp::Eq for ADPCMCOEFSET {}
impl ::core::default::Default for ADPCMCOEFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct ADPCMEWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for ADPCMEWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for ADPCMEWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for ADPCMEWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for ADPCMEWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADPCMEWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for ADPCMEWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for ADPCMEWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wNumCoef: u16,
    pub aCoef: [ADPCMCOEFSET; 1],
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for ADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct APTXWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for APTXWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for APTXWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for APTXWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for APTXWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APTXWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for APTXWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for APTXWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct AUDIOFILE_AF10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for AUDIOFILE_AF10WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for AUDIOFILE_AF10WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for AUDIOFILE_AF10WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for AUDIOFILE_AF10WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIOFILE_AF10WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for AUDIOFILE_AF10WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for AUDIOFILE_AF10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct AUDIOFILE_AF36WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for AUDIOFILE_AF36WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for AUDIOFILE_AF36WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for AUDIOFILE_AF36WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for AUDIOFILE_AF36WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIOFILE_AF36WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for AUDIOFILE_AF36WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for AUDIOFILE_AF36WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AUXDM_GETDEVCAPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AUXDM_GETNUMDEVS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AUXDM_GETVOLUME: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AUXDM_SETVOLUME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AUXM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AUXM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIBuildFilterA<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilter: &mut [u8], fsaving: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIBuildFilterA(lpszfilter: ::windows::core::PSTR, cbfilter: i32, fsaving: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        AVIBuildFilterA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszfilter)), lpszfilter.len() as _, fsaving.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIBuildFilterW<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilter: &mut [u16], fsaving: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIBuildFilterW(lpszfilter: ::windows::core::PWSTR, cbfilter: i32, fsaving: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        AVIBuildFilterW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszfilter)), lpszfilter.len() as _, fsaving.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVICOMPRESSF_DATARATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVICOMPRESSF_INTERLEAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVICOMPRESSF_KEYFRAMES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVICOMPRESSF_VALID: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct AVICOMPRESSOPTIONS {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwKeyFrameEvery: u32,
    pub dwQuality: u32,
    pub dwBytesPerSecond: u32,
    pub dwFlags: u32,
    pub lpFormat: *mut ::core::ffi::c_void,
    pub cbFormat: u32,
    pub lpParms: *mut ::core::ffi::c_void,
    pub cbParms: u32,
    pub dwInterleaveEvery: u32,
}
impl ::core::marker::Copy for AVICOMPRESSOPTIONS {}
impl ::core::clone::Clone for AVICOMPRESSOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AVICOMPRESSOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AVICOMPRESSOPTIONS").field("fccType", &self.fccType).field("fccHandler", &self.fccHandler).field("dwKeyFrameEvery", &self.dwKeyFrameEvery).field("dwQuality", &self.dwQuality).field("dwBytesPerSecond", &self.dwBytesPerSecond).field("dwFlags", &self.dwFlags).field("lpFormat", &self.lpFormat).field("cbFormat", &self.cbFormat).field("lpParms", &self.lpParms).field("cbParms", &self.cbParms).field("dwInterleaveEvery", &self.dwInterleaveEvery).finish()
    }
}
unsafe impl ::windows::core::Abi for AVICOMPRESSOPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AVICOMPRESSOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVICOMPRESSOPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AVICOMPRESSOPTIONS {}
impl ::core::default::Default for AVICOMPRESSOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIClearClipboard() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIClearClipboard() -> ::windows::core::HRESULT;
        }
        AVIClearClipboard().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIERR_OK: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILECAPS_ALLKEYFRAMES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILECAPS_CANREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILECAPS_CANWRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILECAPS_NOCOMPRESSION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEHANDLER_CANACCEPTNONRGB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEHANDLER_CANREAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEHANDLER_CANWRITE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
    pub szFileType: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AVIFILEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AVIFILEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AVIFILEINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AVIFILEINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVIFILEINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AVIFILEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AVIFILEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
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
impl ::core::marker::Copy for AVIFILEINFOW {}
impl ::core::clone::Clone for AVIFILEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
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
unsafe impl ::windows::core::Abi for AVIFILEINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AVIFILEINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVIFILEINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for AVIFILEINFOW {}
impl ::core::default::Default for AVIFILEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEINFO_COPYRIGHTED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEINFO_HASINDEX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEINFO_ISINTERLEAVED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEINFO_MUSTUSEINDEX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIFILEINFO_WASCAPTUREFILE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileAddRef<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileAddRef(pfile: ::windows::core::RawPtr) -> u32;
        }
        ::core::mem::transmute(AVIFileAddRef(pfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIFileCreateStreamA<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, ppavi: *mut ::core::option::Option<IAVIStream>, psi: *const AVISTREAMINFOA) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileCreateStreamA(pfile: ::windows::core::RawPtr, ppavi: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOA) -> ::windows::core::HRESULT;
        }
        AVIFileCreateStreamA(pfile.into_param().abi(), ::core::mem::transmute(ppavi), ::core::mem::transmute(psi)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIFileCreateStreamW<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, ppavi: *mut ::core::option::Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileCreateStreamW(pfile: ::windows::core::RawPtr, ppavi: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::core::HRESULT;
        }
        AVIFileCreateStreamW(pfile.into_param().abi(), ::core::mem::transmute(ppavi), ::core::mem::transmute(psi)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileEndRecord<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileEndRecord(pfile: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        AVIFileEndRecord(pfile.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileExit() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileExit();
        }
        AVIFileExit()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileGetStream<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, ppavi: *mut ::core::option::Option<IAVIStream>, fcctype: u32, lparam: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileGetStream(pfile: ::windows::core::RawPtr, ppavi: *mut ::windows::core::RawPtr, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT;
        }
        AVIFileGetStream(pfile.into_param().abi(), ::core::mem::transmute(ppavi), ::core::mem::transmute(fcctype), ::core::mem::transmute(lparam)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIFileInfoA<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, pfi: *mut AVIFILEINFOA, lsize: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileInfoA(pfile: ::windows::core::RawPtr, pfi: *mut AVIFILEINFOA, lsize: i32) -> ::windows::core::HRESULT;
        }
        AVIFileInfoA(pfile.into_param().abi(), ::core::mem::transmute(pfi), ::core::mem::transmute(lsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileInfoW<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileInfoW(pfile: ::windows::core::RawPtr, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::HRESULT;
        }
        AVIFileInfoW(pfile.into_param().abi(), ::core::mem::transmute(pfi), ::core::mem::transmute(lsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileInit() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileInit();
        }
        AVIFileInit()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileOpenA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(ppfile: *mut ::core::option::Option<IAVIFile>, szfile: Param1, umode: u32, lphandler: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileOpenA(ppfile: *mut ::windows::core::RawPtr, szfile: ::windows::core::PCSTR, umode: u32, lphandler: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        AVIFileOpenA(::core::mem::transmute(ppfile), szfile.into_param().abi(), ::core::mem::transmute(umode), ::core::mem::transmute(lphandler)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileOpenW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(ppfile: *mut ::core::option::Option<IAVIFile>, szfile: Param1, umode: u32, lphandler: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileOpenW(ppfile: *mut ::windows::core::RawPtr, szfile: ::windows::core::PCWSTR, umode: u32, lphandler: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        AVIFileOpenW(::core::mem::transmute(ppfile), szfile.into_param().abi(), ::core::mem::transmute(umode), ::core::mem::transmute(lphandler)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileReadData<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileReadData(pfile: ::windows::core::RawPtr, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::HRESULT;
        }
        AVIFileReadData(pfile.into_param().abi(), ::core::mem::transmute(ckid), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileRelease<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileRelease(pfile: ::windows::core::RawPtr) -> u32;
        }
        ::core::mem::transmute(AVIFileRelease(pfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIFileWriteData<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pfile: Param0, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileWriteData(pfile: ::windows::core::RawPtr, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::HRESULT;
        }
        AVIFileWriteData(pfile.into_param().abi(), ::core::mem::transmute(ckid), ::core::mem::transmute(lpdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIGETFRAMEF_BESTDISPLAYFMT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIGetFromClipboard() -> ::windows::core::Result<IAVIFile> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIGetFromClipboard(lppf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        AVIGetFromClipboard(::core::mem::transmute(&mut result__)).from_abi::<IAVIFile>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIIF_CONTROLFRAME: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVIIF_TWOCC: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIMakeCompressedStream<'a, Param1: ::windows::core::IntoParam<'a, IAVIStream>>(ppscompressed: *mut ::core::option::Option<IAVIStream>, ppssource: Param1, lpoptions: *const AVICOMPRESSOPTIONS, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIMakeCompressedStream(ppscompressed: *mut ::windows::core::RawPtr, ppssource: ::windows::core::RawPtr, lpoptions: *const AVICOMPRESSOPTIONS, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        AVIMakeCompressedStream(::core::mem::transmute(ppscompressed), ppssource.into_param().abi(), ::core::mem::transmute(lpoptions), ::core::mem::transmute(pclsidhandler)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIMakeFileFromStreams(ppfile: *mut ::core::option::Option<IAVIFile>, papstreams: &[::core::option::Option<IAVIStream>]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIMakeFileFromStreams(ppfile: *mut ::windows::core::RawPtr, nstreams: i32, papstreams: *const ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        AVIMakeFileFromStreams(::core::mem::transmute(ppfile), papstreams.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(papstreams))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIMakeStreamFromClipboard<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(cfformat: u32, hglobal: Param1) -> ::windows::core::Result<IAVIStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIMakeStreamFromClipboard(cfformat: u32, hglobal: super::super::Foundation::HANDLE, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        AVIMakeStreamFromClipboard(::core::mem::transmute(cfformat), hglobal.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAVIStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIPutFileOnClipboard<'a, Param0: ::windows::core::IntoParam<'a, IAVIFile>>(pf: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIPutFileOnClipboard(pf: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        AVIPutFileOnClipboard(pf.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type AVISAVECALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: i32) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AVISTREAMINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AVISTREAMINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
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
unsafe impl ::windows::core::Abi for AVISTREAMINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AVISTREAMINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVISTREAMINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AVISTREAMINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AVISTREAMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AVISTREAMINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AVISTREAMINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
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
unsafe impl ::windows::core::Abi for AVISTREAMINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AVISTREAMINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AVISTREAMINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AVISTREAMINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AVISTREAMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVISTREAMINFO_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVISTREAMINFO_FORMATCHANGES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVISTREAMREAD_CONVENIENT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVISaveA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, IAVIStream>>(szfile: Param0, pclsidhandler: *const ::windows::core::GUID, lpfncallback: AVISAVECALLBACK, nstreams: i32, pfile: Param4, lpoptions: *const AVICOMPRESSOPTIONS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveA(szfile: ::windows::core::PCSTR, pclsidhandler: *const ::windows::core::GUID, lpfncallback: ::windows::core::RawPtr, nstreams: i32, pfile: ::windows::core::RawPtr, lpoptions: *const AVICOMPRESSOPTIONS) -> ::windows::core::HRESULT;
        }
        AVISaveA(szfile.into_param().abi(), ::core::mem::transmute(pclsidhandler), ::core::mem::transmute(lpfncallback), ::core::mem::transmute(nstreams), pfile.into_param().abi(), ::core::mem::transmute(lpoptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVISaveOptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, const PARAM2: usize>(hwnd: Param0, uiflags: u32, ppavi: &[::core::option::Option<IAVIStream>; PARAM2], plpoptions: &mut [*mut AVICOMPRESSOPTIONS; PARAM2]) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveOptions(hwnd: super::super::Foundation::HWND, uiflags: u32, nstreams: i32, ppavi: *const ::windows::core::RawPtr, plpoptions: *mut *mut AVICOMPRESSOPTIONS) -> isize;
        }
        ::core::mem::transmute(AVISaveOptions(hwnd.into_param().abi(), ::core::mem::transmute(uiflags), PARAM2 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppavi)), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(plpoptions))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVISaveOptionsFree(plpoptions: &[*const AVICOMPRESSOPTIONS]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveOptionsFree(nstreams: i32, plpoptions: *const *const AVICOMPRESSOPTIONS) -> ::windows::core::HRESULT;
        }
        AVISaveOptionsFree(plpoptions.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(plpoptions))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVISaveVA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, const PARAM3: usize>(szfile: Param0, pclsidhandler: *const ::windows::core::GUID, lpfncallback: AVISAVECALLBACK, ppavi: &[::core::option::Option<IAVIStream>; PARAM3], plpoptions: &[*const AVICOMPRESSOPTIONS; PARAM3]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveVA(szfile: ::windows::core::PCSTR, pclsidhandler: *const ::windows::core::GUID, lpfncallback: ::windows::core::RawPtr, nstreams: i32, ppavi: *const ::windows::core::RawPtr, plpoptions: *const *const AVICOMPRESSOPTIONS) -> ::windows::core::HRESULT;
        }
        AVISaveVA(szfile.into_param().abi(), ::core::mem::transmute(pclsidhandler), ::core::mem::transmute(lpfncallback), PARAM3 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppavi)), ::core::mem::transmute(::windows::core::as_ptr_or_null(plpoptions))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVISaveVW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, const PARAM3: usize>(szfile: Param0, pclsidhandler: *const ::windows::core::GUID, lpfncallback: AVISAVECALLBACK, ppavi: &[::core::option::Option<IAVIStream>; PARAM3], plpoptions: &[*const AVICOMPRESSOPTIONS; PARAM3]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveVW(szfile: ::windows::core::PCWSTR, pclsidhandler: *const ::windows::core::GUID, lpfncallback: ::windows::core::RawPtr, nstreams: i32, ppavi: *const ::windows::core::RawPtr, plpoptions: *const *const AVICOMPRESSOPTIONS) -> ::windows::core::HRESULT;
        }
        AVISaveVW(szfile.into_param().abi(), ::core::mem::transmute(pclsidhandler), ::core::mem::transmute(lpfncallback), PARAM3 as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(ppavi)), ::core::mem::transmute(::windows::core::as_ptr_or_null(plpoptions))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVISaveW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, IAVIStream>>(szfile: Param0, pclsidhandler: *const ::windows::core::GUID, lpfncallback: AVISAVECALLBACK, nstreams: i32, pfile: Param4, lpoptions: *const AVICOMPRESSOPTIONS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveW(szfile: ::windows::core::PCWSTR, pclsidhandler: *const ::windows::core::GUID, lpfncallback: ::windows::core::RawPtr, nstreams: i32, pfile: ::windows::core::RawPtr, lpoptions: *const AVICOMPRESSOPTIONS) -> ::windows::core::HRESULT;
        }
        AVISaveW(szfile.into_param().abi(), ::core::mem::transmute(pclsidhandler), ::core::mem::transmute(lpfncallback), ::core::mem::transmute(nstreams), pfile.into_param().abi(), ::core::mem::transmute(lpoptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamAddRef<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamAddRef(pavi: ::windows::core::RawPtr) -> u32;
        }
        ::core::mem::transmute(AVIStreamAddRef(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamBeginStreaming<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamBeginStreaming(pavi: ::windows::core::RawPtr, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT;
        }
        AVIStreamBeginStreaming(pavi.into_param().abi(), ::core::mem::transmute(lstart), ::core::mem::transmute(lend), ::core::mem::transmute(lrate)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamCreate(ppavi: *mut ::core::option::Option<IAVIStream>, lparam1: i32, lparam2: i32, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamCreate(ppavi: *mut ::windows::core::RawPtr, lparam1: i32, lparam2: i32, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        AVIStreamCreate(::core::mem::transmute(ppavi), ::core::mem::transmute(lparam1), ::core::mem::transmute(lparam2), ::core::mem::transmute(pclsidhandler)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamEndStreaming<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamEndStreaming(pavi: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        AVIStreamEndStreaming(pavi.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamFindSample<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lpos: i32, lflags: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamFindSample(pavi: ::windows::core::RawPtr, lpos: i32, lflags: i32) -> i32;
        }
        ::core::mem::transmute(AVIStreamFindSample(pavi.into_param().abi(), ::core::mem::transmute(lpos), ::core::mem::transmute(lflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamGetFrame<'a, Param0: ::windows::core::IntoParam<'a, IGetFrame>>(pg: Param0, lpos: i32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamGetFrame(pg: ::windows::core::RawPtr, lpos: i32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(AVIStreamGetFrame(pg.into_param().abi(), ::core::mem::transmute(lpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamGetFrameClose<'a, Param0: ::windows::core::IntoParam<'a, IGetFrame>>(pg: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamGetFrameClose(pg: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        AVIStreamGetFrameClose(pg.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AVIStreamGetFrameOpen<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lpbiwanted: *const super::super::Graphics::Gdi::BITMAPINFOHEADER) -> ::core::option::Option<IGetFrame> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamGetFrameOpen(pavi: ::windows::core::RawPtr, lpbiwanted: *const super::super::Graphics::Gdi::BITMAPINFOHEADER) -> ::core::option::Option<IGetFrame>;
        }
        ::core::mem::transmute(AVIStreamGetFrameOpen(pavi.into_param().abi(), ::core::mem::transmute(lpbiwanted)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIStreamInfoA<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, psi: *mut AVISTREAMINFOA, lsize: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamInfoA(pavi: ::windows::core::RawPtr, psi: *mut AVISTREAMINFOA, lsize: i32) -> ::windows::core::HRESULT;
        }
        AVIStreamInfoA(pavi.into_param().abi(), ::core::mem::transmute(psi), ::core::mem::transmute(lsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVIStreamInfoW<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamInfoW(pavi: ::windows::core::RawPtr, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::HRESULT;
        }
        AVIStreamInfoW(pavi.into_param().abi(), ::core::mem::transmute(psi), ::core::mem::transmute(lsize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamLength<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamLength(pavi: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(AVIStreamLength(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamOpenFromFileA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(ppavi: *mut ::core::option::Option<IAVIStream>, szfile: Param1, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamOpenFromFileA(ppavi: *mut ::windows::core::RawPtr, szfile: ::windows::core::PCSTR, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        AVIStreamOpenFromFileA(::core::mem::transmute(ppavi), szfile.into_param().abi(), ::core::mem::transmute(fcctype), ::core::mem::transmute(lparam), ::core::mem::transmute(mode), ::core::mem::transmute(pclsidhandler)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamOpenFromFileW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(ppavi: *mut ::core::option::Option<IAVIStream>, szfile: Param1, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamOpenFromFileW(ppavi: *mut ::windows::core::RawPtr, szfile: ::windows::core::PCWSTR, fcctype: u32, lparam: i32, mode: u32, pclsidhandler: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        AVIStreamOpenFromFileW(::core::mem::transmute(ppavi), szfile.into_param().abi(), ::core::mem::transmute(fcctype), ::core::mem::transmute(lparam), ::core::mem::transmute(mode), ::core::mem::transmute(pclsidhandler)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamRead<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamRead(pavi: ::windows::core::RawPtr, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::HRESULT;
        }
        AVIStreamRead(pavi.into_param().abi(), ::core::mem::transmute(lstart), ::core::mem::transmute(lsamples), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(plbytes), ::core::mem::transmute(plsamples)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamReadData<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamReadData(pavi: ::windows::core::RawPtr, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::HRESULT;
        }
        AVIStreamReadData(pavi.into_param().abi(), ::core::mem::transmute(fcc), ::core::mem::transmute(lp), ::core::mem::transmute(lpcb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamReadFormat<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamReadFormat(pavi: ::windows::core::RawPtr, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::HRESULT;
        }
        AVIStreamReadFormat(pavi.into_param().abi(), ::core::mem::transmute(lpos), ::core::mem::transmute(lpformat), ::core::mem::transmute(lpcbformat)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamRelease<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamRelease(pavi: ::windows::core::RawPtr) -> u32;
        }
        ::core::mem::transmute(AVIStreamRelease(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamSampleToTime<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lsample: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamSampleToTime(pavi: ::windows::core::RawPtr, lsample: i32) -> i32;
        }
        ::core::mem::transmute(AVIStreamSampleToTime(pavi.into_param().abi(), ::core::mem::transmute(lsample)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamSetFormat<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamSetFormat(pavi: ::windows::core::RawPtr, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::HRESULT;
        }
        AVIStreamSetFormat(pavi.into_param().abi(), ::core::mem::transmute(lpos), ::core::mem::transmute(lpformat), ::core::mem::transmute(cbformat)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamStart<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamStart(pavi: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(AVIStreamStart(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamTimeToSample<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, ltime: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamTimeToSample(pavi: ::windows::core::RawPtr, ltime: i32) -> i32;
        }
        ::core::mem::transmute(AVIStreamTimeToSample(pavi.into_param().abi(), ::core::mem::transmute(ltime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamWrite<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamWrite(pavi: ::windows::core::RawPtr, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::HRESULT;
        }
        AVIStreamWrite(pavi.into_param().abi(), ::core::mem::transmute(lstart), ::core::mem::transmute(lsamples), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(plsampwritten), ::core::mem::transmute(plbyteswritten)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn AVIStreamWriteData<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamWriteData(pavi: ::windows::core::RawPtr, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::HRESULT;
        }
        AVIStreamWriteData(pavi.into_param().abi(), ::core::mem::transmute(fcc), ::core::mem::transmute(lp), ::core::mem::transmute(cb)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVSTREAMMASTER_AUDIO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const AVSTREAMMASTER_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const BI_1632: u32 = 842217009u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPCONTROLCALLBACK = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nstate: i32) -> super::super::Foundation::LRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: u32,
    pub fHasOverlay: super::super::Foundation::BOOL,
    pub fHasDlgVideoSource: super::super::Foundation::BOOL,
    pub fHasDlgVideoFormat: super::super::Foundation::BOOL,
    pub fHasDlgVideoDisplay: super::super::Foundation::BOOL,
    pub fCaptureInitialized: super::super::Foundation::BOOL,
    pub fDriverSuppliesPalettes: super::super::Foundation::BOOL,
    pub hVideoIn: super::super::Foundation::HANDLE,
    pub hVideoOut: super::super::Foundation::HANDLE,
    pub hVideoExtIn: super::super::Foundation::HANDLE,
    pub hVideoExtOut: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAPDRIVERCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAPDRIVERCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CAPDRIVERCAPS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAPDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAPDRIVERCAPS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAPDRIVERCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAPDRIVERCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPERRORCALLBACKA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: ::windows::core::PCSTR) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPERRORCALLBACKW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: ::windows::core::PCWSTR) -> super::super::Foundation::LRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct CAPINFOCHUNK {
    pub fccInfoID: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub cbData: i32,
}
impl ::core::marker::Copy for CAPINFOCHUNK {}
impl ::core::clone::Clone for CAPINFOCHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CAPINFOCHUNK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAPINFOCHUNK").field("fccInfoID", &self.fccInfoID).field("lpData", &self.lpData).field("cbData", &self.cbData).finish()
    }
}
unsafe impl ::windows::core::Abi for CAPINFOCHUNK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CAPINFOCHUNK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAPINFOCHUNK>()) == 0 }
    }
}
impl ::core::cmp::Eq for CAPINFOCHUNK {}
impl ::core::default::Default for CAPINFOCHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CAPSTATUS {
    pub uiImageWidth: u32,
    pub uiImageHeight: u32,
    pub fLiveWindow: super::super::Foundation::BOOL,
    pub fOverlayWindow: super::super::Foundation::BOOL,
    pub fScale: super::super::Foundation::BOOL,
    pub ptScroll: super::super::Foundation::POINT,
    pub fUsingDefaultPalette: super::super::Foundation::BOOL,
    pub fAudioHardware: super::super::Foundation::BOOL,
    pub fCapFileExists: super::super::Foundation::BOOL,
    pub dwCurrentVideoFrame: u32,
    pub dwCurrentVideoFramesDropped: u32,
    pub dwCurrentWaveSamples: u32,
    pub dwCurrentTimeElapsedMS: u32,
    pub hPalCurrent: super::super::Graphics::Gdi::HPALETTE,
    pub fCapturingNow: super::super::Foundation::BOOL,
    pub dwReturn: u32,
    pub wNumVideoAllocated: u32,
    pub wNumAudioAllocated: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CAPSTATUS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CAPSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CAPSTATUS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CAPSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAPSTATUS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CAPSTATUS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CAPSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPSTATUSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: ::windows::core::PCSTR) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPSTATUSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, nid: i32, lpsz: ::windows::core::PCWSTR) -> super::super::Foundation::LRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: u32,
    pub fMakeUserHitOKToCapture: super::super::Foundation::BOOL,
    pub wPercentDropForError: u32,
    pub fYield: super::super::Foundation::BOOL,
    pub dwIndexSize: u32,
    pub wChunkGranularity: u32,
    pub fUsingDOSMemory: super::super::Foundation::BOOL,
    pub wNumVideoRequested: u32,
    pub fCaptureAudio: super::super::Foundation::BOOL,
    pub wNumAudioRequested: u32,
    pub vKeyAbort: u32,
    pub fAbortLeftMouse: super::super::Foundation::BOOL,
    pub fAbortRightMouse: super::super::Foundation::BOOL,
    pub fLimitEnabled: super::super::Foundation::BOOL,
    pub wTimeLimit: u32,
    pub fMCIControl: super::super::Foundation::BOOL,
    pub fStepMCIDevice: super::super::Foundation::BOOL,
    pub dwMCIStartTime: u32,
    pub dwMCIStopTime: u32,
    pub fStepCaptureAt2x: super::super::Foundation::BOOL,
    pub wStepCaptureAverageFrames: u32,
    pub dwAudioBufferSize: u32,
    pub fDisableWriteCache: super::super::Foundation::BOOL,
    pub AVStreamMaster: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CAPTUREPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CAPTUREPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CAPTUREPARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CAPTUREPARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CAPTUREPARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CAPTUREPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CAPTUREPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPVIDEOCALLBACK = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpvhdr: *const VIDEOHDR) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub type CAPWAVECALLBACK = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpwhdr: *const super::Audio::WAVEHDR) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type CAPYIELDCALLBACK = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::LRESULT>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
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
impl ::core::marker::Copy for CHANNEL_CAPS {}
impl ::core::clone::Clone for CHANNEL_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANNEL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_CAPS").field("dwFlags", &self.dwFlags).field("dwSrcRectXMod", &self.dwSrcRectXMod).field("dwSrcRectYMod", &self.dwSrcRectYMod).field("dwSrcRectWidthMod", &self.dwSrcRectWidthMod).field("dwSrcRectHeightMod", &self.dwSrcRectHeightMod).field("dwDstRectXMod", &self.dwDstRectXMod).field("dwDstRectYMod", &self.dwDstRectYMod).field("dwDstRectWidthMod", &self.dwDstRectWidthMod).field("dwDstRectHeightMod", &self.dwDstRectHeightMod).finish()
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_CAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHANNEL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHANNEL_CAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHANNEL_CAPS {}
impl ::core::default::Default for CHANNEL_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_AVIFile: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020000_0000_0000_c000_000000000046);
pub const CLSID_AVISimpleUnMarshal: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020009_0000_0000_c000_000000000046);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct COMPVARS {
    pub cbSize: i32,
    pub dwFlags: u32,
    pub hic: HIC,
    pub fccType: u32,
    pub fccHandler: u32,
    pub lpbiIn: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub lpbiOut: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub lpBitsOut: *mut ::core::ffi::c_void,
    pub lpBitsPrev: *mut ::core::ffi::c_void,
    pub lFrame: i32,
    pub lKey: i32,
    pub lDataRate: i32,
    pub lQ: i32,
    pub lKeyCount: i32,
    pub lpState: *mut ::core::ffi::c_void,
    pub cbState: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for COMPVARS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for COMPVARS {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for COMPVARS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for COMPVARS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPVARS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for COMPVARS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for COMPVARS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct CONTRESCR10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for CONTRESCR10WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for CONTRESCR10WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for CONTRESCR10WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for CONTRESCR10WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTRESCR10WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for CONTRESCR10WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CONTRESCR10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct CONTRESVQLPCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for CONTRESVQLPCWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for CONTRESVQLPCWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for CONTRESVQLPCWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for CONTRESVQLPCWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTRESVQLPCWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for CONTRESVQLPCWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CONTRESVQLPCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const CONTROLCALLBACK_CAPTURING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const CONTROLCALLBACK_PREROLL: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct CREATIVEADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for CREATIVEADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for CREATIVEADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for CREATIVEADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for CREATIVEADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATIVEADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for CREATIVEADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CREATIVEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct CREATIVEFASTSPEECH10WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for CREATIVEFASTSPEECH10WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for CREATIVEFASTSPEECH10WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATIVEFASTSPEECH10WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for CREATIVEFASTSPEECH10WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct CREATIVEFASTSPEECH8WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for CREATIVEFASTSPEECH8WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for CREATIVEFASTSPEECH8WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CREATIVEFASTSPEECH8WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for CREATIVEFASTSPEECH8WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const CRYSTAL_NET_SFM_CODEC: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct CSIMAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for CSIMAADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for CSIMAADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for CSIMAADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for CSIMAADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSIMAADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for CSIMAADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for CSIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDriver<'a, Param0: ::windows::core::IntoParam<'a, HDRVR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hdriver: Param0, lparam1: Param1, lparam2: Param2) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseDriver(hdriver: HDRVR, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(CloseDriver(hdriver.into_param().abi(), lparam1.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn CreateEditableStream<'a, Param1: ::windows::core::IntoParam<'a, IAVIStream>>(ppseditable: *mut ::core::option::Option<IAVIStream>, pssource: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEditableStream(ppseditable: *mut ::windows::core::RawPtr, pssource: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CreateEditableStream(::core::mem::transmute(ppseditable), pssource.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_EVENT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_FUNCTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_NOSWITCH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_TASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_TYPEMASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DCB_WINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_0001: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_2000: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_ANIMATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_BACKGROUNDPAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_BUFFER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_DONTDRAW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_FULLSCREEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_HALFTONE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_HURRYUP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_JUSTDRAWIT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_NOTKEYFRAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_PREROLL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_SAME_DIB: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_SAME_DRAW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_SAME_HDC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_SAME_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DDF_UPDATE: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DIALOGICOKIADPCMWAVEFORMAT {
    pub ewf: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DIALOGICOKIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DIALOGICOKIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DIALOGICOKIADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DIALOGICOKIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIALOGICOKIADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DIALOGICOKIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIALOGICOKIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DIGIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DIGIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DIGIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DIGIADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DIGIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIGIADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DIGIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DIGIFIXWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DIGIFIXWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DIGIFIXWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DIGIFIXWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DIGIFIXWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIGIFIXWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DIGIFIXWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGIFIXWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DIGIREALWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DIGIREALWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DIGIREALWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DIGIREALWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DIGIREALWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIGIREALWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DIGIREALWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGIREALWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DIGISTDWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DIGISTDWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DIGISTDWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DIGISTDWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DIGISTDWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIGISTDWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DIGISTDWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DIGISTDWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DLG_ACMFILTERCHOOSE_ID: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DLG_ACMFORMATCHOOSE_ID: u32 = 70u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DOLBYAC2WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub nAuxBitsCode: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DOLBYAC2WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DOLBYAC2WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DOLBYAC2WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DOLBYAC2WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOLBYAC2WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DOLBYAC2WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DOLBYAC2WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct DRAWDIBTIME {
    pub timeCount: i32,
    pub timeDraw: i32,
    pub timeDecompress: i32,
    pub timeDither: i32,
    pub timeStretch: i32,
    pub timeBlt: i32,
    pub timeSetDIBits: i32,
}
impl ::core::marker::Copy for DRAWDIBTIME {}
impl ::core::clone::Clone for DRAWDIBTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRAWDIBTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWDIBTIME").field("timeCount", &self.timeCount).field("timeDraw", &self.timeDraw).field("timeDecompress", &self.timeDecompress).field("timeDither", &self.timeDither).field("timeStretch", &self.timeStretch).field("timeBlt", &self.timeBlt).field("timeSetDIBits", &self.timeSetDIBits).finish()
    }
}
unsafe impl ::windows::core::Abi for DRAWDIBTIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRAWDIBTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRAWDIBTIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRAWDIBTIME {}
impl ::core::default::Default for DRAWDIBTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub type DRIVERMSGPROC = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: usize, param3: usize, param4: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DRIVERPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: HDRVR, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRIVERS_SECTION: &'static str = "DRIVERS32";
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DRMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wReserved: u16,
    pub ulContentId: u32,
    pub wfxSecure: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DRMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DRMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DRMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DRMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DRMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DRMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVCNF_CANCEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVCNF_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVCNF_RESTART: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct DRVCONFIGINFO {
    pub dwDCISize: u32,
    pub lpszDCISectionName: ::windows::core::PCWSTR,
    pub lpszDCIAliasName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for DRVCONFIGINFO {}
impl ::core::clone::Clone for DRVCONFIGINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRVCONFIGINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRVCONFIGINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRVCONFIGINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRVCONFIGINFO {}
impl ::core::default::Default for DRVCONFIGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: u32,
    pub lpszDCISectionName: ::windows::core::PCWSTR,
    pub lpszDCIAliasName: ::windows::core::PCWSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for DRVCONFIGINFOEX {}
impl ::core::clone::Clone for DRVCONFIGINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRVCONFIGINFOEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRVCONFIGINFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRVCONFIGINFOEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRVCONFIGINFOEX {}
impl ::core::default::Default for DRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_ADD_THRU: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_DISABLE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_ENABLE: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_EXIT: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_IOCTL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_IOCTL_CMD_SYSTEM: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_IOCTL_CMD_USER: i32 = 0i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct DRVM_IOCTL_DATA {
    pub dwSize: u32,
    pub dwCmd: u32,
}
impl ::core::marker::Copy for DRVM_IOCTL_DATA {}
impl ::core::clone::Clone for DRVM_IOCTL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DRVM_IOCTL_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRVM_IOCTL_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRVM_IOCTL_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRVM_IOCTL_DATA {}
impl ::core::default::Default for DRVM_IOCTL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_IOCTL_LAST: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_MAPPER_CONSOLEVOICECOM_GET: u32 = 8215u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_MAPPER_PREFERRED_FLAGS_PREFERREDONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_MAPPER_PREFERRED_GET: u32 = 8213u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_MAPPER_RECONFIGURE: u32 = 8193u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_REMOVE_THRU: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRVM_USER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_CANCEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_CONFIGURE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_DISABLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_ENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_EXITSESSION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_FREE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_INSTALL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_LOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_MCI_FIRST: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_MCI_LAST: u32 = 6143u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_OPEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_PNPINSTALL: u32 = 2059u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_POWER: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYCONFIGURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYDEVICEINTERFACE: u32 = 2060u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYDEVICEINTERFACESIZE: u32 = 2061u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYDEVNODE: u32 = 2050u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYFUNCTIONINSTANCEID: u32 = 2065u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYFUNCTIONINSTANCEIDSIZE: u32 = 2066u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYIDFROMSTRINGID: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYMAPPABLE: u32 = 2053u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYMODULE: u32 = 2057u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYSTRINGID: u32 = 2062u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_QUERYSTRINGIDSIZE: u32 = 2063u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_REMOVE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_RESERVED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_RESTART: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DRV_USER: u32 = 16384u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct DVIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for DVIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for DVIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for DVIADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for DVIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DVIADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for DVIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for DVIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_CONFIGURE_END: u32 = 8191u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_CONFIGURE_START: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_DST_RECT: u32 = 4101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_FORMAT: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_PALETTE: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_PALETTERGB555: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_SRC_RECT: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DVM_USER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_13: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_ALLOCATED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_BADDEVICEID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_BADERRNUM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_BADFORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_BADINSTALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_CONFIG1: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_CONFIG2: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_CREATEPALETTE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_DMA_CONFLICT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_FLAGS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_INT_CONFLICT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_INVALHANDLE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_IO_CONFLICT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_LASTERROR: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_MEM_CONFLICT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_NOMEM: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_NONSPECIFIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_NOTDETECTED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_NOTSUPPORTED: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_NO_BUFFERS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_PARAM1: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_PARAM2: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_PROTECT_ONLY: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_SIZEFIELD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_STILLPLAYING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_SYNC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_TOOMANYCHANNELS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_UNPREPARED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_ERR_USER_MSG: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_VM_CLOSE: u32 = 977u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_VM_DATA: u32 = 978u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_VM_ERROR: u32 = 979u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const DV_VM_OPEN: u32 = 976u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefDriverProc<'a, Param1: ::windows::core::IntoParam<'a, HDRVR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(dwdriveridentifier: usize, hdrvr: Param1, umsg: u32, lparam1: Param3, lparam2: Param4) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefDriverProc(dwdriveridentifier: usize, hdrvr: HDRVR, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(DefDriverProc(::core::mem::transmute(dwdriveridentifier), hdrvr.into_param().abi(), ::core::mem::transmute(umsg), lparam1.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibBegin<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdd: isize, hdc: Param1, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, dxsrc: i32, dysrc: i32, wflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibBegin(hdd: isize, hdc: super::super::Graphics::Gdi::HDC, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, dxsrc: i32, dysrc: i32, wflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibBegin(::core::mem::transmute(hdd), hdc.into_param().abi(), ::core::mem::transmute(dxdst), ::core::mem::transmute(dydst), ::core::mem::transmute(lpbi), ::core::mem::transmute(dxsrc), ::core::mem::transmute(dysrc), ::core::mem::transmute(wflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibChangePalette(hdd: isize, istart: i32, lppe: &[super::super::Graphics::Gdi::PALETTEENTRY]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibChangePalette(hdd: isize, istart: i32, ilen: i32, lppe: *const super::super::Graphics::Gdi::PALETTEENTRY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibChangePalette(::core::mem::transmute(hdd), ::core::mem::transmute(istart), lppe.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lppe))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibClose(hdd: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibClose(hdd: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibClose(::core::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibDraw<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdd: isize, hdc: Param1, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, wflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibDraw(hdd: isize, hdc: super::super::Graphics::Gdi::HDC, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, wflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibDraw(::core::mem::transmute(hdd), hdc.into_param().abi(), ::core::mem::transmute(xdst), ::core::mem::transmute(ydst), ::core::mem::transmute(dxdst), ::core::mem::transmute(dydst), ::core::mem::transmute(lpbi), ::core::mem::transmute(lpbits), ::core::mem::transmute(xsrc), ::core::mem::transmute(ysrc), ::core::mem::transmute(dxsrc), ::core::mem::transmute(dysrc), ::core::mem::transmute(wflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibEnd(hdd: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibEnd(hdd: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibEnd(::core::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawDibGetBuffer(hdd: isize, lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, dwsize: u32, dwflags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibGetBuffer(hdd: isize, lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, dwsize: u32, dwflags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(DrawDibGetBuffer(::core::mem::transmute(hdd), ::core::mem::transmute(lpbi), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawDibGetPalette(hdd: isize) -> super::super::Graphics::Gdi::HPALETTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibGetPalette(hdd: isize) -> super::super::Graphics::Gdi::HPALETTE;
        }
        ::core::mem::transmute(DrawDibGetPalette(::core::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn DrawDibOpen() -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibOpen() -> isize;
        }
        ::core::mem::transmute(DrawDibOpen())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibProfileDisplay(lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibProfileDisplay(lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(DrawDibProfileDisplay(::core::mem::transmute(lpbi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibRealize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hdd: isize, hdc: Param1, fbackground: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibRealize(hdd: isize, hdc: super::super::Graphics::Gdi::HDC, fbackground: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DrawDibRealize(::core::mem::transmute(hdd), hdc.into_param().abi(), fbackground.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibSetPalette<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>>(hdd: isize, hpal: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibSetPalette(hdd: isize, hpal: super::super::Graphics::Gdi::HPALETTE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibSetPalette(::core::mem::transmute(hdd), hpal.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibStart(hdd: isize, rate: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibStart(hdd: isize, rate: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibStart(::core::mem::transmute(hdd), ::core::mem::transmute(rate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibStop(hdd: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibStop(hdd: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibStop(::core::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibTime(hdd: isize, lpddtime: *mut DRAWDIBTIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibTime(hdd: isize, lpddtime: *mut DRAWDIBTIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DrawDibTime(::core::mem::transmute(hdd), ::core::mem::transmute(lpddtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DriverCallback<'a, Param2: ::windows::core::IntoParam<'a, HDRVR>>(dwcallback: usize, dwflags: u32, hdevice: Param2, dwmsg: u32, dwuser: usize, dwparam1: usize, dwparam2: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DriverCallback(dwcallback: usize, dwflags: u32, hdevice: HDRVR, dwmsg: u32, dwuser: usize, dwparam1: usize, dwparam2: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DriverCallback(::core::mem::transmute(dwcallback), ::core::mem::transmute(dwflags), hdevice.into_param().abi(), ::core::mem::transmute(dwmsg), ::core::mem::transmute(dwuser), ::core::mem::transmute(dwparam1), ::core::mem::transmute(dwparam2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrvGetModuleHandle<'a, Param0: ::windows::core::IntoParam<'a, HDRVR>>(hdriver: Param0) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrvGetModuleHandle(hdriver: HDRVR) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(DrvGetModuleHandle(hdriver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct ECHOSC1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for ECHOSC1WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for ECHOSC1WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for ECHOSC1WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for ECHOSC1WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ECHOSC1WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for ECHOSC1WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for ECHOSC1WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EXBMINFOHEADER {
    pub bmi: super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub biExtDataOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for EXBMINFOHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for EXBMINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for EXBMINFOHEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EXBMINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXBMINFOHEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EXBMINFOHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EXBMINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn EditStreamClone<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0) -> ::windows::core::Result<IAVIStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamClone(pavi: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        EditStreamClone(pavi.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAVIStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn EditStreamCopy<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamCopy(pavi: ::windows::core::RawPtr, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        EditStreamCopy(pavi.into_param().abi(), ::core::mem::transmute(plstart), ::core::mem::transmute(pllength), ::core::mem::transmute(ppresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn EditStreamCut<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamCut(pavi: ::windows::core::RawPtr, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        EditStreamCut(pavi.into_param().abi(), ::core::mem::transmute(plstart), ::core::mem::transmute(pllength), ::core::mem::transmute(ppresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn EditStreamPaste<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>, Param3: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, plpos: *mut i32, pllength: *mut i32, pstream: Param3, lstart: i32, lend: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamPaste(pavi: ::windows::core::RawPtr, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::core::RawPtr, lstart: i32, lend: i32) -> ::windows::core::HRESULT;
        }
        EditStreamPaste(pavi.into_param().abi(), ::core::mem::transmute(plpos), ::core::mem::transmute(pllength), pstream.into_param().abi(), ::core::mem::transmute(lstart), ::core::mem::transmute(lend)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EditStreamSetInfoA<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lpinfo: *const AVISTREAMINFOA, cbinfo: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetInfoA(pavi: ::windows::core::RawPtr, lpinfo: *const AVISTREAMINFOA, cbinfo: i32) -> ::windows::core::HRESULT;
        }
        EditStreamSetInfoA(pavi.into_param().abi(), ::core::mem::transmute(lpinfo), ::core::mem::transmute(cbinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EditStreamSetInfoW<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>>(pavi: Param0, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetInfoW(pavi: ::windows::core::RawPtr, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT;
        }
        EditStreamSetInfoW(pavi.into_param().abi(), ::core::mem::transmute(lpinfo), ::core::mem::transmute(cbinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn EditStreamSetNameA<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pavi: Param0, lpszname: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetNameA(pavi: ::windows::core::RawPtr, lpszname: ::windows::core::PCSTR) -> ::windows::core::HRESULT;
        }
        EditStreamSetNameA(pavi.into_param().abi(), lpszname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn EditStreamSetNameW<'a, Param0: ::windows::core::IntoParam<'a, IAVIStream>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pavi: Param0, lpszname: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetNameW(pavi: ::windows::core::RawPtr, lpszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        EditStreamSetNameW(pavi.into_param().abi(), lpszname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FACILITY_NS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FACILITY_NS_WIN32: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_ANY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_DIR: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_FORMAT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_FROM_START: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_INDEX: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_KEY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_LENGTH: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_NEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_OFFSET: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_POS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_PREV: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_RET: i32 = 61440i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_SIZE: i32 = 12288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const FIND_TYPE: i32 = 240i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct FMTOWNS_SND_WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for FMTOWNS_SND_WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for FMTOWNS_SND_WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for FMTOWNS_SND_WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for FMTOWNS_SND_WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FMTOWNS_SND_WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for FMTOWNS_SND_WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for FMTOWNS_SND_WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct G721_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub nAuxBlockSize: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for G721_ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for G721_ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for G721_ADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for G721_ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<G721_ADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for G721_ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for G721_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct G723_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub cbExtraSize: u16,
    pub nAuxBlockSize: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for G723_ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for G723_ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for G723_ADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for G723_ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<G723_ADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for G723_ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for G723_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct GSM610WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for GSM610WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for GSM610WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for GSM610WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for GSM610WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GSM610WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for GSM610WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for GSM610WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDriverModuleHandle<'a, Param0: ::windows::core::IntoParam<'a, HDRVR>>(hdriver: Param0) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDriverModuleHandle(hdriver: HDRVR) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(GetDriverModuleHandle(hdriver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetOpenFileNamePreviewA(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNamePreviewA(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOpenFileNamePreviewA(::core::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetOpenFileNamePreviewW(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNamePreviewW(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOpenFileNamePreviewW(::core::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetSaveFileNamePreviewA(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNamePreviewA(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSaveFileNamePreviewA(::core::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetSaveFileNamePreviewW(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNamePreviewW(lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSaveFileNamePreviewW(::core::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDRVR(pub isize);
impl HDRVR {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HDRVR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDRVR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDRVR {}
impl ::core::fmt::Debug for HDRVR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDRVR").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HDRVR {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HIC(pub isize);
impl HIC {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIC {}
impl ::core::fmt::Debug for HIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIC").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HIC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMMIO(pub isize);
impl HMMIO {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HMMIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMMIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMMIO {}
impl ::core::fmt::Debug for HMMIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMMIO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMMIO {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HVIDEO(pub isize);
impl HVIDEO {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for HVIDEO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HVIDEO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HVIDEO {}
impl ::core::fmt::Debug for HVIDEO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HVIDEO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HVIDEO {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[repr(transparent)]
pub struct IAVIEditStream(::windows::core::IUnknown);
impl IAVIEditStream {
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Cut(&self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cut)(::core::mem::transmute_copy(self), ::core::mem::transmute(plstart), ::core::mem::transmute(pllength), ::core::mem::transmute(ppresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Copy(&self, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Copy)(::core::mem::transmute_copy(self), ::core::mem::transmute(plstart), ::core::mem::transmute(pllength), ::core::mem::transmute(ppresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Paste<'a, Param2: ::windows::core::IntoParam<'a, IAVIStream>>(&self, plpos: *mut i32, pllength: *mut i32, pstream: Param2, lstart: i32, lend: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Paste)(::core::mem::transmute_copy(self), ::core::mem::transmute(plpos), ::core::mem::transmute(pllength), pstream.into_param().abi(), ::core::mem::transmute(lstart), ::core::mem::transmute(lend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IAVIStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAVIStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpinfo), ::core::mem::transmute(cbinfo)).ok()
    }
}
impl ::core::convert::From<IAVIEditStream> for ::windows::core::IUnknown {
    fn from(value: IAVIEditStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAVIEditStream> for ::windows::core::IUnknown {
    fn from(value: &IAVIEditStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAVIEditStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAVIEditStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAVIEditStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IAVIEditStream {
    type Vtable = IAVIEditStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020024_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIEditStream_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Cut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Paste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: ::windows::core::RawPtr, lstart: i32, lend: i32) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInfo: usize,
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[repr(transparent)]
pub struct IAVIFile(::windows::core::IUnknown);
impl IAVIFile {
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Info(&self, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Info)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfi), ::core::mem::transmute(lsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn GetStream(&self, ppstream: *mut ::core::option::Option<IAVIStream>, fcctype: u32, lparam: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppstream), ::core::mem::transmute(fcctype), ::core::mem::transmute(lparam)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStream(&self, ppstream: *mut ::core::option::Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppstream), ::core::mem::transmute(psi)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn WriteData(&self, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteData)(::core::mem::transmute_copy(self), ::core::mem::transmute(ckid), ::core::mem::transmute(lpdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn ReadData(&self, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadData)(::core::mem::transmute_copy(self), ::core::mem::transmute(ckid), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpcbdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn EndRecord(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndRecord)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn DeleteStream(&self, fcctype: u32, lparam: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcctype), ::core::mem::transmute(lparam)).ok()
    }
}
impl ::core::convert::From<IAVIFile> for ::windows::core::IUnknown {
    fn from(value: IAVIFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAVIFile> for ::windows::core::IUnknown {
    fn from(value: &IAVIFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAVIFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAVIFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAVIFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IAVIFile {
    type Vtable = IAVIFile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIFile_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows::core::HRESULT,
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr, psi: *const AVISTREAMINFOW) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateStream: usize,
    pub WriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows::core::HRESULT,
    pub ReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows::core::HRESULT,
    pub EndRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAVIPersistFile(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAVIPersistFile {
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Load)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), ::core::mem::transmute(dwmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Save)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SaveCompleted)(::core::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCurFile(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCurFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Reserved1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reserved1)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAVIPersistFile> for ::windows::core::IUnknown {
    fn from(value: IAVIPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAVIPersistFile> for ::windows::core::IUnknown {
    fn from(value: &IAVIPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAVIPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAVIPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAVIPersistFile> for super::super::System::Com::IPersist {
    fn from(value: IAVIPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAVIPersistFile> for super::super::System::Com::IPersist {
    fn from(value: &IAVIPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for IAVIPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersist> for &'a IAVIPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAVIPersistFile> for super::super::System::Com::IPersistFile {
    fn from(value: IAVIPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAVIPersistFile> for super::super::System::Com::IPersistFile {
    fn from(value: &IAVIPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistFile> for IAVIPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistFile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IPersistFile> for &'a IAVIPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IPersistFile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAVIPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IAVIPersistFile {
    type Vtable = IAVIPersistFile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020025_0000_0000_c000_000000000046);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAVIPersistFile_Vtbl {
    pub base: super::super::System::Com::IPersistFile_Vtbl,
    pub Reserved1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[repr(transparent)]
pub struct IAVIStream(::windows::core::IUnknown);
impl IAVIStream {
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(&self, lparam1: Param0, lparam2: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Create)(::core::mem::transmute_copy(self), lparam1.into_param().abi(), lparam2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Info(&self, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Info)(::core::mem::transmute_copy(self), ::core::mem::transmute(psi), ::core::mem::transmute(lsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn FindSample(&self, lpos: i32, lflags: i32) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).FindSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpos), ::core::mem::transmute(lflags)))
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn ReadFormat(&self, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpos), ::core::mem::transmute(lpformat), ::core::mem::transmute(lpcbformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn SetFormat(&self, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpos), ::core::mem::transmute(lpformat), ::core::mem::transmute(cbformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Read(&self, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(lstart), ::core::mem::transmute(lsamples), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(plbytes), ::core::mem::transmute(plsamples)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Write(&self, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(lstart), ::core::mem::transmute(lsamples), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(plsampwritten), ::core::mem::transmute(plbyteswritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Delete(&self, lstart: i32, lsamples: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::core::mem::transmute_copy(self), ::core::mem::transmute(lstart), ::core::mem::transmute(lsamples)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn ReadData(&self, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadData)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcc), ::core::mem::transmute(lp), ::core::mem::transmute(lpcb)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn WriteData(&self, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteData)(::core::mem::transmute_copy(self), ::core::mem::transmute(fcc), ::core::mem::transmute(lp), ::core::mem::transmute(cb)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInfo(&self, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpinfo), ::core::mem::transmute(cbinfo)).ok()
    }
}
impl ::core::convert::From<IAVIStream> for ::windows::core::IUnknown {
    fn from(value: IAVIStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAVIStream> for ::windows::core::IUnknown {
    fn from(value: &IAVIStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAVIStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAVIStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAVIStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IAVIStream {
    type Vtable = IAVIStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020021_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIStream_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Info: usize,
    pub FindSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpos: i32, lflags: i32) -> i32,
    pub ReadFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32) -> ::windows::core::HRESULT,
    pub ReadData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows::core::HRESULT,
    pub WriteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInfo: usize,
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[repr(transparent)]
pub struct IAVIStreaming(::windows::core::IUnknown);
impl IAVIStreaming {
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin)(::core::mem::transmute_copy(self), ::core::mem::transmute(lstart), ::core::mem::transmute(lend), ::core::mem::transmute(lrate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAVIStreaming> for ::windows::core::IUnknown {
    fn from(value: IAVIStreaming) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAVIStreaming> for ::windows::core::IUnknown {
    fn from(value: &IAVIStreaming) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAVIStreaming {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAVIStreaming {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAVIStreaming {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IAVIStreaming {
    type Vtable = IAVIStreaming_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020022_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIStreaming_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICCOMPRESS {
    pub dwFlags: u32,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpOutput: *mut ::core::ffi::c_void,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpInput: *mut ::core::ffi::c_void,
    pub lpckid: *mut u32,
    pub lpdwFlags: *mut u32,
    pub lFrameNum: i32,
    pub dwFrameSize: u32,
    pub dwQuality: u32,
    pub lpbiPrev: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpPrev: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for ICCOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for ICCOMPRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICCOMPRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICCOMPRESS").field("dwFlags", &self.dwFlags).field("lpbiOutput", &self.lpbiOutput).field("lpOutput", &self.lpOutput).field("lpbiInput", &self.lpbiInput).field("lpInput", &self.lpInput).field("lpckid", &self.lpckid).field("lpdwFlags", &self.lpdwFlags).field("lFrameNum", &self.lFrameNum).field("dwFrameSize", &self.dwFrameSize).field("dwQuality", &self.dwQuality).field("lpbiPrev", &self.lpbiPrev).field("lpPrev", &self.lpPrev).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for ICCOMPRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICCOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICCOMPRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICCOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICCOMPRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICCOMPRESSFRAMES {
    pub dwFlags: u32,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lOutput: super::super::Foundation::LPARAM,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lInput: super::super::Foundation::LPARAM,
    pub lStartFrame: i32,
    pub lFrameCount: i32,
    pub lQuality: i32,
    pub lDataRate: i32,
    pub lKeyRate: i32,
    pub dwRate: u32,
    pub dwScale: u32,
    pub dwOverheadPerFrame: u32,
    pub dwReserved2: u32,
    pub GetData: isize,
    pub PutData: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ICCOMPRESSFRAMES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ICCOMPRESSFRAMES {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for ICCOMPRESSFRAMES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICCOMPRESSFRAMES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICCOMPRESSFRAMES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICCOMPRESSFRAMES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICCOMPRESSFRAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICCOMPRESSFRAMES_PADDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICCOMPRESS_KEYFRAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICClose<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICClose(hic: HIC) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ICClose(hic.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICCompress<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, dwflags: u32, lpbioutput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpdata: *mut ::core::ffi::c_void, lpbiinput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, lpckid: *mut u32, lpdwflags: *mut u32, lframenum: i32, dwframesize: u32, dwquality: u32, lpbiprev: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpprev: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICCompress(hic: HIC, dwflags: u32, lpbioutput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpdata: *mut ::core::ffi::c_void, lpbiinput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, lpckid: *mut u32, lpdwflags: *mut u32, lframenum: i32, dwframesize: u32, dwquality: u32, lpbiprev: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpprev: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ICCompress(hic.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpbioutput), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpbiinput), ::core::mem::transmute(lpbits), ::core::mem::transmute(lpckid), ::core::mem::transmute(lpdwflags), ::core::mem::transmute(lframenum), ::core::mem::transmute(dwframesize), ::core::mem::transmute(dwquality), ::core::mem::transmute(lpbiprev), ::core::mem::transmute(lpprev)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICCompressorChoose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwnd: Param0, uiflags: u32, pvin: *const ::core::ffi::c_void, lpdata: *const ::core::ffi::c_void, pc: *mut COMPVARS, lpsztitle: Param5) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICCompressorChoose(hwnd: super::super::Foundation::HWND, uiflags: u32, pvin: *const ::core::ffi::c_void, lpdata: *const ::core::ffi::c_void, pc: *mut COMPVARS, lpsztitle: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ICCompressorChoose(hwnd.into_param().abi(), ::core::mem::transmute(uiflags), ::core::mem::transmute(pvin), ::core::mem::transmute(lpdata), ::core::mem::transmute(pc), lpsztitle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICCompressorFree(pc: *const COMPVARS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICCompressorFree(pc: *const COMPVARS);
        }
        ICCompressorFree(::core::mem::transmute(pc))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICDECOMPRESS {
    pub dwFlags: u32,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpInput: *mut ::core::ffi::c_void,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpOutput: *mut ::core::ffi::c_void,
    pub ckid: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for ICDECOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for ICDECOMPRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICDECOMPRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDECOMPRESS").field("dwFlags", &self.dwFlags).field("lpbiInput", &self.lpbiInput).field("lpInput", &self.lpInput).field("lpbiOutput", &self.lpbiOutput).field("lpOutput", &self.lpOutput).field("ckid", &self.ckid).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for ICDECOMPRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICDECOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICDECOMPRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICDECOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICDECOMPRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICDECOMPRESSEX {
    pub dwFlags: u32,
    pub lpbiSrc: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpSrc: *mut ::core::ffi::c_void,
    pub lpbiDst: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpDst: *mut ::core::ffi::c_void,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for ICDECOMPRESSEX {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for ICDECOMPRESSEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICDECOMPRESSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDECOMPRESSEX").field("dwFlags", &self.dwFlags).field("lpbiSrc", &self.lpbiSrc).field("lpSrc", &self.lpSrc).field("lpbiDst", &self.lpbiDst).field("lpDst", &self.lpDst).field("xDst", &self.xDst).field("yDst", &self.yDst).field("dxDst", &self.dxDst).field("dyDst", &self.dyDst).field("xSrc", &self.xSrc).field("ySrc", &self.ySrc).field("dxSrc", &self.dxSrc).field("dySrc", &self.dySrc).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for ICDECOMPRESSEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICDECOMPRESSEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICDECOMPRESSEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICDECOMPRESSEX {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICDECOMPRESSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDECOMPRESS_HURRYUP: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDECOMPRESS_NOTKEYFRAME: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDECOMPRESS_NULLFRAME: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDECOMPRESS_PREROLL: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDECOMPRESS_UPDATE: i32 = 1073741824i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct ICDRAW {
    pub dwFlags: u32,
    pub lpFormat: *mut ::core::ffi::c_void,
    pub lpData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub lTime: i32,
}
impl ::core::marker::Copy for ICDRAW {}
impl ::core::clone::Clone for ICDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAW").field("dwFlags", &self.dwFlags).field("lpFormat", &self.lpFormat).field("lpData", &self.lpData).field("cbData", &self.cbData).field("lTime", &self.lTime).finish()
    }
}
unsafe impl ::windows::core::Abi for ICDRAW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICDRAW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICDRAW {}
impl ::core::default::Default for ICDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICDRAWBEGIN {
    pub dwFlags: u32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub hwnd: super::super::Foundation::HWND,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dwRate: u32,
    pub dwScale: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ICDRAWBEGIN {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ICDRAWBEGIN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICDRAWBEGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAWBEGIN").field("dwFlags", &self.dwFlags).field("hpal", &self.hpal).field("hwnd", &self.hwnd).field("hdc", &self.hdc).field("xDst", &self.xDst).field("yDst", &self.yDst).field("dxDst", &self.dxDst).field("dyDst", &self.dyDst).field("lpbi", &self.lpbi).field("xSrc", &self.xSrc).field("ySrc", &self.ySrc).field("dxSrc", &self.dxSrc).field("dySrc", &self.dySrc).field("dwRate", &self.dwRate).field("dwScale", &self.dwScale).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for ICDRAWBEGIN {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICDRAWBEGIN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICDRAWBEGIN>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICDRAWBEGIN {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICDRAWBEGIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICDRAWSUGGEST {
    pub lpbiIn: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpbiSuggest: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for ICDRAWSUGGEST {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for ICDRAWSUGGEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICDRAWSUGGEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICDRAWSUGGEST").field("lpbiIn", &self.lpbiIn).field("lpbiSuggest", &self.lpbiSuggest).field("dxSrc", &self.dxSrc).field("dySrc", &self.dySrc).field("dxDst", &self.dxDst).field("dyDst", &self.dyDst).field("hicDecompressor", &self.hicDecompressor).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for ICDRAWSUGGEST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICDRAWSUGGEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICDRAWSUGGEST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICDRAWSUGGEST {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICDRAWSUGGEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_ANIMATE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_BUFFER: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_CONTINUE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_FULLSCREEN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_HDC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_HURRYUP: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_MEMORYDC: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_NOTKEYFRAME: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_NULLFRAME: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_PREROLL: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_QUERY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_RENDER: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_UPDATE: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICDRAW_UPDATING: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICDecompress<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, dwflags: u32, lpbiformat: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpdata: *const ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICDecompress(hic: HIC, dwflags: u32, lpbiformat: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpdata: *const ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(ICDecompress(hic.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpbiformat), ::core::mem::transmute(lpdata), ::core::mem::transmute(lpbi), ::core::mem::transmute(lpbits)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn ICDraw<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, dwflags: u32, lpformat: *const ::core::ffi::c_void, lpdata: *const ::core::ffi::c_void, cbdata: u32, ltime: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICDraw(hic: HIC, dwflags: u32, lpformat: *const ::core::ffi::c_void, lpdata: *const ::core::ffi::c_void, cbdata: u32, ltime: i32) -> u32;
        }
        ::core::mem::transmute(ICDraw(hic.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpformat), ::core::mem::transmute(lpdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(ltime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICDrawBegin<'a, Param0: ::windows::core::IntoParam<'a, HIC>, Param2: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hic: Param0, dwflags: u32, hpal: Param2, hwnd: Param3, hdc: Param4, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, dwrate: u32, dwscale: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICDrawBegin(hic: HIC, dwflags: u32, hpal: super::super::Graphics::Gdi::HPALETTE, hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, xdst: i32, ydst: i32, dxdst: i32, dydst: i32, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, xsrc: i32, ysrc: i32, dxsrc: i32, dysrc: i32, dwrate: u32, dwscale: u32) -> u32;
        }
        ::core::mem::transmute(ICDrawBegin(
            hic.into_param().abi(),
            ::core::mem::transmute(dwflags),
            hpal.into_param().abi(),
            hwnd.into_param().abi(),
            hdc.into_param().abi(),
            ::core::mem::transmute(xdst),
            ::core::mem::transmute(ydst),
            ::core::mem::transmute(dxdst),
            ::core::mem::transmute(dydst),
            ::core::mem::transmute(lpbi),
            ::core::mem::transmute(xsrc),
            ::core::mem::transmute(ysrc),
            ::core::mem::transmute(dxsrc),
            ::core::mem::transmute(dysrc),
            ::core::mem::transmute(dwrate),
            ::core::mem::transmute(dwscale),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_ABORT: i32 = -10i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADBITDEPTH: i32 = -200i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADFLAGS: i32 = -5i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADFORMAT: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADHANDLE: i32 = -8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADIMAGESIZE: i32 = -201i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADPARAM: i32 = -6i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_BADSIZE: i32 = -7i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_CANTUPDATE: i32 = -9i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_CUSTOM: i32 = -400i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_DONTDRAW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_ERROR: i32 = -100i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_GOTOKEYFRAME: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_INTERNAL: i32 = -4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_MEMORY: i32 = -3i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_NEWPALETTE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_OK: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_STOPDRAWING: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICERR_UNSUPPORTED: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICGetDisplayFormat<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbiout: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, bitdepth: i32, dx: i32, dy: i32) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICGetDisplayFormat(hic: HIC, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbiout: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER, bitdepth: i32, dx: i32, dy: i32) -> HIC;
        }
        ::core::mem::transmute(ICGetDisplayFormat(hic.into_param().abi(), ::core::mem::transmute(lpbiin), ::core::mem::transmute(lpbiout), ::core::mem::transmute(bitdepth), ::core::mem::transmute(dx), ::core::mem::transmute(dy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICGetInfo<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, picinfo: *mut ICINFO, cb: u32) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICGetInfo(hic: HIC, picinfo: *mut ICINFO, cb: u32) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ICGetInfo(hic.into_param().abi(), ::core::mem::transmute(picinfo), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
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
impl ::core::marker::Copy for ICINFO {}
impl ::core::clone::Clone for ICINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICINFO").field("dwSize", &self.dwSize).field("fccType", &self.fccType).field("fccHandler", &self.fccHandler).field("dwFlags", &self.dwFlags).field("dwVersion", &self.dwVersion).field("dwVersionICM", &self.dwVersionICM).field("szName", &self.szName).field("szDescription", &self.szDescription).field("szDriver", &self.szDriver).finish()
    }
}
unsafe impl ::windows::core::Abi for ICINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICINFO {}
impl ::core::default::Default for ICINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICINSTALL_DRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICINSTALL_DRIVERW: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICINSTALL_FUNCTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICINSTALL_HDRV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICINSTALL_UNICODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICImageCompress<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, uiflags: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO, lpbits: *const ::core::ffi::c_void, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO, lquality: i32, plsize: *mut i32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICImageCompress(hic: HIC, uiflags: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO, lpbits: *const ::core::ffi::c_void, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO, lquality: i32, plsize: *mut i32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(ICImageCompress(hic.into_param().abi(), ::core::mem::transmute(uiflags), ::core::mem::transmute(lpbiin), ::core::mem::transmute(lpbits), ::core::mem::transmute(lpbiout), ::core::mem::transmute(lquality), ::core::mem::transmute(plsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICImageDecompress<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, uiflags: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO, lpbits: *const ::core::ffi::c_void, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICImageDecompress(hic: HIC, uiflags: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO, lpbits: *const ::core::ffi::c_void, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(ICImageDecompress(hic.into_param().abi(), ::core::mem::transmute(uiflags), ::core::mem::transmute(lpbiin), ::core::mem::transmute(lpbits), ::core::mem::transmute(lpbiout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICInfo(fcctype: u32, fcchandler: u32, lpicinfo: *mut ICINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICInfo(fcctype: u32, fcchandler: u32, lpicinfo: *mut ICINFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ICInfo(::core::mem::transmute(fcctype), ::core::mem::transmute(fcchandler), ::core::mem::transmute(lpicinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICInstall<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(fcctype: u32, fcchandler: u32, lparam: Param2, szdesc: Param3, wflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICInstall(fcctype: u32, fcchandler: u32, lparam: super::super::Foundation::LPARAM, szdesc: ::windows::core::PCSTR, wflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ICInstall(::core::mem::transmute(fcctype), ::core::mem::transmute(fcchandler), lparam.into_param().abi(), szdesc.into_param().abi(), ::core::mem::transmute(wflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICLocate(fcctype: u32, fcchandler: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, wflags: u16) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICLocate(fcctype: u32, fcchandler: u32, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbiout: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, wflags: u16) -> HIC;
        }
        ::core::mem::transmute(ICLocate(::core::mem::transmute(fcctype), ::core::mem::transmute(fcchandler), ::core::mem::transmute(lpbiin), ::core::mem::transmute(lpbiout), ::core::mem::transmute(wflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_ABOUT_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_CHOOSE_ALLCOMPRESSORS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_CHOOSE_DATARATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_CHOOSE_KEYFRAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_CHOOSE_PREVIEW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_COMPVARS_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMF_CONFIGURE_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_COMPRESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_DECOMPRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_DRAW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_FASTCOMPRESS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_FASTDECOMPRESS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_INTERNALF_FUNCTION32: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_INTERNALF_MASK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICMODE_QUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_ABOUT: u32 = 20491u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS: u32 = 16392u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_BEGIN: u32 = 16391u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_END: u32 = 16393u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_FRAMES: u32 = 16455u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_FRAMES_INFO: u32 = 16454u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_GET_FORMAT: u32 = 16388u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_GET_SIZE: u32 = 16389u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_COMPRESS_QUERY: u32 = 16390u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_CONFIGURE: u32 = 20490u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS: u32 = 16397u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESSEX: u32 = 16446u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESSEX_BEGIN: u32 = 16444u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESSEX_END: u32 = 16447u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESSEX_QUERY: u32 = 16445u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS_BEGIN: u32 = 16396u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS_END: u32 = 16398u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS_GET_FORMAT: u32 = 16394u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS_GET_PALETTE: u32 = 16414u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS_QUERY: u32 = 16395u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DECOMPRESS_SET_PALETTE: u32 = 16413u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW: u32 = 16417u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_BEGIN: u32 = 16399u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_BITS: u32 = 16404u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_CHANGEPALETTE: u32 = 16435u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_END: u32 = 16405u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_FLUSH: u32 = 16421u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_GETTIME: u32 = 16416u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_GET_PALETTE: u32 = 16400u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_IDLE: u32 = 16436u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_QUERY: u32 = 16415u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_REALIZE: u32 = 16420u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_RENDERBUFFER: u32 = 16422u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_SETTIME: u32 = 16419u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_START: u32 = 16402u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_START_PLAY: u32 = 16423u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_STOP: u32 = 16403u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_STOP_PLAY: u32 = 16424u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_SUGGESTFORMAT: u32 = 16434u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_UPDATE: u32 = 16401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_DRAW_WINDOW: u32 = 16418u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_ENUMFORMATS: u32 = 20501u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GET: u32 = 20521u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETBUFFERSWANTED: u32 = 16425u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETDEFAULTKEYFRAMERATE: u32 = 16426u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETDEFAULTQUALITY: u32 = 20510u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETERRORTEXT: u32 = 20492u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETFORMATNAME: u32 = 20500u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETINFO: u32 = 20482u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETQUALITY: u32 = 20511u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_GETSTATE: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_RESERVED: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_RESERVED_HIGH: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_RESERVED_LOW: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_SET: u32 = 20520u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_SETQUALITY: u32 = 20512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_SETSTATE: u32 = 20481u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_SET_STATUS_PROC: u32 = 16456u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICM_USER: u32 = 16384u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ICOPEN {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: super::super::Foundation::LRESULT,
    pub pV1Reserved: *mut ::core::ffi::c_void,
    pub pV2Reserved: *mut ::core::ffi::c_void,
    pub dnDevNode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ICOPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ICOPEN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ICOPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICOPEN").field("dwSize", &self.dwSize).field("fccType", &self.fccType).field("fccHandler", &self.fccHandler).field("dwVersion", &self.dwVersion).field("dwFlags", &self.dwFlags).field("dwError", &self.dwError).field("pV1Reserved", &self.pV1Reserved).field("pV2Reserved", &self.pV2Reserved).field("dnDevNode", &self.dnDevNode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ICOPEN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ICOPEN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICOPEN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ICOPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ICOPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn ICOpen(fcctype: u32, fcchandler: u32, wmode: u32) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICOpen(fcctype: u32, fcchandler: u32, wmode: u32) -> HIC;
        }
        ::core::mem::transmute(ICOpen(::core::mem::transmute(fcctype), ::core::mem::transmute(fcchandler), ::core::mem::transmute(wmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICOpenFunction(fcctype: u32, fcchandler: u32, wmode: u32, lpfnhandler: super::super::Foundation::FARPROC) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICOpenFunction(fcctype: u32, fcchandler: u32, wmode: u32, lpfnhandler: ::windows::core::RawPtr) -> HIC;
        }
        ::core::mem::transmute(ICOpenFunction(::core::mem::transmute(fcctype), ::core::mem::transmute(fcchandler), ::core::mem::transmute(wmode), ::core::mem::transmute(lpfnhandler)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICPALETTE {
    pub dwFlags: u32,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: *mut super::super::Graphics::Gdi::PALETTEENTRY,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for ICPALETTE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for ICPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICPALETTE").field("dwFlags", &self.dwFlags).field("iStart", &self.iStart).field("iLen", &self.iLen).field("lppe", &self.lppe).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::core::Abi for ICPALETTE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICPALETTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICPALETTE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICPALETTE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICQUALITY_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICQUALITY_HIGH: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICQUALITY_LOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICRemove(fcctype: u32, fcchandler: u32, wflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICRemove(fcctype: u32, fcchandler: u32, wflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ICRemove(::core::mem::transmute(fcctype), ::core::mem::transmute(fcchandler), ::core::mem::transmute(wflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ICSETSTATUSPROC {
    pub dwFlags: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub Status: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ICSETSTATUSPROC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ICSETSTATUSPROC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ICSETSTATUSPROC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICSETSTATUSPROC").field("dwFlags", &self.dwFlags).field("lParam", &self.lParam).field("Status", &self.Status).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ICSETSTATUSPROC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ICSETSTATUSPROC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICSETSTATUSPROC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ICSETSTATUSPROC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ICSETSTATUSPROC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICSTATUS_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICSTATUS_ERROR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICSTATUS_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICSTATUS_STATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICSTATUS_YIELD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICSendMessage<'a, Param0: ::windows::core::IntoParam<'a, HIC>>(hic: Param0, msg: u32, dw1: usize, dw2: usize) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSendMessage(hic: HIC, msg: u32, dw1: usize, dw2: usize) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(ICSendMessage(hic.into_param().abi(), ::core::mem::transmute(msg), ::core::mem::transmute(dw1), ::core::mem::transmute(dw2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICSeqCompressFrame(pc: *const COMPVARS, uiflags: u32, lpbits: *const ::core::ffi::c_void, pfkey: *mut super::super::Foundation::BOOL, plsize: *mut i32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSeqCompressFrame(pc: *const COMPVARS, uiflags: u32, lpbits: *const ::core::ffi::c_void, pfkey: *mut super::super::Foundation::BOOL, plsize: *mut i32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(ICSeqCompressFrame(::core::mem::transmute(pc), ::core::mem::transmute(uiflags), ::core::mem::transmute(lpbits), ::core::mem::transmute(pfkey), ::core::mem::transmute(plsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICSeqCompressFrameEnd(pc: *const COMPVARS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSeqCompressFrameEnd(pc: *const COMPVARS);
        }
        ICSeqCompressFrameEnd(::core::mem::transmute(pc))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICSeqCompressFrameStart(pc: *const COMPVARS, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSeqCompressFrameStart(pc: *const COMPVARS, lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ICSeqCompressFrameStart(::core::mem::transmute(pc), ::core::mem::transmute(lpbiin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ICVERSION: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFILTERCHOOSE_BTN_DELNAME: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFILTERCHOOSE_BTN_HELP: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFILTERCHOOSE_BTN_SETNAME: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFILTERCHOOSE_CMB_CUSTOM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFILTERCHOOSE_CMB_FILTER: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFILTERCHOOSE_CMB_FILTERTAG: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFORMATCHOOSE_BTN_DELNAME: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFORMATCHOOSE_BTN_HELP: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFORMATCHOOSE_BTN_SETNAME: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFORMATCHOOSE_CMB_CUSTOM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFORMATCHOOSE_CMB_FORMAT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDD_ACMFORMATCHOOSE_CMB_FORMATTAG: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_AUDIO_DROP_COMPERROR: u32 = 442u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_AUDIO_DROP_ERROR: u32 = 441u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_AVI_DRAWDIB_ERROR: u32 = 439u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_AVI_INIT_ERROR: u32 = 433u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_BEGIN: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_CANTOPEN: u32 = 409u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_COMPRESSOR_ERROR: u32 = 440u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_DEFAVIEXT: u32 = 407u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_DEFPALEXT: u32 = 408u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_DRIVER_ERROR: u32 = 418u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_END: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_ERRORDIBSAVE: u32 = 406u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_ERRORPALOPEN: u32 = 404u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_ERRORPALSAVE: u32 = 405u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_FILEEXISTS: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_FILE_OPEN_ERROR: u32 = 429u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_FILE_WRITE_ERROR: u32 = 430u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_INFO: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_MCI_CANT_STEP_ERROR: u32 = 437u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_MCI_CONTROL_ERROR: u32 = 436u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_NODISKSPACE: u32 = 415u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_NO_AUDIO_CAP_ERROR: u32 = 438u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_NO_FRAME_CAP_ERROR: u32 = 434u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_NO_PALETTE_WARN: u32 = 435u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_OUTOFMEM: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_READONLYFILE: u32 = 413u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_RECORDING_ERROR: u32 = 431u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_RECORDING_ERROR2: u32 = 432u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_SAVEASPERCENT: u32 = 417u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_SEQ_MSGSTART: u32 = 410u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_SEQ_MSGSTOP: u32 = 411u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_SETFILESIZE: u32 = 416u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_CAP_AUDIO: u32 = 509u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_CAP_FINI: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_CAP_INIT: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_CAP_L_FRAMES: u32 = 508u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_FRAMESDROPPED: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_I_FRAMES: u32 = 506u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_LIVE_MODE: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_L_FRAMES: u32 = 507u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_OPTPAL_BUILD: u32 = 505u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_OVERLAY_MODE: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_PALETTE_BUILD: u32 = 504u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_VIDEOAUDIO: u32 = 511u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_VIDEOCURRENT: u32 = 510u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_STAT_VIDEOONLY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_VIDEDITERR: u32 = 412u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_VIDEO_ADD_ERROR: u32 = 427u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_VIDEO_ALLOC_ERROR: u32 = 425u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_VIDEO_OPEN_ERROR: u32 = 424u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_VIDEO_PREPARE_ERROR: u32 = 426u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_VIDEO_SIZE_ERROR: u32 = 428u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_WAVE_ADD_ERROR: u32 = 422u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_WAVE_ALLOC_ERROR: u32 = 420u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_WAVE_OPEN_ERROR: u32 = 419u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_WAVE_PREPARE_ERROR: u32 = 421u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_WAVE_SIZE_ERROR: u32 = 423u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const IDS_CAP_WRITEERROR: u32 = 414u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[repr(transparent)]
pub struct IGetFrame(::windows::core::IUnknown);
impl IGetFrame {
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn GetFrame(&self, lpos: i32) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetFrame)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpos)))
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn Begin(&self, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin)(::core::mem::transmute_copy(self), ::core::mem::transmute(lstart), ::core::mem::transmute(lend), ::core::mem::transmute(lrate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
    pub unsafe fn End(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetFormat(&self, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpbi), ::core::mem::transmute(lpbits), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(dx), ::core::mem::transmute(dy)).ok()
    }
}
impl ::core::convert::From<IGetFrame> for ::windows::core::IUnknown {
    fn from(value: IGetFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGetFrame> for ::windows::core::IUnknown {
    fn from(value: &IGetFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGetFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGetFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGetFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for IGetFrame {
    type Vtable = IGetFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020023_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetFrame_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpos: i32) -> *mut ::core::ffi::c_void,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetFormat: usize,
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct IMAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for IMAADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for IMAADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for IMAADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for IMAADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for IMAADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for IMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JDD_CONFIGCHANGED: u32 = 2307u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JDD_GETDEVCAPS: u32 = 2050u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JDD_GETNUMDEVS: u32 = 2049u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JDD_GETPOS: u32 = 2305u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JDD_GETPOSEX: u32 = 2308u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JDD_SETCALIBRATION: u32 = 2306u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_00: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP0: u32 = 65504u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP1: u32 = 65505u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP2: u32 = 65506u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP3: u32 = 65507u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP4: u32 = 65508u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP5: u32 = 65509u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP6: u32 = 65510u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_APP7: u32 = 65511u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_COM: u32 = 65534u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_DAC: u32 = 65484u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_DHP: u32 = 65502u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_DHT: u32 = 65476u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_DNL: u32 = 65500u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_DQT: u32 = 65499u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_DRI: u32 = 65501u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_EOI: u32 = 65497u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_EXP: u32 = 65503u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_FF: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG: u32 = 65480u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG0: u32 = 65520u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG1: u32 = 65521u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG10: u32 = 65530u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG11: u32 = 65531u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG12: u32 = 65532u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG13: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG2: u32 = 65522u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG3: u32 = 65523u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG4: u32 = 65524u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG5: u32 = 65525u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG6: u32 = 65526u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG7: u32 = 65527u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG8: u32 = 65528u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_JPG9: u32 = 65529u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RES: u32 = 65282u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST0: u32 = 65488u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST1: u32 = 65489u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST2: u32 = 65490u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST3: u32 = 65491u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST4: u32 = 65492u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST5: u32 = 65493u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST6: u32 = 65494u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_RST7: u32 = 65495u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF0: u32 = 65472u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF1: u32 = 65473u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF10: u32 = 65482u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF11: u32 = 65483u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF13: u32 = 65485u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF14: u32 = 65486u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF15: u32 = 65487u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF2: u32 = 65474u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF3: u32 = 65475u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF5: u32 = 65477u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF6: u32 = 65478u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF7: u32 = 65479u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOF9: u32 = 65481u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOI: u32 = 65496u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_SOS: u32 = 65498u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JIFMK_TEM: u32 = 65281u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOYCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [super::super::Foundation::CHAR; 32],
    pub szOEMVxD: [super::super::Foundation::CHAR; 260],
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOYCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOYCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOYCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOYCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOYCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOYCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOYCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct JOYCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for JOYCAPS2W {}
impl ::core::clone::Clone for JOYCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOYCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOYCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOYCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOYCAPS2W {}
impl ::core::default::Default for JOYCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOYCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [super::super::Foundation::CHAR; 32],
    pub szOEMVxD: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOYCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOYCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOYCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOYCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOYCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOYCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOYCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct JOYCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
}
impl ::core::marker::Copy for JOYCAPSW {}
impl ::core::clone::Clone for JOYCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOYCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOYCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOYCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOYCAPSW {}
impl ::core::default::Default for JOYCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_HASPOV: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_HASR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_HASU: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_HASV: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_HASZ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_POV4DIR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYCAPS_POVCTS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYERR_NOCANDO: u32 = 166u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYERR_NOERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYERR_PARMS: u32 = 165u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYERR_UNPLUGGED: u32 = 167u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct JOYINFO {
    pub wXpos: u32,
    pub wYpos: u32,
    pub wZpos: u32,
    pub wButtons: u32,
}
impl ::core::marker::Copy for JOYINFO {}
impl ::core::clone::Clone for JOYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOYINFO {}
impl ::core::default::Default for JOYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct JOYINFOEX {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwXpos: u32,
    pub dwYpos: u32,
    pub dwZpos: u32,
    pub dwRpos: u32,
    pub dwUpos: u32,
    pub dwVpos: u32,
    pub dwButtons: u32,
    pub dwButtonNumber: u32,
    pub dwPOV: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for JOYINFOEX {}
impl ::core::clone::Clone for JOYINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOYINFOEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOYINFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOYINFOEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOYINFOEX {}
impl ::core::default::Default for JOYINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYSTICKID1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOYSTICKID2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON10: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON11: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON12: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON13: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON14: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON15: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON16: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON17: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON18: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON19: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON1CHG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON20: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON21: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON22: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON23: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON24: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON25: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON26: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON27: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON28: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON29: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON2CHG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON30: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON31: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON32: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON3CHG: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON4CHG: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON5: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON6: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON7: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON8: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_BUTTON9: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READ3: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READ4: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READ5: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READ6: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READALWAYS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READRONLY: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READUONLY: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READVONLY: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READXONLY: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READXYONLY: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READYONLY: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CAL_READZONLY: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_CONFIGCHANGED_MSGSTRING: &'static str = "MSJSTICK_VJOYD_MSGSTR";
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_POVBACKWARD: u32 = 18000u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_POVFORWARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_POVLEFT: u32 = 27000u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_POVRIGHT: u32 = 9000u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNBUTTONS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNCENTERED: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNPOV: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNPOVCTS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNRAWDATA: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNU: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNV: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_RETURNZ: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JOY_USEDEADZONE: i32 = 2048i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct JPEGINFOHEADER {
    pub JPEGSize: u32,
    pub JPEGProcess: u32,
    pub JPEGColorSpaceID: u32,
    pub JPEGBitsPerSample: u32,
    pub JPEGHSubSampling: u32,
    pub JPEGVSubSampling: u32,
}
impl ::core::marker::Copy for JPEGINFOHEADER {}
impl ::core::clone::Clone for JPEGINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JPEGINFOHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JPEGINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JPEGINFOHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for JPEGINFOHEADER {}
impl ::core::default::Default for JPEGINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JPEG_PROCESS_BASELINE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JPEG_RGB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JPEG_Y: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const JPEG_YCbCr: u32 = 2u32;
pub const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000003_0000_0010_8000_00aa00389b71);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFNEXTDEVIO = ::core::option::Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, dwflags: u32, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPMMIOPROC = ::core::option::Option<unsafe extern "system" fn(lpmmioinfo: ::windows::core::PCSTR, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub type LPTASKCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwinst: usize)>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_AUDIOERROR: u32 = 619u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_BADPALETTE: u32 = 620u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_CANTPLAYFULLSCREEN: u32 = 615u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_DISPLAYERROR: u32 = 618u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_NOCOMPRESSOR: u32 = 617u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_NODISPDIB: u32 = 614u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_NOTINTERLEAVED: u32 = 613u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_OLDAVIFORMAT: u32 = 612u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_AVI_TOOBIGFORVGA: u32 = 616u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_BAD_CONSTANT: u32 = 290u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_BAD_INTEGER: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_BAD_TIME_FORMAT: u32 = 293u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_CANNOT_LOAD_DRIVER: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_CANNOT_USE_ALL: u32 = 279u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_CREATEWINDOW: u32 = 347u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_CUSTOM_DRIVER_BASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_LENGTH: u32 = 310u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_LOCKED: u32 = 288u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_NOT_INSTALLED: u32 = 306u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_NOT_READY: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_OPEN: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_ORD_LENGTH: u32 = 311u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DEVICE_TYPE_REQUIRED: u32 = 287u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DGV_BAD_CLIPBOARD_RANGE: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DGV_DEVICE_LIMIT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DGV_DEVICE_MEMORY_FULL: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DGV_DISK_FULL: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DGV_IOERR: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DGV_WORKSPACE_EMPTY: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DRIVER: u32 = 278u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DRIVER_INTERNAL: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DUPLICATE_ALIAS: u32 = 289u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_DUPLICATE_FLAGS: u32 = 295u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_EXTENSION_NOT_FOUND: u32 = 281u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_EXTRA_CHARACTERS: u32 = 305u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_FILENAME_REQUIRED: u32 = 304u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_FILE_NOT_FOUND: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_FILE_NOT_SAVED: u32 = 286u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_FILE_READ: u32 = 348u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_FILE_WRITE: u32 = 349u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_GET_CD: u32 = 307u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_HARDWARE: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_ILLEGAL_FOR_AUTO_OPEN: u32 = 303u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_INTERNAL: u32 = 277u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_INVALID_DEVICE_ID: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_INVALID_DEVICE_NAME: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_INVALID_FILE: u32 = 296u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_MISSING_COMMAND_STRING: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_MISSING_DEVICE_NAME: u32 = 292u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_MISSING_PARAMETER: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_MISSING_STRING_ARGUMENT: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_MULTIPLE: u32 = 280u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_MUST_USE_SHAREABLE: u32 = 291u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NEW_REQUIRES_ALIAS: u32 = 299u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NONAPPLICABLE_FUNCTION: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NOTIFY_ON_AUTO_OPEN: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NO_CLOSING_QUOTE: u32 = 294u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NO_ELEMENT_ALLOWED: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NO_IDENTITY: u32 = 350u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NO_INTEGER: u32 = 312u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NO_WINDOW: u32 = 346u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_NULL_PARAMETER_BLOCK: u32 = 297u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_OUTOFRANGE: u32 = 282u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_OUT_OF_MEMORY: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_PARAM_OVERFLOW: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_PARSER_INTERNAL: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_DIV_INCOMPATIBLE: u32 = 336u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_NOMIDIPRESENT: u32 = 343u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_PORTUNSPECIFIED: u32 = 342u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_PORT_INUSE: u32 = 337u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_PORT_MAPNODEVICE: u32 = 339u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_PORT_MISCERROR: u32 = 340u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_PORT_NONEXISTENT: u32 = 338u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SEQ_TIMER: u32 = 341u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SET_CD: u32 = 308u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_SET_DRIVE: u32 = 309u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_UNNAMED_RESOURCE: u32 = 298u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_UNRECOGNIZED_COMMAND: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_UNRECOGNIZED_KEYWORD: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_UNSUPPORTED_FUNCTION: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_INPUTSINUSE: u32 = 322u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_INPUTSUNSUITABLE: u32 = 328u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_INPUTUNSPECIFIED: u32 = 325u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_OUTPUTSINUSE: u32 = 320u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_OUTPUTSUNSUITABLE: u32 = 326u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_OUTPUTUNSPECIFIED: u32 = 324u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_SETINPUTINUSE: u32 = 323u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_SETINPUTUNSUITABLE: u32 = 329u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_SETOUTPUTINUSE: u32 = 321u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIERR_WAVE_SETOUTPUTUNSUITABLE: u32 = 327u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOAUTOSIZEMOVIE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOAUTOSIZEWINDOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOERRORDLG: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOMENU: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOOPEN: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOPLAYBAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYALL: u32 = 7936u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYANSI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYERROR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYMEDIA: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYMEDIAA: u32 = 2176u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYMEDIAW: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYMODE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYPOS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_NOTIFYSIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_RECORD: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_SHOWALL: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_SHOWMODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_SHOWNAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDF_SHOWPOS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CAN_CONFIG: u32 = 1173u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CAN_EJECT: u32 = 1172u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CAN_PLAY: u32 = 1168u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CAN_RECORD: u32 = 1170u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CAN_SAVE: u32 = 1171u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CAN_WINDOW: u32 = 1169u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_CHANGESTYLES: u32 = 1159u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_EJECT: u32 = 1131u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETACTIVETIMER: u32 = 1156u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETALIAS: u32 = 1161u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETDEVICE: u32 = 1249u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETDEVICEA: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETDEVICEID: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETDEVICEW: u32 = 1249u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETEND: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETERROR: u32 = 1252u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETERRORA: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETERRORW: u32 = 1252u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETFILENAME: u32 = 1248u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETFILENAMEA: u32 = 1148u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETFILENAMEW: u32 = 1248u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETINACTIVETIMER: u32 = 1157u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETLENGTH: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETMODE: u32 = 1230u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETMODEA: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETMODEW: u32 = 1230u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETPALETTE: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETPOSITION: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETPOSITIONA: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETPOSITIONW: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETREPEAT: u32 = 1139u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETSPEED: u32 = 1137u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETSTART: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETSTYLES: u32 = 1160u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETTIMEFORMAT: u32 = 1244u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETTIMEFORMATA: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETTIMEFORMATW: u32 = 1244u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETVOLUME: u32 = 1135u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GETZOOM: u32 = 1133u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GET_DEST: u32 = 1166u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_GET_SOURCE: u32 = 1164u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NEW: u32 = 1258u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NEWA: u32 = 1158u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NEWW: u32 = 1258u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NOTIFYERROR: u32 = 1229u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NOTIFYMEDIA: u32 = 1227u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NOTIFYMODE: u32 = 1224u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NOTIFYPOS: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_NOTIFYSIZE: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_OPEN: u32 = 1276u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_OPENA: u32 = 1177u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_OPENINTERFACE: u32 = 1175u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_OPENW: u32 = 1276u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_PALETTEKICK: u32 = 1174u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_PLAYFROM: u32 = 1146u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_PLAYREVERSE: u32 = 1163u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_PLAYTO: u32 = 1147u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_PUT_DEST: u32 = 1167u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_PUT_SOURCE: u32 = 1165u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_REALIZE: u32 = 1142u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_RETURNSTRING: u32 = 1262u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_RETURNSTRINGA: u32 = 1162u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_RETURNSTRINGW: u32 = 1262u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SENDSTRING: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SENDSTRINGA: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SENDSTRINGW: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETACTIVETIMER: u32 = 1154u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETINACTIVETIMER: u32 = 1155u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETOWNER: u32 = 1176u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETPALETTE: u32 = 1151u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETREPEAT: u32 = 1138u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETSPEED: u32 = 1136u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETTIMEFORMAT: u32 = 1243u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETTIMEFORMATA: u32 = 1143u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETTIMEFORMATW: u32 = 1243u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETTIMERS: u32 = 1153u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETVOLUME: u32 = 1134u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_SETZOOM: u32 = 1132u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDM_VALIDATEMEDIA: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWNDOPENF_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWND_END: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWND_START: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCIWND_WINDOW_CLASS: &'static str = "MCIWndClass";
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MCIWndCreateA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hwndparent: Param0, hinstance: Param1, dwstyle: u32, szfile: Param3) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MCIWndCreateA(hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, dwstyle: u32, szfile: ::windows::core::PCSTR) -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(MCIWndCreateA(hwndparent.into_param().abi(), hinstance.into_param().abi(), ::core::mem::transmute(dwstyle), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MCIWndCreateW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hwndparent: Param0, hinstance: Param1, dwstyle: u32, szfile: Param3) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MCIWndCreateW(hwndparent: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, dwstyle: u32, szfile: ::windows::core::PCWSTR) -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(MCIWndCreateW(hwndparent.into_param().abi(), hinstance.into_param().abi(), ::core::mem::transmute(dwstyle), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MCIWndRegisterClass() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MCIWndRegisterClass() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MCIWndRegisterClass())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_CAN_REVERSE: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_CAN_STRETCH: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_FAST_RATE: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_MAX_WINDOWS: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_NORMAL_RATE: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_PALETTES: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_GETDEVCAPS_SLOW_RATE: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_INFO_TEXT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_OPEN_NOSTATIC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_OPEN_PARENT: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_ANIM_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCSTR,
    pub lpstrElementName: ::windows::core::PCSTR,
    pub lpstrAlias: ::windows::core::PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_ANIM_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_ANIM_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_ANIM_OPEN_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_ANIM_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_OPEN_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_ANIM_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_ANIM_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCWSTR,
    pub lpstrElementName: ::windows::core::PCWSTR,
    pub lpstrAlias: ::windows::core::PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_ANIM_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_ANIM_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_ANIM_OPEN_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_ANIM_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_OPEN_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_ANIM_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_OPEN_WS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PLAY_FAST: i32 = 262144i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_ANIM_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
impl ::core::marker::Copy for MCI_ANIM_PLAY_PARMS {}
impl ::core::clone::Clone for MCI_ANIM_PLAY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_ANIM_PLAY_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_ANIM_PLAY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_PLAY_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_ANIM_PLAY_PARMS {}
impl ::core::default::Default for MCI_ANIM_PLAY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PLAY_REVERSE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PLAY_SCAN: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PLAY_SLOW: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PLAY_SPEED: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PUT_DESTINATION: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_PUT_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_REALIZE_BKGD: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_REALIZE_NORM: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_RECT: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_ANIM_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_ANIM_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_ANIM_RECT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_ANIM_RECT_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_ANIM_RECT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_RECT_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_ANIM_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STATUS_FORWARD: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STATUS_HPAL: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STATUS_HWND: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STATUS_SPEED: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STATUS_STRETCH: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STEP_FRAMES: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_ANIM_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for MCI_ANIM_STEP_PARMS {}
impl ::core::clone::Clone for MCI_ANIM_STEP_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_ANIM_STEP_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_ANIM_STEP_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_STEP_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_ANIM_STEP_PARMS {}
impl ::core::default::Default for MCI_ANIM_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_STEP_REVERSE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_UPDATE_HDC: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct MCI_ANIM_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for MCI_ANIM_UPDATE_PARMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for MCI_ANIM_UPDATE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for MCI_ANIM_UPDATE_PARMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for MCI_ANIM_UPDATE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_UPDATE_PARMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for MCI_ANIM_UPDATE_PARMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MCI_ANIM_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WHERE_DESTINATION: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WHERE_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WINDOW_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WINDOW_DISABLE_STRETCH: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WINDOW_ENABLE_STRETCH: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WINDOW_HWND: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_ANIM_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_ANIM_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_ANIM_WINDOW_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_ANIM_WINDOW_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_ANIM_WINDOW_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_WINDOW_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_ANIM_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_ANIM_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_ANIM_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_ANIM_WINDOW_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_ANIM_WINDOW_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_ANIM_WINDOW_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_ANIM_WINDOW_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_ANIM_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_ANIM_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WINDOW_STATE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ANIM_WINDOW_TEXT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_AVI_SETVIDEO_DRAW_PROCEDURE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_AVI_SETVIDEO_PALETTE_COLOR: i32 = 33024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_AVI_SETVIDEO_PALETTE_HALFTONE: i32 = 65535i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_AVI_STATUS_AUDIO_BREAKS: i32 = 32771i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_AVI_STATUS_FRAMES_SKIPPED: i32 = 32769i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_AVI_STATUS_LAST_PLAY_SPEED: i32 = 32770i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_BREAK: u32 = 2065u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_BREAK_HWND: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_BREAK_KEY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_BREAK_OFF: i32 = 1024i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_BREAK_PARMS {
    pub dwCallback: usize,
    pub nVirtKey: i32,
    pub hwndBreak: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_BREAK_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_BREAK_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_BREAK_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_BREAK_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_BREAK_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_BREAK_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_BREAK_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CAPTURE: u32 = 2160u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CDA_STATUS_TYPE_TRACK: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CDA_TRACK_AUDIO: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CDA_TRACK_OTHER: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CLOSE: u32 = 2052u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CLOSE_DRIVER: u32 = 2050u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_COLONIZED3_RETURN: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_COLONIZED4_RETURN: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_COMMAND_HEAD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CONFIGURE: u32 = 2170u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CONSTANT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_COPY: u32 = 2130u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CUE: u32 = 2096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_CUT: u32 = 2129u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DELETE: u32 = 2134u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_ANIMATION: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_CD_AUDIO: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_DAT: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_DIGITAL_VIDEO: u32 = 520u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_FIRST: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_FIRST_USER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_LAST: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_OTHER: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_OVERLAY: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_SCANNER: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_SEQUENCER: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_VCR: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_VIDEODISC: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DEVTYPE_WAVEFORM_AUDIO: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CAPTURE_AS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CAPTURE_AT: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_CAPTURE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: ::windows::core::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_CAPTURE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_CAPTURE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_CAPTURE_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_CAPTURE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_CAPTURE_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_CAPTURE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_CAPTURE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_CAPTURE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: ::windows::core::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_CAPTURE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_CAPTURE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_CAPTURE_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_CAPTURE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_CAPTURE_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_CAPTURE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_CAPTURE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_COPY_AT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_COPY_AUDIO_STREAM: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_COPY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_COPY_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_COPY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_COPY_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_COPY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_COPY_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_COPY_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_COPY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_COPY_VIDEO_STREAM: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CUE_INPUT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CUE_NOSHOW: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CUE_OUTPUT: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_CUE_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_DGV_CUE_PARMS {}
impl ::core::clone::Clone for MCI_DGV_CUE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_CUE_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_CUE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_CUE_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_CUE_PARMS {}
impl ::core::default::Default for MCI_DGV_CUE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CUT_AT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CUT_AUDIO_STREAM: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_CUT_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_CUT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_CUT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_CUT_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_CUT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_CUT_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_CUT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_CUT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_CUT_VIDEO_STREAM: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_DELETE_AT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_DELETE_AUDIO_STREAM: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_DELETE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_DELETE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_DELETE_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_DELETE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_DELETE_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_DELETE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_DELETE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_DELETE_VIDEO_STREAM: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_AVI: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_AVSS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_DIB: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_JFIF: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_JPEG: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_MPEG: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_RDIB: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FF_RJPEG: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_EDITING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_EDITING_S: i32 = 32774i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_IDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_IDLE_S: i32 = 32775i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_LOADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_LOADING_S: i32 = 32773i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_SAVING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_MODE_SAVING_S: i32 = 32772i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FILE_S: i32 = 32770i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FREEZE_AT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_FREEZE_OUTSIDE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_CAN_LOCK: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_CAN_REVERSE: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_CAN_STR_IN: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_CAN_TEST: i32 = 16393i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_HAS_STILL: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_MAXIMUM_RATE: i32 = 16394i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_MINIMUM_RATE: i32 = 16395i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_GETDEVCAPS_PALETTES: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_AUDIO_ALG: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_AUDIO_QUALITY: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_ITEM: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_INFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
impl ::core::marker::Copy for MCI_DGV_INFO_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_INFO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_INFO_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_INFO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_INFO_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_INFO_PARMSA {}
impl ::core::default::Default for MCI_DGV_INFO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_INFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PWSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
impl ::core::marker::Copy for MCI_DGV_INFO_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_INFO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_INFO_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_INFO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_INFO_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_INFO_PARMSW {}
impl ::core::default::Default for MCI_DGV_INFO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_STILL_ALG: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_STILL_QUALITY: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_TEXT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_USAGE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_VIDEO_ALG: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INFO_VIDEO_QUALITY: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_INPUT_S: i32 = 32771i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_ALG: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_AUDIO_ALG: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_AUDIO_QUALITY: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_AUDIO_STREAM: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_COUNT: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_ITEM: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_NUMBER: i32 = 262144i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_LIST_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: ::windows::core::PSTR,
}
impl ::core::marker::Copy for MCI_DGV_LIST_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_LIST_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_LIST_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_LIST_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_LIST_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_LIST_PARMSA {}
impl ::core::default::Default for MCI_DGV_LIST_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_LIST_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PWSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MCI_DGV_LIST_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_LIST_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_LIST_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_LIST_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_LIST_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_LIST_PARMSW {}
impl ::core::default::Default for MCI_DGV_LIST_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_STILL_ALG: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_STILL_QUALITY: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_VIDEO_ALG: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_VIDEO_QUALITY: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_VIDEO_SOURCE: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_LIST_VIDEO_STREAM: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_METHOD_DIRECT: i32 = 40962i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_METHOD_POST: i32 = 40961i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_METHOD_PRE: i32 = 40960i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_MONITOR_FILE: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_MONITOR_INPUT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_MONITOR_METHOD: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_MONITOR_PARMS {
    pub dwCallback: usize,
    pub dwSource: u32,
    pub dwMethod: u32,
}
impl ::core::marker::Copy for MCI_DGV_MONITOR_PARMS {}
impl ::core::clone::Clone for MCI_DGV_MONITOR_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_MONITOR_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_MONITOR_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_MONITOR_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_MONITOR_PARMS {}
impl ::core::default::Default for MCI_DGV_MONITOR_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_MONITOR_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_OPEN_16BIT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_OPEN_32BIT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_OPEN_NOSTATIC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_OPEN_PARENT: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PSTR,
    pub lpstrElementName: ::windows::core::PSTR,
    pub lpstrAlias: ::windows::core::PSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_OPEN_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_OPEN_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PWSTR,
    pub lpstrElementName: ::windows::core::PWSTR,
    pub lpstrAlias: ::windows::core::PWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_OPEN_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_OPEN_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_OPEN_WS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PASTE_AT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PASTE_AUDIO_STREAM: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PASTE_INSERT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PASTE_OVERWRITE: i32 = 1048576i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_PASTE_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_PASTE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_PASTE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_PASTE_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_PASTE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_PASTE_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_PASTE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_PASTE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PASTE_VIDEO_STREAM: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PLAY_REPEAT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PLAY_REVERSE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PUT_CLIENT: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PUT_DESTINATION: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PUT_FRAME: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PUT_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PUT_VIDEO: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_PUT_WINDOW: i32 = 2097152i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_QUALITY_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub lpstrName: ::windows::core::PSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
impl ::core::marker::Copy for MCI_DGV_QUALITY_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_QUALITY_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_QUALITY_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_QUALITY_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_QUALITY_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_QUALITY_PARMSA {}
impl ::core::default::Default for MCI_DGV_QUALITY_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_QUALITY_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub lpstrName: ::windows::core::PWSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
impl ::core::marker::Copy for MCI_DGV_QUALITY_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_QUALITY_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_QUALITY_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_QUALITY_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_QUALITY_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_QUALITY_PARMSW {}
impl ::core::default::Default for MCI_DGV_QUALITY_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_REALIZE_BKGD: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_REALIZE_NORM: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RECORD_AUDIO_STREAM: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RECORD_HOLD: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_RECORD_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_RECORD_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_RECORD_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_RECORD_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_RECORD_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_RECORD_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RECORD_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RECORD_VIDEO_STREAM: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RECT: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_RECT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_RECT_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_RECT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_RECT_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RESERVE_IN: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_RESERVE_PARMSA {
    pub dwCallback: usize,
    pub lpstrPath: ::windows::core::PSTR,
    pub dwSize: u32,
}
impl ::core::marker::Copy for MCI_DGV_RESERVE_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_RESERVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_RESERVE_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_RESERVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_RESERVE_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_RESERVE_PARMSA {}
impl ::core::default::Default for MCI_DGV_RESERVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_RESERVE_PARMSW {
    pub dwCallback: usize,
    pub lpstrPath: ::windows::core::PWSTR,
    pub dwSize: u32,
}
impl ::core::marker::Copy for MCI_DGV_RESERVE_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_RESERVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_RESERVE_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_RESERVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_RESERVE_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_RESERVE_PARMSW {}
impl ::core::default::Default for MCI_DGV_RESERVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RESERVE_SIZE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RESTORE_AT: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_RESTORE_FROM: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RESTORE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: ::windows::core::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_RESTORE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_RESTORE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_RESTORE_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_RESTORE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_RESTORE_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_RESTORE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RESTORE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RESTORE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: ::windows::core::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_RESTORE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_RESTORE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_RESTORE_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_RESTORE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_RESTORE_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_RESTORE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_RESTORE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SAVE_ABORT: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SAVE_KEEPRESERVE: i32 = 262144i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: ::windows::core::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_SAVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_SAVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_SAVE_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_SAVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SAVE_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_SAVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: ::windows::core::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_SAVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_SAVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_SAVE_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_SAVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SAVE_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_SAVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_ALG: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_AVGBYTESPERSEC: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_BASS: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_BITSPERSAMPLE: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_BLOCKALIGN: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_CLOCKTIME: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_INPUT: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_ITEM: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_LEFT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_OUTPUT: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_OVER: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_SETAUDIO_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: ::windows::core::PSTR,
    pub lpstrQuality: ::windows::core::PSTR,
}
impl ::core::marker::Copy for MCI_DGV_SETAUDIO_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_SETAUDIO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_SETAUDIO_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_SETAUDIO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SETAUDIO_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETAUDIO_PARMSA {}
impl ::core::default::Default for MCI_DGV_SETAUDIO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_SETAUDIO_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: ::windows::core::PWSTR,
    pub lpstrQuality: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MCI_DGV_SETAUDIO_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_SETAUDIO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_SETAUDIO_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_SETAUDIO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SETAUDIO_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETAUDIO_PARMSW {}
impl ::core::default::Default for MCI_DGV_SETAUDIO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_QUALITY: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_RECORD: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_RIGHT: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SAMPLESPERSEC: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SOURCE: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SOURCE_AVERAGE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SOURCE_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SOURCE_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SOURCE_STEREO: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SRC_AVERAGE_S: i32 = 32802i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SRC_LEFT_S: i32 = 32800i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SRC_RIGHT_S: i32 = 32801i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_SRC_STEREO_S: i32 = 32803i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_STREAM: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_TREBLE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_VALUE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETAUDIO_VOLUME: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_ALG: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_BITSPERPEL: i32 = 16396i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_BRIGHTNESS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_CLOCKTIME: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_COLOR: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_CONTRAST: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_FRAME_RATE: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_GAMMA: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_INPUT: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_ITEM: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_KEY_COLOR: i32 = 16395i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_KEY_INDEX: i32 = 16394i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_OUTPUT: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_OVER: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_PALHANDLE: i32 = 16391i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_SETVIDEO_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: ::windows::core::PSTR,
    pub lpstrQuality: ::windows::core::PSTR,
    pub dwSourceNumber: u32,
}
impl ::core::marker::Copy for MCI_DGV_SETVIDEO_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_SETVIDEO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_SETVIDEO_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_SETVIDEO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SETVIDEO_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETVIDEO_PARMSA {}
impl ::core::default::Default for MCI_DGV_SETVIDEO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_SETVIDEO_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: ::windows::core::PWSTR,
    pub lpstrQuality: ::windows::core::PWSTR,
    pub dwSourceNumber: u32,
}
impl ::core::marker::Copy for MCI_DGV_SETVIDEO_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_SETVIDEO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_SETVIDEO_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_SETVIDEO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SETVIDEO_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_SETVIDEO_PARMSW {}
impl ::core::default::Default for MCI_DGV_SETVIDEO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_QUALITY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_RECORD: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SHARPNESS: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SOURCE: i32 = 16393i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_GENERIC: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_GENERIC_S: i32 = 32789i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_NTSC: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_NTSC_S: i32 = 32784i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_NUMBER: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_PAL: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_PAL_S: i32 = 32787i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_RGB: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_RGB_S: i32 = 32785i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_SECAM: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_SECAM_S: i32 = 32788i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO_S: i32 = 32786i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_STILL: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_STREAM: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_TINT: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SETVIDEO_VALUE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SET_FILEFORMAT: i32 = 524288i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwFileFormat: u32,
    pub dwSpeed: u32,
}
impl ::core::marker::Copy for MCI_DGV_SET_PARMS {}
impl ::core::clone::Clone for MCI_DGV_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_SET_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SET_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_SET_PARMS {}
impl ::core::default::Default for MCI_DGV_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SET_SEEK_EXACTLY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SET_SPEED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SET_STILL: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SIGNAL_AT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SIGNAL_CANCEL: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SIGNAL_EVERY: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_SIGNAL_PARMS {
    pub dwCallback: usize,
    pub dwPosition: u32,
    pub dwPeriod: u32,
    pub dwUserParm: u32,
}
impl ::core::marker::Copy for MCI_DGV_SIGNAL_PARMS {}
impl ::core::clone::Clone for MCI_DGV_SIGNAL_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_SIGNAL_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_SIGNAL_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_SIGNAL_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_SIGNAL_PARMS {}
impl ::core::default::Default for MCI_DGV_SIGNAL_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SIGNAL_POSITION: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_SIGNAL_USERVAL: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_AUDIO: i32 = 16404i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_AUDIO_INPUT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_AUDIO_RECORD: i32 = 16410i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_AUDIO_SOURCE: i32 = 16393i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_AUDIO_STREAM: i32 = 16429i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_AVGBYTESPERSEC: i32 = 16424i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_BASS: i32 = 16399i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_BITSPERPEL: i32 = 16427i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_BITSPERSAMPLE: i32 = 16426i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_BLOCKALIGN: i32 = 16425i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_BRIGHTNESS: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_COLOR: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_CONTRAST: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_DISKSPACE: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_FILEFORMAT: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_FILE_COMPLETION: i32 = 16416i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_FILE_MODE: i32 = 16415i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_FORWARD: i32 = 16428i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_FRAME_RATE: i32 = 16398i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_GAMMA: i32 = 16394i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_HPAL: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_HWND: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_INPUT: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_KEY_COLOR: i32 = 16421i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_KEY_INDEX: i32 = 16420i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_LEFT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_MONITOR: i32 = 16395i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_MONITOR_METHOD: i32 = 16396i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_NOMINAL: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_OUTPUT: i32 = 8388608i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_STATUS_PARMSA {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: ::windows::core::PSTR,
    pub dwReference: u32,
}
impl ::core::marker::Copy for MCI_DGV_STATUS_PARMSA {}
impl ::core::clone::Clone for MCI_DGV_STATUS_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_STATUS_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_STATUS_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_STATUS_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_STATUS_PARMSA {}
impl ::core::default::Default for MCI_DGV_STATUS_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_STATUS_PARMSW {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: ::windows::core::PWSTR,
    pub dwReference: u32,
}
impl ::core::marker::Copy for MCI_DGV_STATUS_PARMSW {}
impl ::core::clone::Clone for MCI_DGV_STATUS_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_STATUS_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_STATUS_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_STATUS_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_STATUS_PARMSW {}
impl ::core::default::Default for MCI_DGV_STATUS_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_PAUSE_MODE: i32 = 16422i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_RECORD: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_REFERENCE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_RIGHT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_SAMPLESPERSEC: i32 = 16423i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_SEEK_EXACTLY: i32 = 16401i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_SHARPNESS: i32 = 16402i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_SIZE: i32 = 16400i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_SMPTE: i32 = 16403i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_SPEED: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_STILL_FILEFORMAT: i32 = 16413i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_TINT: i32 = 16405i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_TREBLE: i32 = 16406i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_UNSAVED: i32 = 16407i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_VIDEO: i32 = 16408i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_VIDEO_RECORD: i32 = 16412i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_VIDEO_SOURCE: i32 = 16411i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_VIDEO_SRC_NUM: i32 = 16414i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_VIDEO_STREAM: i32 = 16430i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_VOLUME: i32 = 16409i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_WINDOW_MAXIMIZED: i32 = 16419i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_WINDOW_MINIMIZED: i32 = 16418i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STATUS_WINDOW_VISIBLE: i32 = 16417i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STEP_FRAMES: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_DGV_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for MCI_DGV_STEP_PARMS {}
impl ::core::clone::Clone for MCI_DGV_STEP_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_DGV_STEP_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_DGV_STEP_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_STEP_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_DGV_STEP_PARMS {}
impl ::core::default::Default for MCI_DGV_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STEP_REVERSE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_STOP_HOLD: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_UPDATE_HDC: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_UPDATE_PAINT: i32 = 262144i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct MCI_DGV_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for MCI_DGV_UPDATE_PARMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for MCI_DGV_UPDATE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for MCI_DGV_UPDATE_PARMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for MCI_DGV_UPDATE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_UPDATE_PARMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for MCI_DGV_UPDATE_PARMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MCI_DGV_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WHERE_DESTINATION: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WHERE_FRAME: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WHERE_MAX: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WHERE_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WHERE_VIDEO: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WHERE_WINDOW: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WINDOW_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WINDOW_HWND: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_WINDOW_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_WINDOW_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_WINDOW_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_WINDOW_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_DGV_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_DGV_WINDOW_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_DGV_WINDOW_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_DGV_WINDOW_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_DGV_WINDOW_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_DGV_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_DGV_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WINDOW_STATE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_DGV_WINDOW_TEXT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_END_COMMAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_END_COMMAND_LIST: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_END_CONSTANT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ESCAPE: u32 = 2053u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FALSE: u32 = 531u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FIRST: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FLAG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_BYTES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_BYTES_S: u32 = 541u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_FRAMES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_FRAMES_S: u32 = 536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_HMS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_HMS_S: u32 = 534u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_MILLISECONDS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_MILLISECONDS_S: u32 = 533u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_MSF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_MSF_S: u32 = 535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SAMPLES: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SAMPLES_S: u32 = 542u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_24: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_24_S: u32 = 537u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_25: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_25_S: u32 = 538u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_30: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_30DROP: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_30DROP_S: u32 = 540u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_SMPTE_30_S: u32 = 539u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_TMSF: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FORMAT_TMSF_S: u32 = 543u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FREEZE: u32 = 2116u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_FROM: i32 = 4i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_GENERIC_PARMS {
    pub dwCallback: usize,
}
impl ::core::marker::Copy for MCI_GENERIC_PARMS {}
impl ::core::clone::Clone for MCI_GENERIC_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_GENERIC_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_GENERIC_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_GENERIC_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_GENERIC_PARMS {}
impl ::core::default::Default for MCI_GENERIC_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS: u32 = 2059u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_CAN_EJECT: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_CAN_PLAY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_CAN_RECORD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_CAN_SAVE: i32 = 9i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_COMPOUND_DEVICE: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_DEVICE_TYPE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_HAS_AUDIO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_HAS_VIDEO: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_ITEM: i32 = 256i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_GETDEVCAPS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: u32,
    pub dwItem: u32,
}
impl ::core::marker::Copy for MCI_GETDEVCAPS_PARMS {}
impl ::core::clone::Clone for MCI_GETDEVCAPS_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_GETDEVCAPS_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_GETDEVCAPS_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_GETDEVCAPS_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_GETDEVCAPS_PARMS {}
impl ::core::default::Default for MCI_GETDEVCAPS_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_GETDEVCAPS_USES_FILES: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_HDC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_HPAL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_HWND: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO: u32 = 2058u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_COPYRIGHT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_FILE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_MEDIA_IDENTITY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_MEDIA_UPC: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_NAME: i32 = 4096i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_INFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PSTR,
    pub dwRetSize: u32,
}
impl ::core::marker::Copy for MCI_INFO_PARMSA {}
impl ::core::clone::Clone for MCI_INFO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_INFO_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_INFO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_INFO_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_INFO_PARMSA {}
impl ::core::default::Default for MCI_INFO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_INFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PWSTR,
    pub dwRetSize: u32,
}
impl ::core::marker::Copy for MCI_INFO_PARMSW {}
impl ::core::clone::Clone for MCI_INFO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_INFO_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_INFO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_INFO_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_INFO_PARMSW {}
impl ::core::default::Default for MCI_INFO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_PRODUCT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INFO_VERSION: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INTEGER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INTEGER64: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_INTEGER_RETURNED: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_LAST: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_LIST: u32 = 2168u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_LOAD: u32 = 2128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_LOAD_FILE: i32 = 256i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for MCI_LOAD_PARMSA {}
impl ::core::clone::Clone for MCI_LOAD_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_LOAD_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_LOAD_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_LOAD_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_LOAD_PARMSA {}
impl ::core::default::Default for MCI_LOAD_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_LOAD_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for MCI_LOAD_PARMSW {}
impl ::core::clone::Clone for MCI_LOAD_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_LOAD_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_LOAD_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_LOAD_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_LOAD_PARMSW {}
impl ::core::default::Default for MCI_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MAX_DEVICE_TYPE_LENGTH: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MCIAVI_PLAY_FULLBY2: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MCIAVI_PLAY_FULLSCREEN: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MCIAVI_PLAY_WINDOW: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_NOT_READY: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_OPEN: u32 = 530u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_PAUSE: u32 = 529u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_PLAY: u32 = 526u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_RECORD: u32 = 527u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_SEEK: u32 = 528u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MODE_STOP: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_MONITOR: u32 = 2161u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_NOTIFY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_NOTIFY_ABORTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_NOTIFY_FAILURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_NOTIFY_SUCCESSFUL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_NOTIFY_SUPERSEDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OFF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OFF_S: i32 = 32769i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_ON_S: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN: u32 = 2051u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_ALIAS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_DRIVER: u32 = 2049u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_OPEN_DRIVER_PARMS {
    pub wDeviceID: u32,
    pub lpstrParams: ::windows::core::PCWSTR,
    pub wCustomCommandTable: u32,
    pub wType: u32,
}
impl ::core::marker::Copy for MCI_OPEN_DRIVER_PARMS {}
impl ::core::clone::Clone for MCI_OPEN_DRIVER_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_OPEN_DRIVER_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_OPEN_DRIVER_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OPEN_DRIVER_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_OPEN_DRIVER_PARMS {}
impl ::core::default::Default for MCI_OPEN_DRIVER_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_ELEMENT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_ELEMENT_ID: i32 = 2048i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCSTR,
    pub lpstrElementName: ::windows::core::PCSTR,
    pub lpstrAlias: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for MCI_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_OPEN_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OPEN_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_OPEN_PARMSA {}
impl ::core::default::Default for MCI_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCWSTR,
    pub lpstrElementName: ::windows::core::PCWSTR,
    pub lpstrAlias: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for MCI_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_OPEN_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OPEN_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_OPEN_PARMSW {}
impl ::core::default::Default for MCI_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_SHAREABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_TYPE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OPEN_TYPE_ID: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_INFO_TEXT: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_LOAD_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_LOAD_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_LOAD_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_LOAD_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_LOAD_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_LOAD_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_LOAD_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_LOAD_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_LOAD_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_LOAD_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_LOAD_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_LOAD_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_LOAD_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_LOAD_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_LOAD_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_LOAD_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_OPEN_PARENT: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCSTR,
    pub lpstrElementName: ::windows::core::PCSTR,
    pub lpstrAlias: ::windows::core::PCSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_OPEN_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_OPEN_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCWSTR,
    pub lpstrElementName: ::windows::core::PCWSTR,
    pub lpstrAlias: ::windows::core::PCWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_OPEN_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_OPEN_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_OPEN_WS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_PUT_DESTINATION: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_PUT_FRAME: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_PUT_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_PUT_VIDEO: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_RECT: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_RECT_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_RECT_PARMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_RECT_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_RECT_PARMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_SAVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_SAVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_SAVE_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_SAVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_SAVE_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_SAVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_SAVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_SAVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_SAVE_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_SAVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_SAVE_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_SAVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_STATUS_HWND: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_STATUS_STRETCH: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WHERE_DESTINATION: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WHERE_FRAME: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WHERE_SOURCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WHERE_VIDEO: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WINDOW_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WINDOW_DISABLE_STRETCH: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WINDOW_ENABLE_STRETCH: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WINDOW_HWND: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: ::windows::core::PCSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_WINDOW_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_WINDOW_PARMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_WINDOW_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_WINDOW_PARMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OVLY_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: ::windows::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MCI_OVLY_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MCI_OVLY_WINDOW_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MCI_OVLY_WINDOW_PARMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MCI_OVLY_WINDOW_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_OVLY_WINDOW_PARMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MCI_OVLY_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MCI_OVLY_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WINDOW_STATE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_OVLY_WINDOW_TEXT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_PASTE: u32 = 2131u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_PAUSE: u32 = 2057u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_PLAY: u32 = 2054u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_PLAY_PARMS {}
impl ::core::clone::Clone for MCI_PLAY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_PLAY_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_PLAY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_PLAY_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_PLAY_PARMS {}
impl ::core::default::Default for MCI_PLAY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_PUT: u32 = 2114u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY: u32 = 2167u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_ALG: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_DIALOG: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_HANDLE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_ITEM: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_ITEM_AUDIO: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_ITEM_STILL: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_ITEM_VIDEO: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_QUALITY_NAME: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_REALIZE: u32 = 2112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RECORD: u32 = 2063u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RECORD_INSERT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RECORD_OVERWRITE: i32 = 512i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_RECORD_PARMS {}
impl ::core::clone::Clone for MCI_RECORD_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_RECORD_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_RECORD_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_RECORD_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_RECORD_PARMS {}
impl ::core::default::Default for MCI_RECORD_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RECT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RESERVE: u32 = 2162u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RESOURCE_DRIVER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RESOURCE_RETURNED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RESTORE: u32 = 2171u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RESUME: u32 = 2133u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_RETURN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SAVE: u32 = 2067u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SAVE_FILE: i32 = 256i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for MCI_SAVE_PARMSA {}
impl ::core::clone::Clone for MCI_SAVE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SAVE_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SAVE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SAVE_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SAVE_PARMSA {}
impl ::core::default::Default for MCI_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpfilename: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for MCI_SAVE_PARMSW {}
impl ::core::clone::Clone for MCI_SAVE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SAVE_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SAVE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SAVE_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SAVE_PARMSW {}
impl ::core::default::Default for MCI_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SECTION: &'static str = "MCI32";
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEEK: u32 = 2055u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SEEK_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_SEEK_PARMS {}
impl ::core::clone::Clone for MCI_SEEK_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SEEK_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SEEK_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SEEK_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SEEK_PARMS {}
impl ::core::default::Default for MCI_SEEK_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEEK_TO_END: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEEK_TO_START: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_FILE: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_FILE_S: u32 = 1222u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_FORMAT_SONGPTR: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_FORMAT_SONGPTR_S: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_MAPPER: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_MAPPER_S: u32 = 1221u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_MIDI: u32 = 16387u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_MIDI_S: u32 = 1223u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_NONE: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_NONE_S: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SET_MASTER: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SET_OFFSET: i32 = 16777216i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SEQ_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwTempo: u32,
    pub dwPort: u32,
    pub dwSlave: u32,
    pub dwMaster: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for MCI_SEQ_SET_PARMS {}
impl ::core::clone::Clone for MCI_SEQ_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SEQ_SET_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SEQ_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SEQ_SET_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SEQ_SET_PARMS {}
impl ::core::default::Default for MCI_SEQ_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SET_PORT: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SET_SLAVE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SET_TEMPO: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SMPTE: u32 = 16388u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_SMPTE_S: u32 = 1224u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_COPYRIGHT: i32 = 16396i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_DIVTYPE: i32 = 16394i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_MASTER: i32 = 16392i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_NAME: i32 = 16395i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_OFFSET: i32 = 16393i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_PORT: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_SLAVE: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SEQ_STATUS_TEMPO: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET: u32 = 2061u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SETAUDIO: u32 = 2163u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SETVIDEO: u32 = 2166u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_AUDIO: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_AUDIO_ALL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_AUDIO_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_AUDIO_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_DOOR_CLOSED: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_DOOR_OPEN: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_OFF: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_ON: i32 = 8192i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
}
impl ::core::marker::Copy for MCI_SET_PARMS {}
impl ::core::clone::Clone for MCI_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SET_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SET_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SET_PARMS {}
impl ::core::default::Default for MCI_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_TIME_FORMAT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SET_VIDEO: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SIGNAL: u32 = 2165u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SPIN: u32 = 2060u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS: u32 = 2068u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_CURRENT_TRACK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_ITEM: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_LENGTH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_MEDIA_PRESENT: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_MODE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_NUMBER_OF_TRACKS: i32 = 3i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_STATUS_PARMS {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
}
impl ::core::marker::Copy for MCI_STATUS_PARMS {}
impl ::core::clone::Clone for MCI_STATUS_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_STATUS_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_STATUS_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_STATUS_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_STATUS_PARMS {}
impl ::core::default::Default for MCI_STATUS_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_POSITION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_READY: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_START: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STATUS_TIME_FORMAT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STEP: u32 = 2062u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STOP: u32 = 2056u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_STRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SYSINFO: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SYSINFO_INSTALLNAME: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SYSINFO_NAME: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SYSINFO_OPEN: i32 = 512i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SYSINFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
impl ::core::marker::Copy for MCI_SYSINFO_PARMSA {}
impl ::core::clone::Clone for MCI_SYSINFO_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SYSINFO_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SYSINFO_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SYSINFO_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SYSINFO_PARMSA {}
impl ::core::default::Default for MCI_SYSINFO_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_SYSINFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: ::windows::core::PWSTR,
    pub dwRetSize: u32,
    pub dwNumber: u32,
    pub wDeviceType: u32,
}
impl ::core::marker::Copy for MCI_SYSINFO_PARMSW {}
impl ::core::clone::Clone for MCI_SYSINFO_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_SYSINFO_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_SYSINFO_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_SYSINFO_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_SYSINFO_PARMSW {}
impl ::core::default::Default for MCI_SYSINFO_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_SYSINFO_QUANTITY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_TEST: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_TO: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_TRACK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_TRUE: u32 = 532u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_UNDO: u32 = 2169u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_UNFREEZE: u32 = 2117u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_UPDATE: u32 = 2132u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_USER_MESSAGES: u32 = 3072u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_VD_ESCAPE_PARMSA {
    pub dwCallback: usize,
    pub lpstrCommand: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for MCI_VD_ESCAPE_PARMSA {}
impl ::core::clone::Clone for MCI_VD_ESCAPE_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_VD_ESCAPE_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_VD_ESCAPE_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_VD_ESCAPE_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_VD_ESCAPE_PARMSA {}
impl ::core::default::Default for MCI_VD_ESCAPE_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_VD_ESCAPE_PARMSW {
    pub dwCallback: usize,
    pub lpstrCommand: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for MCI_VD_ESCAPE_PARMSW {}
impl ::core::clone::Clone for MCI_VD_ESCAPE_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_VD_ESCAPE_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_VD_ESCAPE_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_VD_ESCAPE_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_VD_ESCAPE_PARMSW {}
impl ::core::default::Default for MCI_VD_ESCAPE_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_ESCAPE_STRING: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_FORMAT_TRACK: u32 = 16385u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_FORMAT_TRACK_S: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_GETDEVCAPS_CAN_REVERSE: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_GETDEVCAPS_CAV: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_GETDEVCAPS_CLV: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_GETDEVCAPS_FAST_RATE: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_GETDEVCAPS_NORMAL_RATE: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_GETDEVCAPS_SLOW_RATE: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_MEDIA_CAV: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_MEDIA_CLV: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_MEDIA_OTHER: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_MODE_PARK: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_PLAY_FAST: i32 = 131072i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_VD_PLAY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub dwSpeed: u32,
}
impl ::core::marker::Copy for MCI_VD_PLAY_PARMS {}
impl ::core::clone::Clone for MCI_VD_PLAY_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_VD_PLAY_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_VD_PLAY_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_VD_PLAY_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_VD_PLAY_PARMS {}
impl ::core::default::Default for MCI_VD_PLAY_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_PLAY_REVERSE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_PLAY_SCAN: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_PLAY_SLOW: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_PLAY_SPEED: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_SEEK_REVERSE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_SPIN_DOWN: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_SPIN_UP: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STATUS_DISC_SIZE: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STATUS_FORWARD: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STATUS_MEDIA_TYPE: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STATUS_SIDE: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STATUS_SPEED: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STEP_FRAMES: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_VD_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for MCI_VD_STEP_PARMS {}
impl ::core::clone::Clone for MCI_VD_STEP_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_VD_STEP_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_VD_STEP_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_VD_STEP_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_VD_STEP_PARMS {}
impl ::core::default::Default for MCI_VD_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_VD_STEP_REVERSE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAIT: i32 = 2i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_WAVE_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
}
impl ::core::marker::Copy for MCI_WAVE_DELETE_PARMS {}
impl ::core::clone::Clone for MCI_WAVE_DELETE_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_WAVE_DELETE_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_WAVE_DELETE_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_WAVE_DELETE_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_WAVE_DELETE_PARMS {}
impl ::core::default::Default for MCI_WAVE_DELETE_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_GETDEVCAPS_INPUTS: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_GETDEVCAPS_OUTPUTS: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_INPUT: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_MAPPER: u32 = 1153u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_OPEN_BUFFER: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_WAVE_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCSTR,
    pub lpstrElementName: ::windows::core::PCSTR,
    pub lpstrAlias: ::windows::core::PCSTR,
    pub dwBufferSeconds: u32,
}
impl ::core::marker::Copy for MCI_WAVE_OPEN_PARMSA {}
impl ::core::clone::Clone for MCI_WAVE_OPEN_PARMSA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_WAVE_OPEN_PARMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_WAVE_OPEN_PARMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_WAVE_OPEN_PARMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_WAVE_OPEN_PARMSA {}
impl ::core::default::Default for MCI_WAVE_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_WAVE_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: ::windows::core::PCWSTR,
    pub lpstrElementName: ::windows::core::PCWSTR,
    pub lpstrAlias: ::windows::core::PCWSTR,
    pub dwBufferSeconds: u32,
}
impl ::core::marker::Copy for MCI_WAVE_OPEN_PARMSW {}
impl ::core::clone::Clone for MCI_WAVE_OPEN_PARMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_WAVE_OPEN_PARMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_WAVE_OPEN_PARMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_WAVE_OPEN_PARMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_WAVE_OPEN_PARMSW {}
impl ::core::default::Default for MCI_WAVE_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_OUTPUT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_PCM: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_ANYINPUT: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_ANYOUTPUT: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_AVGBYTESPERSEC: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_BITSPERSAMPLE: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_BLOCKALIGN: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_CHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_FORMATTAG: i32 = 65536i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MCI_WAVE_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub wInput: u32,
    pub wOutput: u32,
    pub wFormatTag: u16,
    pub wReserved2: u16,
    pub nChannels: u16,
    pub wReserved3: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wReserved4: u16,
    pub wBitsPerSample: u16,
    pub wReserved5: u16,
}
impl ::core::marker::Copy for MCI_WAVE_SET_PARMS {}
impl ::core::clone::Clone for MCI_WAVE_SET_PARMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MCI_WAVE_SET_PARMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCI_WAVE_SET_PARMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCI_WAVE_SET_PARMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCI_WAVE_SET_PARMS {}
impl ::core::default::Default for MCI_WAVE_SET_PARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_SET_SAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_AVGBYTESPERSEC: i32 = 16388i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_BITSPERSAMPLE: i32 = 16390i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_BLOCKALIGN: i32 = 16389i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_CHANNELS: i32 = 16386i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_FORMATTAG: i32 = 16385i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_LEVEL: i32 = 16391i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WAVE_STATUS_SAMPLESPERSEC: i32 = 16387i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WHERE: u32 = 2115u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCI_WINDOW: u32 = 2113u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCMADM_E_REGKEY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889750i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MCMADM_I_NO_EVENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593897i32);
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct MEDIASPACEADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for MEDIASPACEADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for MEDIASPACEADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for MEDIASPACEADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for MEDIASPACEADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEDIASPACEADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for MEDIASPACEADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for MEDIASPACEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDIMAPPER_S: u32 = 1227u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MIDIOPENSTRMID {
    pub dwStreamID: u32,
    pub uDeviceID: u32,
}
impl ::core::marker::Copy for MIDIOPENSTRMID {}
impl ::core::clone::Clone for MIDIOPENSTRMID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIOPENSTRMID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIOPENSTRMID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIOPENSTRMID>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIOPENSTRMID {}
impl ::core::default::Default for MIDIOPENSTRMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDI_IO_COOKED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDI_IO_PACKED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_ADDBUFFER: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_CLOSE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_GETDEVCAPS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_GETNUMDEVS: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_OPEN: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_PREPARE: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_RESET: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_START: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_STOP: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_UNPREPARE: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIDM_USER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SRS_MTS: u32 = 536936454u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SRS_ONOFF: u32 = 536936455u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT: u32 = 536936456u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct MIXEROPENDESC {
    pub hmx: super::Audio::HMIXER,
    pub pReserved0: *mut ::core::ffi::c_void,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for MIXEROPENDESC {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for MIXEROPENDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for MIXEROPENDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for MIXEROPENDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXEROPENDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for MIXEROPENDESC {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for MIXEROPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct MMCKINFO {
    pub ckid: u32,
    pub cksize: u32,
    pub fccType: u32,
    pub dwDataOffset: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for MMCKINFO {}
impl ::core::clone::Clone for MMCKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MMCKINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MMCKINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMCKINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MMCKINFO {}
impl ::core::default::Default for MMCKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_ACCESSDENIED: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_BASE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CANNOTCLOSE: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CANNOTEXPAND: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CANNOTOPEN: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CANNOTREAD: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CANNOTSEEK: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CANNOTWRITE: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_CHUNKNOTFOUND: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_FILENOTFOUND: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_INVALIDFILE: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_NETWORKERROR: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_OUTOFMEMORY: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_PATHNOTFOUND: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_SHARINGVIOLATION: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_TOOMANYOPENFILES: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOERR_UNBUFFERED: u32 = 266u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMIOINFO {
    pub dwFlags: u32,
    pub fccIOProc: u32,
    pub pIOProc: LPMMIOPROC,
    pub wErrorRet: u32,
    pub htask: super::HTASK,
    pub cchBuffer: i32,
    pub pchBuffer: *mut i8,
    pub pchNext: *mut i8,
    pub pchEndRead: *mut i8,
    pub pchEndWrite: *mut i8,
    pub lBufOffset: i32,
    pub lDiskOffset: i32,
    pub adwInfo: [u32; 3],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub hmmio: HMMIO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMIOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMIOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MMIOINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMIOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MMIOINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMIOINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMIOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_OPEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_READ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_RENAME: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_SEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_USER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIOM_WRITEFLUSH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_ALLOCBUF: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_COMPAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_CREATE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_CREATELIST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_CREATERIFF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_DEFAULTBUFFER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_DELETE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_DENYNONE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_DENYREAD: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_DENYWRITE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_DIRTY: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_EMPTYBUF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_EXCLUSIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_EXIST: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_FHOPEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_FINDCHUNK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_FINDLIST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_FINDPROC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_FINDRIFF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_GETTEMP: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_GLOBALPROC: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_INSTALLPROC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_PARSE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_READ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_READWRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_REMOVEPROC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_RWMODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_SHAREMODE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_TOUPPER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_UNICODEPROC: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MMIO_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_3COM: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_3COM_CB_MIXER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_3COM_CB_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_3COM_CB_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_3DFX: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AARDVARK: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AARDVARK_STUDIO12_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AARDVARK_STUDIO12_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AARDVARK_STUDIO88_WAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AARDVARK_STUDIO88_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ACTIVEVOICE: u32 = 225u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ACTIVEVOICE_ACM_VOXADPCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ACULAB: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX_PCTV_AUX_CD: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX_PCTV_AUX_LINE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX_PCTV_DIGITALMIX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX_PCTV_MIXER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX_PCTV_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADDX_PCTV_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADLACC: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADMOS: u32 = 235u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADMOS_FM_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADMOS_QS3AMIDIIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADMOS_QS3AMIDIOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADMOS_QS3AWAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ADMOS_QS3AWAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AHEAD: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AHEAD_GENERIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AHEAD_MULTISOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AHEAD_PROAUDIO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AHEAD_SOUNDBLASTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALARIS: u32 = 174u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALDIGITAL: u32 = 143u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALESIS: u32 = 243u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALGOVISION: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALGOVISION_VB80AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALGOVISION_VB80AUX2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALGOVISION_VB80MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALGOVISION_VB80WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ALGOVISION_VB80WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD: u32 = 146u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_AUX1: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_AUX2: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_AUX_CD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_AUX_MIC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_EX_CD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_EX_TELEPHONY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_JOYSTICK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_MIDIIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_MIDIOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_MIXER1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_MIXER2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_MONO_IN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_MONO_OUT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_STEREO_ENHANCED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_SYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_WAVEOUT_BASE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AMD_INTERWAVE_WAVEOUT_TREBLE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANALOGDEVICES: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_AUDIOPORT22_FEEDTHRU: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_AUDIOPORT22_WAVEIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_AUDIOPORT22_WAVEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_SX12_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_SX12_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_SX15_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_SX15_WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_VP625_WAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ANTEX_VP625_WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_APICOM: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_APPLE: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_APPS: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_APT: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_APT_ACE100CD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ARRAY: u32 = 231u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ARTISOFT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ARTISOFT_SBWAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ARTISOFT_SBWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AST_MODEMWAVE_WAVEIN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AST_MODEMWAVE_WAVEOUT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ATI: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ATT: u32 = 185u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ATT_G729A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ATT_MICROELECTRONICS: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_AUX: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_MIDIIN: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_MIDIOUT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_MIXER: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_SYNTH: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_WAVEIN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8820_WAVEOUT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_AUX: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_MIDIIN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_MIDIOUT: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_MIXER: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_SYNTH: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_WAVEIN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AU8830_WAVEOUT: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AUDIOFILE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AUDIOPT: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AUDIOSCIENCE: u32 = 217u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AURAVISION: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AUREAL: u32 = 181u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AUREAL_AU8820: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AUREAL_AU8830: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_AUX: u32 = 404u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_AUX_CD: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_AUX_LINE: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_AUX_MIC: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_DSP16_FMSYNTH: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_DSP16_WAVEIN: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_DSP16_WAVEOUT: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_DSP16_WAVESYNTH: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_FMSYNTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_MIXER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_NOVA16_MIXER: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_NOVA16_WAVEIN: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_NOVA16_WAVEOUT: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_PRO16_FMSYNTH: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_PRO16_WAVEIN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_PRO16_WAVEOUT: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_WASH16_MIXER: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_WASH16_WAVEIN: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_WASH16_WAVEOUT: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_WAVEIN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_AZTECH_WAVEOUT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BCB: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BCB_NETBOARD_10: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BCB_TT75_10: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BECUBED: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BERCOS: u32 = 199u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BERCOS_MIXER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BERCOS_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BERCOS_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BERKOM: u32 = 189u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BINTEC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BINTEC_TAPI_WAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BROOKTREE: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_AUX_CD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_AUX_LINE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_AUX_MIC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_DIGITALIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_DIGITALOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_MIDIIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_MIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_MIDISYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_MIDIWAVESTREAM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_MIXER: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_BTV_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CANAM: u32 = 148u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CANAM_CBXWAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CANAM_CBXWAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CANOPUS: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CANOPUS_ACM_DVREX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CASIO: u32 = 162u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CASIO_LSG_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CASIO_WP150_MIDIIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CASIO_WP150_MIDIOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CAT: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CAT_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_AUX: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_MIDIIN: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_MIDIOUT: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_MIXER: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_SYNTH: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_WAVEIN: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CDPC_WAVEOUT: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC: u32 = 155u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_AUX_CD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_FMSYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_MIDIIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_MIDIOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_MIXER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_MPEGWAVEIN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_MPEGWAVEOUT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M1_WTSYNTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_AUX: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_AUX_CD: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_FMSYNTH: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_MIDIIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_MIDIOUT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_MIXER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_MPEGWAVEIN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_MPEGWAVEOUT: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_WAVEIN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_WAVEOUT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CHROMATIC_M2_WTSYNTH: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CIRRUSLOGIC: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COLORGRAPH: u32 = 179u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COMPAQ: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COMPAQ_BB_WAVEAUX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COMPAQ_BB_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COMPAQ_BB_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COMPUSIC: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COMPUTER_FRIENDS: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CONCEPTS: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CONNECTIX: u32 = 158u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CONNECTIX_VIDEC_CODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CONTROLRES: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS: u32 = 147u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNAGRAFX_VGA: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNAMIXHR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_IN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_IN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_OUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_SYNTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_IN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_OUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_CD: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_LINE: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_MASTER: u32 = 404u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_MIC: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_MIDI: u32 = 407u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_PCSPK: u32 = 405u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_AUX_WAVE: u32 = 406u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_FMSYNTH_MONO: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_FMSYNTH_STEREO: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_MIDIIN: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_MIDIOUT: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_MIDI_AWE32: u32 = 303u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_PHNBLST_WAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_PHNBLST_WAVEOUT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SB15_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SB15_WAVEOUT: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SB16_MIXER: u32 = 409u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SB20_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SB20_WAVEOUT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SBP16_WAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SBP16_WAVEOUT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SBPRO_MIXER: u32 = 408u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SBPRO_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CREATIVE_SBPRO_WAVEOUT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_INPUTGAIN_AUX1: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_INPUTGAIN_LOOP: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_MIDIIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_MIDIOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEAUX_LINE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEAUX_MASTER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEAUX_MONO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEMIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_CS4232_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_NET: u32 = 154u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_SOUND_FUSION_JOYSTICK: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_SOUND_FUSION_MIDIIN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_SOUND_FUSION_MIDIOUT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_SOUND_FUSION_MIXER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_SOUND_FUSION_WAVEIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CRYSTAL_SOUND_FUSION_WAVEOUT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CS: u32 = 242u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XAAUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XAMIDIIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XAMIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XAMIXER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XASYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XAWAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_CYRIX_XAWAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DATAFUSION: u32 = 196u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DATARAN: u32 = 232u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DDD: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DDD_MIDILINK_MIDIIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DDD_MIDILINK_MIDIOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DF_ACM_G726: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DF_ACM_GSM610: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIACOUSTICS: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIACOUSTICS_DRUM_ACTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIALOGIC: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIAMONDMM: u32 = 163u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DICTAPHONE: u32 = 214u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DICTAPHONE_G726: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGIGRAM: u32 = 227u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_ACM_G723: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS: u32 = 136u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_CDLX: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_CPRO: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_CTDIF: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_DOC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_TC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_V8: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AUDIO_LABS_VP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AV320_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_AV320_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_ICM_H261: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIGITAL_ICM_H263: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_AUX_LINE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_DIRSOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_MIDIIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_MIDIOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_MIXER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_PLATFORM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_VIRTJOY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_VIRTMPU: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_VIRTSB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WSS_AUX: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WSS_MIXER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WSS_SYNTH: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WSS_WAVEIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DIMD_WSS_WAVEOUT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DOLBY: u32 = 78u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DPSINC: u32 = 191u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_GROUP: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_GROUP_TRUESPEECH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_SOLUTIONS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_SOLUTIONS_AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_SOLUTIONS_SYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_SOLUTIONS_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DSP_SOLUTIONS_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DTS: u32 = 226u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DTS_DS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DUCK: u32 = 197u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_DVISION: u32 = 165u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO_MIDIIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO_MIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECHO_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECS: u32 = 145u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECS_AADF_MIDI_IN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECS_AADF_MIDI_OUT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ECS_AADF_WAVE2MIDI_IN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES: u32 = 219u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES_PCMIDI14: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES_PCMIDI14_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES_PCMIDI14_OUT1: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES_PCMIDI14_OUT2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES_PCMIDI14_OUT3: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EES_PCMIDI14_OUT4: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMAGIC: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMAGIC_UNITOR8: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMU: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMU_APSMIDIIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMU_APSMIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMU_APSSYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMU_APSWAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EMU_APSWAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENET: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENET_T2000_HANDSETIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENET_T2000_HANDSETOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENET_T2000_LINEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENET_T2000_LINEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENSONIQ: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ENSONIQ_SOUNDSCAPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EPSON: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EPS_FMSND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AMAUX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AMMIDIIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AMMIDIOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AMSYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AMWAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AMWAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_AUX_CD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1488_MIXER: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1488_WAVEIN: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1488_WAVEOUT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1688_MIXER: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1688_WAVEIN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1688_WAVEOUT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1788_MIXER: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1788_WAVEIN: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1788_WAVEOUT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1868_MIXER: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1868_WAVEIN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1868_WAVEOUT: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1878_MIXER: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1878_WAVEIN: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1878_WAVEOUT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1888_MIXER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1888_WAVEIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES1888_WAVEOUT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES488_MIXER: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES488_WAVEIN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES488_WAVEOUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES688_MIXER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES688_WAVEIN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_ES688_WAVEOUT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_MIXER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_MPU401_MIDIIN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ESS_MPU401_MIDIOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ETEK: u32 = 241u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ETEK_KWIKMIDI_MIDIIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ETEK_KWIKMIDI_MIDIOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS: u32 = 152u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_AUX_CD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_AUX_LINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_AUX_MASTER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_AUX_MIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_AUX_MIDI: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_AUX_WAVE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_EUSYNTH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_FMSYNTH_MONO: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_FMSYNTH_STEREO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_MIDIIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_MIDIOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_MIXER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_WAVEIN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EUPHONICS_WAVEOUT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EVEREX: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EVEREX_CARRIER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_EXAN: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FAITH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FAST: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3_ADVANCED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3_BASIC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3_DECODE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3_LITE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FHGIIS_MPEGLAYER3_PROFESSIONAL: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FLEXION: u32 = 249u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FLEXION_X300_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FLEXION_X300_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FORTEMEDIA: u32 = 229u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FORTEMEDIA_AUX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FORTEMEDIA_FMSYNC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FORTEMEDIA_MIXER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FORTEMEDIA_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FORTEMEDIA_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FRAUNHOFER_IIS: u32 = 172u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FRONTIER: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FRONTIER_WAVECENTER_MIDIIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FRONTIER_WAVECENTER_MIDIOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FRONTIER_WAVECENTER_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FRONTIER_WAVECENTER_WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FTR: u32 = 198u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FTR_ACM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FTR_ENCODER_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_FUJITSU: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS: u32 = 159u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS_WAVE42_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS_WAVE42_WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS_WAVE44_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS_WAVE44_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS_WAVE4_MIDIIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GADGETLABS_WAVE4_MIDIOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GRANDE: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GRAVIS: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GUILLEMOT: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_GULBRANSEN: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HAFTMANN: u32 = 220u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HAFTMANN_LPTDAC2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEADSPACE: u32 = 222u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEADSPACE_HAEMIXER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEADSPACE_HAESYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEADSPACE_HAEWAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEADSPACE_HAEWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEWLETT_PACKARD: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HEWLETT_PACKARD_CU_CODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HORIZONS: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HP: u32 = 253u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HP_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HP_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_HYPERACTIVE: u32 = 246u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_MWAVE_AUX: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_MWAVE_MIDIIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_MWAVE_MIDIOUT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_MWAVE_MIXER: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_MWAVE_WAVEIN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_MWAVE_WAVEOUT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_PCMCIA_AUX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_PCMCIA_MIDIIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_PCMCIA_MIDIOUT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_PCMCIA_SYNTH: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_PCMCIA_WAVEIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_PCMCIA_WAVEOUT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_THINKPAD200: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_WC_MIDIOUT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_WC_MIXEROUT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IBM_WC_WAVEOUT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICCC: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICCC_UNA3_AUX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICCC_UNA3_MIXER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICCC_UNA3_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICCC_UNA3_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE: u32 = 239u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_AUX: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MIDIIN1: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MIDIIN2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MIDIOUT1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MIDIOUT2: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MIXER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MTWAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_MTWAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_SYNTH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICE_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICL_PS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICOM_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICOM_LINE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICOM_MIXER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICOM_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICOM_WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_2115_LITE_MIDIOUT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_2120_LITE_MIDIOUT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDECK_AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDECK_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDECK_SYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDECK_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDECK_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_AUX: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_FM_MIDIOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_MIXER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIIN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_WAVEIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ICS_WAVEDEC_SB_WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INSOFT: u32 = 94u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTEL: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTELOPD_AUX: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTELOPD_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTELOPD_WAVEOUT: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTEL_NSPMODEMLINEIN: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTEL_NSPMODEMLINEOUT: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERACTIVE: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERACTIVE_WAVEIN: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERACTIVE_WAVEOUT: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERNET: u32 = 244u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERNET_SSW_MIDIIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERNET_SSW_MIDIOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERNET_SSW_WAVEIN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INTERNET_SSW_WAVEOUT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_INVISION: u32 = 188u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IODD: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC_TEMPO_AUXOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC_TEMPO_MIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC_TEMPO_MXDOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC_TEMPO_SYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC_TEMPO_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IOMAGIC_TEMPO_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI: u32 = 238u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI_ACM_HSX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI_ACM_RPELP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI_AT_MIXER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI_AT_WAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI_AT_WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_IPI_WF_ASSS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ISOLUTION: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ISOLUTION_PASCAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ITERATEDSYS: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ITERATEDSYS_FUFCODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_I_LINK: u32 = 233u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_I_LINK_VOICE_CODER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KAY_ELEMETRICS: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KAY_ELEMETRICS_CSL: u32 = 17152u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KAY_ELEMETRICS_CSL_4CHANNEL: u32 = 17161u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KAY_ELEMETRICS_CSL_DAT: u32 = 17160u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KORG: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KORG_1212IO_MSWAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KORG_1212IO_MSWAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KORG_PCIF_MIDIIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_KORG_PCIF_MIDIOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LERNOUT_ANDHAUSPIE_LHCODECACM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LERNOUT_AND_HAUSPIE: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LEXICON: u32 = 236u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LEXICON_STUDIO_WAVE_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LEXICON_STUDIO_WAVE_OUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LOGITECH: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUCENT: u32 = 184u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUCENT_ACM_G723: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUCID: u32 = 221u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUCID_PCI24WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUCID_PCI24WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUMINOSITI: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUMINOSITI_SCWAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUMINOSITI_SCWAVEMIX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LUMINOSITI_SCWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LYNX: u32 = 212u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LYRRUS: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_LYRRUS_BRIDGE_GUITAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MALDEN: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN: u32 = 190u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN_ARC44WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN_ARC44WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN_ARC88WAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN_ARC88WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN_PRODIF24WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MARIAN_PRODIF24WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATROX_DIV: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATSUSHITA: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATSUSHITA_AUX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATSUSHITA_FMSYNTH_STEREO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATSUSHITA_MIXER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATSUSHITA_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MATSUSHITA_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIASONIC: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIASONIC_ACM_G723: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIASONIC_ICOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIATRIX: u32 = 141u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_CDPC: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_OPUS1208: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_OPUS1216: u32 = 144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_PROAUDIO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_PROAUDIO_16: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_PROAUDIO_PLUS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_PROSTUDIO_16: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_THUNDER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MEDIAVISION_TPORT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MELABS: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MELABS_MIDI2GO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MERGING_MPEGL3: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MERGING_TECHNOLOGIES: u32 = 177u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_METHEUS: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_METHEUS_ZIPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MICRONAS: u32 = 251u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MICRONAS_CLP833: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MICRONAS_SC4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MINDMAKER: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MINDMAKER_GC_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MINDMAKER_GC_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MINDMAKER_GC_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_DC30_MIX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_DC30_WAVEIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_DC30_WAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_MOVIEPRO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_VIDEOD1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_VIDEODC1TV: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MIRO_VIDEOTD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MEDIAPATH_WAVEIN: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MEDIAPATH_WAVEOUT: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_HANDSET_WAVEIN: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_HANDSET_WAVEOUT: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_HANDSFREE_WAVEIN: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_HANDSFREE_WAVEOUT: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_LINE1_WAVEIN: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_LINE1_WAVEOUT: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_LINE2_WAVEIN: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_MPA_LINE2_WAVEOUT: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_TALKTO_BRIDGED_WAVEIN: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_TALKTO_BRIDGED_WAVEOUT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_TALKTO_HANDSET_WAVEIN: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_TALKTO_HANDSET_WAVEOUT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_TALKTO_LINE_WAVEIN: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MITEL_TALKTO_LINE_WAVEOUT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MMOTION_WAVEAUX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MMOTION_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MMOTION_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOSCOM: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOSCOM_VPC2400_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOSCOM_VPC2400_OUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTIONPIXELS: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTIONPIXELS_MVI2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTOROLA: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_DTX_MIDI_IN_A: u32 = 801u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_DTX_MIDI_IN_B: u32 = 802u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_DTX_MIDI_IN_SYNC: u32 = 800u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_DTX_MIDI_OUT_A: u32 = 801u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_DTX_MIDI_OUT_B: u32 = 802u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_FLYER_MIDI_IN_A: u32 = 601u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_FLYER_MIDI_IN_B: u32 = 602u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_FLYER_MIDI_IN_SYNC: u32 = 600u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_FLYER_MIDI_OUT_A: u32 = 601u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_FLYER_MIDI_OUT_B: u32 = 602u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_1: u32 = 901u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_2: u32 = 902u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_3: u32 = 903u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_4: u32 = 904u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_5: u32 = 905u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_6: u32 = 906u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_7: u32 = 907u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_8: u32 = 908u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_ADAT: u32 = 917u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIIN_SYNC: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_1: u32 = 901u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_2: u32 = 902u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_3: u32 = 903u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_4: u32 = 904u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_5: u32 = 905u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_6: u32 = 906u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_7: u32 = 907u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_8: u32 = 908u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_ADAT: u32 = 917u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_MIDIOUT_ALL: u32 = 900u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_1: u32 = 909u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_2: u32 = 910u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_3: u32 = 911u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_4: u32 = 912u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_5: u32 = 913u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_6: u32 = 914u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_7: u32 = 915u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIIN_8: u32 = 916u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_1: u32 = 909u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_2: u32 = 910u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_3: u32 = 911u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_4: u32 = 912u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_5: u32 = 913u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_6: u32 = 914u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_7: u32 = 915u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPAV_NET_MIDIOUT_8: u32 = 916u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_1: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_2: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_3: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_4: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_5: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_6: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_7: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_8: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIIN_SYNC: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_1: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_2: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_3: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_4: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_5: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_6: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_7: u32 = 207u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_8: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_MIDIOUT_ALL: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_1: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_2: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_3: u32 = 211u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_4: u32 = 212u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_5: u32 = 213u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_6: u32 = 214u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_7: u32 = 215u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIIN_8: u32 = 216u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_1: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_2: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_3: u32 = 211u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_4: u32 = 212u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_5: u32 = 213u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_6: u32 = 214u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_7: u32 = 215u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTPII_NET_MIDIOUT_8: u32 = 216u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_1: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_2: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_3: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_4: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_5: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_6: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_7: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIIN_8: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_1: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_2: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_3: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_4: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_5: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_6: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_7: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_8: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MTP_MIDIOUT_ALL: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIIN_1: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIIN_2: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIIN_3: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIIN_4: u32 = 504u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIIN_SYNC: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIOUT_1: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIOUT_2: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIOUT_3: u32 = 503u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIOUT_4: u32 = 504u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXN_MIDIOUT_ALL: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_1: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_2: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_3: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_4: u32 = 404u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_5: u32 = 405u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_6: u32 = 406u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIIN_SYNC: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_1: u32 = 401u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_2: u32 = 402u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_3: u32 = 403u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_4: u32 = 404u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_5: u32 = 405u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_6: u32 = 406u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPMPU_MIDIOUT_ALL: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_1: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_2: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_3: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_4: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_5: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_6: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_7: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_8: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIIN_SYNC: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_1: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_2: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_3: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_4: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_5: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_6: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_7: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_8: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXPXT_MIDIOUT_ALL: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_1: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_2: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_3: u32 = 303u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_4: u32 = 304u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_5: u32 = 305u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_6: u32 = 306u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_1: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_2: u32 = 302u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_3: u32 = 303u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_4: u32 = 304u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_5: u32 = 305u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_6: u32 = 306u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_MXP_MIDIIN_SYNC: u32 = 300u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_PKX_MIDI_IN_A: u32 = 701u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_PKX_MIDI_IN_B: u32 = 702u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_PKX_MIDI_IN_SYNC: u32 = 700u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_PKX_MIDI_OUT_A: u32 = 701u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MOTU_PKX_MIDI_OUT_B: u32 = 702u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MPTUS: u32 = 95u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MPTUS_SPWAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_G711: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_GSM610: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_IMAADPCM: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_MSADPCM: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_MSAUDIO1: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_MSFILTER: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_MSG723: u32 = 92u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_MSNAUDIO: u32 = 91u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_MSRT24: u32 = 93u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_PCM: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_WMAUDIO: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_ACM_WMAUDIO2: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_AUX_CD: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_AUX_LINE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_AUX_MIC: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_MIDIIN: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_MIDIOUT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_MIDISYNTH: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_WAVEIN: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_GENERIC_WAVEOUT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_MSACM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_MSOPL_SYNTH: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_AUX_CD: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_AUX_LINE: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_MIDIIN: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_MIDIOUT: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_MIXER: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_SYNTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_WAVEIN: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SB16_WAVEOUT: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_AUX_CD: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_AUX_LINE: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_MIDIIN: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_MIDIOUT: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_MIXER: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_SYNTH: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_WAVEIN: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_SBPRO_WAVEOUT: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMS_HANDSET_WAVEIN: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMS_HANDSET_WAVEOUT: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMS_LINE_WAVEIN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMS_LINE_WAVEOUT: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMW_HANDSET_WAVEIN: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMW_HANDSET_WAVEOUT: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMW_LINE_WAVEIN: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMW_LINE_WAVEOUT: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDMW_MIXER: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDM_GAME_WAVEIN: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_VMDM_GAME_WAVEOUT: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WDMAUDIO_AUX: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WDMAUDIO_MIDIIN: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WDMAUDIO_MIDIOUT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WDMAUDIO_MIXER: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WDMAUDIO_WAVEIN: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WDMAUDIO_WAVEOUT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_AUX: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_FMSYNTH_STEREO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_MIXER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_NT_AUX: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_NT_FMSYNTH_STEREO: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_NT_MIXER: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_NT_WAVEIN: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_NT_WAVEOUT: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_OEM_AUX: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_OEM_FMSYNTH_STEREO: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_OEM_MIXER: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_OEM_WAVEIN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_OEM_WAVEOUT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_WAVEIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MSFT_WSS_WAVEOUT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_MWM: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NCR: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NCR_BA_AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NCR_BA_MIXER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NCR_BA_SYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NCR_BA_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NCR_BA_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_26_SYNTH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_73_86_SYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_73_86_WAVEIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_73_86_WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_JOYSTICK: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_MPU401_MIDIIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEC_MPU401_MIDIOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC: u32 = 176u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MIDIIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_AUX: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_FMSYNTH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_GMSYNTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_MIDIIN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_MIDIOUT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_MIXER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_WAVEIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MW3DX_WAVEOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MWAVE_AUX: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MWAVE_MIDIIN: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MWAVE_MIDIOUT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MWAVE_MIXER: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MWAVE_WAVEIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_MWAVE_WAVEOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEOMAGIC_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NETSCAPE: u32 = 166u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NETXL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NETXL_XLVIDEO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEWMEDIA: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NEWMEDIA_WAVJAMMER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NMP: u32 = 195u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NMP_ACM_AMR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NMP_CCP_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NMP_CCP_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NMS: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NOGATECH: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NORRIS: u32 = 150u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NORRIS_VOICELINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NORTEL_MPXAC_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NORTEL_MPXAC_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NORTHERN_TELECOM: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_AUX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_GAMEPORT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_MIXER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_NVIDIA_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKI: u32 = 79u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_EXT_MIC1: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_EXT_MIC2: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_FM_OPL4: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIDIIN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIDIOUT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_AUX1: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_CD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_ECHO: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_FM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_LINE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_LINE1: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_MASTER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_MIC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MIX_WAVE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_MPEG_CDVISION: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_OSR16_WAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_OSR16_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_OSR8_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OKSORI_OSR8_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_ACM_ADPCM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_ACM_CELP: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_ACM_GSM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_ACM_OPR: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_ACM_SBC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_JOYSTICK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_MIDIIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_MIDIOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_SYNTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OLIVETTI_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ONLIVE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ONLIVE_MPCODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPCODE: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI: u32 = 90u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_AUX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_FMSYNTH_STEREO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_MIDIIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_MIXER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_WAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M16_WAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_AUX: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_MIDIIN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_MIDIOUT: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_MIXER: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_SYNTH_STEREO: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_WAVEIN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_M32_WAVEOUT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_AUX: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_FMSYNTH_STEREO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_MIDIIN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_MIDIOUT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_MIXER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_WAVEIN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPTI_P16_WAVEOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1208_AUX: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1208_MIXER: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1208_SYNTH: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1208_WAVEIN: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1208_WAVEOUT: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_AUX: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_MIDIIN: u32 = 146u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_MIDIOUT: u32 = 145u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_MIXER: u32 = 150u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_SYNTH: u32 = 147u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_WAVEIN: u32 = 149u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS1216_WAVEOUT: u32 = 148u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS401_MIDIIN: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OPUS401_MIDIOUT: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OSITECH: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OSITECH_TRUMPCARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OSPREY: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OSPREY_1000WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OSPREY_1000WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OTI: u32 = 180u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OTI_611MIDIN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OTI_611MIDIOUT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OTI_611MIXER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OTI_611WAVEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_OTI_611WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PACIFICRESEARCH: u32 = 210u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PCSPEAKER_WAVEOUT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PHILIPS_ACM_LPCBB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PHILIPS_SPEECH_PROCESSING: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PHONET: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PHONET_PP_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PHONET_PP_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PHONET_PP_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PICTURETEL: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PID_UNMAPPED: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PINNACLE: u32 = 218u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PRAGMATRAX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PRECEPT: u32 = 153u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_AUX: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_MIDIIN: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_MIDIOUT: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_MIXER: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_SYNTH: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_WAVEIN: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_16_WAVEOUT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_AUX: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_MIDIIN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_MIDIOUT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_MIXER: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_AUX: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_MIDIIN: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_MIDIOUT: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_MIXER: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_SYNTH: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_WAVEIN: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_PLUS_WAVEOUT: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_SYNTH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_WAVEIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_PROAUD_WAVEOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QCIAR: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QDESIGN: u32 = 194u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QDESIGN_ACM_MPEG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QDESIGN_ACM_QDESIGN_MUSIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QTEAM: u32 = 169u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUALCOMM: u32 = 215u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUANTUM3D: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUARTERDECK: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUARTERDECK_LHWAVEIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUARTERDECK_LHWAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUICKAUDIO: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUICKAUDIO_MAXIMIDI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUICKAUDIO_MINIMIDI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUICKNET: u32 = 173u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUICKNET_PJWAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_QUICKNET_PJWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RADIUS: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RHETOREX: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RHETOREX_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RHETOREX_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RICHMOND: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROCKWELL: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_MPU401_MIDIIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_MPU401_MIDIOUT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_RAP10_MIDIIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_RAP10_MIDIOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_RAP10_SYNTH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_RAP10_WAVEIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_RAP10_WAVEOUT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SC7_MIDIIN: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SC7_MIDIOUT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SCP_AUX: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SCP_MIDIIN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SCP_MIDIOUT: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SCP_MIXER: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SCP_WAVEIN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SCP_WAVEOUT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SERIAL_MIDIIN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SERIAL_MIDIOUT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SMPU_MIDIINA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SMPU_MIDIINB: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SMPU_MIDIOUTA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ROLAND_SMPU_MIDIOUTB: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RZS: u32 = 216u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_RZS_ACM_TUBGSM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3: u32 = 164u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_AUX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_FMSYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_MIXER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_S3_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SANYO: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SANYO_ACM_LD_ADPCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SCALACS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS: u32 = 137u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS_REALITY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS_SEERMIX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS_SEERSYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS_SEERWAVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS_WAVESYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SEERSYS_WAVESYNTH_WG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SELSIUS_SYSTEMS: u32 = 234u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SELSIUS_SYSTEMS_RTPWAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SELSIUS_SYSTEMS_RTPWAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI: u32 = 237u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_320_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_320_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_320_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_540_MIXER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_540_WAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_540_WAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADAT8CHAN_WAVEIN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADAT8CHAN_WAVEOUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO1_WAVEIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO1_WAVEOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO2_WAVEIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO2_WAVEOUT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO3_WAVEIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO3_WAVEOUT: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO4_WAVEIN: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO4_WAVEOUT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO5_WAVEIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO5_WAVEOUT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO6_WAVEIN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO6_WAVEOUT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO7_WAVEIN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO7_WAVEOUT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO8_WAVEIN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATMONO8_WAVEOUT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO12_WAVEIN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO12_WAVEOUT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO32_WAVEOUT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO34_WAVEIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO56_WAVEIN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO56_WAVEOUT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO78_WAVEIN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_ADATSTEREO78_WAVEOUT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_AESMONO1_WAVEIN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_AESMONO1_WAVEOUT: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_AESMONO2_WAVEIN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_AESMONO2_WAVEOUT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_AESSTEREO_WAVEIN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SGI_RAD_AESSTEREO_WAVEOUT: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP: u32 = 183u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_BASS: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_CHR: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_MASTER: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_MIDI_VOL: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_RVB: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_TREBLE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_VOL: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_WAVE_CHR: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_WAVE_RVB: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_AUX_WAVE_VOL: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_MIDI_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_MIDI_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_MIDI_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_MIXER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_WAVE_IN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SHARP_MDC_WAVE_OUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SICRESOURCE: u32 = 175u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SICRESOURCE_SSO3D: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SICRESOURCE_SSOW3DI: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIEMENS_SBC: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_AUX: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_AUX2: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_MIDIIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_MIDIOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_SYNTH: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_WAVEIN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_ARIA_WAVEOUT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_AUX_CD: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_AUX_LINE: u32 = 86u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_AUX_MODEM: u32 = 87u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_MIDIIN: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_MIDIOUT: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_MIXER: u32 = 88u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_SYNTH: u32 = 84u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_WAVEIN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIERRA_QUARTET_WAVEOUT: u32 = 81u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SC1_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SC1_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SC2_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SC2_WAVEOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SOUNDJR2_WAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SILICONSOFT_SOUNDJR3_WAVEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIPROLAB: u32 = 211u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SIPROLAB_ACELPNET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SNI: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SNI_ACM_G721: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTLAB_NSK: u32 = 228u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTLAB_NSK_FRW_AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTLAB_NSK_FRW_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTLAB_NSK_FRW_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTLAB_NSK_FRW_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTSOUND: u32 = 149u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOFTSOUND_CODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SONICFOUNDRY: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SONORUS: u32 = 230u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SONORUS_STUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SONY: u32 = 245u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SONY_ACM_SCX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SORVIS: u32 = 187u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDESIGNS: u32 = 142u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDESIGNS_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDESIGNS_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_AUX: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_MIDIIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_MIDIOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_MIXER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_SYNTH: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_WAVEIN: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_WAVEOUT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSCAPE_WAVEOUT_AUX: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SOUNDSPACE: u32 = 167u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SPECTRUM_PRODUCTIONS: u32 = 213u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SPECTRUM_SIGNAL_PROCESSING: u32 = 144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SPEECHCOMP: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SPLASH_STUDIOS: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESAUX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESMIDIIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESMIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESMIX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESSYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESWAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SSP_SNDFESWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDER: u32 = 171u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_AUX: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_MIDIIN: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_MIDIOUT: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_MIXER: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_SYNTH: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_WAVEIN: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_STUDIO_16_WAVEOUT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ST_MICROELECTRONICS: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SUNCOM: u32 = 186u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SUPERMAC: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SYDEC_NV: u32 = 248u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SYDEC_NV_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_SYDEC_NV_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_PSSJWAVEIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_PSSJWAVEOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_SENS_MMAMIDIIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_SENS_MMAMIDIOUT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_SENS_MMAWAVEIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_SENS_MMAWAVEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_SENS_VISWAVEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_VISBIOSSYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_VISWAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TANDY_VISWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TBS_TROPEZ_AUX1: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TBS_TROPEZ_AUX2: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TBS_TROPEZ_LINE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TBS_TROPEZ_WAVEIN: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TBS_TROPEZ_WAVEOUT: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_BASS: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_CHR: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_MASTER: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_MIDI_VOL: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_RVB: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_TREBLE: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_VOL: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_WAVE_CHR: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_WAVE_RVB: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_AUX_WAVE_VOL: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_MIDI_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_MIDI_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_MIDI_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_MIXER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_WAVE_IN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TDK_MW_WAVE_OUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TELEKOL: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TELEKOL_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TELEKOL_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TERALOGIC: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TERRATEC: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_THUNDER_AUX: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_THUNDER_SYNTH: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_THUNDER_WAVEIN: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_THUNDER_WAVEOUT: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TPORT_SYNTH: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TPORT_WAVEIN: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TPORT_WAVEOUT: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TRUEVISION: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TRUEVISION_WAVEIN1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TRUEVISION_WAVEOUT1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_AUX: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_MIDIIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_MIDIMONITOR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_MIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_MIDISYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_MIXER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_VMIDIIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_VMIDIOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TTEWS_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_TURTLE_BEACH: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_UHER_INFORMATIC: u32 = 247u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_UH_ACM_ADPCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_UNISYS: u32 = 223u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_UNISYS_ACM_NAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_UNMAPPED: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VAL: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VAL_MICROKEY_AP_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VAL_MICROKEY_AP_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VANKOEVERING: u32 = 168u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA: u32 = 250u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_AUX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_MIXER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_MPU401_MIDIIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_MPU401_MIDIOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_SWFM_SYNTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WDM_MIXER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WDM_MPU401_MIDIIN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WDM_MPU401_MIDIOUT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WDM_WAVEIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIA_WDM_WAVEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIDEOLOGIC: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIDEOLOGIC_MSWAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIDEOLOGIC_MSWAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIENNASYS: u32 = 157u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIENNASYS_TSP_WAVE_DRIVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONA: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONAQVINPCI_WAVEOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONA_BUSTER_MIXER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONA_CINEMASTER_MIXER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONA_CONCERTO_MIXER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONA_QVINPCI_MIXER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIONA_QVINPCI_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIRTUALMUSIC: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VITEC: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VITEC_VMAKER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VITEC_VMPRO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIVO: u32 = 182u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VIVO_AUDIO_CODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VKC_MPU401_MIDIIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VKC_MPU401_MIDIOUT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VKC_SERIAL_MIDIIN: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VKC_SERIAL_MIDIOUT: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOCALTEC: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOCALTEC_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOCALTEC_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOICEINFO: u32 = 156u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOICEMIXER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOXWARE: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOXWARE_CODEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VOYETRA: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VQST: u32 = 240u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VQST_VQC1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VQST_VQC2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_VTG: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WANGLABS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WANGLABS_WAVEIN1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WANGLABS_WAVEOUT1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WEITEK: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILDCAT: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILDCAT_AUTOSCOREMIDIIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOPOND_SNDCOMM_WAVEIN: u32 = 108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_FMSYNTH_STEREO: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_GENERIC_AUX: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_GENERIC_MIXER: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_GENERIC_WAVEIN: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_GENERIC_WAVEOUT: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_MPU401: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_PH_AUX: u32 = 107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_PH_MIXER: u32 = 106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_PH_WAVEIN: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_PH_WAVEOUT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDCOMM_AUX: u32 = 111u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDCOMM_MIXER: u32 = 110u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDCOMM_WAVEOUT: u32 = 109u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDPORT_AUX: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDPORT_MIXER: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDPORT_WAVEIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WILLOWPOND_SNDPORT_WAVEOUT: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINBOND: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINNOV: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINNOV_CAVIAR_CHAMPAGNE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINNOV_CAVIAR_VIDC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINNOV_CAVIAR_WAVEIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINNOV_CAVIAR_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WINNOV_CAVIAR_YUV8: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_AUX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_FMSYNTH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_JOYSTICK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_MIDIOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_MIXER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WORKBIT_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_AUX_CD: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_AUX_LINE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_MIDIIN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_MIDIOUT: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_MIXER: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_SYNTH: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_WAVEIN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SB16_WAVEOUT: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_AUX_CD: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_AUX_LINE: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_MIDIIN: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_MIDIOUT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_MIXER: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_SYNTH: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_WAVEIN: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_WSS_SBPRO_WAVEOUT: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_XEBEC: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_XIRLINK: u32 = 178u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_XIRLINK_VISIONLINK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_XYZ: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_ACXG_AUX: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_ACXG_MIDIOUT: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_ACXG_MIXER: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_ACXG_WAVEIN: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_ACXG_WAVEOUT: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_GSS_AUX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_GSS_MIDIIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_GSS_MIDIOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_GSS_SYNTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_GSS_WAVEIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_GSS_WAVEOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_FMSYNTH: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_JOYSTICK: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_MIDIIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_MIDIOUT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_MIXER: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_WAVEIN: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_WAVEOUT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_OPL3SA_YSYNTH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_SERIAL_MIDIIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_SERIAL_MIDIOUT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_SXG_MIDIOUT: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_SXG_MIXER: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_SXG_WAVEOUT: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724LEG_FMSYNTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724LEG_MIDIIN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724LEG_MIDIOUT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724LEG_MIXER: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724_AUX: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724_MIDIOUT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724_MIXER: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724_WAVEIN: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YAMAHA_YMF724_WAVEOUT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_YOUCOM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ZEFIRO: u32 = 170u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ZEFIRO_ZA2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ZYXEL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MM_ZYXEL_ACM_ADPCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_CACHEDRUMPATCHES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_CACHEPATCHES: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_DATA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_GETDEVCAPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_GETNUMDEVS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_GETPOS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_GETVOLUME: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_LONGDATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_OPEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_PAUSE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_PREFERRED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_PREPARE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_PROPERTIES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_RECONFIGURE: u32 = 18280u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_RESET: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_RESTART: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_SETVOLUME: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_STOP: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_STRMDATA: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_UNPREPARE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MODM_USER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MPEGLAYER3_ID_CONSTANTFRAMESIZE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MPEGLAYER3_ID_MPEG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MPEGLAYER3_ID_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MPEGLAYER3_WFX_EXTRA_BYTES: u32 = 12u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct MSAUDIO1WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wEncodeOptions: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for MSAUDIO1WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for MSAUDIO1WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for MSAUDIO1WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for MSAUDIO1WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MSAUDIO1WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for MSAUDIO1WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for MSAUDIO1WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MSAUDIO1_BITS_PER_SAMPLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MSAUDIO1_MAX_CHANNELS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_GETCONTROLDETAILS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_GETDEVCAPS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_GETLINECONTROLS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_GETLINEINFO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_GETNUMDEVS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_OPEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_SETCONTROLDETAILS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const MXDM_USER: u32 = 16384u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct NMS_VBXADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for NMS_VBXADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for NMS_VBXADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for NMS_VBXADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for NMS_VBXADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMS_VBXADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for NMS_VBXADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for NMS_VBXADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_DRM_E_MIGRATION_IMAGE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879730i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_DRM_E_MIGRATION_SOURCE_MACHINE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879732i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_DRM_E_MIGRATION_TARGET_MACHINE_LESS_THAN_LH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879731i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_DRM_E_MIGRATION_UPGRADE_WITH_DIFF_SID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879733i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_8BIT_WAVE_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886834i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ACTIVE_SG_DEVICE_CONTROL_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882778i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ACTIVE_SG_DEVICE_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882779i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ADVANCEDEDIT_TOO_MANY_PICTURES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884886i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ALLOCATE_FILE_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889759i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ALL_PROTOCOLS_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ALREADY_CONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889840i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ANALOG_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879353i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ARCHIVE_ABORT_DUE_TO_BCAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884338i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ARCHIVE_FILENAME_NOTSET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ARCHIVE_GAP_DETECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884337i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ARCHIVE_REACH_QUOTA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884339i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ARCHIVE_SAME_AS_INPUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882812i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ASSERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889653i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ASX_INVALIDFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885655i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ASX_INVALIDVERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885654i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ASX_INVALID_REPEAT_BLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885653i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ASX_NOTHING_TO_WRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885652i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ATTRIBUTE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886825i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ATTRIBUTE_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886826i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIENCE_CONTENTTYPE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882791i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIENCE__LANGUAGE_CONTENTTYPE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882717i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIODEVICE_BADFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIODEVICE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIODEVICE_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIO_BITRATE_STEPDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882759i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIO_CODEC_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUDIO_CODEC_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_AUTHORIZATION_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884336i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BACKUP_RESTORE_BAD_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879803i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BACKUP_RESTORE_BAD_REQUEST_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879826i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BACKUP_RESTORE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879827i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BACKUP_RESTORE_TOO_MANY_RESETS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879770i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_ADAPTER_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889799i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_ADAPTER_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889652i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_BLOCK0_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889757i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_CONTENTEDL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882774i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_CONTROL_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889806i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_CUB_UID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889454i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_DELIVERY_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889798i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_DISK_UID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889756i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_FSMAJOR_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889755i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_MARKIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882856i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_MARKOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_MULTICAST_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889800i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_STAMPNUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889754i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BAD_SYNTAX_IN_SERVER_RESPONSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877826i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_CALLFUNCENDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885145i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_CALLFUNCFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885147i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_CALLFUNCTIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885146i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_CANCELCOMPLETEDJOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885153i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_COMPLETECANCELLEDJOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885154i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_FAILEDINITIALIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885143i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_FAILED_TO_CREATE_TEMPFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885150i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_INVALIDJOBSIGNATURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885151i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_INVALID_FILE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885141i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_NOJOBPOINTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885152i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDINITIALIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885149i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDTOMOVEFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885148i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_WMDUNPACKFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885144i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BKGDOWNLOAD_WRONG_NO_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885155i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889819i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CACHE_ARCHIVE_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884756i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CACHE_CANNOT_BE_CACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884752i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CACHE_NOT_BROADCAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884753i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CACHE_NOT_MODIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884751i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CACHE_ORIGIN_SERVER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884755i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CACHE_ORIGIN_SERVER_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884754i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOTCONNECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOTCONNECTEVENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889745i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOTDESTROYTITLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOTOFFLINEDISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOTONLINEDISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOTRENAMETITLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884904i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_FROM_MULTIPLE_SERVICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884905i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_CONNECT_TO_PROXY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_DELETE_ACTIVE_SOURCEGROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_GENERATE_BROADCAST_INFO_FOR_QUALITYVBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882721i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_PAUSE_LIVEBROADCAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882802i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_READ_PLAYLIST_FROM_MEDIASERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_REMOVE_PLUGIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884655i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_REMOVE_PUBLISHING_POINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884656i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_SYNC_DRM_TO_NON_JANUS_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885178i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANNOT_SYNC_PREVIOUS_SYNC_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885177i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CANT_READ_DIGITAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CCLINK_DOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889821i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_COPYTO_CD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_DRIVER_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_EMPTY_TRACK_QUEUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885255i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_ISRC_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885253i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_MEDIA_CATALOG_NUMBER_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885252i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_NO_BUFFERS_READ: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885256i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_NO_READER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885254i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_QUEUEING_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885249i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_READ_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_READ_ERROR_NO_CORRECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_REFRESH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885839i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_SLOW_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CD_SPEEDDETECT_NOT_ENOUGH_READS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885250i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CHANGING_PROXYBYPASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885565i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CHANGING_PROXY_EXCEPTIONLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885566i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CHANGING_PROXY_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885568i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CHANGING_PROXY_PORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885567i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CHANGING_PROXY_PROTOCOL_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885564i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CLOSED_ON_SUSPEND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877839i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CODEC_DMO_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CODEC_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882813i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_COMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879352i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_COMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879355i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CONNECTION_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889815i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CONNECT_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877818i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CONTENT_PARTNER_STILL_INITIALIZING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884894i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CORECD_NOTAMEDIACD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885561i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CRITICAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884452i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CUB_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889773i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CUB_FAIL_LINK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889456i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURLHELPER_NOTADIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884947i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURLHELPER_NOTAFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884946i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURLHELPER_NOTRELATIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884944i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_CANTDECODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884945i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_CANTWALK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884949i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDBUFFERSIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884943i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDCHAR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884955i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDHOSTNAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884954i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDPATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884953i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884948i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDSCHEME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884952i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_INVALIDURL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884951i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_CURL_NOTSAFE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884956i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DAMAGED_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885813i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DATAPATH_NO_SINK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884456i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DATA_SOURCE_ENUMERATION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884352i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DATA_UNIT_EXTENSION_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DDRAW_GENERIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885571i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVCONTROL_FAILED_SEEK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882796i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVICECONTROL_UNSTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882719i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVICE_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVICE_IS_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885385i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVICE_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885814i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVICE_NOT_SUPPORT_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DEVICE_NOT_WMDRM_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879749i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DISK_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889771i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DISK_READ: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889833i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DISK_WRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889834i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DISPLAY_MODE_CHANGE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885570i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRMPROFILE_NOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882731i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ACQUIRING_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879829i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ACTION_NOT_QUERIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879830i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ALREADY_INDIVIDUALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879831i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_APPCERT_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879790i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ATTRIBUTE_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879438i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BACKUPRESTORE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879804i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BACKUP_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879805i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BACKUP_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879806i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BAD_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879440i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BB_UNABLE_TO_INITIALIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879744i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BUFFER_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879780i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879551i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CACHED_CONTENT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879797i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CERTIFICATE_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879455i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CERTIFICATE_SECURITY_LEVEL_INADEQUATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879442i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CHAIN_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879540i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CHECKPOINT_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879721i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CHECKPOINT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879745i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CHECKPOINT_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879722i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_CLIENT_CODE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879545i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DATASTORE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879741i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DEBUGGING_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879769i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DECRYPT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DEVICE_ACTIVATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879771i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DEVICE_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879445i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DEVICE_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879453i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DEVICE_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879446i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DEVICE_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879646i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DRIVER_AUTH_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879795i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DRIVER_DIGIOUT_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879792i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_DRMV2CLT_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879434i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ENCRYPT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ENUM_LICENSE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_ERROR_BAD_NET_RESP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879778i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_EXPIRED_LICENSEBLOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879437i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_GET_CONTENTSTRING_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879811i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_GET_LICENSESTRING_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879812i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_GET_LICENSE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879815i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_HARDWAREID_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879729i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_HARDWARE_INCONSISTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879788i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INCLUSION_LIST_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879435i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INDIVIDUALIZATION_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879796i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INDIVIDUALIZE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879818i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INDIVIDUALIZING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879828i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INDIV_FRAUD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879549i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INDIV_NO_CABS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879548i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INDIV_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879547i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_APPCERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879748i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_APPDATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879808i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_APPDATA_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879807i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_APPLICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879456i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_CRL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879439i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879775i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_KID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879543i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_LICENSEBLOB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879436i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_LICENSE_ACQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_LICENSE_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_MACHINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_MIGRATION_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879736i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879799i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_PROXIMITY_RESPONSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879448i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_SECURESTORE_PASSWORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879791i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_INVALID_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879447i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_KEY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879839i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_APPSECLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879654i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_APP_NOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879651i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_CERT_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879649i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_CLOSE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879816i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_CONTENT_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879647i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_DELETION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879538i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879656i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_INITIALIZATION_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879542i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_INVALID_XML: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_NOSAP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879606i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_NOSVP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879605i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_NOTACQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879783i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_NOTENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879655i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_NOTRUSTEDCODEC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879603i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_NOWDM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879604i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_OPEN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879817i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_SECLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879648i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_SERVER_INFO_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879552i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_STORE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_STORE_SAVE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879454i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LICENSE_UNUSABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879800i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_LIC_NEEDS_DEVICE_CLOCK_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879751i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MALFORMED_CONTENT_HEADER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879716i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_IMPORTER_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879734i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879727i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_SST_PASSWORD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879725i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_LICENSE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879726i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879724i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_OBJECT_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879717i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_OPERATION_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879718i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_TARGET_NOT_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879737i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MIGRATION_TARGET_STATES_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879735i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MONITOR_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879810i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MUST_APPROVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879450i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MUST_REGISTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879451i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_MUST_REVALIDATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879449i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NEEDS_INDIVIDUALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879832i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NEEDS_UPGRADE_TEMPFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879555i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NEED_UPGRADE_MSSAP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879794i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NEED_UPGRADE_PD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879554i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879772i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NO_RIGHTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879840i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_NO_UPLINK_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879544i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_OPERATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879768i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_PARAMETERS_MISMATCHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879825i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_PASSWORD_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882797i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_PD_TOO_MANY_DEVICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879550i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_POLICY_DISABLE_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879774i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_POLICY_METERING_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879754i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_PROFILE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882801i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_CHALLENGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879746i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_PETITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879747i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_QUERY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879814i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_REOPEN_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879793i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_REPORT_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879813i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_RESTORE_FRAUD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879789i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_RESTORE_SERVICE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879546i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_RESTRICTIONS_NOT_RETRIEVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879767i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_RIV_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879433i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SDK_VERSIONMISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879752i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SDMI_NOMORECOPIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879786i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SDMI_TRIGGER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879787i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SECURE_STORE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SECURE_STORE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879798i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SECURE_STORE_UNLOCK_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SECURITY_COMPONENT_SIGNATURE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879776i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SIGNATURE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879553i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_SOURCEID_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879602i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_STORE_NEEDINDI: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879653i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_STORE_NOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879652i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_STORE_NOTALLSTORED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879777i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_STUBLIB_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879739i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_TRACK_EXCEEDED_PLAYLIST_RESTICTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879760i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_TRACK_EXCEEDED_TRACKBURN_RESTRICTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879759i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_TRANSFER_CHAINED_LICENSES_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879753i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_ACQUIRE_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_AUTHENTICATION_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879773i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_BACKUP_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879819i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_CERTIFICATE_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879738i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_CODING_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879782i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_DECRYPT_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879821i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_DEVICE_REGISTRATION_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879764i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_ENCRYPT_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_HEADER_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879785i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_INDI_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_INMEMORYSTORE_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879740i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_KEYS_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879784i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_LICENSE_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879824i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_METERING_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879763i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_MIGRATION_IMPORTER_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879723i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_BURN_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879765i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879766i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_PROPERTIES_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879820i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_CREATE_STATE_DATA_OBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879781i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_GET_DEVICE_CERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879758i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879757i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK_FROM_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879755i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_INITIALIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_LOAD_HARDWARE_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879743i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_OPEN_DATA_STORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879742i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_OPEN_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_OPEN_PORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879441i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_SET_PARAMETER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879809i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_SET_SECURE_CLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879756i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNABLE_TO_VERIFY_PROXIMITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879452i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNSUPPORTED_ACTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879443i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNSUPPORTED_ALGORITHM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879539i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNSUPPORTED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879779i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DRM_UNSUPPORTED_PROTOCOL_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879444i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DUPLICATE_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889801i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DUPLICATE_DRMPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882800i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889802i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DUPLICATE_PACKET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886829i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_AUTHORING_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885404i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_CANNOT_COPY_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885390i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_CANNOT_JUMP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885393i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_COMPATIBLE_VIDEO_CARD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885402i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_COPY_PROTECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885405i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_DEVICE_CONTENTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885392i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885407i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_NS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885408i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_DISC_DECODER_REGION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885399i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_GRAPH_BUILDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885396i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_INVALID_DISC_REGION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885403i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_INVALID_TITLE_CHAPTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885388i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_MACROVISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885401i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_NO_AUDIO_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885397i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_NO_DECODER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885395i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_NO_SUBPICTURE_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885406i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_NO_VIDEO_MEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885391i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_NO_VIDEO_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885398i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_PARENTAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885394i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_REQUIRED_PROPERTY_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885389i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_DVD_SYSTEM_DECODER_REGION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885400i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_EDL_REQUIRED_FOR_DEVICE_MULTIPASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882713i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_EMPTY_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884555i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_EMPTY_PROGRAM_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889642i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ENACTPLAN_GIVEUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889752i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_END_OF_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876856i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_END_OF_TAPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882770i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ERROR_FROM_PROXY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_EXCEED_MAX_DRM_PROFILE_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882720i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_EXPECT_MONO_WAV_INPUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882783i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FAILED_DOWNLOAD_ABORT_BURN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885540i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FAIL_LAUNCH_ROXIO_PLUGIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885376i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FEATURE_DISABLED_BY_GROUP_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886820i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FEATURE_DISABLED_IN_SKU: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886819i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FEATURE_REQUIRES_ENTERPRISE_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884349i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_ALLOCATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889826i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_BANDWIDTH_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889808i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889829i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_FAILED_CHECKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885811i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_INIT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889825i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889830i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_OPEN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889827i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_PLAY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889824i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_READ: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889831i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FILE_WRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889832i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FIREWALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877831i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_FLASH_PLAYBACK_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885553i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_GLITCH_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889451i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_GRAPH_NOAUDIOLANGUAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885563i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_GRAPH_NOAUDIOLANGUAGESELECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885562i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_HDS_KEY_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879719i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_HEADER_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884449i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_HTTP_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889645i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_HTTP_TEXT_DATACONTAINER_INVALID_SERVER_RESPONSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884340i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_HTTP_TEXT_DATACONTAINER_SIZE_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884343i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_ICMQUERYFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882836i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_IE_DISALLOWS_ACTIVEX_CONTROLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885554i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_IMAGE_DOWNLOAD_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885106i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_IMAPI_LOSSOFSTREAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885378i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_IMAPI_MEDIUM_INVALIDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885374i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INCOMPATIBLE_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889791i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INCOMPATIBLE_PUSH_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877812i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INCOMPATIBLE_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INCOMPATIBLE_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INCOMPLETE_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885182i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INCORRECTCLIPSETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882820i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INDUCED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INPUTSOURCE_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882806i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INPUT_DOESNOT_SUPPORT_SMPTE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882776i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INPUT_WAVFORMAT_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882782i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INSUFFICIENT_BANDWIDTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889812i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INSUFFICIENT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889654i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INTERFACE_NOT_REGISTERED_IN_GIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885142i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INTERLACEMODE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882773i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INTERLACE_REQUIRE_SAMESIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882795i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889820i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INTERNAL_SERVER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALIDCALL_WHILE_ARCHIVAL_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882828i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALIDCALL_WHILE_ENCODER_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALIDCALL_WHILE_ENCODER_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882817i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALIDINPUTFPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882815i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALIDPACKETSIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882827i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALIDPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_ARCHIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889795i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_AUDIO_BUFFERMAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882756i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_AUDIO_PEAKRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882758i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_AUDIO_PEAKRATE_2: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882757i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_BLACKHOLE_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889792i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_CHANNEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889797i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_CLIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889793i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889809i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882799i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_DRMV2CLT_STUBLIB: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879728i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_EDL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886824i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_FILE_BITRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882735i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_FOLDDOWN_COEFFICIENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882732i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889839i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INDEX2: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889639i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INPUT_AUDIENCE_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882786i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INPUT_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886856i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INPUT_LANGUAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882785i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INPUT_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882784i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INTERLACEMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882725i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_INTERLACE_COMPAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882724i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889790i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_LOG_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884347i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_MTU_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884346i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889828i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_NONSQUAREPIXEL_COMPAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882723i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_NUM_PASSES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886827i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_OPERATING_SYSTEM_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884647i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_OUTPUT_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PIXEL_ASPECT_RATIO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882718i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PLAY_STATISTICS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884345i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PLUGIN_LOAD_TYPE_CONFIGURATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884652i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889789i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PROFILE_CONTENTTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882716i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PUBLISHING_POINT_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884651i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884453i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT_START_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884645i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_PUSH_TEMPLATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884454i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_QUERY_OPERATOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_QUERY_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_REDIRECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889813i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_SAMPLING_RATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886832i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_SCRIPT_BITRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882737i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_SOURCE_WITH_DEVICE_CONTROL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882722i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889796i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_TIMECODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882730i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_TTL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889788i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VBR_COMPAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882766i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VBR_WITH_UNCOMP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882764i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_BITRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882753i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882743i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_BUFFERMAX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882742i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_BUFFERMAX_2: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882741i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_CQUALITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882744i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_FPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882747i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_HEIGHT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882748i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_HEIGHT_ALIGN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882739i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_IQUALITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882745i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_KEYFRAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882746i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_PEAKRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882751i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_PEAKRATE_2: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882750i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_WIDTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882749i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_WIDTH_ALIGN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882740i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_INVALID_VIDEO_WIDTH_FOR_INTERLACED_ENCODING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882712i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LANGUAGE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882788i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LATE_OPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889810i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LATE_PACKET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886830i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LICENSE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889644i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LICENSE_HEADER_MISSING_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879750i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LICENSE_INCORRECT_RIGHTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LICENSE_OUTOFDATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LICENSE_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LOGFILEPERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889784i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LOG_FILE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889782i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_LOG_NEED_TO_BE_SKIPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884344i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MARKIN_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882711i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MAX_BITRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889785i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MAX_CLIENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889783i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MAX_FILERATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889781i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MAX_FUNNELS_ALERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889760i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MAX_PACKET_SIZE_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886831i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MEDIACD_READ_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885555i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MEDIA_LIBRARY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885810i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MEDIA_PARSER_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884351i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MEMSTORAGE_BAD_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885381i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_CACHE_DATA_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_CANNOT_RETRIEVE_FROM_OFFLINE_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876834i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_CANNOT_SET_LOCALE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_IDENTIFIER_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_INVALID_DOCUMENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876836i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_LANGUAGE_NOT_SUPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876840i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_NO_EDITING_CAPABILITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_METADATA_NO_RFC1766_NAME_FOR_LOCALE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876839i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MISMATCHED_MEDIACONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MISSING_AUDIENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882792i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MISSING_CHANNEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889641i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MISSING_SOURCE_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882790i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MIXER_INVALID_CONTROL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MIXER_INVALID_LINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MIXER_INVALID_VALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MIXER_NODRIVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MIXER_UNKNOWN_MMRESULT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MLS_SMARTPLAYLIST_FILTER_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885643i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MMSAUTOSERVER_CANTFINDWALKER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889786i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MMS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877830i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MONITOR_GIVEUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889656i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MP3_FORMAT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MPDB_GENERIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885812i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MSAUDIO_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MSBD_NO_LONGER_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTICAST_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTICAST_PLUGIN_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884648i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_AUDIO_CODECS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882761i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_AUDIO_FORMATS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882760i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_FILE_BITRATES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882736i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_SCRIPT_BITRATES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882738i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_VBR_AUDIENCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882763i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_VIDEO_CODECS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882755i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_MULTIPLE_VIDEO_SIZES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882754i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_BAD_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_BUFFER_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_CALLBACK_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_DUPLICATE_CALLBACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_EMPTY_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_INDEX_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_NAME_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_NODE_CONFLICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_NODE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_TOO_MANY_CALLBACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_WRONG_PERSIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_WRONG_SECURITY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NAMESPACE_WRONG_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NEED_CORE_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885556i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NEED_TO_ASK_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885798i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NETWORK_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889842i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NETWORK_RESOURCE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889816i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NETWORK_SERVICE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889817i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NETWORK_SINK_WRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877832i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NET_READ: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NET_WRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889836i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOCONNECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOFUNNEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOMATCHING_ELEMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOMATCHING_MEDIASOURCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NONSQUAREPIXELMODE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882772i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOREGISTEREDWALKER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889845i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOSOURCEGROUPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882816i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOSTATSAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882819i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOTARCHIVING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882818i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOTHING_TO_DO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072887823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOTITLES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889794i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOT_CONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOT_CONTENT_PARTNER_TRACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884902i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOT_LICENSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889651i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NOT_REBUILDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889811i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_ACTIVE_SOURCEGROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882830i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_AUDIENCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882768i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_AUDIODATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882807i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_AUDIO_COMPAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882767i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_AUDIO_TIMECOMPRESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882729i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_CD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885856i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_CD_BURNER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885386i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_CHANNELS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889640i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_DATAVIEW_SUPPORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882814i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889743i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_ERROR_STRING_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885808i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_EXISTING_PACKETIZER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877827i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_FORMATS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889749i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_FRAMES_SUBMITTED_TO_ANALYZER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882777i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_LOCALPLAY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MBR_WITH_TIMECODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882726i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MEDIAFORMAT_IN_SOURCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882833i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MEDIA_IN_AUDIENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882769i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MEDIA_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889445i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MORE_SAMPLES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886833i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MULTICAST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072887822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_MULTIPASS_FOR_LIVEDEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882793i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_NEW_CONNECTIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884451i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_PAL_INVERSE_TELECINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882780i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_PDA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885383i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_PROFILE_IN_SOURCEGROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_PROFILE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882765i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_REALTIME_PREPROCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882804i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_REALTIME_TIMECOMPRESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882810i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_REFERENCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889748i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_REPEAT_PREPROCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882803i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_SCRIPT_ENGINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884356i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_SCRIPT_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882829i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_SERVER_CONTACT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889650i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_SMPTE_WITH_MULTIPLE_SOURCEGROUPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882775i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_SPECIFIED_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889742i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889805i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_TWOPASS_TIMECOMPRESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882728i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_VALID_OUTPUT_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882832i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NO_VALID_SOURCE_PLUGIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882831i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_NUM_LANGUAGE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882789i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_OFFLINE_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_OPEN_CONTAINING_FOLDER_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884893i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_OPEN_FILE_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889807i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_OUTPUT_PROTECTION_LEVEL_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879356i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_OUTPUT_PROTECTION_SCHEME_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879350i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PACKETSINK_UNKNOWN_FEC_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877814i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PAGING_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889758i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PARTIALLY_REBUILT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889753i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_CREATE_ADDITIONAL_SYNC_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885371i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_SYNC_FROM_INTERNET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885196i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_SYNC_FROM_LOCATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885357i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_SYNC_INVALID_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885195i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_TRANSCODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885367i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_AUDIO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885187i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885185i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_VIDEO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885186i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_CEWMDM_DRM_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885183i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_DELETE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885192i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_DEVICESUPPORTDISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885360i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_DEVICE_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885377i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_DEVICE_FULL_IN_SESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885375i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_DEVICE_NOT_RESPONDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885190i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_ENCODER_NOT_RESPONDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885358i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAILED_TO_BURN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885542i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAILED_TO_ENCRYPT_TRANSCODED_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885188i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAILED_TO_RETRIEVE_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885191i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAILED_TO_SYNCHRONIZE_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885194i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAILED_TO_TRANSCODE_PHOTO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885189i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAIL_READ_WAVE_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885379i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_FAIL_SELECT_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885380i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_INITIALIZINGDEVICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885363i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_MANUALDEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885373i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_NO_LONGER_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885359i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_NO_TRANSCODE_OF_DRM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885370i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_OBSOLETE_SP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885362i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_PARTNERSHIPNOTEXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885372i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_RETRIEVED_FILE_FILENAME_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885184i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_SYNC_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885193i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_SYNC_LOGIN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885180i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_SYNC_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885181i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_TITLE_COLLISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885361i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_TOO_MANY_FILES_IN_DIRECTORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885366i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_TOO_MANY_FILE_COLLISIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885368i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_TRANSCODECACHEFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885369i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_TRANSCODE_CODEC_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885179i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_TRANSCODE_NOT_PERMITTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885364i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_UNSPECIFIED_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885382i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PDA_UNSUPPORTED_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885384i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_CONTAINS_ERRORS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885569i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_END_RECEDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884547i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_ENTRY_ALREADY_PLAYING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884556i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_ENTRY_HAS_CHANGED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_ENTRY_NOT_IN_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884552i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_ENTRY_SEEK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884551i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_PARSE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884554i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_PLUGIN_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884353i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_RECURSIVE_PLAYLISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884550i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_SHUTDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884548i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_TOO_MANY_NESTED_PLAYLISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884549i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLAYLIST_UNSUPPORTED_ENTRY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884553i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLUGIN_CLSID_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882826i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLUGIN_ERROR_REPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884355i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PLUGIN_NOTSHUTDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885802i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PORT_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884342i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PORT_IN_USE_HTTP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884341i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROCESSINGSHOWSYNCWIZARD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885365i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROFILE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882821i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROPERTY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROPERTY_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876846i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROPERTY_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROTECTED_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROTOCOL_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROXY_ACCESSDENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877834i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROXY_CONNECT_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877817i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROXY_DNS_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877840i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROXY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROXY_SOURCE_ACCESSDENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877833i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PROXY_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PUBLISHING_POINT_INVALID_REQUEST_WHILE_STARTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884649i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PUBLISHING_POINT_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884646i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PUBLISHING_POINT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884642i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PUSH_CANNOTCONNECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877813i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_PUSH_DUPLICATE_PUBLISHING_POINT_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884448i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REBOOT_RECOMMENDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REBOOT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_RECORDQ_DISK_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882781i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REDBOOK_ENABLED_WHILE_COPYING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885840i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REDIRECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884856i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REDIRECT_TO_PROXY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REFUSED_BY_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REG_FLUSH_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879720i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REMIRRORED_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889655i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_REQUIRE_STREAMING_CLIENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877836i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_RESET_SOCKET_CONNECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877824i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_RESOURCE_GONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877828i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SAME_AS_INPUT_COMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882734i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SCHEMA_CLASSIFY_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SCRIPT_DEBUGGER_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884350i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SDK_BUFFERTOOSMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886828i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SERVER_ACCESSDENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877829i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SERVER_DNS_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SERVER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889803i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SERVER_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SESSION_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877816i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SESSION_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SETUP_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878848i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SETUP_DRM_MIGRATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SETUP_DRM_MIGRATION_FAILED_AND_IGNORABLE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SETUP_IGNORABLE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878850i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SETUP_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072878852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SET_DISK_UID_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SHARING_STATE_OUT_OF_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885772i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SHARING_VIOLATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885809i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SHUTDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889814i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SLOW_READ_DIGITAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SLOW_READ_DIGITAL_WITH_ERRORCORRECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885251i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SMPTEMODE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882771i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SOURCEGROUP_NOTPREPARED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SOURCE_CANNOT_LOOP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882733i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SOURCE_NOTSPECIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882811i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SOURCE_PLUGIN_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884354i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SPEECHEDL_ON_NON_MIXEDMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882798i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_STALE_PRESENTATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_STREAM_END: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889804i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_STRIDE_REFUSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889787i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SUBSCRIPTIONSERVICE_DOWNLOAD_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884896i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SUBSCRIPTIONSERVICE_LOGIN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884897i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SUBSCRIPTIONSERVICE_PLAYBACK_DISALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884906i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SYNCWIZ_CANNOT_CHANGE_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885265i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_SYNCWIZ_DEVICE_FULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885266i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TABLE_KEY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TAMPERED_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886849i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TCP_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889646i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TIGER_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889776i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TIMECODE_REQUIRES_VIDEOSTREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882727i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TITLE_BITRATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889643i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TITLE_SIZE_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889648i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_AUDIO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882852i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_DEVICECONTROL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882794i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_HOPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_MULTICAST_SINKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884650i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_SESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889841i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_TITLES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889649i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MANY_VIDEO: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882851i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MUCH_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886836i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TOO_MUCH_DATA_FROM_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877819i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_ALBUM_PURCHASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884901i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_PURCHASE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884900i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TRACK_PURCHASE_MAXIMUM_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884899i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TRANSCODE_DELETECACHEERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885264i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TRANSFORM_PLUGIN_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882714i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_TRANSFORM_PLUGIN_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882715i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UDP_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889647i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNABLE_TO_CREATE_RIP_LOCATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885552i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNCOMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879351i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNCOMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072879354i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNCOMP_COMP_COMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882762i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNEXPECTED_DISPLAY_SETTINGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882808i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNEXPECTED_MSAUDIO_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886854i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNKNOWN_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072877856i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNRECOGNIZED_STREAM_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889818i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_ARCHIVEOPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882824i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_ARCHIVETYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882825i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_ENCODER_DEVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882809i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_LANGUAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884644i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_LOAD_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884653i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_UNSUPPORTED_SOURCETYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882853i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_URLLIST_INVALIDFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885651i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_USER_STOP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885847i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_USE_FILE_SOURCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072876855i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VBRMODE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882787i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDCAPCREATEWINDOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDCAPDRVINUSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882834i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDCAPSTARTFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882839i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDEODEVICE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDEODEVICE_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDEODRIVER_UNSTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882840i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDEO_BITRATE_STEPDOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882752i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDEO_CODEC_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886843i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDEO_CODEC_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886844i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDSOURCECOMPRESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882838i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_VIDSOURCESIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WALKER_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889779i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WALKER_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889780i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WALKER_USAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889778i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WAVE_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072889747i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WINSOCK_ERROR_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885463i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WIZARD_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884348i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMDM_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885572i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMDRM_DEPRECATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072886818i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WME_VERSION_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072882805i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_CANNOTQUEUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885684i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_COPP_SECURITY_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885678i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_COPP_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885677i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_FILETRANSFERNOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885672i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_INVALIDSTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885676i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_INVALID_COPP_CERTIFICATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885679i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_LICENSE_TAMPERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885660i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_NOSDKINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885674i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_NOTALLOUTPUTSRENDERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885673i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_PLUGINUNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885685i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_PREROLLLICENSEACQUISITIONNOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885683i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_RATEUNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885686i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_SINKALREADYEXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885675i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMG_UNEXPECTEDPREROLLSTATUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885682i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_BACKUPCANCEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885455i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_BACKUPRESTOREFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885448i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_DRIVE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885449i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_ERRORWITHURL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885453i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_NAMECOLLISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885452i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_NOLISTENER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885456i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPBR_RESTORECANCEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885454i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_BUFFERTOOSMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885633i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885577i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_COCREATEFAILEDFORGITOBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885635i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_CODEC_DOWNLOAD_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885604i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_CODEC_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885605i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_CODEC_NOT_TRUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885606i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_CURRENT_MEDIA_NOT_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885591i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_DEVICE_DRIVERS_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885539i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_ERRORMANAGERNOTAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885619i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_ERRORSINKNOTREGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885620i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_ERROR_DOWNLOADING_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885603i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_FAILEDTOGETMARSHALLEDEVENTHANDLERINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885634i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_FAILED_TO_BUILD_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885602i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885574i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_GRAPH_NOT_IN_LIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885622i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_INVALIDPLAYLISTMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885631i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_INVALID_PLAYLIST_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885585i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_ITEMNOTINPLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885626i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_LIST_ENTRY_NO_REF: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885608i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MEDIA_ALTERNATE_REF_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885596i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MEDIA_CHILD_PLAYLIST_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885576i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MEDIA_ERROR_RESUME_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885617i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MEDIA_NO_CHILD_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885575i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MEDIA_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885581i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MEDIA_URL_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885560i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MISMATCHED_RUNTIME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885584i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_MISNAMED_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885607i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_NOBROWSER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885624i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_NOSOURCEURLSTRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885636i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_NO_PLAYABLE_MEDIA_IN_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885579i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_NO_REF_IN_ENTRY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885616i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLISTEMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885625i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_NESTED_PLAYLIST_SKIPPED_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885578i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_OR_SINGLE_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885621i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_EVENT_ATTRIBUTE_ABSENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885594i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_EVENT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885593i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_IMPORT_FAILED_NO_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885583i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_EXHAUSTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885600i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_INIT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885597i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_MORPH_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885598i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NAME_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885599i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885601i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_NO_EVENT_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885595i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885588i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_END_MEDIA_NONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885586i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_START_MEDIA_NONE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885587i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_PLAYLIST_STACK_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885592i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_SOME_CODECS_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885551i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_TEMP_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885573i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885632i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_UNRECOGNIZED_MEDIA_URL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885623i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_USER_CANCEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885589i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_VIDEO_TRANSFORM_FILTER_INSERTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885582i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WEBHELPFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885618i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_ENTRYREF_NO_REF: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885580i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885615i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_ILLEGAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885614i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885613i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_ILLEGAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885612i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885611i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_ILLEGAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885610i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_VALUE_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885609i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPFLASH_CANT_FIND_COM_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885559i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPFLASH_INCOMPATIBLEVERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885558i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPIM_DIALUPFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885464i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPIM_USERCANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885465i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPIM_USEROFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885466i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCXGRAPH_IE_DISALLOWS_ACTIVEX_CONTROLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885557i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_ERRORMANAGERNOTAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885803i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_NOT_RUNNING_REMOTELY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885805i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_NO_ACTIVE_CORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885806i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_NO_REMOTE_CORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885807i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_NO_REMOTE_WINDOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885804i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_PLAYER_NOT_DOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885797i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_REMOTE_PLAYER_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885766i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPOCX_UNABLE_TO_LOAD_SKIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885781i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPXML_ATTRIBUTENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885833i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPXML_EMPTYDOC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885831i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPXML_ENDOFDATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885835i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPXML_NOERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885836i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPXML_PARSEERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885834i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPXML_PINOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885832i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPZIP_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885735i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPZIP_FILENOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885734i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMPZIP_NOTAZIPFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885736i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885294i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_ADDTOLIBRARY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885817i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_ALREADY_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885346i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_AUDIO_CODEC_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885305i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_AUDIO_DEVICE_LOST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885275i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_AUDIO_HW_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885318i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_AUTOPLAY_INVALID_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884996i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BAD_DRIVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885295i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BMP_BITMAP_NOT_CREATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885712i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BMP_COMPRESSION_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885711i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BMP_INVALID_BITMASK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885714i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BMP_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885710i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BMP_TOPDOWN_DIB_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885713i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BSTR_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885006i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_BURN_DISC_OVERFLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885287i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CANNOT_BURN_NON_LOCAL_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885546i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CANNOT_FIND_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885353i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CANNOT_FIND_FOLDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885801i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CANT_PLAY_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885773i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CD_ANOTHER_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885297i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CD_STASH_NO_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885291i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CODEC_NEEDED_WITH_4CC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885343i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CODEC_NEEDED_WITH_FORMATTAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885342i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_COMPONENT_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884986i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONNECT_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885311i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_FILE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885413i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_FILE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885416i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_NO_RIGHTS_ERRORURL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885415i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_NO_RIGHTS_NOERRORURL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885414i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_ERRORURL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885412i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_NOERRORURL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885411i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CONVERT_PLUGIN_UNKNOWN_FILE_OWNER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885410i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CS_JPGPOSITIONIMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885746i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_CS_NOTEVENLYDIVISIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885745i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DAI_SONGTOOSHORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885687i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_ACQUIRING_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885246i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_CANNOT_RESTORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885288i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_COMPONENT_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885278i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_CORRUPT_BACKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885324i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_DRIVER_AUTH_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885302i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_GENERIC_LICENSE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885286i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_INDIV_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885283i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_INVALID_SIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885289i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_CONTENT_REVOKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885241i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885245i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_NOSAP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885240i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_NOTACQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885244i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_NOTENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885243i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_SERVER_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885323i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_LICENSE_UNUSABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885242i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_NEEDS_AUTHORIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885296i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_NEW_HARDWARE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885290i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_NOT_ACQUIRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885055i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_NO_DEVICE_CERT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885277i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_NO_RIGHTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885284i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_NO_SECURE_CLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885285i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DRM_UNABLE_TO_ACQUIRE_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885239i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_DSHOW_UNSUPPORTED_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885350i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_ERASE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885548i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_EXTERNAL_NOTREADY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885796i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FAILED_TO_OPEN_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885692i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FAILED_TO_OPEN_WMD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885774i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FAILED_TO_RIP_TRACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885549i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FAILED_TO_SAVE_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885777i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FAILED_TO_SAVE_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885775i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FILESCANALREADYSTARTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885826i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FILE_DOES_NOT_FIT_ON_CD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885544i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FILE_NO_DURATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885543i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FILE_OPEN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885327i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FILE_TYPE_CANNOT_BURN_TO_AUDIO_CD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885545i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_FORMAT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885547i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_GIF_BAD_VERSION_NUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885722i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_GIF_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885723i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_GIF_NO_IMAGE_IN_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885721i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_GIF_UNEXPECTED_ENDOFFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885724i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_GOFULLSCREEN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885313i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_HME_INVALIDOBJECTID: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885825i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_HME_NOTSEARCHABLEFORITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_HME_STALEREQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885822i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_HWND_NOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885156i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAGE_FILETYPE_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885726i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAGE_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885725i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI2_ERASE_DEVICE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885279i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI2_ERASE_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885280i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_DEVICE_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885330i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_DEVICE_INVALIDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885303i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_DEVICE_NOTPRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885331i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885345i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_GENERIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885333i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_LOSS_OF_STREAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885329i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_IMAPI_MEDIA_INCOMPATIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885274i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_ASX: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885347i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885298i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_LIBRARY_ADD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885316i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_MAX_VAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885751i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_MIN_VAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885750i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_PROTOCOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885317i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_REQUEST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885292i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_INVALID_SKIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885780i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPGTRANSPARENCY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885755i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_BAD_DCTSIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885707i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_BAD_PRECISION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885705i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_BAD_VERSION_NUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885706i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_CCIR601_NOTIMPL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885704i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_FRACT_SAMPLE_NOTIMPL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885701i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_IMAGE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885700i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_INVALID_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885708i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_JERR_ARITHCODING_NOTIMPL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885709i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_NO_IMAGE_IN_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885703i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_READ_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885702i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_SOF_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885698i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_UNEXPECTED_ENDOFFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885699i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_JPG_UNKNOWN_MARKER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885697i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_LICENSE_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885238i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_LICENSE_RESTRICTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885293i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_LOCKEDINSKINMODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885778i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_LOGON_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885354i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_MF_CODE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885824i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_MLS_STALE_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885795i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_MMS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885315i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_MSSAP_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885341i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_MULTICAST_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885310i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_MULTIPLE_ERROR_IN_PLAYLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885281i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NEED_UPGRADE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885319i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NETWORK_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885312i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NETWORK_FIREWALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885322i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NETWORK_RESOURCE_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885301i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NONMEDIA_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885348i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NO_DISK_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885355i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NO_PROTOCOLS_SELECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885314i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_NO_REMOVABLE_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885321i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_OUTOFMEMORY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885306i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PATH_ALREADY_IN_LIBRARY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885830i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PLAYLIST_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885349i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PLUGINDLL_NOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885799i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PNG_INVALIDFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885720i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PNG_UNSUPPORTED_BAD_CRC: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885715i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PNG_UNSUPPORTED_BITDEPTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885719i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PNG_UNSUPPORTED_COMPRESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885718i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PNG_UNSUPPORTED_FILTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885717i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PNG_UNSUPPORTED_INTERLACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885716i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_POLICY_VALUE_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885206i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PROTECTED_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885237i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PROTOCOL_PROBLEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885356i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PROXY_CONNECT_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885320i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_PROXY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885308i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_RBC_JPGMAPPINGIMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885756i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_RECORDING_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885815i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_RIP_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885550i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SAVEAS_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885776i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SENDMAILFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885779i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SERVER_DNS_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885309i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SERVER_INACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885352i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SERVER_NONEWCONNECTIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885282i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SERVER_NOT_RESPONDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885325i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SERVER_SECURITY_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885276i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_SERVER_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885328i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_STREAMING_RECORDING_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885800i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_TAMPERED_CONTENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885307i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UDRM_NOUSERLIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885056i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_NOSKININZIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885785i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_NOTATHEMEFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885792i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_OBJECTNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885787i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_PASSTHROUGH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885788i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_SECONDHANDLER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885786i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_SUBCONTROLSNOTSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885794i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_SUBELEMENTNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885791i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_VERSIONMISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885793i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_VERSIONPARSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885790i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UI_VIEWIDNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885789i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UNKNOWN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885299i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UNSUPPORTED_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885351i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_UPGRADE_APPLICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885300i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_URLDOWNLOADFAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885782i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_VERIFY_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885326i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_VIDEO_CODEC_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885304i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WINDOWSAPIFAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885816i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_BUSY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885336i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885344i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_INCORRECT_RIGHTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885334i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_INTERFACEDEAD: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885340i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_LICENSE_EXPIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885337i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_LICENSE_NOTEXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885338i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_NORIGHTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885335i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMP_WMDM_NOTCERTIFIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885339i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_CANNOT_RENDER_BINARY_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885661i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_NOCALLBACKAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885666i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_NOSOURCEFILTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885668i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_PINNOTFOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885670i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_PINTYPENOMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885667i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_SAMPLEPROPERTYNOTSET: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885662i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_UNSUPPORTEDSTREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885671i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_WAITINGONFORMATSWITCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885669i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMR_WILLNOT_RENDER_BINARY_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885659i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_ATTRIBUTE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885649i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_ATTRIBUTE_DOES_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885650i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_ATTRIBUTE_UNRETRIEVABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885648i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_INVALID_FORMAT_OVER_NESTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885642i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_ITEM_DOES_NOT_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885647i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_ITEM_TYPE_ILLEGAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885646i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_ITEM_UNSETTABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885645i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_PLAYLIST_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885644i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WMX_UNRECOGNIZED_PLAYLIST_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885656i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WONT_DO_DIGITAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072885837i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WRONG_OS_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884643i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WRONG_PUBLISHING_POINT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884654i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_E_WSX_INVALID_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1072884450i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_CATATONIC_AUTO_UNFAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146631270i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_CATATONIC_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146631271i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_CUB_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593874i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_CUB_START: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593873i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_CUB_UNFAIL_LINK: ::windows::core::HRESULT = ::windows::core::HRESULT(1074594193i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_DISK_REBUILD_ABORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593880i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_DISK_REBUILD_FINISHED: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593879i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_DISK_REBUILD_STARTED: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593878i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_DISK_START: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593876i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_DISK_STOP: ::windows::core::HRESULT = ::windows::core::HRESULT(1074594200i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_EXISTING_PACKETIZER: ::windows::core::HRESULT = ::windows::core::HRESULT(1074605827i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_KILL_CONNECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593886i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_KILL_USERSESSION: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593885i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_LIMIT_BANDWIDTH: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593904i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_LIMIT_FUNNELS: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593881i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_LOGGING_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593902i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_MANUAL_PROXY: ::windows::core::HRESULT = ::windows::core::HRESULT(1074605828i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_NOLOG_STOP: ::windows::core::HRESULT = ::windows::core::HRESULT(1074605825i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_PLAYLIST_CHANGE_RECEDING: ::windows::core::HRESULT = ::windows::core::HRESULT(1074599102i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_REBUILD_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593887i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_RECONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(1074605823i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_RESTRIPE_CUB_OUT: ::windows::core::HRESULT = ::windows::core::HRESULT(1074594199i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_RESTRIPE_DISK_OUT: ::windows::core::HRESULT = ::windows::core::HRESULT(1074594198i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_RESTRIPE_DONE: ::windows::core::HRESULT = ::windows::core::HRESULT(1074594196i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_RESTRIPE_START: ::windows::core::HRESULT = ::windows::core::HRESULT(1074594195i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_START_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593882i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_STOP_CUB: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593884i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_STOP_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593883i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_I_TIGER_START: ::windows::core::HRESULT = ::windows::core::HRESULT(1074593871i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_CALLABORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(851969i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_CALLPENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(851968i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_CHANGENOTICE: ::windows::core::HRESULT = ::windows::core::HRESULT(864013i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DEGRADING_QUALITY: ::windows::core::HRESULT = ::windows::core::HRESULT(854985i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_ACQUIRE_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(862023i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_BURNABLE_TRACK: ::windows::core::HRESULT = ::windows::core::HRESULT(862062i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_BURNABLE_TRACK_WITH_PLAYLIST_RESTRICTION: ::windows::core::HRESULT = ::windows::core::HRESULT(862063i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_INDIVIDUALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(861991i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_LICENSE_ACQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(861990i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_MONITOR_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(862022i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_DRM_NEEDS_INDIVIDUALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(862174i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_EOSRECEDING: ::windows::core::HRESULT = ::windows::core::HRESULT(864009i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_NAVIGATION_COMPLETE_WITH_ERRORS: ::windows::core::HRESULT = ::windows::core::HRESULT(856926i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_NEED_TO_BUY_BURN_RIGHTS: ::windows::core::HRESULT = ::windows::core::HRESULT(856283i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(856398i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_PUBLISHING_POINT_STARTED_WITH_FAILED_SINKS: ::windows::core::HRESULT = ::windows::core::HRESULT(857369i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_REBOOT_RECOMMENDED: ::windows::core::HRESULT = ::windows::core::HRESULT(862968i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_REBOOT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(862969i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_REBUFFERING: ::windows::core::HRESULT = ::windows::core::HRESULT(854984i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_STREAM_TRUNCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(851970i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_TRACK_ALREADY_DOWNLOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(856929i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_TRACK_BUY_REQUIRES_ALBUM_PURCHASE: ::windows::core::HRESULT = ::windows::core::HRESULT(856921i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_TRANSCRYPTOR_EOF: ::windows::core::HRESULT = ::windows::core::HRESULT(855003i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMG_ADVISE_DROP_FRAME: ::windows::core::HRESULT = ::windows::core::HRESULT(856166i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMG_ADVISE_DROP_TO_KEYFRAME: ::windows::core::HRESULT = ::windows::core::HRESULT(856167i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMG_FORCE_DROP_FRAME: ::windows::core::HRESULT = ::windows::core::HRESULT(856143i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPBR_PARTIALSUCCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(856374i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPBR_SUCCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(856373i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_COMMAND_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(856325i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_MEDIA_CHILD_PLAYLIST_OPEN_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(856329i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_MEDIA_VALIDATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(856323i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_MORE_NODES_AVAIABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(856330i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLISTCLEARABORT: ::windows::core::HRESULT = ::windows::core::HRESULT(856318i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLISTREMOVEITEMABORT: ::windows::core::HRESULT = ::windows::core::HRESULT(856319i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLIST_COLLAPSED_TO_SINGLE_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(856328i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLIST_CREATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(856322i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLIST_IMPORT_MISSING_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(856327i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLIST_NAME_AUTO_GENERATED: ::windows::core::HRESULT = ::windows::core::HRESULT(856326i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPCORE_PLAYLIST_REPEAT_SECONDARY_SEGMENTS_IGNORED: ::windows::core::HRESULT = ::windows::core::HRESULT(856324i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPEFFECT_OPAQUE: ::windows::core::HRESULT = ::windows::core::HRESULT(856389i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMPEFFECT_TRANSPARENT: ::windows::core::HRESULT = ::windows::core::HRESULT(856388i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMP_EXCEPTION: ::windows::core::HRESULT = ::windows::core::HRESULT(856041i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMP_LOADED_BMP_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(856130i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMP_LOADED_GIF_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(856128i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMP_LOADED_JPG_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(856131i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMP_LOADED_PNG_IMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(856129i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMP_UI_VERSIONMISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(856040i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMR_ALREADYRENDERED: ::windows::core::HRESULT = ::windows::core::HRESULT(856159i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMR_PINTYPEFULLMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(856161i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_S_WMR_PINTYPEPARTIALMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(856160i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_W_FILE_BANDWIDTH_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146631676i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_W_SERVER_BANDWIDTH_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146631677i32);
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const NS_W_UNKNOWN_EVENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146631584i32);
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct OLIADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for OLIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for OLIADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for OLIADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for OLIADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLIADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for OLIADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct OLICELPWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for OLICELPWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for OLICELPWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for OLICELPWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for OLICELPWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLICELPWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for OLICELPWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLICELPWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct OLIGSMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for OLIGSMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for OLIGSMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for OLIGSMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for OLIGSMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLIGSMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for OLIGSMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLIGSMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct OLIOPRWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for OLIOPRWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for OLIOPRWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for OLIOPRWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for OLIOPRWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLIOPRWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for OLIOPRWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLIOPRWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct OLISBCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for OLISBCWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for OLISBCWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for OLISBCWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for OLISBCWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OLISBCWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for OLISBCWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for OLISBCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDriver<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(szdrivername: Param0, szsectionname: Param1, lparam2: Param2) -> HDRVR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDriver(szdrivername: ::windows::core::PCWSTR, szsectionname: ::windows::core::PCWSTR, lparam2: super::super::Foundation::LPARAM) -> HDRVR;
        }
        ::core::mem::transmute(OpenDriver(szdrivername.into_param().abi(), szsectionname.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const PD_CAN_DRAW_DIB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const PD_CAN_STRETCHDIB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const PD_STRETCHDIB_1_1_OK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const PD_STRETCHDIB_1_2_OK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const PD_STRETCHDIB_1_N_OK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA1_MIXER: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA1_MPU401_IN: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA1_MPU401_OUT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA1_SYNTH: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA1_WAVEIN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA1_WAVEOUT: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA2_MIXER: u32 = 203u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA2_MPU401_IN: u32 = 204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA2_MPU401_OUT: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA2_SYNTH: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA2_WAVEIN: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const ROCKWELL_WA2_WAVEOUT: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEARCH_ANY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEARCH_BACKWARD: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEARCH_FORWARD: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEARCH_KEY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEARCH_NEAREST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEEK_CUR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEEK_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const SEEK_SET: u32 = 0u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct SIERRAADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for SIERRAADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for SIERRAADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for SIERRAADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for SIERRAADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SIERRAADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for SIERRAADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for SIERRAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct SONARCWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wCompType: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for SONARCWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for SONARCWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for SONARCWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for SONARCWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SONARCWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for SONARCWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for SONARCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendDriverMessage<'a, Param0: ::windows::core::IntoParam<'a, HDRVR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hdriver: Param0, message: u32, lparam1: Param2, lparam2: Param3) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendDriverMessage(hdriver: HDRVR, message: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(SendDriverMessage(hdriver.into_param().abi(), ::core::mem::transmute(message), lparam1.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TARGET_DEVICE_FRIENDLY_NAME: &'static str = "TargetDeviceFriendlyName";
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TARGET_DEVICE_OPEN_EXCLUSIVELY: &'static str = "TargetDeviceOpenExclusively";
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TASKERR_NOTASKSUPPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TASKERR_OUTOFMEMORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TDD_BEGINMINPERIOD: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TDD_ENDMINPERIOD: u32 = 2068u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TDD_GETDEVCAPS: u32 = 2060u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TDD_GETSYSTEMTIME: u32 = 2056u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TDD_KILLTIMEREVENT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const TDD_SETTIMEREVENT: u32 = 2052u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct TIMEREVENT {
    pub wDelay: u16,
    pub wResolution: u16,
    pub lpFunction: super::LPTIMECALLBACK,
    pub dwUser: u32,
    pub wFlags: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for TIMEREVENT {}
impl ::core::clone::Clone for TIMEREVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TIMEREVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TIMEREVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TIMEREVENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for TIMEREVENT {}
impl ::core::default::Default for TIMEREVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct TRUESPEECHWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wRevision: u16,
    pub nSamplesPerBlock: u16,
    pub abReserved: [u8; 28],
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for TRUESPEECHWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for TRUESPEECHWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for TRUESPEECHWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for TRUESPEECHWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRUESPEECHWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for TRUESPEECHWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for TRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VADMAD_Device_ID: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VCAPS_CAN_SCALE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VCAPS_DST_CAN_CLIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VCAPS_OVERLAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VCAPS_SRC_CAN_CLIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type VFWWDMExtensionProc = ::core::option::Option<unsafe extern "system" fn(pfndeviceiocontrol: *mut ::core::ffi::c_void, pfnaddpropertypage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_HIDE_CAMERACONTROL_PAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_HIDE_SETTINGS_PAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_HIDE_VIDEOSRC_PAGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_OEM_ADD_PAGE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_QUERY_DEV_CHANGED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_USE_DEVICE_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VFW_USE_STREAM_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VHDR_INQUEUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VHDR_KEYFRAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VHDR_VALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_COMPRESSFRAMES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_CRUNCH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_DRAW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_FASTTEMPORALC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_FASTTEMPORALD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_QUALITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDCF_TEMPORAL: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub struct VIDEOHDR {
    pub lpData: *mut u8,
    pub dwBufferLength: u32,
    pub dwBytesUsed: u32,
    pub dwTimeCaptured: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwReserved: [usize; 4],
}
impl ::core::marker::Copy for VIDEOHDR {}
impl ::core::clone::Clone for VIDEOHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VIDEOHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEOHDR").field("lpData", &self.lpData).field("dwBufferLength", &self.dwBufferLength).field("dwBytesUsed", &self.dwBytesUsed).field("dwTimeCaptured", &self.dwTimeCaptured).field("dwUser", &self.dwUser).field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for VIDEOHDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VIDEOHDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VIDEOHDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for VIDEOHDR {}
impl ::core::default::Default for VIDEOHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_CURRENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_GET: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_MAX: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_MIN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_NOMINAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_QUERY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_QUERYSIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_CONFIGURE_SET: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_DLG_QUERY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_EXTERNALIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_EXTERNALOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_IN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VIDEO_OUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_COMMAND_GET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_COMMAND_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_CP_CMD_ACTIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_CP_CMD_CHANGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_CP_CMD_DEACTIVATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_CP_TYPE_APS_TRIGGER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_CP_TYPE_MACROVISION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_BRIGHTNESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_CONTRAST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_COPYPROTECT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_FLICKER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_MAX_UNSCALED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_OVERSCAN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_POSITION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_TV_MODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_FLAGS_TV_STANDARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_MODE_TV_PLAYBACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_MODE_WIN_GRAPHICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_NTSC_433: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_NTSC_M: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_NTSC_M_J: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_60: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_B: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_D: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_G: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_H: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_I: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_M: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_PAL_N: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_B: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_D: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_G: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_H: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_K: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_K1: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_L: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_SECAM_L1: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const VP_TV_STANDARD_WIN_VGA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn VideoForWindowsVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VideoForWindowsVersion() -> u32;
        }
        ::core::mem::transmute(VideoForWindowsVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct WAVEOPENDESC {
    pub hWave: super::Audio::HWAVE,
    pub lpFormat: *mut super::Audio::WAVEFORMAT,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub uMappedDeviceID: u32,
    pub dnDevNode: usize,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for WAVEOPENDESC {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for WAVEOPENDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for WAVEOPENDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for WAVEOPENDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEOPENDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for WAVEOPENDESC {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for WAVEOPENDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FILTER_DEVELOPMENT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FILTER_ECHO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FILTER_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FILTER_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_3COM_NBX: u32 = 28672u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ADPCM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ALAC: u32 = 27745u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ALAW: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_AMR_NB: u32 = 29537u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_AMR_WB: u32 = 29538u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_AMR_WP: u32 = 29539u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ANTEX_ADPCME: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_APTX: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_AUDIOFILE_AF10: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_AUDIOFILE_AF36: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_BTV_DIGITAL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CANOPUS_ATRAC: u32 = 99u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CIRRUS: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CODIAN: u32 = 41252u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: u32 = 41217u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: u32 = 41216u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: u32 = 41218u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CONGRUENCY: u32 = 141u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CONTROL_RES_CR10: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CONVEDIA_G729: u32 = 140u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CREATIVE_ADPCM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CS2: u32 = 608u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CS_IMAADPCM: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CUSEEME: u32 = 7939u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_CU_CODEC: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DEVELOPMENT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DF_G726: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DF_GSM610: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DICTAPHONE_CELP54: u32 = 322u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DICTAPHONE_CELP68: u32 = 321u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIGIADPCM: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIGIFIX: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIGIREAL: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIGISTD: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIGITAL_G723: u32 = 291u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIVIO_G726: u32 = 16963u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: u32 = 16707u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DOLBY_AC2: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: u32 = 146u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DOLBY_AC4: u32 = 44096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DRM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DSAT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DSAT_DISPLAY: u32 = 103u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DTS2: u32 = 8193u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DTS_DS: u32 = 400u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DVI_ADPCM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_DVM: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ECHOSC1: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ECHOSC3: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ENCORE_G726: u32 = 41223u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ESPCM: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ESST_AC3: u32 = 577u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_FAAD_AAC: u32 = 28781u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_FLAC: u32 = 61868u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_FM_TOWNS_SND: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_FRACE_TELECOM_G729: u32 = 41251u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: u32 = 384u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G721_ADPCM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G722_ADPCM: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G723_ADPCM: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G726ADPCM: u32 = 320u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G726_ADPCM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G728_CELP: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_G729A: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GENERIC_PASSTHRU: u32 = 585u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: u32 = 41238u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM610: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_610: u32 = 41229u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_620: u32 = 41230u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_660: u32 = 41231u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_690: u32 = 41232u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: u32 = 41233u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_AMR_CBR: u32 = 31265u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: u32 = 31266u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_HP_DYN_VOICE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_IBM_CVSD: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_IEEE_FLOAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ILINK_VC: u32 = 560u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_IMA_ADPCM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INDEO_AUDIO: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: u32 = 139u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INGENIENT_G726: u32 = 41221u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: u32 = 6521u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INTEL_G723_1: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INTEL_G729: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_IPI_HSX: u32 = 592u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_IPI_RPELP: u32 = 593u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_IRAT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ISIAUDIO: u32 = 136u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ISIAUDIO_2: u32 = 5121u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: u32 = 376u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LEAD_SPEECH: u32 = 17228u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LEAD_VORBIS: u32 = 22092u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LH_CODEC: u32 = 4352u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LH_CODEC_CELP: u32 = 4353u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LH_CODEC_SBC12: u32 = 4355u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LH_CODEC_SBC16: u32 = 4356u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LH_CODEC_SBC8: u32 = 4354u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: u32 = 2222u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LRC: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LUCENT_G723: u32 = 89u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LUCENT_SX5363S: u32 = 7180u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_LUCENT_SX8300P: u32 = 7175u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MAKEAVIS: u32 = 13075u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MALDEN_PHONYTALK: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MEDIASONIC_G723: u32 = 147u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MICRONAS: u32 = 848u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MICRONAS_CELP833: u32 = 849u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEG: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEG4_AAC: u32 = 41222u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEGLAYER3: u32 = 85u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEG_ADTS_AAC: u32 = 5632u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEG_HEAAC: u32 = 5648u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEG_LOAS: u32 = 5634u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MPEG_RAW_AAC: u32 = 5633u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MSAUDIO1: u32 = 352u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MSG723: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MSNAUDIO: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MSRT24: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MULAW: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: u32 = 138u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_MVI_MVI2: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NEC_AAC: u32 = 176u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NICE_ACA: u32 = 41240u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NICE_ADPCM: u32 = 41241u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NICE_G728: u32 = 41250u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NMS_VBXADPCM: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: u32 = 16897u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: u32 = 5640u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: u32 = 5641u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: u32 = 645u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NORRIS: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: u32 = 8132u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: u32 = 26447u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: u32 = 26479u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: u32 = 26448u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: u32 = 26480u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: u32 = 26449u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: u32 = 26481u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OKI_ADPCM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OLIADPCM: u32 = 4097u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OLICELP: u32 = 4098u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OLIGSM: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OLIOPR: u32 = 4100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OLISBC: u32 = 4099u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ON2_VP6_AUDIO: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ON2_VP7_AUDIO: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ONLIVE: u32 = 137u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_OPUS: u32 = 28751u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PAC: u32 = 83u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PACKED: u32 = 153u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PCM_S: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PHILIPS_CELP: u32 = 288u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: u32 = 289u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PHILIPS_LPCBB: u32 = 152u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_POLYCOM_G722: u32 = 41234u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_POLYCOM_G728: u32 = 41235u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_POLYCOM_G729_A: u32 = 41236u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_POLYCOM_SIREN: u32 = 41237u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PROSODY_1612: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_PROSODY_8KBPS: u32 = 148u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_QDESIGN_MUSIC: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: u32 = 337u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: u32 = 336u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_QUARTERDECK: u32 = 544u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: u32 = 162u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: u32 = 163u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: u32 = 164u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: u32 = 41239u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RAW_AAC1: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RAW_SPORT: u32 = 576u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RHETOREX_ADPCM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ROCKWELL_ADPCM: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_RT24: u32 = 82u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SANYO_LD_ADPCM: u32 = 293u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SBC24: u32 = 145u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SHARP_G726: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIERRA_ADPCM: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: u32 = 305u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: u32 = 306u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: u32 = 304u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIPROLAB_G729: u32 = 307u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIPROLAB_G729A: u32 = 308u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SIPROLAB_KELVIN: u32 = 309u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SOFTSOUND: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SONARC: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: u32 = 6513u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SONY_ATRAC3: u32 = 626u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SONY_SCX: u32 = 624u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SONY_SCY: u32 = 625u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SONY_SPC: u32 = 627u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: u32 = 5376u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SPEEX_VOICE: u32 = 41225u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: u32 = 372u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: u32 = 374u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: u32 = 375u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: u32 = 373u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_SYMBOL_G729_A: u32 = 41219u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_TELUM_AUDIO: u32 = 640u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_TELUM_IA_AUDIO: u32 = 641u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_TPC: u32 = 1665u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_TUBGSM: u32 = 341u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_UHER_ADPCM: u32 = 528u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: u32 = 533u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: u32 = 534u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_UNISYS_NAP_16K: u32 = 371u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: u32 = 368u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: u32 = 370u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: u32 = 369u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VIANIX_MASC: u32 = 41226u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VIVO_G723: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VIVO_SIREN: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VME_VMPCM: u32 = 1664u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G721: u32 = 41242u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G722_1: u32 = 41244u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G723_1: u32 = 41248u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G726: u32 = 41243u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G728: u32 = 41245u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G729: u32 = 41246u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_G729_A: u32 = 41247u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOCORD_LBC: u32 = 41249u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: u32 = 5642u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: u32 = 5643u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOICEAGE_AMR: u32 = 310u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: u32 = 41220u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE: u32 = 98u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_AC10: u32 = 113u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_AC16: u32 = 114u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_AC20: u32 = 115u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_AC8: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: u32 = 105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_RT24: u32 = 116u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: u32 = 6172u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_RT29: u32 = 117u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_RT29HW: u32 = 118u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_SC3: u32 = 122u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_SC3_1: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_TQ40: u32 = 121u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_TQ60: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_VR12: u32 = 119u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VOXWARE_VR18: u32 = 120u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_VSELP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WAVPACK_AUDIO: u32 = 22358u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: u32 = 41227u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMASPDIF: u32 = 356u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMAUDIO2: u32 = 353u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMAUDIO3: u32 = 354u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: u32 = 355u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMAVOICE10: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMAVOICE9: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: u32 = 41228u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_XEBEC: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_YAMAHA_ADPCM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ZOLL_ASAO: u32 = 41224u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_FORMAT_ZYXEL_ADPCM: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WAVE_MAPPER_S: u32 = 1153u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_ADDBUFFER: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_CLOSE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_GETDEVCAPS: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_GETNUMDEVS: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_GETPOS: u32 = 60u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_OPEN: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_PREFERRED: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_PREPARE: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_RESET: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_START: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_STOP: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WIDM_UNPREPARE: u32 = 55u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct WMAUDIO2WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub dwSamplesPerBlock: u32,
    pub wEncodeOptions: u16,
    pub dwSuperBlockAlign: u32,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for WMAUDIO2WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for WMAUDIO2WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for WMAUDIO2WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for WMAUDIO2WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMAUDIO2WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for WMAUDIO2WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for WMAUDIO2WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WMAUDIO2_BITS_PER_SAMPLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WMAUDIO2_MAX_CHANNELS: u32 = 2u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct WMAUDIO3WAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub wEncodeOptions: u16,
    pub wReserved3: u16,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for WMAUDIO3WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for WMAUDIO3WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for WMAUDIO3WAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for WMAUDIO3WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMAUDIO3WAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for WMAUDIO3WAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for WMAUDIO3WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WMAUDIO_BITS_PER_SAMPLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WMAUDIO_MAX_CHANNELS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_ABORT: u32 = 1093u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DLG_VIDEOCOMPRESSION: u32 = 1070u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DLG_VIDEODISPLAY: u32 = 1067u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DLG_VIDEOFORMAT: u32 = 1065u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DLG_VIDEOSOURCE: u32 = 1066u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_CONNECT: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_DISCONNECT: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_CAPS: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_NAME: u32 = 1136u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_NAMEA: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_NAMEW: u32 = 1136u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_VERSION: u32 = 1137u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_VERSIONA: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_DRIVER_GET_VERSIONW: u32 = 1137u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_EDIT_COPY: u32 = 1054u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_END: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_ALLOCATE: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_GET_CAPTURE_FILE: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_GET_CAPTURE_FILEA: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_GET_CAPTURE_FILEW: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SAVEAS: u32 = 1147u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SAVEASA: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SAVEASW: u32 = 1147u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SAVEDIB: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SAVEDIBA: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SAVEDIBW: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SET_CAPTURE_FILE: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SET_CAPTURE_FILEA: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SET_CAPTURE_FILEW: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_FILE_SET_INFOCHUNK: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_AUDIOFORMAT: u32 = 1060u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_CAPSTREAMPTR: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_MCI_DEVICE: u32 = 1191u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_MCI_DEVICEA: u32 = 1091u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_MCI_DEVICEW: u32 = 1191u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_SEQUENCE_SETUP: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_STATUS: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_USER_DATA: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GET_VIDEOFORMAT: u32 = 1068u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GRAB_FRAME: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_GRAB_FRAME_NOSTOP: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_AUTOCREATE: u32 = 1107u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_MANUALCREATE: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_OPEN: u32 = 1204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_OPENA: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_OPENW: u32 = 1204u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_PASTE: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_SAVE: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_SAVEA: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_PAL_SAVEW: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SEQUENCE: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SEQUENCE_NOFILE: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_AUDIOFORMAT: u32 = 1059u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_CAPCONTROL: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_ERROR: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_ERRORA: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_ERRORW: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_FRAME: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_STATUS: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_STATUSA: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_STATUSW: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_VIDEOSTREAM: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_WAVESTREAM: u32 = 1031u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_CALLBACK_YIELD: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_MCI_DEVICE: u32 = 1190u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_MCI_DEVICEA: u32 = 1090u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_MCI_DEVICEW: u32 = 1190u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_OVERLAY: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_PREVIEW: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_PREVIEWRATE: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_SCALE: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_SCROLL: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_SEQUENCE_SETUP: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_USER_DATA: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SET_VIDEOFORMAT: u32 = 1069u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SINGLE_FRAME: u32 = 1096u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SINGLE_FRAME_CLOSE: u32 = 1095u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_SINGLE_FRAME_OPEN: u32 = 1094u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_START: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_STOP: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_UNICODE_END: u32 = 1205u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WM_CAP_UNICODE_START: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_BREAKLOOP: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_BUSY: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_CLOSE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_GETDEVCAPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_GETNUMDEVS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_GETPITCH: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_GETPLAYBACKRATE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_GETPOS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_GETVOLUME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_INIT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_INIT_EX: u32 = 104u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_OPEN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_PAUSE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_PREFERRED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_PREPARE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_RESET: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_RESTART: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_SETPITCH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_SETPLAYBACKRATE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_SETVOLUME: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_UNPREPARE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub const WODM_WRITE: u32 = 9u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Media_Audio\"`*"]
#[cfg(feature = "Win32_Media_Audio")]
pub struct YAMAHA_ADPCMWAVEFORMAT {
    pub wfx: super::Audio::WAVEFORMATEX,
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::marker::Copy for YAMAHA_ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::clone::Clone for YAMAHA_ADPCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Media_Audio")]
unsafe impl ::windows::core::Abi for YAMAHA_ADPCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::PartialEq for YAMAHA_ADPCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<YAMAHA_ADPCMWAVEFORMAT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::cmp::Eq for YAMAHA_ADPCMWAVEFORMAT {}
#[cfg(feature = "Win32_Media_Audio")]
impl ::core::default::Default for YAMAHA_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
pub type YIELDPROC = ::core::option::Option<unsafe extern "system" fn(mciid: u32, dwyielddata: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capCreateCaptureWindowA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszwindowname: Param0, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Param6, nid: i32) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capCreateCaptureWindowA(lpszwindowname: ::windows::core::PCSTR, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, nid: i32) -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(capCreateCaptureWindowA(lpszwindowname.into_param().abi(), ::core::mem::transmute(dwstyle), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(nwidth), ::core::mem::transmute(nheight), hwndparent.into_param().abi(), ::core::mem::transmute(nid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capCreateCaptureWindowW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszwindowname: Param0, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: Param6, nid: i32) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capCreateCaptureWindowW(lpszwindowname: ::windows::core::PCWSTR, dwstyle: u32, x: i32, y: i32, nwidth: i32, nheight: i32, hwndparent: super::super::Foundation::HWND, nid: i32) -> super::super::Foundation::HWND;
        }
        ::core::mem::transmute(capCreateCaptureWindowW(lpszwindowname.into_param().abi(), ::core::mem::transmute(dwstyle), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(nwidth), ::core::mem::transmute(nheight), hwndparent.into_param().abi(), ::core::mem::transmute(nid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capGetDriverDescriptionA(wdriverindex: u32, lpszname: &mut [u8], lpszver: &mut [u8]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capGetDriverDescriptionA(wdriverindex: u32, lpszname: ::windows::core::PSTR, cbname: i32, lpszver: ::windows::core::PSTR, cbver: i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(capGetDriverDescriptionA(::core::mem::transmute(wdriverindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszname)), lpszname.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszver)), lpszver.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capGetDriverDescriptionW(wdriverindex: u32, lpszname: &mut [u16], lpszver: &mut [u16]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capGetDriverDescriptionW(wdriverindex: u32, lpszname: ::windows::core::PWSTR, cbname: i32, lpszver: ::windows::core::PWSTR, cbver: i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(capGetDriverDescriptionW(::core::mem::transmute(wdriverindex), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszname)), lpszname.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszver)), lpszver.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn joyGetDevCapsA(ujoyid: usize, pjc: *mut JOYCAPSA, cbjc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetDevCapsA(ujoyid: usize, pjc: *mut JOYCAPSA, cbjc: u32) -> u32;
        }
        ::core::mem::transmute(joyGetDevCapsA(::core::mem::transmute(ujoyid), ::core::mem::transmute(pjc), ::core::mem::transmute(cbjc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joyGetDevCapsW(ujoyid: usize, pjc: *mut JOYCAPSW, cbjc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetDevCapsW(ujoyid: usize, pjc: *mut JOYCAPSW, cbjc: u32) -> u32;
        }
        ::core::mem::transmute(joyGetDevCapsW(::core::mem::transmute(ujoyid), ::core::mem::transmute(pjc), ::core::mem::transmute(cbjc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joyGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetNumDevs() -> u32;
        }
        ::core::mem::transmute(joyGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joyGetPos(ujoyid: u32, pji: *mut JOYINFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetPos(ujoyid: u32, pji: *mut JOYINFO) -> u32;
        }
        ::core::mem::transmute(joyGetPos(::core::mem::transmute(ujoyid), ::core::mem::transmute(pji)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joyGetPosEx(ujoyid: u32, pji: *mut JOYINFOEX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetPosEx(ujoyid: u32, pji: *mut JOYINFOEX) -> u32;
        }
        ::core::mem::transmute(joyGetPosEx(::core::mem::transmute(ujoyid), ::core::mem::transmute(pji)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joyGetThreshold(ujoyid: u32, puthreshold: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetThreshold(ujoyid: u32, puthreshold: *mut u32) -> u32;
        }
        ::core::mem::transmute(joyGetThreshold(::core::mem::transmute(ujoyid), ::core::mem::transmute(puthreshold)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joyReleaseCapture(ujoyid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyReleaseCapture(ujoyid: u32) -> u32;
        }
        ::core::mem::transmute(joyReleaseCapture(::core::mem::transmute(ujoyid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn joySetCapture<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, ujoyid: u32, uperiod: u32, fchanged: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joySetCapture(hwnd: super::super::Foundation::HWND, ujoyid: u32, uperiod: u32, fchanged: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(joySetCapture(hwnd.into_param().abi(), ::core::mem::transmute(ujoyid), ::core::mem::transmute(uperiod), fchanged.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn joySetThreshold(ujoyid: u32, uthreshold: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joySetThreshold(ujoyid: u32, uthreshold: u32) -> u32;
        }
        ::core::mem::transmute(joySetThreshold(::core::mem::transmute(ujoyid), ::core::mem::transmute(uthreshold)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciDriverNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hwndcallback: Param0, wdeviceid: u32, ustatus: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciDriverNotify(hwndcallback: super::super::Foundation::HANDLE, wdeviceid: u32, ustatus: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mciDriverNotify(hwndcallback.into_param().abi(), ::core::mem::transmute(wdeviceid), ::core::mem::transmute(ustatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciDriverYield(wdeviceid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciDriverYield(wdeviceid: u32) -> u32;
        }
        ::core::mem::transmute(mciDriverYield(::core::mem::transmute(wdeviceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciFreeCommandResource(wtable: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciFreeCommandResource(wtable: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mciFreeCommandResource(::core::mem::transmute(wtable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetCreatorTask(mciid: u32) -> super::HTASK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetCreatorTask(mciid: u32) -> super::HTASK;
        }
        ::core::mem::transmute(mciGetCreatorTask(::core::mem::transmute(mciid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetDeviceIDA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszdevice: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetDeviceIDA(pszdevice: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(mciGetDeviceIDA(pszdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetDeviceIDFromElementIDA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwelementid: u32, lpstrtype: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetDeviceIDFromElementIDA(dwelementid: u32, lpstrtype: ::windows::core::PCSTR) -> u32;
        }
        ::core::mem::transmute(mciGetDeviceIDFromElementIDA(::core::mem::transmute(dwelementid), lpstrtype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetDeviceIDFromElementIDW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwelementid: u32, lpstrtype: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetDeviceIDFromElementIDW(dwelementid: u32, lpstrtype: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(mciGetDeviceIDFromElementIDW(::core::mem::transmute(dwelementid), lpstrtype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetDeviceIDW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszdevice: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetDeviceIDW(pszdevice: ::windows::core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(mciGetDeviceIDW(pszdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetDriverData(wdeviceid: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetDriverData(wdeviceid: u32) -> usize;
        }
        ::core::mem::transmute(mciGetDriverData(::core::mem::transmute(wdeviceid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciGetErrorStringA(mcierr: u32, psztext: &mut [u8]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetErrorStringA(mcierr: u32, psztext: ::windows::core::PSTR, cchtext: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mciGetErrorStringA(::core::mem::transmute(mcierr), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciGetErrorStringW(mcierr: u32, psztext: &mut [u16]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetErrorStringW(mcierr: u32, psztext: ::windows::core::PWSTR, cchtext: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mciGetErrorStringW(::core::mem::transmute(mcierr), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciGetYieldProc(mciid: u32, pdwyielddata: *const u32) -> YIELDPROC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciGetYieldProc(mciid: u32, pdwyielddata: *const u32) -> YIELDPROC;
        }
        ::core::mem::transmute(mciGetYieldProc(::core::mem::transmute(mciid), ::core::mem::transmute(pdwyielddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciLoadCommandResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hinstance: Param0, lpresname: Param1, wtype: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciLoadCommandResource(hinstance: super::super::Foundation::HANDLE, lpresname: ::windows::core::PCWSTR, wtype: u32) -> u32;
        }
        ::core::mem::transmute(mciLoadCommandResource(hinstance.into_param().abi(), lpresname.into_param().abi(), ::core::mem::transmute(wtype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciSendCommandA(mciid: u32, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciSendCommandA(mciid: u32, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
        }
        ::core::mem::transmute(mciSendCommandA(::core::mem::transmute(mciid), ::core::mem::transmute(umsg), ::core::mem::transmute(dwparam1), ::core::mem::transmute(dwparam2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mciSendCommandW(mciid: u32, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciSendCommandW(mciid: u32, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
        }
        ::core::mem::transmute(mciSendCommandW(::core::mem::transmute(mciid), ::core::mem::transmute(umsg), ::core::mem::transmute(dwparam1), ::core::mem::transmute(dwparam2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciSendStringA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpstrcommand: Param0, lpstrreturnstring: &mut [u8], hwndcallback: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciSendStringA(lpstrcommand: ::windows::core::PCSTR, lpstrreturnstring: ::windows::core::PSTR, ureturnlength: u32, hwndcallback: super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(mciSendStringA(lpstrcommand.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstrreturnstring)), lpstrreturnstring.len() as _, hwndcallback.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciSendStringW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpstrcommand: Param0, lpstrreturnstring: &mut [u16], hwndcallback: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciSendStringW(lpstrcommand: ::windows::core::PCWSTR, lpstrreturnstring: ::windows::core::PWSTR, ureturnlength: u32, hwndcallback: super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(mciSendStringW(lpstrcommand.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpstrreturnstring)), lpstrreturnstring.len() as _, hwndcallback.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciSetDriverData(wdeviceid: u32, dwdata: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciSetDriverData(wdeviceid: u32, dwdata: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mciSetDriverData(::core::mem::transmute(wdeviceid), ::core::mem::transmute(dwdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mciSetYieldProc(mciid: u32, fpyieldproc: YIELDPROC, dwyielddata: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mciSetYieldProc(mciid: u32, fpyieldproc: ::windows::core::RawPtr, dwyielddata: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mciSetYieldProc(::core::mem::transmute(mciid), ::core::mem::transmute(fpyieldproc), ::core::mem::transmute(dwyielddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmDrvInstall<'a, Param0: ::windows::core::IntoParam<'a, HDRVR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hdriver: Param0, wszdrventry: Param1, drvmessage: DRIVERMSGPROC, wflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmDrvInstall(hdriver: HDRVR, wszdrventry: ::windows::core::PCWSTR, drvmessage: ::windows::core::RawPtr, wflags: u32) -> u32;
        }
        ::core::mem::transmute(mmDrvInstall(hdriver.into_param().abi(), wszdrventry.into_param().abi(), ::core::mem::transmute(drvmessage), ::core::mem::transmute(wflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmGetCurrentTask() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmGetCurrentTask() -> u32;
        }
        ::core::mem::transmute(mmGetCurrentTask())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmTaskBlock(h: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskBlock(h: u32);
        }
        mmTaskBlock(::core::mem::transmute(h))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmTaskCreate(lpfn: LPTASKCALLBACK, lph: *mut super::super::Foundation::HANDLE, dwinst: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskCreate(lpfn: ::windows::core::RawPtr, lph: *mut super::super::Foundation::HANDLE, dwinst: usize) -> u32;
        }
        ::core::mem::transmute(mmTaskCreate(::core::mem::transmute(lpfn), ::core::mem::transmute(lph), ::core::mem::transmute(dwinst)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmTaskSignal(h: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskSignal(h: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(mmTaskSignal(::core::mem::transmute(h)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmTaskYield() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskYield();
        }
        mmTaskYield()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioAdvance<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pmmioinfo: *const MMIOINFO, fuadvance: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioAdvance(hmmio: HMMIO, pmmioinfo: *const MMIOINFO, fuadvance: u32) -> u32;
        }
        ::core::mem::transmute(mmioAdvance(hmmio.into_param().abi(), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fuadvance)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioAscend<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pmmcki: *const MMCKINFO, fuascend: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioAscend(hmmio: HMMIO, pmmcki: *const MMCKINFO, fuascend: u32) -> u32;
        }
        ::core::mem::transmute(mmioAscend(hmmio.into_param().abi(), ::core::mem::transmute(pmmcki), ::core::mem::transmute(fuascend)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioClose<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, fuclose: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioClose(hmmio: HMMIO, fuclose: u32) -> u32;
        }
        ::core::mem::transmute(mmioClose(hmmio.into_param().abi(), ::core::mem::transmute(fuclose)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioCreateChunk<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pmmcki: *const MMCKINFO, fucreate: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioCreateChunk(hmmio: HMMIO, pmmcki: *const MMCKINFO, fucreate: u32) -> u32;
        }
        ::core::mem::transmute(mmioCreateChunk(hmmio.into_param().abi(), ::core::mem::transmute(pmmcki), ::core::mem::transmute(fucreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioDescend<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pmmcki: *mut MMCKINFO, pmmckiparent: *const MMCKINFO, fudescend: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioDescend(hmmio: HMMIO, pmmcki: *mut MMCKINFO, pmmckiparent: *const MMCKINFO, fudescend: u32) -> u32;
        }
        ::core::mem::transmute(mmioDescend(hmmio.into_param().abi(), ::core::mem::transmute(pmmcki), ::core::mem::transmute(pmmckiparent), ::core::mem::transmute(fudescend)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioFlush<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, fuflush: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioFlush(hmmio: HMMIO, fuflush: u32) -> u32;
        }
        ::core::mem::transmute(mmioFlush(hmmio.into_param().abi(), ::core::mem::transmute(fuflush)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioGetInfo<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pmmioinfo: *mut MMIOINFO, fuinfo: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioGetInfo(hmmio: HMMIO, pmmioinfo: *mut MMIOINFO, fuinfo: u32) -> u32;
        }
        ::core::mem::transmute(mmioGetInfo(hmmio.into_param().abi(), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fuinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioInstallIOProcA(fccioproc: u32, pioproc: LPMMIOPROC, dwflags: u32) -> LPMMIOPROC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioInstallIOProcA(fccioproc: u32, pioproc: ::windows::core::RawPtr, dwflags: u32) -> LPMMIOPROC;
        }
        ::core::mem::transmute(mmioInstallIOProcA(::core::mem::transmute(fccioproc), ::core::mem::transmute(pioproc), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioInstallIOProcW(fccioproc: u32, pioproc: LPMMIOPROC, dwflags: u32) -> LPMMIOPROC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioInstallIOProcW(fccioproc: u32, pioproc: ::windows::core::RawPtr, dwflags: u32) -> LPMMIOPROC;
        }
        ::core::mem::transmute(mmioInstallIOProcW(::core::mem::transmute(fccioproc), ::core::mem::transmute(pioproc), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioOpenA(pszfilename: &mut [u8; 128], pmmioinfo: *mut MMIOINFO, fdwopen: u32) -> HMMIO {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioOpenA(pszfilename: ::windows::core::PSTR, pmmioinfo: *mut MMIOINFO, fdwopen: u32) -> HMMIO;
        }
        ::core::mem::transmute(mmioOpenA(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszfilename)), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioOpenW(pszfilename: &mut [u16; 128], pmmioinfo: *mut MMIOINFO, fdwopen: u32) -> HMMIO {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioOpenW(pszfilename: ::windows::core::PWSTR, pmmioinfo: *mut MMIOINFO, fdwopen: u32) -> HMMIO;
        }
        ::core::mem::transmute(mmioOpenW(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszfilename)), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioRead<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pch: *mut i8, cch: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioRead(hmmio: HMMIO, pch: *mut i8, cch: i32) -> i32;
        }
        ::core::mem::transmute(mmioRead(hmmio.into_param().abi(), ::core::mem::transmute(pch), ::core::mem::transmute(cch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioRenameA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszfilename: Param0, psznewfilename: Param1, pmmioinfo: *const MMIOINFO, fdwrename: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioRenameA(pszfilename: ::windows::core::PCSTR, psznewfilename: ::windows::core::PCSTR, pmmioinfo: *const MMIOINFO, fdwrename: u32) -> u32;
        }
        ::core::mem::transmute(mmioRenameA(pszfilename.into_param().abi(), psznewfilename.into_param().abi(), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fdwrename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioRenameW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszfilename: Param0, psznewfilename: Param1, pmmioinfo: *const MMIOINFO, fdwrename: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioRenameW(pszfilename: ::windows::core::PCWSTR, psznewfilename: ::windows::core::PCWSTR, pmmioinfo: *const MMIOINFO, fdwrename: u32) -> u32;
        }
        ::core::mem::transmute(mmioRenameW(pszfilename.into_param().abi(), psznewfilename.into_param().abi(), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fdwrename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioSeek<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, loffset: i32, iorigin: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSeek(hmmio: HMMIO, loffset: i32, iorigin: i32) -> i32;
        }
        ::core::mem::transmute(mmioSeek(hmmio.into_param().abi(), ::core::mem::transmute(loffset), ::core::mem::transmute(iorigin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioSendMessage<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(hmmio: Param0, umsg: u32, lparam1: Param2, lparam2: Param3) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSendMessage(hmmio: HMMIO, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(mmioSendMessage(hmmio.into_param().abi(), ::core::mem::transmute(umsg), lparam1.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioSetBuffer<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pchbuffer: &mut [u8], fubuffer: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSetBuffer(hmmio: HMMIO, pchbuffer: ::windows::core::PSTR, cchbuffer: i32, fubuffer: u32) -> u32;
        }
        ::core::mem::transmute(mmioSetBuffer(hmmio.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pchbuffer)), pchbuffer.len() as _, ::core::mem::transmute(fubuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioSetInfo<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>>(hmmio: Param0, pmmioinfo: *const MMIOINFO, fuinfo: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSetInfo(hmmio: HMMIO, pmmioinfo: *const MMIOINFO, fuinfo: u32) -> u32;
        }
        ::core::mem::transmute(mmioSetInfo(hmmio.into_param().abi(), ::core::mem::transmute(pmmioinfo), ::core::mem::transmute(fuinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioStringToFOURCCA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sz: Param0, uflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioStringToFOURCCA(sz: ::windows::core::PCSTR, uflags: u32) -> u32;
        }
        ::core::mem::transmute(mmioStringToFOURCCA(sz.into_param().abi(), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioStringToFOURCCW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sz: Param0, uflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioStringToFOURCCW(sz: ::windows::core::PCWSTR, uflags: u32) -> u32;
        }
        ::core::mem::transmute(mmioStringToFOURCCW(sz.into_param().abi(), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`*"]
#[inline]
pub unsafe fn mmioWrite<'a, Param0: ::windows::core::IntoParam<'a, HMMIO>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hmmio: Param0, pch: Param1, cch: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioWrite(hmmio: HMMIO, pch: ::windows::core::PCSTR, cch: i32) -> i32;
        }
        ::core::mem::transmute(mmioWrite(hmmio.into_param().abi(), pch.into_param().abi(), ::core::mem::transmute(cch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct s_RIFFWAVE_inst {
    pub bUnshiftedNote: u8,
    pub chFineTune: super::super::Foundation::CHAR,
    pub chGain: super::super::Foundation::CHAR,
    pub bLowNote: u8,
    pub bHighNote: u8,
    pub bLowVelocity: u8,
    pub bHighVelocity: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for s_RIFFWAVE_inst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for s_RIFFWAVE_inst {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for s_RIFFWAVE_inst {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("s_RIFFWAVE_inst").field("bUnshiftedNote", &self.bUnshiftedNote).field("chFineTune", &self.chFineTune).field("chGain", &self.chGain).field("bLowNote", &self.bLowNote).field("bHighNote", &self.bHighNote).field("bLowVelocity", &self.bLowVelocity).field("bHighVelocity", &self.bHighVelocity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for s_RIFFWAVE_inst {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for s_RIFFWAVE_inst {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<s_RIFFWAVE_inst>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for s_RIFFWAVE_inst {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for s_RIFFWAVE_inst {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Multimedia\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndOpenSound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(eventname: Param0, appname: Param1, flags: i32, filehandle: *mut super::super::Foundation::HANDLE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndOpenSound(eventname: ::windows::core::PCWSTR, appname: ::windows::core::PCWSTR, flags: i32, filehandle: *mut super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(sndOpenSound(eventname.into_param().abi(), appname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(filehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
