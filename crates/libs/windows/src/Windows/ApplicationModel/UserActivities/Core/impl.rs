#[cfg(feature = "implement_exclusive")]
pub trait ICoreUserActivityManagerStaticsImpl: Sized {
    fn CreateUserActivitySessionInBackground(&self, activity: &::core::option::Option<super::UserActivity>) -> ::windows::core::Result<super::UserActivitySession>;
    fn DeleteUserActivitySessionsInTimeRangeAsync(&self, channel: &::core::option::Option<super::UserActivityChannel>, starttime: &super::super::super::Foundation::DateTime, endtime: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
