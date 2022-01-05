pub trait ILowLevelDevicesAggregateProviderImpl: Sized {
    fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLevelDevicesAggregateProviderFactoryImpl: Sized {
    fn Create(&self, adc: &::core::option::Option<Adc::Provider::IAdcControllerProvider>, pwm: &::core::option::Option<Pwm::Provider::IPwmControllerProvider>, gpio: &::core::option::Option<Gpio::Provider::IGpioControllerProvider>, i2c: &::core::option::Option<I2c::Provider::II2cControllerProvider>, spi: &::core::option::Option<Spi::Provider::ISpiControllerProvider>) -> ::windows::core::Result<LowLevelDevicesAggregateProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLevelDevicesControllerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLevelDevicesControllerStaticsImpl: Sized {
    fn DefaultProvider(&self) -> ::windows::core::Result<ILowLevelDevicesAggregateProvider>;
    fn SetDefaultProvider(&self, value: &::core::option::Option<ILowLevelDevicesAggregateProvider>) -> ::windows::core::Result<()>;
}
