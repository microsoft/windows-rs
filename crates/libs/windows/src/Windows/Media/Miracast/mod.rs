#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a315258_e444_51b4_aff7_b88daa1229e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefaultSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSettingsAsync: usize,
    pub DisconnectAllAndApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisconnectAllAndApplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAllAndApplySettingsAsync: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateSession: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    CreateSessionAsync: usize,
    pub ClearKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveKnownTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transmitter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverApplySettingsResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverApplySettingsResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0aa6272_09cd_58e1_a4f2_5d5143d312f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverApplySettingsResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverApplySettingsStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x704b2f36_d2e5_551f_a854_f822b7917d28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason) -> ::windows_core::HRESULT,
    pub DisconnectWithMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason, message: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
    pub Transmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InputDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CursorImageChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StreamControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverConnectionCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverConnectionCreatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d8dfa39_307a_5c0f_94bd_d0c69d169982);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverConnectionCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverCursorImageChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverCursorImageChannel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9ac332d_723a_5a9d_b90a_81153efa2a0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverCursorImageChannel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::PointInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Position: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImageStream: usize,
    #[cfg(feature = "Foundation")]
    pub ImageStreamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageStreamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverCursorImageChannelSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverCursorImageChannelSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccdbedff_bd00_5b9c_8e4c_00cacf86b634);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverCursorImageChannelSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub SetMaxImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetMaxImageSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverDisconnectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverDisconnectedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9a15e5e_5fee_57e6_b4b0_04727db93229);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverDisconnectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverGameControllerDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverGameControllerDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d7171e8_bed4_5118_a058_e2477eb5888d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverGameControllerDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverGameControllerDeviceUsageMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverInputDevices(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverInputDevices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda35bb02_28aa_5ee8_96f5_a42901c66f00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverInputDevices_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Keyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverKeyboardDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverKeyboardDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbeb67272_06c0_54ff_ac96_217464ff2501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverKeyboardDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverMediaSourceCreatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17cf519e_1246_531d_945a_6b158e39c3aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    pub CursorImageChannelSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d2bcdb4_ef8b_5209_bfc9_c32116504803);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ConnectionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
    pub AllowConnectionTakeover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowConnectionTakeover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverSessionStartResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverSessionStartResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7c573ee_40ca_51ff_95f2_c9de34f2e90e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSessionStartResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverSessionStartStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57cd2f24_c55a_5fbe_9464_eb05307705dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AuthorizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverAuthorizationMethod) -> ::windows_core::HRESULT,
    pub SetAuthorizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastReceiverAuthorizationMethod) -> ::windows_core::HRESULT,
    pub RequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverStatus {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc28a5591_23ab_519e_ad09_90bff6dcc87e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverStatus_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ListeningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverListeningStatus) -> ::windows_core::HRESULT,
    pub WiFiStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverWiFiStatus) -> ::windows_core::HRESULT,
    pub IsConnectionTakeoverSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownTransmitters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverStreamControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverStreamControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38ea2d8b_2769_5ad7_8a8a_254b9df7ba82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverStreamControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetVideoStreamSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVideoStreamSettingsAsync: usize,
    pub SuggestVideoStreamSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestVideoStreamSettingsAsync: usize,
    pub MuteAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetMuteAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastReceiverVideoStreamSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastReceiverVideoStreamSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x169b5e1b_149d_52d0_b126_6f89744e4f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverVideoStreamSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Size: usize,
    #[cfg(feature = "Graphics")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetSize: usize,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMiracastTransmitter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMiracastTransmitter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x342d79fd_2e64_5508_8a30_833d1eac70d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastTransmitter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AuthorizationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastTransmitterAuthorizationStatus) -> ::windows_core::HRESULT,
    pub SetAuthorizationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastTransmitterAuthorizationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnections: usize,
    pub MacAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastConnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastConnectionTime: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiver(::windows_core::IUnknown);
