pub trait IReferenceClockImpl: Sized {
    fn GetTime();
    fn AdviseTime();
    fn AdvisePeriodic();
    fn Unadvise();
}
pub trait IReferenceClock2Impl: Sized + IReferenceClockImpl {}
pub trait IReferenceClockTimerControlImpl: Sized {
    fn SetDefaultTimerResolution();
    fn GetDefaultTimerResolution();
}
