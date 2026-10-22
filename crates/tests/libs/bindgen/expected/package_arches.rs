pub mod Windows {
    pub mod Win32 {
        pub mod combined {
            #[cfg(target_arch = "aarch64")]
            pub const ARM64: u32 = 4;
            #[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
            pub const X64: u32 = 2;
            #[cfg(target_arch = "x86")]
            pub const X86: u32 = 1;
        }
        pub mod complete {
            #[inline]
            pub unsafe fn GetWindowLongPtrW() -> CompleteValue {
                windows_core::link!("user32.dll" "system" fn GetWindowLongPtrW() -> CompleteValue);
                unsafe { GetWindowLongPtrW() }
            }
            #[cfg(target_arch = "x86")]
            pub type CompleteValue = i32;
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            pub type CompleteValue = isize;
        }
        pub mod constants {
            #[cfg(target_arch = "aarch64")]
            pub const ARM: u32 = 3;
        }
        pub mod legacy {
            #[cfg(target_arch = "x86")]
            #[inline]
            pub unsafe fn GetWindowLongA() -> i32 {
                windows_core::link!("user32.dll" "system" fn GetWindowLongA() -> i32);
                unsafe { GetWindowLongA() }
            }
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            #[inline]
            pub unsafe fn GetWindowLongPtrA() -> isize {
                windows_core::link!("user32.dll" "system" fn GetWindowLongPtrA() -> isize);
                unsafe { GetWindowLongPtrA() }
            }
            #[cfg(target_pointer_width = "32")]
            pub use GetWindowLongA as GetWindowLongPtrA;
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            pub const LEGACY: u32 = 1;
        }
        pub mod narrow {
            #[repr(C)]
            #[cfg(target_arch = "x86")]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct Narrow {
                pub value: u32,
            }
        }
        pub mod native {
            #[cfg(target_arch = "x86")]
            #[inline]
            pub unsafe fn GetWindowLongPtrA() -> i32 {
                windows_core::link!("user32.dll" "system" "GetWindowLongA" fn GetWindowLongPtrA() -> i32);
                unsafe { GetWindowLongPtrA() }
            }
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            #[inline]
            pub unsafe fn GetWindowLongPtrA() -> isize {
                windows_core::link!("user32.dll" "system" fn GetWindowLongPtrA() -> isize);
                unsafe { GetWindowLongPtrA() }
            }
            #[cfg(target_arch = "x86")]
            #[inline]
            pub unsafe fn GetWindowLongPtrW() -> i32 {
                windows_core::link!("user32.dll" "system" "GetWindowLongW" fn GetWindowLongPtrW() -> i32);
                unsafe { GetWindowLongPtrW() }
            }
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            #[inline]
            pub unsafe fn GetWindowLongPtrW() -> isize {
                windows_core::link!("user32.dll" "system" fn GetWindowLongPtrW() -> isize);
                unsafe { GetWindowLongPtrW() }
            }
            #[cfg(target_arch = "x86")]
            #[inline]
            pub unsafe fn SetWindowLongPtrA() -> i32 {
                windows_core::link!("user32.dll" "system" "SetWindowLongA" fn SetWindowLongPtrA() -> i32);
                unsafe { SetWindowLongPtrA() }
            }
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            #[inline]
            pub unsafe fn SetWindowLongPtrA() -> isize {
                windows_core::link!("user32.dll" "system" fn SetWindowLongPtrA() -> isize);
                unsafe { SetWindowLongPtrA() }
            }
            #[cfg(target_arch = "x86")]
            #[inline]
            pub unsafe fn SetWindowLongPtrW() -> i32 {
                windows_core::link!("user32.dll" "system" "SetWindowLongW" fn SetWindowLongPtrW() -> i32);
                unsafe { SetWindowLongPtrW() }
            }
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            #[inline]
            pub unsafe fn SetWindowLongPtrW() -> isize {
                windows_core::link!("user32.dll" "system" fn SetWindowLongPtrW() -> isize);
                unsafe { SetWindowLongPtrW() }
            }
        }
        pub mod split_legacy {
            #[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
            #[inline]
            pub unsafe fn SetWindowLongPtrW() -> isize {
                windows_core::link!("user32.dll" "system" fn SetWindowLongPtrW() -> isize);
                unsafe { SetWindowLongPtrW() }
            }
            #[cfg(target_pointer_width = "32")]
            pub use SetWindowLongW as SetWindowLongPtrW;
            #[cfg(target_arch = "aarch64")]
            #[inline]
            pub unsafe fn SetWindowLongPtrW() -> isize {
                windows_core::link!("user32.dll" "system" fn SetWindowLongPtrW() -> isize);
                unsafe { SetWindowLongPtrW() }
            }
            #[cfg(target_arch = "x86")]
            #[inline]
            pub unsafe fn SetWindowLongW() -> i32 {
                windows_core::link!("user32.dll" "system" fn SetWindowLongW() -> i32);
                unsafe { SetWindowLongW() }
            }
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            pub const SPLIT: u32 = 2;
        }
        pub mod wide {
            #[repr(C)]
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct Wide {
                pub value: u64,
            }
        }
    }
}
