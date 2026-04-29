#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentSessionChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CurrentSessionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CurrentSessionChangedEventArgs {}
impl windows_core::RuntimeType for CurrentSessionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICurrentSessionChangedEventArgs>();
}
unsafe impl windows_core::Interface for CurrentSessionChangedEventArgs {
    type Vtable = <ICurrentSessionChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICurrentSessionChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CurrentSessionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
}
unsafe impl Send for CurrentSessionChangedEventArgs {}
unsafe impl Sync for CurrentSessionChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSession, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSession {
    pub fn SourceAppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SourceAppUserModelId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TryGetMediaPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryGetMediaPropertiesAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetTimelineProperties(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimelineProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetPlaybackInfo(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlaybackInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryPlayAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryPlayAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryPauseAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryPauseAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryStopAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryStopAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRecordAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryRecordAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryFastForwardAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryFastForwardAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryRewindAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryRewindAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySkipNextAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrySkipNextAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySkipPreviousAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrySkipPreviousAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeChannelUpAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryChangeChannelUpAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeChannelDownAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryChangeChannelDownAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryTogglePlayPauseAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryTogglePlayPauseAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeAutoRepeatModeAsync(&self, requestedautorepeatmode: super::MediaPlaybackAutoRepeatMode) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryChangeAutoRepeatModeAsync)(windows_core::Interface::as_raw(self), requestedautorepeatmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangePlaybackRateAsync(&self, requestedplaybackrate: f64) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryChangePlaybackRateAsync)(windows_core::Interface::as_raw(self), requestedplaybackrate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangeShuffleActiveAsync(&self, requestedshufflestate: bool) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryChangeShuffleActiveAsync)(windows_core::Interface::as_raw(self), requestedshufflestate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryChangePlaybackPositionAsync(&self, requestedplaybackposition: i64) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryChangePlaybackPositionAsync)(windows_core::Interface::as_raw(self), requestedplaybackposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimelinePropertiesChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, TimelinePropertiesChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TimelinePropertiesChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTimelinePropertiesChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveTimelinePropertiesChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn PlaybackInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, PlaybackInfoChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlaybackInfoChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePlaybackInfoChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemovePlaybackInfoChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn MediaPropertiesChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSession, MediaPropertiesChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaPropertiesChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMediaPropertiesChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveMediaPropertiesChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSession>();
}
unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSession {
    type Vtable = <IGlobalSystemMediaTransportControlsSession as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSession {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
}
unsafe impl Send for GlobalSystemMediaTransportControlsSession {}
unsafe impl Sync for GlobalSystemMediaTransportControlsSession {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSessionManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionManager, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionManager {
    pub fn GetCurrentSession(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSession)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetSessions(&self) -> windows_core::Result<windows_collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CurrentSessionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, CurrentSessionChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentSessionChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCurrentSessionChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveCurrentSessionChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn SessionsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GlobalSystemMediaTransportControlsSessionManager, SessionsChangedEventArgs>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionsChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSessionsChanged(&self, token: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveSessionsChanged)(windows_core::Interface::as_raw(self), token).ok() }
    }
    pub fn RequestAsync() -> windows_core::Result<windows_future::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>> {
        Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IGlobalSystemMediaTransportControlsSessionManagerStatics<R, F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionManager>();
}
unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionManager {
    type Vtable = <IGlobalSystemMediaTransportControlsSessionManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
}
unsafe impl Send for GlobalSystemMediaTransportControlsSessionManager {}
unsafe impl Sync for GlobalSystemMediaTransportControlsSessionManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionMediaProperties, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionMediaProperties {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subtitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AlbumArtist(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlbumArtist)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Artist(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Artist)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AlbumTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlbumTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Genres(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Genres)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AlbumTrackCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlbumTrackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PlaybackType(&self) -> windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlaybackType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Thumbnail)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionMediaProperties>();
}
unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
    type Vtable = <IGlobalSystemMediaTransportControlsSessionMediaProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionMediaProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
}
unsafe impl Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
unsafe impl Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackControls, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
    pub fn IsPlayEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPlayEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPauseEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPauseEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsStopEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStopEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsRecordEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRecordEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsFastForwardEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFastForwardEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsRewindEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRewindEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsNextEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNextEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPreviousEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPreviousEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsChannelUpEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsChannelUpEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsChannelDownEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsChannelDownEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlayPauseToggleEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPlayPauseToggleEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsShuffleEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsShuffleEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsRepeatEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRepeatEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlaybackRateEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPlaybackRateEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlaybackPositionEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPlaybackPositionEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionPlaybackControls>();
}
unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    type Vtable = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
}
unsafe impl Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
unsafe impl Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionPlaybackInfo, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    pub fn Controls(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Controls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlaybackStatus(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlaybackStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PlaybackType(&self) -> windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackType>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlaybackType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AutoRepeatMode(&self) -> windows_core::Result<super::super::Foundation::IReference<super::MediaPlaybackAutoRepeatMode>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoRepeatMode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PlaybackRate(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlaybackRate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsShuffleActive(&self) -> windows_core::Result<super::super::Foundation::IReference<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsShuffleActive)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionPlaybackInfo>();
}
unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    type Vtable = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
}
unsafe impl Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
unsafe impl Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
}
impl windows_core::TypeKind for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GlobalSystemMediaTransportControlsSessionTimelineProperties, windows_core::IUnknown, windows_core::IInspectable);
impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
    pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MinSeekTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinSeekTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MaxSeekTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxSeekTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn LastUpdatedTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastUpdatedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionTimelineProperties>();
}
unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    type Vtable = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
}
unsafe impl Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
unsafe impl Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
windows_core::imp::define_interface!(ICurrentSessionChangedEventArgs, ICurrentSessionChangedEventArgs_Vtbl, 0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1);
impl windows_core::RuntimeType for ICurrentSessionChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentSessionChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSession, IGlobalSystemMediaTransportControlsSession_Vtbl, 0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceAppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryGetMediaPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTimelineProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPlaybackInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryPlayAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryPauseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryStopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryRecordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryFastForwardAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryRewindAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySkipNextAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySkipPreviousAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryChangeChannelUpAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryChangeChannelDownAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryTogglePlayPauseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryChangeAutoRepeatModeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::MediaPlaybackAutoRepeatMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryChangePlaybackRateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryChangeShuffleActiveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryChangePlaybackPositionAsync: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TimelinePropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PlaybackInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePlaybackInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MediaPropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSessionManager, IGlobalSystemMediaTransportControlsSessionManager_Vtbl, 0xcace8eac_e86e_504a_ab31_5ff8ff1bce49);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentSessionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCurrentSessionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SessionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSessionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSessionManagerStatics, IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl, 0x2050c4ee_11a0_57de_aed7_c97c70338245);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSessionMediaProperties, IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl, 0x68856cf6_adb4_54b2_ac16_05837907acb6);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AlbumTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Genres: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AlbumTrackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PlaybackType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSessionPlaybackControls, IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl, 0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPlayEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRepeatEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPlaybackRateEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPlaybackPositionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSessionPlaybackInfo, IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl, 0x94b4b6cf_e8ba_51ad_87a7_c10ade106127);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Controls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlaybackStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus) -> windows_core::HRESULT,
    pub PlaybackType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsShuffleActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalSystemMediaTransportControlsSessionTimelineProperties, IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl, 0xede34136_6f25_588d_8ecf_ea5b6735aaa5);
impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MinSeekTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub MaxSeekTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub LastUpdatedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPropertiesChangedEventArgs, IMediaPropertiesChangedEventArgs_Vtbl, 0x7d3741cb_adf0_5cef_91ba_cfabcdd77678);
impl windows_core::RuntimeType for IMediaPropertiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPropertiesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPlaybackInfoChangedEventArgs, IPlaybackInfoChangedEventArgs_Vtbl, 0x786756c2_bc0d_50a5_8807_054291fef139);
impl windows_core::RuntimeType for IPlaybackInfoChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackInfoChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISessionsChangedEventArgs, ISessionsChangedEventArgs_Vtbl, 0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0);
impl windows_core::RuntimeType for ISessionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISessionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ITimelinePropertiesChangedEventArgs, ITimelinePropertiesChangedEventArgs_Vtbl, 0x29033a2f_c923_5a77_bcaf_055ff415ad32);
impl windows_core::RuntimeType for ITimelinePropertiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelinePropertiesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPropertiesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPropertiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPropertiesChangedEventArgs {}
impl windows_core::RuntimeType for MediaPropertiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPropertiesChangedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPropertiesChangedEventArgs {
    type Vtable = <IMediaPropertiesChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaPropertiesChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
}
unsafe impl Send for MediaPropertiesChangedEventArgs {}
unsafe impl Sync for MediaPropertiesChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackInfoChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackInfoChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PlaybackInfoChangedEventArgs {}
impl windows_core::RuntimeType for PlaybackInfoChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackInfoChangedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackInfoChangedEventArgs {
    type Vtable = <IPlaybackInfoChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlaybackInfoChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
}
unsafe impl Send for PlaybackInfoChangedEventArgs {}
unsafe impl Sync for PlaybackInfoChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SessionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl SessionsChangedEventArgs {}
impl windows_core::RuntimeType for SessionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISessionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for SessionsChangedEventArgs {
    type Vtable = <ISessionsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISessionsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SessionsChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
}
unsafe impl Send for SessionsChangedEventArgs {}
unsafe impl Sync for SessionsChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelinePropertiesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimelinePropertiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TimelinePropertiesChangedEventArgs {}
impl windows_core::RuntimeType for TimelinePropertiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimelinePropertiesChangedEventArgs>();
}
unsafe impl windows_core::Interface for TimelinePropertiesChangedEventArgs {
    type Vtable = <ITimelinePropertiesChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimelinePropertiesChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimelinePropertiesChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
}
unsafe impl Send for TimelinePropertiesChangedEventArgs {}
unsafe impl Sync for TimelinePropertiesChangedEventArgs {}
