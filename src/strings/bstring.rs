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

impl From<BString> for String {
    fn from(from: BString) -> Self {
        if from.is_empty() {
            return String::new();
        }

        unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(from.0 as *const u16, from.len()))
        }
    }
}

#[link(name = "oleaut32")]
extern "system" {
    fn SysStringLen(bstr: RawPtr) -> u32;
    fn SysFreeString(bstr: RawPtr);
}
