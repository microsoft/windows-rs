pub const STRSAFE_E_END_OF_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80070026_u32 as _);
pub const STRSAFE_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x8007007A_u32 as _);
pub const STRSAFE_E_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const STRSAFE_FILL_BEHIND_NULL: u32 = 512;
pub const STRSAFE_FILL_ON_FAILURE: u32 = 1024;
pub const STRSAFE_IGNORE_NULLS: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_LPCSTR(pub *const i8);
impl STRSAFE_LPCSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_LPCSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_LPCUWSTR(pub *const u16);
impl STRSAFE_LPCUWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_LPCUWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_LPCWSTR(pub *const u16);
impl STRSAFE_LPCWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_LPCWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_LPSTR(pub *mut i8);
impl STRSAFE_LPSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_LPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_LPWSTR(pub *mut u16);
impl STRSAFE_LPWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_LPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STRSAFE_MAX_CCH: u32 = 2147483647;
pub const STRSAFE_MAX_LENGTH: u32 = 2147483646;
pub const STRSAFE_NO_TRUNCATION: u32 = 4096;
pub const STRSAFE_NULL_ON_FAILURE: u32 = 2048;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_PCNZCH(pub *const i8);
impl STRSAFE_PCNZCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_PCNZCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_PCNZWCH(pub *const u16);
impl STRSAFE_PCNZWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_PCNZWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct STRSAFE_PCUNZWCH(pub *const u16);
impl STRSAFE_PCUNZWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for STRSAFE_PCUNZWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STRSAFE_USE_SECURE_CRT: u32 = 0;
pub const STRSAFE_VALID_FLAGS: u32 = 8191;
