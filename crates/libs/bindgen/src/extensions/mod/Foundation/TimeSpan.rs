impl ::core::convert::From<::core::time::Duration> for TimeSpan {
    fn from(value: ::core::time::Duration) -> Self {
        Self { Duration: (value.as_nanos() / 100) as i64 }
    }
}
impl ::core::convert::From<TimeSpan> for ::core::time::Duration {
    fn from(value: TimeSpan) -> Self {
        ::core::time::Duration::from_nanos((value.Duration * 100) as u64)
    }
}
