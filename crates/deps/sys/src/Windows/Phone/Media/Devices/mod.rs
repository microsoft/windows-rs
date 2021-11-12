#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioRoutingEndpoint(pub i32);
impl AudioRoutingEndpoint {
    pub const Default: AudioRoutingEndpoint = AudioRoutingEndpoint(0i32);
    pub const Earpiece: AudioRoutingEndpoint = AudioRoutingEndpoint(1i32);
    pub const Speakerphone: AudioRoutingEndpoint = AudioRoutingEndpoint(2i32);
    pub const Bluetooth: AudioRoutingEndpoint = AudioRoutingEndpoint(3i32);
    pub const WiredHeadset: AudioRoutingEndpoint = AudioRoutingEndpoint(4i32);
    pub const WiredHeadsetSpeakerOnly: AudioRoutingEndpoint = AudioRoutingEndpoint(5i32);
    pub const BluetoothWithNoiseAndEchoCancellation: AudioRoutingEndpoint = AudioRoutingEndpoint(6i32);
    pub const BluetoothPreferred: AudioRoutingEndpoint = AudioRoutingEndpoint(7i32);
}
#[repr(transparent)]
pub struct AudioRoutingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AvailableAudioRoutingEndpoints(pub u32);
impl AvailableAudioRoutingEndpoints {
    pub const None: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(0u32);
    pub const Earpiece: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(1u32);
    pub const Speakerphone: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(2u32);
    pub const Bluetooth: AvailableAudioRoutingEndpoints = AvailableAudioRoutingEndpoints(4u32);
}
#[repr(transparent)]
pub struct IAudioRoutingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioRoutingManagerStatics(pub *mut ::core::ffi::c_void);
