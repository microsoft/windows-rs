pub trait ICustomGameControllerFactoryImpl: Sized {
    fn CreateGameController(&self, provider: &::core::option::Option<IGameControllerProvider>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn OnGameControllerAdded(&self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
    fn OnGameControllerRemoved(&self, value: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerFactoryManagerStaticsImpl: Sized {
    fn RegisterCustomFactoryForGipInterface(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, interfaceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterCustomFactoryForHardwareId(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows::core::Result<()>;
    fn RegisterCustomFactoryForXusbType(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGameControllerFactoryManagerStatics2Impl: Sized + IGameControllerFactoryManagerStaticsImpl {
    fn TryGetFactoryControllerFromGameController(&self, factory: &::core::option::Option<ICustomGameControllerFactory>, gamecontroller: &::core::option::Option<super::IGameController>) -> ::windows::core::Result<super::IGameController>;
}
pub trait IGameControllerInputSinkImpl: Sized {
    fn OnInputResumed(&self, timestamp: u64) -> ::windows::core::Result<()>;
    fn OnInputSuspended(&self, timestamp: u64) -> ::windows::core::Result<()>;
}
pub trait IGameControllerProviderImpl: Sized {
    fn FirmwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVendorId(&self) -> ::windows::core::Result<u16>;
    fn HardwareVersionInfo(&self) -> ::windows::core::Result<GameControllerVersionInfo>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGipFirmwareUpdateResultImpl: Sized {
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<u32>;
    fn FinalComponentId(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<GipFirmwareUpdateStatus>;
}
pub trait IGipGameControllerInputSinkImpl: Sized + IGameControllerInputSinkImpl {
    fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows::core::Result<()>;
    fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGipGameControllerProviderImpl: Sized + IGameControllerProviderImpl {
    fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsemessagebuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UpdateFirmwareAsync(&self, firmwareimage: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>;
}
pub trait IHidGameControllerInputSinkImpl: Sized + IGameControllerInputSinkImpl {
    fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHidGameControllerProviderImpl: Sized + IGameControllerProviderImpl {
    fn UsageId(&self) -> ::windows::core::Result<u16>;
    fn UsagePage(&self) -> ::windows::core::Result<u16>;
    fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SendOutputReport(&self, reportid: u8, reportbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
pub trait IXusbGameControllerInputSinkImpl: Sized + IGameControllerInputSinkImpl {
    fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXusbGameControllerProviderImpl: Sized + IGameControllerProviderImpl {
    fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows::core::Result<()>;
}
