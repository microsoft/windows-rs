#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlDrainNonVolatileFlush();
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlFillNonVolatileMemory();
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlFlushNonVolatileMemory();
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlFlushNonVolatileMemoryRanges();
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlFreeNonVolatileToken();
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlGetNonVolatileToken();
    #[doc = "*Required features: `Win32_System_Memory_NonVolatile`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlWriteNonVolatileMemory();
}
