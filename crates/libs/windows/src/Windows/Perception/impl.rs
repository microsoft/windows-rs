#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestampImpl: Sized {
    fn TargetTime(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn PredictionAmount(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestamp2Impl: Sized {
    fn SystemRelativeTargetTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestampHelperStaticsImpl: Sized {
    fn FromHistoricalTargetTime(&self, targettime: &super::Foundation::DateTime) -> ::windows::core::Result<PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerceptionTimestampHelperStatics2Impl: Sized {
    fn FromSystemRelativeTargetTime(&self, targettime: &super::Foundation::TimeSpan) -> ::windows::core::Result<PerceptionTimestamp>;
}
