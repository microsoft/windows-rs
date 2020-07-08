use crate::*;

/// Represents a time interval as a signed 64-bit integer value.
///
/// TimeSpan represents the WinRT [TimeSpan](https://docs.microsoft.com/en-us/uwp/api/Windows.Foundation.TimeSpan)
/// struct and provides convertibility with `std::time::Duration`.
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TimeSpan {
    pub duration: i64,
}

unsafe impl RuntimeType for TimeSpan {
    fn signature() -> String {
        "struct(Windows.Foundation.TimeSpan;i8)".to_owned()
    }
}

unsafe impl AbiTransferable for TimeSpan {
    type Abi = Self;

    fn get_abi(&self) -> Self::Abi {
        self.clone()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}

impl From<std::time::Duration> for TimeSpan {
    fn from(value: std::time::Duration) -> Self {
        Self {
            duration: (value.as_nanos() / 100) as i64,
        }
    }
}

impl From<TimeSpan> for std::time::Duration {
    fn from(value: TimeSpan) -> Self {
        std::time::Duration::from_nanos((value.duration * 100) as u64)
    }
}

impl<'a> Into<Param<'a, TimeSpan>> for std::time::Duration {
    fn into(self) -> Param<'a, TimeSpan> {
        Param::Owned(self.into())
    }
}
