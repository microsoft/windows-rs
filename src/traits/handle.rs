use crate::*;

pub unsafe trait Handle: Sized + PartialEq {
    fn is_null(&self) -> bool {
        *self == unsafe { std::mem::zeroed() }
    }

    fn ok(self) -> Result<Self> {
        if !self.is_null() {
            Ok(self)
        } else {
            Err(HRESULT::from_thread().into())
        }
    }
}
