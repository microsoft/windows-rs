pub trait IAudioEndpointFormatControlImpl: Sized {
    fn ResetToDefault();
}
pub trait IAudioEndpointLastBufferControlImpl: Sized {
    fn IsLastBufferControlSupported();
    fn ReleaseOutputDataPointerForLastBuffer();
}
pub trait IAudioEndpointOffloadStreamMeterImpl: Sized {
    fn GetMeterChannelCount();
    fn GetMeteringData();
}
pub trait IAudioEndpointOffloadStreamMuteImpl: Sized {
    fn SetMute();
    fn GetMute();
}
pub trait IAudioEndpointOffloadStreamVolumeImpl: Sized {
    fn GetVolumeChannelCount();
    fn SetChannelVolumes();
    fn GetChannelVolumes();
}
pub trait IAudioEndpointVolumeImpl: Sized {
    fn RegisterControlChangeNotify();
    fn UnregisterControlChangeNotify();
    fn GetChannelCount();
    fn SetMasterVolumeLevel();
    fn SetMasterVolumeLevelScalar();
    fn GetMasterVolumeLevel();
    fn GetMasterVolumeLevelScalar();
    fn SetChannelVolumeLevel();
    fn SetChannelVolumeLevelScalar();
    fn GetChannelVolumeLevel();
    fn GetChannelVolumeLevelScalar();
    fn SetMute();
    fn GetMute();
    fn GetVolumeStepInfo();
    fn VolumeStepUp();
    fn VolumeStepDown();
    fn QueryHardwareSupport();
    fn GetVolumeRange();
}
pub trait IAudioEndpointVolumeCallbackImpl: Sized {
    fn OnNotify();
}
pub trait IAudioEndpointVolumeExImpl: Sized + IAudioEndpointVolumeImpl {
    fn GetVolumeRangeChannel();
}
pub trait IAudioLfxControlImpl: Sized {
    fn SetLocalEffectsState();
    fn GetLocalEffectsState();
}
pub trait IAudioMeterInformationImpl: Sized {
    fn GetPeakValue();
    fn GetMeteringChannelCount();
    fn GetChannelsPeakValues();
    fn QueryHardwareSupport();
}
pub trait IHardwareAudioEngineBaseImpl: Sized {
    fn GetAvailableOffloadConnectorCount();
    fn GetEngineFormat();
    fn SetEngineDeviceFormat();
    fn SetGfxState();
    fn GetGfxState();
}
