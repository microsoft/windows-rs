#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentSessionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICurrentSessionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICurrentSessionChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentSessionChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SourceAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryGetMediaPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetMediaPropertiesAsync: usize,
    pub GetTimelineProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPlaybackInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryPlayAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPlayAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryStopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryFastForwardAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryFastForwardAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRewindAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRewindAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipNextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipNextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipPreviousAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipPreviousAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeChannelUpAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeChannelUpAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeChannelDownAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeChannelDownAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryTogglePlayPauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryTogglePlayPauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeAutoRepeatModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeAutoRepeatModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangePlaybackRateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackrate: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangePlaybackRateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangeShuffleActiveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedshufflestate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangeShuffleActiveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryChangePlaybackPositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedplaybackposition: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryChangePlaybackPositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TimelinePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimelinePropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimelinePropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackInfoChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackInfoChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MediaPropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaPropertiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaPropertiesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSessionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessions: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentSessionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentSessionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentSessionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentSessionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SessionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2050c4ee_11a0_57de_aed7_c97c70338245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68856cf6_adb4_54b2_ac16_05837907acb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPlaybackPositionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Controls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackType: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatMode: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRate: usize,
    #[cfg(feature = "Foundation")]
    pub IsShuffleActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsShuffleActive: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub MinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub LastUpdatedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastUpdatedTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPropertiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaPropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaPropertiesChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackInfoChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPlaybackInfoChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPlaybackInfoChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786756c2_bc0d_50a5_8807_054291fef139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackInfoChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISessionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISessionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISessionsChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISessionsChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelinePropertiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITimelinePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimelinePropertiesChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29033a2f_c923_5a77_bcaf_055ff415ad32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelinePropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct CurrentSessionChangedEventArgs(::windows::core::IUnknown);
