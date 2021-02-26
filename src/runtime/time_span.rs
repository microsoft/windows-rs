use super::*;

#[repr(C)]
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct TimeSpan {
    pub duration: i64,
}

unsafe impl Abi for TimeSpan {
    type Abi = Self;
}

unsafe impl RuntimeType for TimeSpan {
    type DefaultType = Self;

    const SIGNATURE: ConstBuffer =
        ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
}

impl std::convert::From<std::time::Duration> for TimeSpan {
    fn from(value: std::time::Duration) -> Self {
        Self {
            duration: (value.as_nanos() / 100) as i64,
        }
    }
}

impl std::convert::From<TimeSpan> for std::time::Duration {
    fn from(value: TimeSpan) -> Self {
        std::time::Duration::from_nanos((value.duration * 100) as u64)
    }
}

impl<'a> std::convert::Into<Param<'a, TimeSpan>> for std::time::Duration {
    fn into(self) -> Param<'a, TimeSpan> {
        Param::Owned(self.into())
    }
}
