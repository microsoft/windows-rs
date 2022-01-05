pub trait INetDiagExtensibleHelperImpl: Sized {
    fn ResolveAttributes();
}
pub trait INetDiagHelperImpl: Sized {
    fn Initialize();
    fn GetDiagnosticsInfo();
    fn GetKeyAttributes();
    fn LowHealth();
    fn HighUtilization();
    fn GetLowerHypotheses();
    fn GetDownStreamHypotheses();
    fn GetHigherHypotheses();
    fn GetUpStreamHypotheses();
    fn Repair();
    fn Validate();
    fn GetRepairInfo();
    fn GetLifeTime();
    fn SetLifeTime();
    fn GetCacheTime();
    fn GetAttributes();
    fn Cancel();
    fn Cleanup();
}
pub trait INetDiagHelperExImpl: Sized {
    fn ReconfirmLowHealth();
    fn SetUtilities();
    fn ReproduceFailure();
}
pub trait INetDiagHelperInfoImpl: Sized {
    fn GetAttributeInfo();
}
pub trait INetDiagHelperUtilFactoryImpl: Sized {
    fn CreateUtilityInstance();
}