impl CurrentSessionChangedEventArgs {}
impl ::core::cmp::PartialEq for CurrentSessionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentSessionChangedEventArgs {}
impl ::core::fmt::Debug for CurrentSessionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentSessionChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CurrentSessionChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.CurrentSessionChangedEventArgs;{6969cb39-0bfa-5fe0-8d73-09cc5e5408e1})");
}
impl ::core::clone::Clone for CurrentSessionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CurrentSessionChangedEventArgs {
    const IID: ::windows::core::GUID = <ICurrentSessionChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CurrentSessionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
}
::windows::imp::interface_hierarchy!(CurrentSessionChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CurrentSessionChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentSessionChangedEventArgs {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSession(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSession {
    pub fn SourceAppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SourceAppUserModelId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetMediaPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>>();
            (::windows::core::Interface::vtable(this).TryGetMediaPropertiesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetTimelineProperties(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GlobalSystemMediaTransportControlsSessionTimelineProperties>();
            (::windows::core::Interface::vtable(this).GetTimelineProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPlaybackInfo(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GlobalSystemMediaTransportControlsSessionPlaybackInfo>();
            (::windows::core::Interface::vtable(this).GetPlaybackInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPlayAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryPlayAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryPauseAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryStopAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryRecordAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryFastForwardAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryFastForwardAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRewindAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryRewindAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipNextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TrySkipNextAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipPreviousAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TrySkipPreviousAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeChannelUpAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryChangeChannelUpAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeChannelDownAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryChangeChannelDownAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryTogglePlayPauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryTogglePlayPauseAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryChangeAutoRepeatModeAsync)(::windows::core::Interface::as_raw(this), requestedautorepeatmode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryChangePlaybackRateAsync)(::windows::core::Interface::as_raw(this), requestedplaybackrate, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryChangeShuffleActiveAsync)(::windows::core::Interface::as_raw(this), requestedshufflestate, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryChangePlaybackPositionAsync)(::windows::core::Interface::as_raw(this), requestedplaybackposition, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimelinePropertiesChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TimelinePropertiesChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimelinePropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTimelinePropertiesChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackInfoChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PlaybackInfoChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePlaybackInfoChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaPropertiesChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MediaPropertiesChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaPropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMediaPropertiesChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSession {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSession;{7148c835-9b14-5ae2-ab85-dc9b1c14e1a8})");
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GlobalSystemMediaTransportControlsSession {
    const IID: ::windows::core::GUID = <IGlobalSystemMediaTransportControlsSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSession {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
}
::windows::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSession {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSession {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionManager(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionManager {
    pub fn GetCurrentSession(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GlobalSystemMediaTransportControlsSession>();
            (::windows::core::Interface::vtable(this).GetCurrentSession)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>>();
            (::windows::core::Interface::vtable(this).GetSessions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentSessionChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CurrentSessionChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentSessionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCurrentSessionChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionsChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SessionsChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSessionsChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>> {
        Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>>();
            (::windows::core::Interface::vtable(this).RequestAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGlobalSystemMediaTransportControlsSessionManagerStatics<R, F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionManager {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager;{cace8eac-e86e-504a-ab31-5ff8ff1bce49})");
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GlobalSystemMediaTransportControlsSessionManager {
    const IID: ::windows::core::GUID = <IGlobalSystemMediaTransportControlsSessionManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
}
::windows::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionManager {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionManager {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionMediaProperties {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Subtitle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AlbumArtist)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Artist)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AlbumTitle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrackNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).TrackNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Genres)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumTrackCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).AlbumTrackCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackType(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::MediaPlaybackType>>();
            (::windows::core::Interface::vtable(this).PlaybackType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionMediaProperties {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionMediaProperties").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties;{68856cf6-adb4-54b2-ac16-05837907acb6})");
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const IID: ::windows::core::GUID = <IGlobalSystemMediaTransportControlsSessionMediaProperties as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
}
::windows::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionMediaProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
    pub fn IsPlayEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPlayEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPauseEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPauseEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsStopEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsStopEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRecordEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRecordEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsFastForwardEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRewindEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRewindEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsNextEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsNextEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPreviousEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPreviousEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsChannelUpEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsChannelUpEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsChannelDownEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsChannelDownEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPlayPauseToggleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPlayPauseToggleEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsShuffleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsShuffleEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRepeatEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRepeatEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPlaybackRateEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPlaybackRateEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsPlaybackPositionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPlaybackPositionEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackControls").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls;{6501a3e6-bc7a-503a-bb1b-68f158f3fb03})");
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const IID: ::windows::core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
}
::windows::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackControls, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    pub fn Controls(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GlobalSystemMediaTransportControlsSessionPlaybackControls>();
            (::windows::core::Interface::vtable(this).Controls)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PlaybackStatus(&self) -> ::windows::core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GlobalSystemMediaTransportControlsSessionPlaybackStatus>();
            (::windows::core::Interface::vtable(this).PlaybackStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackType(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::MediaPlaybackType>>();
            (::windows::core::Interface::vtable(this).PlaybackType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>>();
            (::windows::core::Interface::vtable(this).AutoRepeatMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<f64>>();
            (::windows::core::Interface::vtable(this).PlaybackRate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsShuffleActive(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<bool>>();
            (::windows::core::Interface::vtable(this).IsShuffleActive)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo;{94b4b6cf-e8ba-51ad-87a7-c10ade106127})");
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const IID: ::windows::core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
}
::windows::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(::windows::core::IUnknown);
impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).EndTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSeekTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).MinSeekTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSeekTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).MaxSeekTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Position)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).LastUpdatedTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionTimelineProperties").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties;{ede34136-6f25-588d-8ecf-ea5b6735aaa5})");
}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const IID: ::windows::core::GUID = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
}
::windows::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionTimelineProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct MediaPropertiesChangedEventArgs(::windows::core::IUnknown);
impl MediaPropertiesChangedEventArgs {}
impl ::core::cmp::PartialEq for MediaPropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPropertiesChangedEventArgs {}
impl ::core::fmt::Debug for MediaPropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPropertiesChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaPropertiesChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.MediaPropertiesChangedEventArgs;{7d3741cb-adf0-5cef-91ba-cfabcdd77678})");
}
impl ::core::clone::Clone for MediaPropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MediaPropertiesChangedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPropertiesChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MediaPropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
}
::windows::imp::interface_hierarchy!(MediaPropertiesChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPropertiesChangedEventArgs {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct PlaybackInfoChangedEventArgs(::windows::core::IUnknown);
impl PlaybackInfoChangedEventArgs {}
impl ::core::cmp::PartialEq for PlaybackInfoChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackInfoChangedEventArgs {}
impl ::core::fmt::Debug for PlaybackInfoChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackInfoChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PlaybackInfoChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.PlaybackInfoChangedEventArgs;{786756c2-bc0d-50a5-8807-054291fef139})");
}
impl ::core::clone::Clone for PlaybackInfoChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PlaybackInfoChangedEventArgs {
    const IID: ::windows::core::GUID = <IPlaybackInfoChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
}
::windows::imp::interface_hierarchy!(PlaybackInfoChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PlaybackInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackInfoChangedEventArgs {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct SessionsChangedEventArgs(::windows::core::IUnknown);
impl SessionsChangedEventArgs {}
impl ::core::cmp::PartialEq for SessionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SessionsChangedEventArgs {}
impl ::core::fmt::Debug for SessionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SessionsChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SessionsChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.SessionsChangedEventArgs;{bbf0cd32-42c4-5a58-b317-f34bbfbd26e0})");
}
impl ::core::clone::Clone for SessionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SessionsChangedEventArgs {
    const IID: ::windows::core::GUID = <ISessionsChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SessionsChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
}
::windows::imp::interface_hierarchy!(SessionsChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SessionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for SessionsChangedEventArgs {}
#[doc = "*Required features: `\"Media_Control\"`*"]
#[repr(transparent)]
pub struct TimelinePropertiesChangedEventArgs(::windows::core::IUnknown);
impl TimelinePropertiesChangedEventArgs {}
impl ::core::cmp::PartialEq for TimelinePropertiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimelinePropertiesChangedEventArgs {}
impl ::core::fmt::Debug for TimelinePropertiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelinePropertiesChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TimelinePropertiesChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Control.TimelinePropertiesChangedEventArgs;{29033a2f-c923-5a77-bcaf-055ff415ad32})");
}
impl ::core::clone::Clone for TimelinePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TimelinePropertiesChangedEventArgs {
    const IID: ::windows::core::GUID = <ITimelinePropertiesChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TimelinePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
}
::windows::imp::interface_hierarchy!(TimelinePropertiesChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TimelinePropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimelinePropertiesChangedEventArgs {}
#[doc = "*Required features: `\"Media_Control\"`*"]
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
impl ::windows::core::TypeKind for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GlobalSystemMediaTransportControlsSessionPlaybackStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
