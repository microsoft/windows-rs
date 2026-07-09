#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCGUID(pub *const windows_core::GUID);
impl LPCGUID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCGUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCLSID(pub *mut windows_core::GUID);
impl LPCLSID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCLSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFMTID(pub *mut windows_core::GUID);
impl LPFMTID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPFMTID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPGUID(pub *mut windows_core::GUID);
impl LPGUID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPGUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPIID(pub *mut windows_core::GUID);
impl LPIID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPIID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
