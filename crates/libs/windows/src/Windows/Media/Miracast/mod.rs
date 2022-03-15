#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiver(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a315258_e444_51b4_aff7_b88daa1229e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiver_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefaultSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSettingsAsync: usize,
    pub DisconnectAllAndApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DisconnectAllAndApplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAllAndApplySettingsAsync: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub CreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    CreateSession: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    CreateSessionAsync: usize,
    pub ClearKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveKnownTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transmitter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverApplySettingsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0aa6272_09cd_58e1_a4f2_5d5143d312f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverApplySettingsResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverApplySettingsStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x704b2f36_d2e5_551f_a854_f822b7917d28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverConnection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason) -> ::windows::core::HRESULT,
    pub DisconnectWithMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: MiracastReceiverDisconnectReason, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseAsync: usize,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
    pub Transmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InputDevices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CursorImageChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StreamControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverConnectionCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d8dfa39_307a_5c0f_94bd_d0c69d169982);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverConnectionCreatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9ac332d_723a_5a9d_b90a_81153efa2a0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverCursorImageChannel_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    pub ImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImageStream: usize,
    #[cfg(feature = "Foundation")]
    pub ImageStreamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImageStreamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImageStreamChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IMiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccdbedff_bd00_5b9c_8e4c_00cacf86b634);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverCursorImageChannelSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IMiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9a15e5e_5fee_57e6_b4b0_04727db93229);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverDisconnectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverGameControllerDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d7171e8_bed4_5118_a058_e2477eb5888d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverGameControllerDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IMiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda35bb02_28aa_5ee8_96f5_a42901c66f00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverInputDevices_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Keyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverKeyboardDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb67272_06c0_54ff_ac96_217464ff2501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverKeyboardDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTransmitInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRequestedByTransmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransmittingInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IMiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17cf519e_1246_531d_945a_6b158e39c3aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    pub CursorImageChannelSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d2bcdb4_ef8b_5209_bfc9_c32116504803);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ConnectionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionCreated: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCreated: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverSessionStartResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7c573ee_40ca_51ff_95f2_c9de34f2e90e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSessionStartResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverSessionStartStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57cd2f24_c55a_5fbe_9464_eb05307705dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AuthorizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT,
    pub SetAuthorizationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::HRESULT,
    pub RequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRequireAuthorizationFromKnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc28a5591_23ab_519e_ad09_90bff6dcc87e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ListeningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverListeningStatus) -> ::windows::core::HRESULT,
    pub WiFiStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastReceiverWiFiStatus) -> ::windows::core::HRESULT,
    pub IsConnectionTakeoverSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxSimultaneousConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub KnownTransmitters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    KnownTransmitters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverStreamControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ea2d8b_2769_5ad7_8a8a_254b9df7ba82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverStreamControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetVideoStreamSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVideoStreamSettingsAsync: usize,
    pub SuggestVideoStreamSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SuggestVideoStreamSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestVideoStreamSettingsAsync: usize,
    pub MuteAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMuteAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMiracastReceiverVideoStreamSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x169b5e1b_149d_52d0_b126_6f89744e4f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastReceiverVideoStreamSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IMiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342d79fd_2e64_5508_8a30_833d1eac70d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMiracastTransmitter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AuthorizationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT,
    pub SetAuthorizationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnections: usize,
    pub MacAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MiracastReceiver, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn GetDefaultSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn GetCurrentSettings(&self) -> ::windows::core::Result<MiracastReceiverSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSettingsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MiracastReceiverSettings>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn DisconnectAllAndApplySettings<'a, Param0: ::windows::core::IntoParam<'a, MiracastReceiverSettings>>(&self, settings: Param0) -> ::windows::core::Result<MiracastReceiverApplySettingsResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisconnectAllAndApplySettings)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<MiracastReceiverApplySettingsResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectAllAndApplySettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, MiracastReceiverSettings>>(&self, settings: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisconnectAllAndApplySettingsAsync)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MiracastReceiverApplySettingsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn GetStatus(&self) -> ::windows::core::Result<MiracastReceiverStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatusAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MiracastReceiverStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"ApplicationModel_Core\"`*"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::CoreApplicationView>>(&self, view: Param0) -> ::windows::core::Result<MiracastReceiverSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSession)(::core::mem::transmute_copy(this), view.into_param().abi(), &mut result__).from_abi::<MiracastReceiverSession>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn CreateSessionAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::ApplicationModel::Core::CoreApplicationView>>(&self, view: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateSessionAsync)(::core::mem::transmute_copy(this), view.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MiracastReceiverSession>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn ClearKnownTransmitters(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearKnownTransmitters)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn RemoveKnownTransmitter<'a, Param0: ::windows::core::IntoParam<'a, MiracastTransmitter>>(&self, transmitter: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveKnownTransmitter)(::core::mem::transmute_copy(this), transmitter.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiver {
    type Vtable = IMiracastReceiver_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiver {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiver";
}
impl ::core::convert::From<MiracastReceiver> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiver> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiver> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiver> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiver {}
unsafe impl ::core::marker::Sync for MiracastReceiver {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsResult(::windows::core::IUnknown);
impl MiracastReceiverApplySettingsResult {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MiracastReceiverApplySettingsStatus> {
        let this = self;
        unsafe {
            let mut result__: MiracastReceiverApplySettingsStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverApplySettingsStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverApplySettingsResult {
    type Vtable = IMiracastReceiverApplySettingsResult_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverApplySettingsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverApplySettingsResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverApplySettingsResult";
}
impl ::core::convert::From<MiracastReceiverApplySettingsResult> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverApplySettingsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverApplySettingsResult> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverApplySettingsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverApplySettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverApplySettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverApplySettingsResult> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverApplySettingsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverApplySettingsResult> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverApplySettingsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverApplySettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverApplySettingsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverApplySettingsResult {}
unsafe impl ::core::marker::Sync for MiracastReceiverApplySettingsResult {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct MiracastReceiverConnection(::windows::core::IUnknown);
impl MiracastReceiverConnection {
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Disconnect(&self, reason: MiracastReceiverDisconnectReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Disconnect)(::core::mem::transmute_copy(this), reason).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn DisconnectWithMessage<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reason: MiracastReceiverDisconnectReason, message: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisconnectWithMessage)(::core::mem::transmute_copy(this), reason, message.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Pause)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PauseAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Resume)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Transmitter(&self) -> ::windows::core::Result<MiracastTransmitter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Transmitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastTransmitter>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn InputDevices(&self) -> ::windows::core::Result<MiracastReceiverInputDevices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputDevices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverInputDevices>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn CursorImageChannel(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CursorImageChannel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverCursorImageChannel>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn StreamControl(&self) -> ::windows::core::Result<MiracastReceiverStreamControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamControl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverStreamControl>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverConnection {
    type Vtable = IMiracastReceiverConnection_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverConnection {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnection";
}
impl ::core::convert::From<MiracastReceiverConnection> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverConnection> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverConnection> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverConnection> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MiracastReceiverConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MiracastReceiverConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverConnection {}
unsafe impl ::core::marker::Sync for MiracastReceiverConnection {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverConnectionCreatedEventArgs(::windows::core::IUnknown);
impl MiracastReceiverConnectionCreatedEventArgs {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Connection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverConnection>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverConnectionCreatedEventArgs {
    type Vtable = IMiracastReceiverConnectionCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverConnectionCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverConnectionCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverConnectionCreatedEventArgs";
}
impl ::core::convert::From<MiracastReceiverConnectionCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverConnectionCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverConnectionCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverConnectionCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverConnectionCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverConnectionCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverConnectionCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverConnectionCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverConnectionCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverConnectionCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverConnectionCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverConnectionCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverConnectionCreatedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverConnectionCreatedEventArgs {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannel(::windows::core::IUnknown);
impl MiracastReceiverCursorImageChannel {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxImageSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::PointInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::PointInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ImageStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImageStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageStreamChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImageStreamChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveImageStreamChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveImageStreamChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverCursorImageChannel, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePositionChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverCursorImageChannel {
    type Vtable = IMiracastReceiverCursorImageChannel_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverCursorImageChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverCursorImageChannel {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannel";
}
impl ::core::convert::From<MiracastReceiverCursorImageChannel> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverCursorImageChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverCursorImageChannel> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverCursorImageChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverCursorImageChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverCursorImageChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverCursorImageChannel> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverCursorImageChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverCursorImageChannel> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverCursorImageChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverCursorImageChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverCursorImageChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverCursorImageChannel {}
unsafe impl ::core::marker::Sync for MiracastReceiverCursorImageChannel {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannelSettings(::windows::core::IUnknown);
impl MiracastReceiverCursorImageChannelSettings {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn MaxImageSize(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxImageSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn SetMaxImageSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::SizeInt32>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxImageSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverCursorImageChannelSettings {
    type Vtable = IMiracastReceiverCursorImageChannelSettings_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverCursorImageChannelSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverCursorImageChannelSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverCursorImageChannelSettings";
}
impl ::core::convert::From<MiracastReceiverCursorImageChannelSettings> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverCursorImageChannelSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverCursorImageChannelSettings> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverCursorImageChannelSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverCursorImageChannelSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverCursorImageChannelSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverCursorImageChannelSettings> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverCursorImageChannelSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverCursorImageChannelSettings> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverCursorImageChannelSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverCursorImageChannelSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverCursorImageChannelSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverCursorImageChannelSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverCursorImageChannelSettings {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct MiracastReceiverDisconnectedEventArgs(::windows::core::IUnknown);
impl MiracastReceiverDisconnectedEventArgs {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Connection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverConnection>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverDisconnectedEventArgs {
    type Vtable = IMiracastReceiverDisconnectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverDisconnectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverDisconnectedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverDisconnectedEventArgs";
}
impl ::core::convert::From<MiracastReceiverDisconnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverDisconnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverDisconnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverDisconnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverDisconnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverDisconnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverDisconnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverDisconnectedEventArgs {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDevice(::windows::core::IUnknown);
impl MiracastReceiverGameControllerDevice {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn TransmitInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransmitInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTransmitInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequestedByTransmitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsTransmittingInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransmittingInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Mode(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDeviceUsageMode> {
        let this = self;
        unsafe {
            let mut result__: MiracastReceiverGameControllerDeviceUsageMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverGameControllerDeviceUsageMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetMode(&self, value: MiracastReceiverGameControllerDeviceUsageMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverGameControllerDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverGameControllerDevice {
    type Vtable = IMiracastReceiverGameControllerDevice_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverGameControllerDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverGameControllerDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverGameControllerDevice";
}
impl ::core::convert::From<MiracastReceiverGameControllerDevice> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverGameControllerDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverGameControllerDevice> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverGameControllerDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverGameControllerDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverGameControllerDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverGameControllerDevice> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverGameControllerDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverGameControllerDevice> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverGameControllerDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverGameControllerDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverGameControllerDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverGameControllerDevice {}
unsafe impl ::core::marker::Sync for MiracastReceiverGameControllerDevice {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct MiracastReceiverInputDevices(::windows::core::IUnknown);
impl MiracastReceiverInputDevices {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Keyboard(&self) -> ::windows::core::Result<MiracastReceiverKeyboardDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Keyboard)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverKeyboardDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn GameController(&self) -> ::windows::core::Result<MiracastReceiverGameControllerDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GameController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverGameControllerDevice>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverInputDevices {
    type Vtable = IMiracastReceiverInputDevices_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverInputDevices as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverInputDevices {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverInputDevices";
}
impl ::core::convert::From<MiracastReceiverInputDevices> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverInputDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverInputDevices> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverInputDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverInputDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverInputDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverInputDevices> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverInputDevices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverInputDevices> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverInputDevices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverInputDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverInputDevices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverInputDevices {}
unsafe impl ::core::marker::Sync for MiracastReceiverInputDevices {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverKeyboardDevice(::windows::core::IUnknown);
impl MiracastReceiverKeyboardDevice {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn TransmitInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TransmitInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetTransmitInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTransmitInput)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsRequestedByTransmitter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRequestedByTransmitter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsTransmittingInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransmittingInput)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverKeyboardDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverKeyboardDevice {
    type Vtable = IMiracastReceiverKeyboardDevice_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverKeyboardDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverKeyboardDevice {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverKeyboardDevice";
}
impl ::core::convert::From<MiracastReceiverKeyboardDevice> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverKeyboardDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverKeyboardDevice> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverKeyboardDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverKeyboardDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverKeyboardDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverKeyboardDevice> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverKeyboardDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverKeyboardDevice> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverKeyboardDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverKeyboardDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverKeyboardDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverKeyboardDevice {}
unsafe impl ::core::marker::Sync for MiracastReceiverKeyboardDevice {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct MiracastReceiverMediaSourceCreatedEventArgs(::windows::core::IUnknown);
impl MiracastReceiverMediaSourceCreatedEventArgs {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Connection(&self) -> ::windows::core::Result<MiracastReceiverConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Connection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverConnection>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::MediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn CursorImageChannelSettings(&self) -> ::windows::core::Result<MiracastReceiverCursorImageChannelSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CursorImageChannelSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverCursorImageChannelSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverMediaSourceCreatedEventArgs {
    type Vtable = IMiracastReceiverMediaSourceCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverMediaSourceCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverMediaSourceCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverMediaSourceCreatedEventArgs";
}
impl ::core::convert::From<MiracastReceiverMediaSourceCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverMediaSourceCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverMediaSourceCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverMediaSourceCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverMediaSourceCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverMediaSourceCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverMediaSourceCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverMediaSourceCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverMediaSourceCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverMediaSourceCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverMediaSourceCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverMediaSourceCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverMediaSourceCreatedEventArgs {}
unsafe impl ::core::marker::Sync for MiracastReceiverMediaSourceCreatedEventArgs {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverSession(::windows::core::IUnknown);
impl MiracastReceiverSession {
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverConnectionCreatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionCreated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConnectionCreated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverMediaSourceCreatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaSourceCreated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMediaSourceCreated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Disconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MiracastReceiverSession, MiracastReceiverDisconnectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Disconnected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDisconnected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn AllowConnectionTakeover(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowConnectionTakeover)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetAllowConnectionTakeover(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowConnectionTakeover)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxSimultaneousConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetMaxSimultaneousConnections(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxSimultaneousConnections)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<MiracastReceiverSessionStartResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverSessionStartResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MiracastReceiverSessionStartResult>>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverSession {
    type Vtable = IMiracastReceiverSession_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverSession {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSession";
}
impl ::core::convert::From<MiracastReceiverSession> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverSession> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverSession> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverSession> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MiracastReceiverSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MiracastReceiverSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverSession {}
unsafe impl ::core::marker::Sync for MiracastReceiverSession {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverSessionStartResult(::windows::core::IUnknown);
impl MiracastReceiverSessionStartResult {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MiracastReceiverSessionStartStatus> {
        let this = self;
        unsafe {
            let mut result__: MiracastReceiverSessionStartStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverSessionStartStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverSessionStartResult {
    type Vtable = IMiracastReceiverSessionStartResult_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverSessionStartResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverSessionStartResult {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSessionStartResult";
}
impl ::core::convert::From<MiracastReceiverSessionStartResult> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverSessionStartResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverSessionStartResult> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverSessionStartResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverSessionStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverSessionStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverSessionStartResult> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverSessionStartResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverSessionStartResult> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverSessionStartResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverSessionStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverSessionStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverSessionStartResult {}
unsafe impl ::core::marker::Sync for MiracastReceiverSessionStartResult {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct MiracastReceiverSettings(::windows::core::IUnknown);
impl MiracastReceiverSettings {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FriendlyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFriendlyName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModelName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetModelName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModelName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModelNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetModelNumber<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModelNumber)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn AuthorizationMethod(&self) -> ::windows::core::Result<MiracastReceiverAuthorizationMethod> {
        let this = self;
        unsafe {
            let mut result__: MiracastReceiverAuthorizationMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthorizationMethod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverAuthorizationMethod>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetAuthorizationMethod(&self, value: MiracastReceiverAuthorizationMethod) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuthorizationMethod)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn RequireAuthorizationFromKnownTransmitters(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequireAuthorizationFromKnownTransmitters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetRequireAuthorizationFromKnownTransmitters(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequireAuthorizationFromKnownTransmitters)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverSettings {
    type Vtable = IMiracastReceiverSettings_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverSettings";
}
impl ::core::convert::From<MiracastReceiverSettings> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverSettings> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverSettings> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverSettings> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverSettings {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverStatus(::windows::core::IUnknown);
impl MiracastReceiverStatus {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn ListeningStatus(&self) -> ::windows::core::Result<MiracastReceiverListeningStatus> {
        let this = self;
        unsafe {
            let mut result__: MiracastReceiverListeningStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListeningStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverListeningStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn WiFiStatus(&self) -> ::windows::core::Result<MiracastReceiverWiFiStatus> {
        let this = self;
        unsafe {
            let mut result__: MiracastReceiverWiFiStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WiFiStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverWiFiStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn IsConnectionTakeoverSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsConnectionTakeoverSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn MaxSimultaneousConnections(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxSimultaneousConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn KnownTransmitters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KnownTransmitters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MiracastTransmitter>>(result__)
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
unsafe impl ::windows::core::Interface for MiracastReceiverStatus {
    type Vtable = IMiracastReceiverStatus_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverStatus {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStatus";
}
impl ::core::convert::From<MiracastReceiverStatus> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverStatus> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverStatus> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverStatus> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverStatus {}
unsafe impl ::core::marker::Sync for MiracastReceiverStatus {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverStreamControl(::windows::core::IUnknown);
impl MiracastReceiverStreamControl {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn GetVideoStreamSettings(&self) -> ::windows::core::Result<MiracastReceiverVideoStreamSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVideoStreamSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastReceiverVideoStreamSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVideoStreamSettingsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetVideoStreamSettingsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MiracastReceiverVideoStreamSettings>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SuggestVideoStreamSettings<'a, Param0: ::windows::core::IntoParam<'a, MiracastReceiverVideoStreamSettings>>(&self, settings: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SuggestVideoStreamSettings)(::core::mem::transmute_copy(this), settings.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SuggestVideoStreamSettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, MiracastReceiverVideoStreamSettings>>(&self, settings: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SuggestVideoStreamSettingsAsync)(::core::mem::transmute_copy(this), settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn MuteAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MuteAudio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetMuteAudio(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMuteAudio)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverStreamControl {
    type Vtable = IMiracastReceiverStreamControl_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverStreamControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverStreamControl {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverStreamControl";
}
impl ::core::convert::From<MiracastReceiverStreamControl> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverStreamControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverStreamControl> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverStreamControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverStreamControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverStreamControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverStreamControl> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverStreamControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverStreamControl> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverStreamControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverStreamControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverStreamControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverStreamControl {}
unsafe impl ::core::marker::Sync for MiracastReceiverStreamControl {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
pub struct MiracastReceiverVideoStreamSettings(::windows::core::IUnknown);
impl MiracastReceiverVideoStreamSettings {
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::SizeInt32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::SizeInt32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Graphics\"`*"]
    #[cfg(feature = "Graphics")]
    pub fn SetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::SizeInt32>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Bitrate(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetBitrate(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitrate)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for MiracastReceiverVideoStreamSettings {
    type Vtable = IMiracastReceiverVideoStreamSettings_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastReceiverVideoStreamSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastReceiverVideoStreamSettings {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastReceiverVideoStreamSettings";
}
impl ::core::convert::From<MiracastReceiverVideoStreamSettings> for ::windows::core::IUnknown {
    fn from(value: MiracastReceiverVideoStreamSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverVideoStreamSettings> for ::windows::core::IUnknown {
    fn from(value: &MiracastReceiverVideoStreamSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastReceiverVideoStreamSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastReceiverVideoStreamSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastReceiverVideoStreamSettings> for ::windows::core::IInspectable {
    fn from(value: MiracastReceiverVideoStreamSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastReceiverVideoStreamSettings> for ::windows::core::IInspectable {
    fn from(value: &MiracastReceiverVideoStreamSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastReceiverVideoStreamSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastReceiverVideoStreamSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastReceiverVideoStreamSettings {}
unsafe impl ::core::marker::Sync for MiracastReceiverVideoStreamSettings {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct MiracastTransmitter(::windows::core::IUnknown);
impl MiracastTransmitter {
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn AuthorizationStatus(&self) -> ::windows::core::Result<MiracastTransmitterAuthorizationStatus> {
        let this = self;
        unsafe {
            let mut result__: MiracastTransmitterAuthorizationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthorizationStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MiracastTransmitterAuthorizationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn SetAuthorizationStatus(&self, value: MiracastTransmitterAuthorizationStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuthorizationStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConnections)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MiracastReceiverConnection>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`*"]
    pub fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MacAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Miracast\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastConnectionTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastConnectionTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
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
unsafe impl ::windows::core::Interface for MiracastTransmitter {
    type Vtable = IMiracastTransmitter_Vtbl;
    const IID: ::windows::core::GUID = <IMiracastTransmitter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MiracastTransmitter {
    const NAME: &'static str = "Windows.Media.Miracast.MiracastTransmitter";
}
impl ::core::convert::From<MiracastTransmitter> for ::windows::core::IUnknown {
    fn from(value: MiracastTransmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastTransmitter> for ::windows::core::IUnknown {
    fn from(value: &MiracastTransmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MiracastTransmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MiracastTransmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MiracastTransmitter> for ::windows::core::IInspectable {
    fn from(value: MiracastTransmitter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MiracastTransmitter> for ::windows::core::IInspectable {
    fn from(value: &MiracastTransmitter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MiracastTransmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MiracastTransmitter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MiracastTransmitter {}
unsafe impl ::core::marker::Sync for MiracastTransmitter {}
#[doc = "*Required features: `\"Media_Miracast\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
