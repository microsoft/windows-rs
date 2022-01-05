#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundEnergyDiagnosticsStaticsImpl: Sized {
    fn DeviceSpecificConversionFactor(&self) -> ::windows::core::Result<f64>;
    fn ComputeTotalEnergyUsage(&self) -> ::windows::core::Result<u64>;
    fn ResetTotalEnergyUsage(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IForegroundEnergyDiagnosticsStaticsImpl: Sized {
    fn DeviceSpecificConversionFactor(&self) -> ::windows::core::Result<f64>;
    fn ComputeTotalEnergyUsage(&self) -> ::windows::core::Result<u64>;
    fn ResetTotalEnergyUsage(&self) -> ::windows::core::Result<()>;
}
