use crate::foundation::TimeSpan;
use crate::Param;

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
