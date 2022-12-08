#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiver(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a315258_e444_51b4_aff7_b88daa1229e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefaultSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSettingsAsync: usize,
    pub DisconnectAllAndApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisconnectAllAndApplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAllAndApplySettingsAsync: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateSession: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    CreateSessionAsync: usize,
    pub ClearKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveKnownTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transmitter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverApplySettingsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverApplySettingsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0aa6272_09cd_58e1_a4f2_5d5143d312f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverApplySettingsResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverApplySettingsStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x704b2f36_d2e5_551f_a854_f822b7917d28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason) -> ::windows::core::HRESULT,
    pub DisconnectWithMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
    pub Transmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InputDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CursorImageChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StreamControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverConnectionCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverConnectionCreatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d8dfa39_307a_5c0f_94bd_d0c69d169982);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverConnectionCreatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverCursorImageChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9ac332d_723a_5a9d_b90a_81153efa2a0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverCursorImageChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::PointInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Position: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImageStream: usize,
    #[cfg(feature = "Foundation")]
    pub ImageStreamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageStreamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannelSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverCursorImageChannelSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccdbedff_bd00_5b9c_8e4c_00cacf86b634);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverCursorImageChannelSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics")]
    pub MaxImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    MaxImageSize: usize,
    #[cfg(feature = "Graphics")]
    pub SetMaxImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetMaxImageSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverDisconnectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverDisconnectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9a15e5e_5fee_57e6_b4b0_04727db93229);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverDisconnectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverGameControllerDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverGameControllerDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d7171e8_bed4_5118_a058_e2477eb5888d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverGameControllerDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverInputDevices(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverInputDevices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda35bb02_28aa_5ee8_96f5_a42901c66f00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverInputDevices_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Keyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverKeyboardDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverKeyboardDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb67272_06c0_54ff_ac96_217464ff2501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverKeyboardDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverMediaSourceCreatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf519e_1246_531d_945a_6b158e39c3aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    pub CursorImageChannelSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d2bcdb4_ef8b_5209_bfc9_c32116504803);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ConnectionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
    pub AllowConnectionTakeover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowConnectionTakeover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetMaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverSessionStartResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverSessionStartResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7c573ee_40ca_51ff_95f2_c9de34f2e90e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSessionStartResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverSessionStartStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57cd2f24_c55a_5fbe_9464_eb05307705dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AuthorizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT,
    pub SetAuthorizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT,
    pub RequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc28a5591_23ab_519e_ad09_90bff6dcc87e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ListeningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverListeningStatus) -> ::windows::core::HRESULT,
    pub WiFiStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverWiFiStatus) -> ::windows::core::HRESULT,
    pub IsConnectionTakeoverSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownTransmitters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverStreamControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverStreamControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ea2d8b_2769_5ad7_8a8a_254b9df7ba82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverStreamControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetVideoStreamSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVideoStreamSettingsAsync: usize,
    pub SuggestVideoStreamSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestVideoStreamSettingsAsync: usize,
    pub MuteAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMuteAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverVideoStreamSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastReceiverVideoStreamSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x169b5e1b_149d_52d0_b126_6f89744e4f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverVideoStreamSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Size: usize,
    #[cfg(feature = "Graphics")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SetSize: usize,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastTransmitter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
}
unsafe impl ::windows::core::Interface for IMiracastTransmitter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342d79fd_2e64_5508_8a30_833d1eac70d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastTransmitter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AuthorizationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT,
    pub SetAuthorizationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnections: usize,
    pub MacAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastConnectionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastConnectionTime: usize,
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiver(::windows::core::IUnknown);
impl MiracastReceiver {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MiracastReceiver, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GetDefaultSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultSettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetCurrentSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentSettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentSettingsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisconnectAllAndApplySettings(&self, settings: &MiracastReceiverSettings) -> ::windows::core::Result<MiracastReceiverApplySettingsResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisconnectAllAndApplySettings)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectAllAndApplySettingsAsync(&self, settings: &MiracastReceiverSettings) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisconnectAllAndApplySettingsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetStatus(&self) -> ::windows::core::Result<MiracastReceiverStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatusAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StatusChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStatusChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateSession(&self, view: &super::super::ApplicationModel::Core::CoreApplicationView) -> ::windows::core::Result<MiracastReceiverSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSession)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(view), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn CreateSessionAsync(&self, view: &super::super::ApplicationModel::Core::CoreApplicationView) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSessionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(view), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ClearKnownTransmitters(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClearKnownTransmitters)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn RemoveKnownTransmitter(&self, transmitter: &MiracastTransmitter) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveKnownTransmitter)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(transmitter)).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiver {}
impl ::core::fmt::Debug for MiracastReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiver;{7a315258-e444-51b4-aff7-b88daa1229e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiver {
    const IID: ::windows::core::GUID = <IMiracastReceiver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiver {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiver";
}
::windows::core::interface_hierarchy!(MiracastReceiver, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiver {}
unsafe impl ::core::marker::Sync for MiracastReceiver {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsResult(::windows::core::IUnknown);
impl MiracastReceiverApplySettingsResult {
    pub fn Status(&self) -> ::windows::core::Result<MiracastReceiverApplySettingsStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverApplySettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverApplySettingsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverApplySettingsResult {}
impl ::core::fmt::Debug for MiracastReceiverApplySettingsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverApplySettingsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverApplySettingsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverApplySettingsResult;{d0aa6272-09cd-58e1-a4f2-5d5143d312f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverApplySettingsResult {
    const IID: ::windows::core::GUID = <IMiracastReceiverApplySettingsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverApplySettingsResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverApplySettingsResult";
}
::windows::core::interface_hierarchy!(MiracastReceiverApplySettingsResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverApplySettingsResult {}
unsafe impl ::core::marker::Sync for MiracastReceiverApplySettingsResult {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverConnection(::windows::core::IUnknown);
impl MiracastReceiverConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Disconnect(&self, reason: MiracastReceiverDisconnectReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Disconnect)(::windows::core::Vtable::as_raw(this), reason).ok() }
    }
    pub fn DisconnectWithMessage(&self, reason: MiracastReceiverDisconnectReason, message: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DisconnectWithMessage)(::windows::core::Vtable::as_raw(this), reason, ::core::mem::transmute_copy(message)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Pause)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PauseAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Resume)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResumeAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Transmitter(&self) -> ::windows::core::Result<MiracastTransmitter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Transmitter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InputDevices(&self) -> ::windows::core::Result<MiracastReceiverInputDevices> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputDevices)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CursorImageChannel(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CursorImageChannel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StreamControl(&self) -> ::windows::core::Result<MiracastReceiverStreamControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StreamControl)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverConnection {}
impl ::core::fmt::Debug for MiracastReceiverConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverConnection;{704b2f36-d2e5-551f-a854-f822b7917d28})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverConnection {
    const IID: ::windows::core::GUID = <IMiracastReceiverConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverConnection {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnection";
}
::windows::core::interface_hierarchy!(MiracastReceiverConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MiracastReceiverConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MiracastReceiverConnection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MiracastReceiverConnection> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MiracastReceiverConnection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MiracastReceiverConnection> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MiracastReceiverConnection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverConnection {}
unsafe impl ::core::marker::Sync for MiracastReceiverConnection {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverConnectionCreatedEventArgs(::windows::core::IUnknown);
impl MiracastReceiverConnectionCreatedEventArgs {
    pub fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Pin)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverConnectionCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverConnectionCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverConnectionCreatedEventArgs {}
impl ::core::fmt::Debug for MiracastReceiverConnectionCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverConnectionCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverConnectionCreatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs;{7d8dfa39-307a-5c0f-94bd-d0c69d169982})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverConnectionCreatedEventArgs {
    const IID: ::windows::core::GUID = <IMiracastReceiverConnectionCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverConnectionCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs";
}
::windows::core::interface_hierarchy!(MiracastReceiverConnectionCreatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverConnectionCreatedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverConnectionCreatedEventArgs {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannel(::windows::core::IUnknown);
impl MiracastReceiverCursorImageChannel {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxImageSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImageStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageStreamChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImageStreamChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveImageStreamChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveImageStreamChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePositionChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverCursorImageChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverCursorImageChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverCursorImageChannel {}
impl ::core::fmt::Debug for MiracastReceiverCursorImageChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverCursorImageChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverCursorImageChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverCursorImageChannel;{d9ac332d-723a-5a9d-b90a-81153efa2a0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverCursorImageChannel {
    const IID: ::windows::core::GUID = <IMiracastReceiverCursorImageChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverCursorImageChannel {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannel";
}
::windows::core::interface_hierarchy!(MiracastReceiverCursorImageChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverCursorImageChannel {}
unsafe impl ::core::marker::Sync for MiracastReceiverCursorImageChannel {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannelSettings(::windows::core::IUnknown);
impl MiracastReceiverCursorImageChannelSettings {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxImageSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn SetMaxImageSize(&self, value: super::super::Graphics::SizeInt32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxImageSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverCursorImageChannelSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverCursorImageChannelSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverCursorImageChannelSettings {}
impl ::core::fmt::Debug for MiracastReceiverCursorImageChannelSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverCursorImageChannelSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverCursorImageChannelSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings;{ccdbedff-bd00-5b9c-8e4c-00cacf86b634})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverCursorImageChannelSettings {
    const IID: ::windows::core::GUID = <IMiracastReceiverCursorImageChannelSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverCursorImageChannelSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings";
}
::windows::core::interface_hierarchy!(MiracastReceiverCursorImageChannelSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverCursorImageChannelSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverCursorImageChannelSettings {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverDisconnectedEventArgs(::windows::core::IUnknown);
impl MiracastReceiverDisconnectedEventArgs {
    pub fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverDisconnectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverDisconnectedEventArgs {}
impl ::core::fmt::Debug for MiracastReceiverDisconnectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverDisconnectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverDisconnectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs;{d9a15e5e-5fee-57e6-b4b0-04727db93229})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverDisconnectedEventArgs {
    const IID: ::windows::core::GUID = <IMiracastReceiverDisconnectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverDisconnectedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs";
}
::windows::core::interface_hierarchy!(MiracastReceiverDisconnectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverDisconnectedEventArgs {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDevice(::windows::core::IUnknown);
impl MiracastReceiverGameControllerDevice {
    pub fn TransmitInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransmitInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransmitInput)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRequestedByTransmitter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTransmittingInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTransmittingInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Mode(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDeviceUsageMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Mode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Changed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverGameControllerDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverGameControllerDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverGameControllerDevice {}
impl ::core::fmt::Debug for MiracastReceiverGameControllerDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverGameControllerDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverGameControllerDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverGameControllerDevice;{2d7171e8-bed4-5118-a058-e2477eb5888d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverGameControllerDevice {
    const IID: ::windows::core::GUID = <IMiracastReceiverGameControllerDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverGameControllerDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverGameControllerDevice";
}
::windows::core::interface_hierarchy!(MiracastReceiverGameControllerDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverGameControllerDevice {}
unsafe impl ::core::marker::Sync for MiracastReceiverGameControllerDevice {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverInputDevices(::windows::core::IUnknown);
impl MiracastReceiverInputDevices {
    pub fn Keyboard(&self) -> ::windows::core::Result<MiracastReceiverKeyboardDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Keyboard)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GameController(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GameController)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverInputDevices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverInputDevices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverInputDevices {}
impl ::core::fmt::Debug for MiracastReceiverInputDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverInputDevices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverInputDevices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverInputDevices;{da35bb02-28aa-5ee8-96f5-a42901c66f00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverInputDevices {
    const IID: ::windows::core::GUID = <IMiracastReceiverInputDevices as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverInputDevices {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverInputDevices";
}
::windows::core::interface_hierarchy!(MiracastReceiverInputDevices, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverInputDevices {}
unsafe impl ::core::marker::Sync for MiracastReceiverInputDevices {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverKeyboardDevice(::windows::core::IUnknown);
impl MiracastReceiverKeyboardDevice {
    pub fn TransmitInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransmitInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransmitInput)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRequestedByTransmitter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTransmittingInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTransmittingInput)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Changed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverKeyboardDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverKeyboardDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverKeyboardDevice {}
impl ::core::fmt::Debug for MiracastReceiverKeyboardDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverKeyboardDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverKeyboardDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverKeyboardDevice;{beb67272-06c0-54ff-ac96-217464ff2501})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverKeyboardDevice {
    const IID: ::windows::core::GUID = <IMiracastReceiverKeyboardDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverKeyboardDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverKeyboardDevice";
}
::windows::core::interface_hierarchy!(MiracastReceiverKeyboardDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverKeyboardDevice {}
unsafe impl ::core::marker::Sync for MiracastReceiverKeyboardDevice {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverMediaSourceCreatedEventArgs(::windows::core::IUnknown);
impl MiracastReceiverMediaSourceCreatedEventArgs {
    pub fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CursorImageChannelSettings(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannelSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CursorImageChannelSettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverMediaSourceCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverMediaSourceCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverMediaSourceCreatedEventArgs {}
impl ::core::fmt::Debug for MiracastReceiverMediaSourceCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverMediaSourceCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverMediaSourceCreatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs;{17cf519e-1246-531d-945a-6b158e39c3aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverMediaSourceCreatedEventArgs {
    const IID: ::windows::core::GUID = <IMiracastReceiverMediaSourceCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverMediaSourceCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs";
}
::windows::core::interface_hierarchy!(MiracastReceiverMediaSourceCreatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverMediaSourceCreatedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverMediaSourceCreatedEventArgs {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverSession(::windows::core::IUnknown);
impl MiracastReceiverSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCreated(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionCreated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveConnectionCreated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCreated(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaSourceCreated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCreated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMediaSourceCreated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Disconnected(&self, handler: &super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Disconnected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDisconnected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AllowConnectionTakeover(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowConnectionTakeover)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAllowConnectionTakeover(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAllowConnectionTakeover)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxSimultaneousConnections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMaxSimultaneousConnections(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxSimultaneousConnections)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<MiracastReceiverSessionStartResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverSession {}
impl ::core::fmt::Debug for MiracastReceiverSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverSession;{1d2bcdb4-ef8b-5209-bfc9-c32116504803})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverSession {
    const IID: ::windows::core::GUID = <IMiracastReceiverSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverSession {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSession";
}
::windows::core::interface_hierarchy!(MiracastReceiverSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MiracastReceiverSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MiracastReceiverSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MiracastReceiverSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MiracastReceiverSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MiracastReceiverSession> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MiracastReceiverSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverSession {}
unsafe impl ::core::marker::Sync for MiracastReceiverSession {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverSessionStartResult(::windows::core::IUnknown);
impl MiracastReceiverSessionStartResult {
    pub fn Status(&self) -> ::windows::core::Result<MiracastReceiverSessionStartStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverSessionStartResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverSessionStartResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverSessionStartResult {}
impl ::core::fmt::Debug for MiracastReceiverSessionStartResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverSessionStartResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverSessionStartResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverSessionStartResult;{b7c573ee-40ca-51ff-95f2-c9de34f2e90e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverSessionStartResult {
    const IID: ::windows::core::GUID = <IMiracastReceiverSessionStartResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverSessionStartResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSessionStartResult";
}
::windows::core::interface_hierarchy!(MiracastReceiverSessionStartResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverSessionStartResult {}
unsafe impl ::core::marker::Sync for MiracastReceiverSessionStartResult {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverSettings(::windows::core::IUnknown);
impl MiracastReceiverSettings {
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFriendlyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetModelName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetModelName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetModelNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetModelNumber)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AuthorizationMethod(&self) -> ::windows::core::Result<MiracastReceiverAuthorizationMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthorizationMethod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAuthorizationMethod(&self, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAuthorizationMethod)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RequireAuthorizationFromKnownTransmitters(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequireAuthorizationFromKnownTransmitters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRequireAuthorizationFromKnownTransmitters(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRequireAuthorizationFromKnownTransmitters)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverSettings {}
impl ::core::fmt::Debug for MiracastReceiverSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverSettings;{57cd2f24-c55a-5fbe-9464-eb05307705dd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverSettings {
    const IID: ::windows::core::GUID = <IMiracastReceiverSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSettings";
}
::windows::core::interface_hierarchy!(MiracastReceiverSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverSettings {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverStatus(::windows::core::IUnknown);
impl MiracastReceiverStatus {
    pub fn ListeningStatus(&self) -> ::windows::core::Result<MiracastReceiverListeningStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ListeningStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WiFiStatus(&self) -> ::windows::core::Result<MiracastReceiverWiFiStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WiFiStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsConnectionTakeoverSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsConnectionTakeoverSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxSimultaneousConnections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KnownTransmitters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KnownTransmitters)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastReceiverStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverStatus {}
impl ::core::fmt::Debug for MiracastReceiverStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverStatus;{c28a5591-23ab-519e-ad09-90bff6dcc87e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverStatus {
    const IID: ::windows::core::GUID = <IMiracastReceiverStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverStatus {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStatus";
}
::windows::core::interface_hierarchy!(MiracastReceiverStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverStatus {}
unsafe impl ::core::marker::Sync for MiracastReceiverStatus {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverStreamControl(::windows::core::IUnknown);
impl MiracastReceiverStreamControl {
    pub fn GetVideoStreamSettings(&self) -> ::windows::core::Result<MiracastReceiverVideoStreamSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVideoStreamSettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVideoStreamSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVideoStreamSettingsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SuggestVideoStreamSettings(&self, settings: &MiracastReceiverVideoStreamSettings) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SuggestVideoStreamSettings)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SuggestVideoStreamSettingsAsync(&self, settings: &MiracastReceiverVideoStreamSettings) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuggestVideoStreamSettingsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(settings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MuteAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MuteAudio)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMuteAudio(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMuteAudio)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverStreamControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverStreamControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverStreamControl {}
impl ::core::fmt::Debug for MiracastReceiverStreamControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverStreamControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverStreamControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverStreamControl;{38ea2d8b-2769-5ad7-8a8a-254b9df7ba82})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverStreamControl {
    const IID: ::windows::core::GUID = <IMiracastReceiverStreamControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverStreamControl {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStreamControl";
}
::windows::core::interface_hierarchy!(MiracastReceiverStreamControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverStreamControl {}
unsafe impl ::core::marker::Sync for MiracastReceiverStreamControl {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverVideoStreamSettings(::windows::core::IUnknown);
impl MiracastReceiverVideoStreamSettings {
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn SetSize(&self, value: super::super::Graphics::SizeInt32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Bitrate(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bitrate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBitrate(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBitrate)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MiracastReceiverVideoStreamSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastReceiverVideoStreamSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastReceiverVideoStreamSettings {}
impl ::core::fmt::Debug for MiracastReceiverVideoStreamSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverVideoStreamSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverVideoStreamSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastReceiverVideoStreamSettings;{169b5e1b-149d-52d0-b126-6f89744e4f50})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastReceiverVideoStreamSettings {
    const IID: ::windows::core::GUID = <IMiracastReceiverVideoStreamSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverVideoStreamSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverVideoStreamSettings";
}
::windows::core::interface_hierarchy!(MiracastReceiverVideoStreamSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastReceiverVideoStreamSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverVideoStreamSettings {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastTransmitter(::windows::core::IUnknown);
impl MiracastTransmitter {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AuthorizationStatus(&self) -> ::windows::core::Result<MiracastTransmitterAuthorizationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthorizationStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAuthorizationStatus(&self, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAuthorizationStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConnections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MacAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastConnectionTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LastConnectionTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MiracastTransmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MiracastTransmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MiracastTransmitter {}
impl ::core::fmt::Debug for MiracastTransmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastTransmitter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastTransmitter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Miracast.MiracastTransmitter;{342d79fd-2e64-5508-8a30-833d1eac70d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
}
unsafe impl ::windows::core::Interface for MiracastTransmitter {
    const IID: ::windows::core::GUID = <IMiracastTransmitter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastTransmitter {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastTransmitter";
}
::windows::core::interface_hierarchy!(MiracastTransmitter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MiracastTransmitter {}
unsafe impl ::core::marker::Sync for MiracastTransmitter {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverApplySettingsStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverApplySettingsStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverApplySettingsStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverApplySettingsStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverApplySettingsStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverAuthorizationMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverAuthorizationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverAuthorizationMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverAuthorizationMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverAuthorizationMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverDisconnectReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverDisconnectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverDisconnectReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverDisconnectReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverDisconnectReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverGameControllerDeviceUsageMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverGameControllerDeviceUsageMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverGameControllerDeviceUsageMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverGameControllerDeviceUsageMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverGameControllerDeviceUsageMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverListeningStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverListeningStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverListeningStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverListeningStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverListeningStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverSessionStartStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverSessionStartStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverSessionStartStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverSessionStartStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverSessionStartStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastReceiverWiFiStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastReceiverWiFiStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastReceiverWiFiStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastReceiverWiFiStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastReceiverWiFiStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
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
unsafe impl ::windows::core::Abi for MiracastTransmitterAuthorizationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MiracastTransmitterAuthorizationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MiracastTransmitterAuthorizationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MiracastTransmitterAuthorizationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Miracast.MiracastTransmitterAuthorizationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