impl MiracastReceiver {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MiracastReceiver, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetDefaultSettings(&self) -> ::windows_core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentSettings(&self) -> ::windows_core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSettingsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSettingsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisconnectAllAndApplySettings<P0>(&self, settings: P0) -> ::windows_core::Result<MiracastReceiverApplySettingsResult>
    where
        P0: ::windows_core::IntoParam<MiracastReceiverSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisconnectAllAndApplySettings)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectAllAndApplySettingsAsync<P0>(&self, settings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>>
    where
        P0: ::windows_core::IntoParam<MiracastReceiverSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisconnectAllAndApplySettingsAsync)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetStatus(&self) -> ::windows_core::Result<MiracastReceiverStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetStatusAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"ApplicationModel_Core\"`"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateSession<P0>(&self, view: P0) -> ::windows_core::Result<MiracastReceiverSession>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::Core::CoreApplicationView>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSession)(::windows_core::Interface::as_raw(this), view.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn CreateSessionAsync<P0>(&self, view: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::Core::CoreApplicationView>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSessionAsync)(::windows_core::Interface::as_raw(this), view.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ClearKnownTransmitters(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearKnownTransmitters)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveKnownTransmitter<P0>(&self, transmitter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MiracastTransmitter>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveKnownTransmitter)(::windows_core::Interface::as_raw(this), transmitter.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiver {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiver;{7a315258-e444-51b4-aff7-b88daa1229e0})");
}
unsafe impl ::windows_core::Interface for MiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiver {
    const IID: ::windows_core::GUID = <IMiracastReceiver as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiver {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiver";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiver, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiver {}
unsafe impl ::core::marker::Sync for MiracastReceiver {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverApplySettingsResult(::windows_core::IUnknown);
impl MiracastReceiverApplySettingsResult {
    pub fn Status(&self) -> ::windows_core::Result<MiracastReceiverApplySettingsStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverApplySettingsResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverApplySettingsResult;{d0aa6272-09cd-58e1-a4f2-5d5143d312f9})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverApplySettingsResult {
    const IID: ::windows_core::GUID = <IMiracastReceiverApplySettingsResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverApplySettingsResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverApplySettingsResult";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverApplySettingsResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverApplySettingsResult {}
unsafe impl ::core::marker::Sync for MiracastReceiverApplySettingsResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverConnection(::windows_core::IUnknown);
impl MiracastReceiverConnection {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Disconnect(&self, reason: MiracastReceiverDisconnectReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Disconnect)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
    pub fn DisconnectWithMessage(&self, reason: MiracastReceiverDisconnectReason, message: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DisconnectWithMessage)(::windows_core::Interface::as_raw(this), reason, ::core::mem::transmute_copy(message)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PauseAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResumeAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Transmitter(&self) -> ::windows_core::Result<MiracastTransmitter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Transmitter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InputDevices(&self) -> ::windows_core::Result<MiracastReceiverInputDevices> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputDevices)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CursorImageChannel(&self) -> ::windows_core::Result<MiracastReceiverCursorImageChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CursorImageChannel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StreamControl(&self) -> ::windows_core::Result<MiracastReceiverStreamControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamControl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverConnection;{704b2f36-d2e5-551f-a854-f822b7917d28})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverConnection {
    const IID: ::windows_core::GUID = <IMiracastReceiverConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverConnection {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnection";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MiracastReceiverConnection {}
unsafe impl ::core::marker::Send for MiracastReceiverConnection {}
unsafe impl ::core::marker::Sync for MiracastReceiverConnection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverConnectionCreatedEventArgs(::windows_core::IUnknown);
impl MiracastReceiverConnectionCreatedEventArgs {
    pub fn Connection(&self) -> ::windows_core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Pin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverConnectionCreatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs;{7d8dfa39-307a-5c0f-94bd-d0c69d169982})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverConnectionCreatedEventArgs {
    const IID: ::windows_core::GUID = <IMiracastReceiverConnectionCreatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverConnectionCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverConnectionCreatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverConnectionCreatedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverConnectionCreatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverCursorImageChannel(::windows_core::IUnknown);
impl MiracastReceiverCursorImageChannel {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> ::windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxImageSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImageStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ImageStreamChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageStreamChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveImageStreamChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveImageStreamChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverCursorImageChannel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverCursorImageChannel;{d9ac332d-723a-5a9d-b90a-81153efa2a0f})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverCursorImageChannel {
    const IID: ::windows_core::GUID = <IMiracastReceiverCursorImageChannel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverCursorImageChannel {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannel";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverCursorImageChannel, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverCursorImageChannel {}
unsafe impl ::core::marker::Sync for MiracastReceiverCursorImageChannel {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverCursorImageChannelSettings(::windows_core::IUnknown);
impl MiracastReceiverCursorImageChannelSettings {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> ::windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxImageSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn SetMaxImageSize(&self, value: super::super::Graphics::SizeInt32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxImageSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverCursorImageChannelSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings;{ccdbedff-bd00-5b9c-8e4c-00cacf86b634})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverCursorImageChannelSettings {
    const IID: ::windows_core::GUID = <IMiracastReceiverCursorImageChannelSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverCursorImageChannelSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverCursorImageChannelSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverCursorImageChannelSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverCursorImageChannelSettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverDisconnectedEventArgs(::windows_core::IUnknown);
impl MiracastReceiverDisconnectedEventArgs {
    pub fn Connection(&self) -> ::windows_core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverDisconnectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs;{d9a15e5e-5fee-57e6-b4b0-04727db93229})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverDisconnectedEventArgs {
    const IID: ::windows_core::GUID = <IMiracastReceiverDisconnectedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverDisconnectedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverDisconnectedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverDisconnectedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverGameControllerDevice(::windows_core::IUnknown);
impl MiracastReceiverGameControllerDevice {
    pub fn TransmitInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransmitInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransmitInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransmitInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRequestedByTransmitter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequestedByTransmitter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTransmittingInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTransmittingInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows_core::Result<MiracastReceiverGameControllerDeviceUsageMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverGameControllerDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverGameControllerDevice;{2d7171e8-bed4-5118-a058-e2477eb5888d})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverGameControllerDevice {
    const IID: ::windows_core::GUID = <IMiracastReceiverGameControllerDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverGameControllerDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverGameControllerDevice";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverGameControllerDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverGameControllerDevice {}
unsafe impl ::core::marker::Sync for MiracastReceiverGameControllerDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverInputDevices(::windows_core::IUnknown);
impl MiracastReceiverInputDevices {
    pub fn Keyboard(&self) -> ::windows_core::Result<MiracastReceiverKeyboardDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Keyboard)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GameController(&self) -> ::windows_core::Result<MiracastReceiverGameControllerDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GameController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverInputDevices {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverInputDevices;{da35bb02-28aa-5ee8-96f5-a42901c66f00})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverInputDevices {
    const IID: ::windows_core::GUID = <IMiracastReceiverInputDevices as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverInputDevices {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverInputDevices";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverInputDevices, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverInputDevices {}
unsafe impl ::core::marker::Sync for MiracastReceiverInputDevices {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverKeyboardDevice(::windows_core::IUnknown);
impl MiracastReceiverKeyboardDevice {
    pub fn TransmitInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransmitInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransmitInput(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransmitInput)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRequestedByTransmitter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRequestedByTransmitter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTransmittingInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTransmittingInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Changed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverKeyboardDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverKeyboardDevice;{beb67272-06c0-54ff-ac96-217464ff2501})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverKeyboardDevice {
    const IID: ::windows_core::GUID = <IMiracastReceiverKeyboardDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverKeyboardDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverKeyboardDevice";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverKeyboardDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverKeyboardDevice {}
unsafe impl ::core::marker::Sync for MiracastReceiverKeyboardDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverMediaSourceCreatedEventArgs(::windows_core::IUnknown);
impl MiracastReceiverMediaSourceCreatedEventArgs {
    pub fn Connection(&self) -> ::windows_core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Media_Core\"`"]
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows_core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CursorImageChannelSettings(&self) -> ::windows_core::Result<MiracastReceiverCursorImageChannelSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CursorImageChannelSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverMediaSourceCreatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs;{17cf519e-1246-531d-945a-6b158e39c3aa})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverMediaSourceCreatedEventArgs {
    const IID: ::windows_core::GUID = <IMiracastReceiverMediaSourceCreatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverMediaSourceCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverMediaSourceCreatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverMediaSourceCreatedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverMediaSourceCreatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverSession(::windows_core::IUnknown);
impl MiracastReceiverSession {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCreated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionCreated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionCreated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCreated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceCreated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaSourceCreated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Disconnected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Disconnected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisconnected)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AllowConnectionTakeover(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowConnectionTakeover)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowConnectionTakeover(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowConnectionTakeover)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxSimultaneousConnections(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSimultaneousConnections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxSimultaneousConnections(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSimultaneousConnections)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<MiracastReceiverSessionStartResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverSession;{1d2bcdb4-ef8b-5209-bfc9-c32116504803})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverSession {
    const IID: ::windows_core::GUID = <IMiracastReceiverSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverSession {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSession";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MiracastReceiverSession {}
unsafe impl ::core::marker::Send for MiracastReceiverSession {}
unsafe impl ::core::marker::Sync for MiracastReceiverSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverSessionStartResult(::windows_core::IUnknown);
impl MiracastReceiverSessionStartResult {
    pub fn Status(&self) -> ::windows_core::Result<MiracastReceiverSessionStartStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverSessionStartResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverSessionStartResult;{b7c573ee-40ca-51ff-95f2-c9de34f2e90e})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverSessionStartResult {
    const IID: ::windows_core::GUID = <IMiracastReceiverSessionStartResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverSessionStartResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSessionStartResult";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverSessionStartResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverSessionStartResult {}
unsafe impl ::core::marker::Sync for MiracastReceiverSessionStartResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverSettings(::windows_core::IUnknown);
impl MiracastReceiverSettings {
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFriendlyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ModelName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModelName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetModelName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModelName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetModelNumber(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetModelNumber)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AuthorizationMethod(&self) -> ::windows_core::Result<MiracastReceiverAuthorizationMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthorizationMethod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAuthorizationMethod(&self, value: MiracastReceiverAuthorizationMethod) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthorizationMethod)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequireAuthorizationFromKnownTransmitters(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequireAuthorizationFromKnownTransmitters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequireAuthorizationFromKnownTransmitters(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequireAuthorizationFromKnownTransmitters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverSettings;{57cd2f24-c55a-5fbe-9464-eb05307705dd})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverSettings {
    const IID: ::windows_core::GUID = <IMiracastReceiverSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSettings";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverSettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverStatus(::windows_core::IUnknown);
impl MiracastReceiverStatus {
    pub fn ListeningStatus(&self) -> ::windows_core::Result<MiracastReceiverListeningStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ListeningStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WiFiStatus(&self) -> ::windows_core::Result<MiracastReceiverWiFiStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WiFiStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConnectionTakeoverSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsConnectionTakeoverSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxSimultaneousConnections(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSimultaneousConnections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KnownTransmitters(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KnownTransmitters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverStatus;{c28a5591-23ab-519e-ad09-90bff6dcc87e})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverStatus {
    const IID: ::windows_core::GUID = <IMiracastReceiverStatus as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverStatus {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStatus";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverStatus, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverStatus {}
unsafe impl ::core::marker::Sync for MiracastReceiverStatus {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverStreamControl(::windows_core::IUnknown);
impl MiracastReceiverStreamControl {
    pub fn GetVideoStreamSettings(&self) -> ::windows_core::Result<MiracastReceiverVideoStreamSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVideoStreamSettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetVideoStreamSettingsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVideoStreamSettingsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SuggestVideoStreamSettings<P0>(&self, settings: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MiracastReceiverVideoStreamSettings>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SuggestVideoStreamSettings)(::windows_core::Interface::as_raw(this), settings.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SuggestVideoStreamSettingsAsync<P0>(&self, settings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<MiracastReceiverVideoStreamSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestVideoStreamSettingsAsync)(::windows_core::Interface::as_raw(this), settings.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn MuteAudio(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MuteAudio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMuteAudio(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMuteAudio)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverStreamControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverStreamControl;{38ea2d8b-2769-5ad7-8a8a-254b9df7ba82})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverStreamControl {
    const IID: ::windows_core::GUID = <IMiracastReceiverStreamControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverStreamControl {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStreamControl";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverStreamControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverStreamControl {}
unsafe impl ::core::marker::Sync for MiracastReceiverStreamControl {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastReceiverVideoStreamSettings(::windows_core::IUnknown);
impl MiracastReceiverVideoStreamSettings {
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn Size(&self) -> ::windows_core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Graphics\"`"]
    #[cfg(feature = "Graphics")]
    pub fn SetSize(&self, value: super::super::Graphics::SizeInt32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bitrate(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBitrate(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverVideoStreamSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverVideoStreamSettings;{169b5e1b-149d-52d0-b126-6f89744e4f50})");
}
unsafe impl ::windows_core::Interface for MiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastReceiverVideoStreamSettings {
    const IID: ::windows_core::GUID = <IMiracastReceiverVideoStreamSettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastReceiverVideoStreamSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverVideoStreamSettings";
}
::windows_core::imp::interface_hierarchy!(MiracastReceiverVideoStreamSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverVideoStreamSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverVideoStreamSettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MiracastTransmitter(::windows_core::IUnknown);
impl MiracastTransmitter {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AuthorizationStatus(&self) -> ::windows_core::Result<MiracastTransmitterAuthorizationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AuthorizationStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAuthorizationStatus(&self, value: MiracastTransmitterAuthorizationStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAuthorizationStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnections(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetConnections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MacAddress(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MacAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LastConnectionTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastConnectionTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MiracastTransmitter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastTransmitter;{342d79fd-2e64-5508-8a30-833d1eac70d0})");
}
unsafe impl ::windows_core::Interface for MiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MiracastTransmitter {
    const IID: ::windows_core::GUID = <IMiracastTransmitter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MiracastTransmitter {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastTransmitter";
}
::windows_core::imp::interface_hierarchy!(MiracastTransmitter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MiracastTransmitter {}
unsafe impl ::core::marker::Sync for MiracastTransmitter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverApplySettingsStatus(pub i32);
impl MiracastReceiverApplySettingsStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const FriendlyNameTooLong: Self = Self(4i32);
    pub const ModelNameTooLong: Self = Self(5i32);
    pub const ModelNumberTooLong: Self = Self(6i32);
    pub const InvalidSettings: Self = Self(7i32);
}
impl ::core::marker::Copy for MiracastReceiverApplySettingsStatus {}
impl ::core::clone::Clone for MiracastReceiverApplySettingsStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverApplySettingsStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverApplySettingsStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverApplySettingsStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverApplySettingsStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverApplySettingsStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverApplySettingsStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverAuthorizationMethod(pub i32);
impl MiracastReceiverAuthorizationMethod {
    pub const None: Self = Self(0i32);
    pub const ConfirmConnection: Self = Self(1i32);
    pub const PinDisplayIfRequested: Self = Self(2i32);
    pub const PinDisplayRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverAuthorizationMethod {}
impl ::core::clone::Clone for MiracastReceiverAuthorizationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverAuthorizationMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverAuthorizationMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverAuthorizationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverAuthorizationMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverAuthorizationMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverAuthorizationMethod;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverDisconnectReason(pub i32);
impl MiracastReceiverDisconnectReason {
    pub const Finished: Self = Self(0i32);
    pub const AppSpecificError: Self = Self(1i32);
    pub const ConnectionNotAccepted: Self = Self(2i32);
    pub const DisconnectedByUser: Self = Self(3i32);
    pub const FailedToStartStreaming: Self = Self(4i32);
    pub const MediaDecodingError: Self = Self(5i32);
    pub const MediaStreamingError: Self = Self(6i32);
    pub const MediaDecryptionError: Self = Self(7i32);
}
impl ::core::marker::Copy for MiracastReceiverDisconnectReason {}
impl ::core::clone::Clone for MiracastReceiverDisconnectReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverDisconnectReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverDisconnectReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverDisconnectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverDisconnectReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverDisconnectReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverDisconnectReason;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverGameControllerDeviceUsageMode(pub i32);
impl MiracastReceiverGameControllerDeviceUsageMode {
    pub const AsGameController: Self = Self(0i32);
    pub const AsMouseAndKeyboard: Self = Self(1i32);
}
impl ::core::marker::Copy for MiracastReceiverGameControllerDeviceUsageMode {}
impl ::core::clone::Clone for MiracastReceiverGameControllerDeviceUsageMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverGameControllerDeviceUsageMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverGameControllerDeviceUsageMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverGameControllerDeviceUsageMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverGameControllerDeviceUsageMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverGameControllerDeviceUsageMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverGameControllerDeviceUsageMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverListeningStatus(pub i32);
impl MiracastReceiverListeningStatus {
    pub const NotListening: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const ConnectionPending: Self = Self(2i32);
    pub const Connected: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
    pub const TemporarilyDisabled: Self = Self(5i32);
}
impl ::core::marker::Copy for MiracastReceiverListeningStatus {}
impl ::core::clone::Clone for MiracastReceiverListeningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverListeningStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverListeningStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverListeningStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverListeningStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverListeningStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverListeningStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverSessionStartStatus(pub i32);
impl MiracastReceiverSessionStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverSessionStartStatus {}
impl ::core::clone::Clone for MiracastReceiverSessionStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverSessionStartStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverSessionStartStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverSessionStartStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverSessionStartStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverSessionStartStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverSessionStartStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastReceiverWiFiStatus(pub i32);
impl MiracastReceiverWiFiStatus {
    pub const MiracastSupportUndetermined: Self = Self(0i32);
    pub const MiracastNotSupported: Self = Self(1i32);
    pub const MiracastSupportNotOptimized: Self = Self(2i32);
    pub const MiracastSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverWiFiStatus {}
impl ::core::clone::Clone for MiracastReceiverWiFiStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastReceiverWiFiStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastReceiverWiFiStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastReceiverWiFiStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverWiFiStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastReceiverWiFiStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverWiFiStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MiracastTransmitterAuthorizationStatus(pub i32);
impl MiracastTransmitterAuthorizationStatus {
    pub const Undecided: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const AlwaysPrompt: Self = Self(2i32);
    pub const Blocked: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastTransmitterAuthorizationStatus {}
impl ::core::clone::Clone for MiracastTransmitterAuthorizationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MiracastTransmitterAuthorizationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MiracastTransmitterAuthorizationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MiracastTransmitterAuthorizationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastTransmitterAuthorizationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MiracastTransmitterAuthorizationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastTransmitterAuthorizationStatus;i4)");
}
