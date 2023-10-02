#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICurrentSessionChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICurrentSessionChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentSessionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourceAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetMediaPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetMediaPropertiesAsync: usize,
    pub GetTimelineProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPlaybackInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryPlayAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPlayAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryStopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryFastForwardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryFastForwardAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRewindAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRewindAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipNextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipNextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipPreviousAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipPreviousAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeChannelUpAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeChannelUpAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeChannelDownAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeChannelDownAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTogglePlayPauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTogglePlayPauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeAutoRepeatModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeAutoRepeatModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangePlaybackRateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackrate: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangePlaybackRateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeShuffleActiveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedshufflestate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeShuffleActiveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangePlaybackPositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackposition: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangePlaybackPositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TimelinePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimelinePropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimelinePropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MediaPropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaPropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaPropertiesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSessionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSessionManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrentSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessions: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentSessionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentSessionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentSessionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentSessionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SessionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2050c4ee_11a0_57de_aed7_c97c70338245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68856cf6_adb4_54b2_ac16_05837907acb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaybackPositionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Controls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackType: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatMode: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRate: usize,
    #[cfg(feature = "Foundation")]
    pub IsShuffleActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsShuffleActive: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub MinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaPropertiesChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaPropertiesChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlaybackInfoChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlaybackInfoChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x786756c2_bc0d_50a5_8807_054291fef139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackInfoChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISessionsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISessionsChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISessionsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITimelinePropertiesChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITimelinePropertiesChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29033a2f_c923_5a77_bcaf_055ff415ad32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelinePropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CurrentSessionChangedEventArgs(::windows_core::IUnknown);
impl CurrentSessionChangedEventArgs {}
impl ::windows_core::RuntimeType for CurrentSessionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.CurrentSessionChangedEventArgs;{6969cb39-0bfa-5fe0-8d73-09cc5e5408e1})");
}
unsafe impl ::windows_core::Interface for CurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CurrentSessionChangedEventArgs {
    const IID: ::windows_core::GUID = <ICurrentSessionChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CurrentSessionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CurrentSessionChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CurrentSessionChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentSessionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSession(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSession {
    pub fn SourceAppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppUserModelId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetMediaPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetMediaPropertiesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetTimelineProperties(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTimelineProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPlaybackInfo(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaybackInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryPlayAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryPlayAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryPauseAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryPauseAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryStopAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryStopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryRecordAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRecordAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryFastForwardAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryFastForwardAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryRewindAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRewindAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipNextAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipNextAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipPreviousAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipPreviousAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeChannelUpAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeChannelUpAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeChannelDownAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeChannelDownAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryTogglePlayPauseAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTogglePlayPauseAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeAutoRepeatModeAsync)(::windows_core::Interface::as_raw(this), requestedautorepeatmode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangePlaybackRateAsync)(::windows_core::Interface::as_raw(this), requestedplaybackrate, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeShuffleActiveAsync)(::windows_core::Interface::as_raw(this), requestedshufflestate, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangePlaybackPositionAsync)(::windows_core::Interface::as_raw(this), requestedplaybackposition, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimelinePropertiesChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimelinePropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimelinePropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTimelinePropertiesChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackInfoChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackInfoChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackInfoChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MediaPropertiesChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaPropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaPropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaPropertiesChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSession;{7148c835-9b14-5ae2-ab85-dc9b1c14e1a8})");
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalSystemMediaTransportControlsSession {
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSession {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
}
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSession {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionManager(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionManager {
    pub fn GetCurrentSession(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSessions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentSessionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSessionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentSessionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentSessionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SessionsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSessionsChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>> {
        Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGlobalSystemMediaTransportControlsSessionManagerStatics<R, F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager;{cace8eac-e86e-504a-ab31-5ff8ff1bce49})");
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalSystemMediaTransportControlsSessionManager {
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
}
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionManager {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionMediaProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumArtist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArtist)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrackNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumTrackCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTrackCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackType(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties;{68856cf6-adb4-54b2-ac16-05837907acb6})");
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionMediaProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
}
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionMediaProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
    pub fn IsPlayEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPauseEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPauseEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsStopEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStopEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRecordEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRecordEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFastForwardEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFastForwardEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRewindEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRewindEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsNextEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNextEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPreviousEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviousEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsChannelUpEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelUpEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsChannelDownEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelDownEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPlayPauseToggleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayPauseToggleEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRepeatEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRepeatEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPlaybackRateEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackRateEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPlaybackPositionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackPositionEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls;{6501a3e6-bc7a-503a-bb1b-68f158f3fb03})");
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
}
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackControls, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    pub fn Controls(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Controls)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackType(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatMode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsShuffleActive(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleActive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo;{94b4b6cf-e8ba-51ad-87a7-c10ade106127})");
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
}
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(::windows_core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MinSeekTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinSeekTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSeekTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSeekTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties;{ede34136-6f25-588d-8ecf-ea5b6735aaa5})");
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
}
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionTimelineProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaPropertiesChangedEventArgs(::windows_core::IUnknown);
impl MediaPropertiesChangedEventArgs {}
impl ::windows_core::RuntimeType for MediaPropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.MediaPropertiesChangedEventArgs;{7d3741cb-adf0-5cef-91ba-cfabcdd77678})");
}
unsafe impl ::windows_core::Interface for MediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaPropertiesChangedEventArgs {
    const IID: ::windows_core::GUID = <IMediaPropertiesChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaPropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaPropertiesChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaPropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPropertiesChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlaybackInfoChangedEventArgs(::windows_core::IUnknown);
impl PlaybackInfoChangedEventArgs {}
impl ::windows_core::RuntimeType for PlaybackInfoChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.PlaybackInfoChangedEventArgs;{786756c2-bc0d-50a5-8807-054291fef139})");
}
unsafe impl ::windows_core::Interface for PlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaybackInfoChangedEventArgs {
    const IID: ::windows_core::GUID = <IPlaybackInfoChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PlaybackInfoChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PlaybackInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SessionsChangedEventArgs(::windows_core::IUnknown);
impl SessionsChangedEventArgs {}
impl ::windows_core::RuntimeType for SessionsChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.SessionsChangedEventArgs;{bbf0cd32-42c4-5a58-b317-f34bbfbd26e0})");
}
unsafe impl ::windows_core::Interface for SessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SessionsChangedEventArgs {
    const IID: ::windows_core::GUID = <ISessionsChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SessionsChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SessionsChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SessionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for SessionsChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimelinePropertiesChangedEventArgs(::windows_core::IUnknown);
impl TimelinePropertiesChangedEventArgs {}
impl ::windows_core::RuntimeType for TimelinePropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.TimelinePropertiesChangedEventArgs;{29033a2f-c923-5a77-bcaf-055ff415ad32})");
}
unsafe impl ::windows_core::Interface for TimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimelinePropertiesChangedEventArgs {
    const IID: ::windows_core::GUID = <ITimelinePropertiesChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimelinePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(TimelinePropertiesChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimelinePropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimelinePropertiesChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
}
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackStatus {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus;i4)");
}
