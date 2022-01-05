#[cfg(feature = "implement_exclusive")]
pub trait IRemoteTextConnectionImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RegisterThread(&self, threadid: u32) -> ::windows::core::Result<()>;
    fn UnregisterThread(&self, threadid: u32) -> ::windows::core::Result<()>;
    fn ReportDataReceived(&self, pdudata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteTextConnectionFactoryImpl: Sized {
    fn CreateInstance(&self, connectionid: &::windows::core::GUID, pduforwarder: &::core::option::Option<RemoteTextConnectionDataHandler>) -> ::windows::core::Result<RemoteTextConnection>;
}
