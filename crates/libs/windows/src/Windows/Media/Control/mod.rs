::windows_core::imp::com_interface!(ICurrentSessionChangedEventArgs, ICurrentSessionChangedEventArgs_Vtbl, 0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
#[repr(C)]
pub struct ICurrentSessionChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSession, IGlobalSystemMediaTransportControlsSession_Vtbl, 0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourceAppUserModelId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryGetMediaPropertiesAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTimelineProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPlaybackInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryPlayAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryPauseAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryStopAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryRecordAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryFastForwardAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryRewindAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TrySkipNextAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TrySkipPreviousAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryChangeChannelUpAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryChangeChannelDownAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryTogglePlayPauseAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryChangeAutoRepeatModeAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::MediaPlaybackAutoRepeatMode, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryChangePlaybackRateAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, f64, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryChangeShuffleActiveAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryChangePlaybackPositionAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, i64, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TimelinePropertiesChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub PlaybackInfoChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePlaybackInfoChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaPropertiesChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManager_Vtbl, 0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetCurrentSession: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSessions: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSessions: usize,
    pub CurrentSessionChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCurrentSessionChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SessionsChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSessionsChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSessionManagerStatics, IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl, 0x2050c4ee_11a0_57de_aed7_c97c70338245);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSessionMediaProperties, IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl, 0x68856cf6_adb4_54b2_ac16_05837907acb6);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
    pub AlbumTrackCount: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    pub PlaybackType: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSessionPlaybackControls, IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl, 0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPlayEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsShuffleEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPlaybackPositionEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSessionPlaybackInfo, IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl, 0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Controls: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PlaybackStatus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> ::windows_core::HRESULT,
    pub PlaybackType: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsShuffleActive: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IGlobalSystemMediaTransportControlsSessionTimelineProperties, IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl, 0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
#[repr(C)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MinSeekTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub MaxSeekTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub LastUpdatedTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaPropertiesChangedEventArgs, IMediaPropertiesChangedEventArgs_Vtbl, 0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
#[repr(C)]
pub struct IMediaPropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
::windows_core::imp::com_interface!(IPlaybackInfoChangedEventArgs, IPlaybackInfoChangedEventArgs_Vtbl, 0x786756c2_bc0d_50a5_8807_054291fef139);
#[repr(C)]
pub struct IPlaybackInfoChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
::windows_core::imp::com_interface!(ISessionsChangedEventArgs, ISessionsChangedEventArgs_Vtbl, 0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
#[repr(C)]
pub struct ISessionsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
::windows_core::imp::com_interface!(ITimelinePropertiesChangedEventArgs, ITimelinePropertiesChangedEventArgs_Vtbl, 0x29033a2f_c923_5a77_bcaf_055ff415ad32);
#[repr(C)]
pub struct ITimelinePropertiesChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CurrentSessionChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(CurrentSessionChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl CurrentSessionChangedEventArgs {}
impl ::windows_core::RuntimeType for CurrentSessionChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CurrentSessionChangedEventArgs {
    type Vtable = ICurrentSessionChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICurrentSessionChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CurrentSessionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
}
unsafe impl ::core::marker::Send for CurrentSessionChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentSessionChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSession(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSession {
    pub fn SourceAppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppUserModelId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetMediaPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryGetMediaPropertiesAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTimelineProperties(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTimelineProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPlaybackInfo(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaybackInfo)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryPlayAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryPlayAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryPauseAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryPauseAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryStopAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryStopAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRecordAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRecordAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryFastForwardAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryFastForwardAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRewindAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRewindAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySkipNextAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipNextAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySkipPreviousAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrySkipPreviousAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeChannelUpAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeChannelUpAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeChannelDownAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeChannelDownAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryTogglePlayPauseAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryTogglePlayPauseAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeAutoRepeatModeAsync)(::windows_core::Interface::as_raw(this), requestedautorepeatmode, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangePlaybackRateAsync)(::windows_core::Interface::as_raw(this), requestedplaybackrate, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangeShuffleActiveAsync)(::windows_core::Interface::as_raw(this), requestedshufflestate, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryChangePlaybackPositionAsync)(::windows_core::Interface::as_raw(this), requestedplaybackposition, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimelinePropertiesChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimelinePropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTimelinePropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTimelinePropertiesChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PlaybackInfoChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackInfoChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackInfoChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaPropertiesChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaPropertiesChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaPropertiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMediaPropertiesChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSession {
    type Vtable = IGlobalSystemMediaTransportControlsSession_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSession {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSession {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionManager(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionManager {
    pub fn GetCurrentSession(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentSession)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSessions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSessions)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentSessionChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentSessionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCurrentSessionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCurrentSessionChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SessionsChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionsChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSessionsChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>> {
        Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IGlobalSystemMediaTransportControlsSessionManagerStatics<R, F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionManager {
    type Vtable = IGlobalSystemMediaTransportControlsSessionManager_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionManager {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionMediaProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionMediaProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AlbumArtist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArtist)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AlbumTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTitle)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackNumber)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AlbumTrackCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTrackCount)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PlaybackType(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackType)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionMediaProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackControls, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
    pub fn IsPlayEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPauseEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPauseEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsStopEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStopEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRecordEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRecordEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsFastForwardEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFastForwardEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRewindEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRewindEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsNextEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNextEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPreviousEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviousEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsChannelUpEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelUpEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsChannelDownEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelDownEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlayPauseToggleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayPauseToggleEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRepeatEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRepeatEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlaybackRateEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackRateEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlaybackPositionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaybackPositionEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    pub fn Controls(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Controls)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PlaybackType(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackType)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoRepeatMode(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatMode)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlaybackRate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsShuffleActive(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsShuffleActive)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionTimelineProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinSeekTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinSeekTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxSeekTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSeekTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LastUpdatedTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdatedTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl;
    const IID: ::windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
}
unsafe impl ::core::marker::Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
unsafe impl ::core::marker::Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaPropertiesChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaPropertiesChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaPropertiesChangedEventArgs {}
impl ::windows_core::RuntimeType for MediaPropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaPropertiesChangedEventArgs {
    type Vtable = IMediaPropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaPropertiesChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaPropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
}
unsafe impl ::core::marker::Send for MediaPropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPropertiesChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlaybackInfoChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(PlaybackInfoChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl PlaybackInfoChangedEventArgs {}
impl ::windows_core::RuntimeType for PlaybackInfoChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for PlaybackInfoChangedEventArgs {
    type Vtable = IPlaybackInfoChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPlaybackInfoChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
}
unsafe impl ::core::marker::Send for PlaybackInfoChangedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SessionsChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SessionsChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl SessionsChangedEventArgs {}
impl ::windows_core::RuntimeType for SessionsChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SessionsChangedEventArgs {
    type Vtable = ISessionsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISessionsChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SessionsChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
}
unsafe impl ::core::marker::Send for SessionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for SessionsChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimelinePropertiesChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimelinePropertiesChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimelinePropertiesChangedEventArgs {}
impl ::windows_core::RuntimeType for TimelinePropertiesChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimelinePropertiesChangedEventArgs {
    type Vtable = ITimelinePropertiesChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimelinePropertiesChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimelinePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
}
unsafe impl ::core::marker::Send for TimelinePropertiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimelinePropertiesChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
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
