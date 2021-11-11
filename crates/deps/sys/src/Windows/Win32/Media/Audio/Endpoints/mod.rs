#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AUDIO_ENDPOINT_SHARED_CREATE_PARAMS();
    fn DEVINTERFACE_AUDIOENDPOINTPLUGIN();
    fn DEVPKEY_AudioEndpointPlugin2_FactoryCLSID();
    fn DEVPKEY_AudioEndpointPlugin_DataFlow();
    fn DEVPKEY_AudioEndpointPlugin_FactoryCLSID();
    fn DEVPKEY_AudioEndpointPlugin_PnPInterface();
    fn EndpointConnectorType();
    fn IAudioEndpointFormatControl();
    fn IAudioEndpointLastBufferControl();
    fn IAudioEndpointOffloadStreamMeter();
    fn IAudioEndpointOffloadStreamMute();
    fn IAudioEndpointOffloadStreamVolume();
    fn IAudioEndpointVolume();
    fn IAudioEndpointVolumeCallback();
    fn IAudioEndpointVolumeEx();
    fn IAudioLfxControl();
    fn IAudioMeterInformation();
    fn IHardwareAudioEngineBase();
}
