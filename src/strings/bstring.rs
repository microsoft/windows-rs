use crate::*;

#[repr(C)]
pub struct BString(RawPtr);

impl BString {
    pub fn new() -> BString {
        Self(std::ptr::null_mut())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { SysStringLen(self.0) as usize }
    }

    pub fn clear(&mut self) {
        unsafe {
            SysFreeString(self.0);
        }

        self.0 = std::ptr::null_mut();
    }
}

unsafe impl Abi for BString {
    type Abi = RawPtr;

    unsafe fn set_abi(&mut self) -> *mut RawPtr {
        debug_assert!(self.is_empty());
        &mut self.0 as *mut _ as _
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        self.clear();
    }
}

impl From<&BString> for String {
    fn from(from: &BString) -> Self {
        if from.is_empty() {
            return String::new();
        }

        unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(from.0 as *const u16, from.len()))
        }
    }
}

impl From<BString> for String {
    fn from(from: BString) -> Self {
        String::from(&from)
    }
}

impl std::fmt::Display for BString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: avoid copy
        f.write_str(&String::from(self))
    }
}

#[link(name = "oleaut32")]
extern "system" {
    fn SysStringLen(bstr: RawPtr) -> u32;
    fn SysFreeString(bstr: RawPtr);
}
