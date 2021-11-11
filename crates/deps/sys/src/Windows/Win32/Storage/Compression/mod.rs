#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseCompressor();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDecompressor();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Compress();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCompressor();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDecompressor();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Decompress();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryCompressorInformation();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryDecompressorInformation();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetCompressor();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDecompressor();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCompressorInformation();
    #[doc = "*Required features: `Win32_Storage_Compression`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDecompressorInformation();
}
