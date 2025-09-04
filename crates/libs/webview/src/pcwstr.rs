use super::*;

pub struct OwnedPcwstr(Vec<u16>);

pub fn pcwstr<T: AsRef<str>>(value: T) -> OwnedPcwstr {
    OwnedPcwstr(
        value
            .as_ref()
            .encode_utf16()
            .chain(core::iter::once(0))
            .collect(),
    )
}

impl OwnedPcwstr {
    pub fn as_ptr(&self) -> *const u16 {
        debug_assert!(
            self.0.last() == Some(&0),
            "`OwnedPcwstr` isn't null-terminated"
        );
        self.0.as_ptr()
    }

    pub fn as_raw(&self) -> PCWSTR {
        PCWSTR(self.as_ptr())
    }
}
