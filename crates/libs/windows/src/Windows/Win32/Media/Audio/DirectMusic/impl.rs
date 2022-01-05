pub trait IDirectMusicImpl: Sized {
    fn EnumPort();
    fn CreateMusicBuffer();
    fn CreatePort();
    fn EnumMasterClock();
    fn GetMasterClock();
    fn SetMasterClock();
    fn Activate();
    fn GetDefaultPort();
    fn SetDirectSound();
}
pub trait IDirectMusic8Impl: Sized + IDirectMusicImpl {
    fn SetExternalMasterClock();
}
pub trait IDirectMusicBufferImpl: Sized {
    fn Flush();
    fn TotalTime();
    fn PackStructured();
    fn PackUnstructured();
    fn ResetReadPtr();
    fn GetNextEvent();
    fn GetRawBufferPtr();
    fn GetStartTime();
    fn GetUsedBytes();
    fn GetMaxBytes();
    fn GetBufferFormat();
    fn SetStartTime();
    fn SetUsedBytes();
}
pub trait IDirectMusicCollectionImpl: Sized {
    fn GetInstrument();
    fn EnumInstrument();
}
pub trait IDirectMusicDownloadImpl: Sized {
    fn GetBuffer();
}
pub trait IDirectMusicDownloadedInstrumentImpl: Sized {}
pub trait IDirectMusicInstrumentImpl: Sized {
    fn GetPatch();
    fn SetPatch();
}
pub trait IDirectMusicPortImpl: Sized {
    fn PlayBuffer();
    fn SetReadNotificationHandle();
    fn Read();
    fn DownloadInstrument();
    fn UnloadInstrument();
    fn GetLatencyClock();
    fn GetRunningStats();
    fn Compact();
    fn GetCaps();
    fn DeviceIoControl();
    fn SetNumChannelGroups();
    fn GetNumChannelGroups();
    fn Activate();
    fn SetChannelPriority();
    fn GetChannelPriority();
    fn SetDirectSound();
    fn GetFormat();
}
pub trait IDirectMusicPortDownloadImpl: Sized {
    fn GetBuffer();
    fn AllocateBuffer();
    fn GetDLId();
    fn GetAppend();
    fn Download();
    fn Unload();
}
pub trait IDirectMusicSynthImpl: Sized {
    fn Open();
    fn Close();
    fn SetNumChannelGroups();
    fn Download();
    fn Unload();
    fn PlayBuffer();
    fn GetRunningStats();
    fn GetPortCaps();
    fn SetMasterClock();
    fn GetLatencyClock();
    fn Activate();
    fn SetSynthSink();
    fn Render();
    fn SetChannelPriority();
    fn GetChannelPriority();
    fn GetFormat();
    fn GetAppend();
}
pub trait IDirectMusicSynth8Impl: Sized + IDirectMusicSynthImpl {
    fn PlayVoice();
    fn StopVoice();
    fn GetVoiceState();
    fn Refresh();
    fn AssignChannelToBuses();
}
pub trait IDirectMusicSynthSinkImpl: Sized {
    fn Init();
    fn SetMasterClock();
    fn GetLatencyClock();
    fn Activate();
    fn SampleToRefTime();
    fn RefTimeToSample();
    fn SetDirectSound();
    fn GetDesiredBufferSize();
}
pub trait IDirectMusicThruImpl: Sized {
    fn ThruChannel();
}
