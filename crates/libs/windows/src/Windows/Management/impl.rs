#[cfg(feature = "implement_exclusive")]
pub trait IMdmAlertImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetData(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Format(&self) -> ::windows::core::Result<MdmAlertDataType>;
    fn SetFormat(&self, value: MdmAlertDataType) -> ::windows::core::Result<()>;
    fn Mark(&self) -> ::windows::core::Result<MdmAlertMark>;
    fn SetMark(&self, value: MdmAlertMark) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<u32>;
    fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTarget(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMdmSessionImpl: Sized {
    fn Alerts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<MdmAlert>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<MdmSessionState>;
    fn AttachAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn Delete(&self) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn StartWithAlertsAsync(&self, alerts: &::core::option::Option<super::Foundation::Collections::IIterable<MdmAlert>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMdmSessionManagerStaticsImpl: Sized {
    fn SessionIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn TryCreateSession(&self) -> ::windows::core::Result<MdmSession>;
    fn DeleteSessionById(&self, sessionid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetSessionById(&self, sessionid: &::windows::core::HSTRING) -> ::windows::core::Result<MdmSession>;
}
