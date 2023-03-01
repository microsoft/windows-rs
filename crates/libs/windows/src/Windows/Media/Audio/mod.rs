#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioDeviceInputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb01b6be1_6f4e_49e2_ac01_559d62beb3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceInputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioDeviceOutputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioDeviceOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioDeviceOutputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x362edbff_ff1c_4434_9e0f_bd2ef522ac82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceOutputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFileInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioFileInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFileInputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905b67c8_6f65_4cd4_8890_4694843c276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileInputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub LoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetLoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage")]
    pub SourceFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SourceFile: usize,
    #[cfg(feature = "Foundation")]
    pub FileCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFileOutputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioFileOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFileOutputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e01980_5166_4093_80f8_ada00089e9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFileOutputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub FileEncodingProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FileEncodingProfile: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub FinalizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding")))]
    FinalizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAudioFrameCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFrameCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc7c829e_0208_4504_a5a8_f0f268920a65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioFrameInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFrameInputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b266c7_fd96_4ff5_a3c5_d27a9bf44237);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameInputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AddFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DiscardQueuedFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueuedSampleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioFrameCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioFrameCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioFrameCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioFrameCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub QuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumStarted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameOutputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioFrameOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioFrameOutputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb847371b_3299_45f5_88b3_c9d12a3f1cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameOutputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraph {
    type Vtable = IAudioGraph_Vtbl;
}
impl ::core::clone::Clone for IAudioGraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraph {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad46eed_e48c_4e14_9660_2c4f83e9cdd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFrameInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormat: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub CreateDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture")))]
    CreateDeviceInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatOnDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatOnDeviceAsync: usize,
    pub CreateFrameOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameOutputNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameOutputNodeWithFormat: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDeviceOutputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDeviceOutputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileOutputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileOutputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub CreateFileOutputNodeWithFileProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, fileencodingprofile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    CreateFileOutputNodeWithFileProfileAsync: usize,
    pub CreateSubmixNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormat: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResetAllNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub QuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumStarted: usize,
    #[cfg(feature = "Foundation")]
    pub QuantumProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    QuantumProcessed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveQuantumProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveQuantumProcessed: usize,
    #[cfg(feature = "Foundation")]
    pub UnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnrecoverableErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnrecoverableErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnrecoverableErrorOccurred: usize,
    pub CompletedQuantumCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub LatencyInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    pub RenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
    pub SamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraph2 {
    type Vtable = IAudioGraph2_Vtbl;
}
impl ::core::clone::Clone for IAudioGraph2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraph2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e4c3bd5_4fc1_45f6_a947_3cd38f4fd839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateFrameInputNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateFrameInputNodeWithFormatAndEmitter: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, encodingproperties: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties")))]
    CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFileInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFileInputNodeWithEmitterAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateSubmixNodeWithFormatAndEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateSubmixNodeWithFormatAndEmitter: usize,
    #[cfg(feature = "Foundation")]
    pub CreateBatchUpdater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateBatchUpdater: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraph3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraph3 {
    type Vtable = IAudioGraph3_Vtbl;
}
impl ::core::clone::Clone for IAudioGraph3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraph3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddcd25ae_1185_42a7_831d_6a9b0fc86820);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraph3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateMediaSourceAudioInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateMediaSourceAudioInputNodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateMediaSourceAudioInputNodeWithEmitterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void, emitter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateMediaSourceAudioInputNodeWithEmitterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
}
impl ::core::clone::Clone for IAudioGraphConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraphConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x763070ed_d04e_4fac_b233_600b42edd469);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
}
impl ::core::clone::Clone for IAudioGraphSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraphSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d59647f_e6fe_4628_84f8_9d8bdba25785);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetEncodingProperties: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub PrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    PrimaryRenderDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetPrimaryRenderDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetPrimaryRenderDevice: usize,
    pub QuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut QuantumSizeSelectionMode) -> ::windows::core::HRESULT,
    pub SetQuantumSizeSelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: QuantumSizeSelectionMode) -> ::windows::core::HRESULT,
    pub DesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetDesiredSamplesPerQuantum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub AudioRenderCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Render::AudioRenderCategory) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    AudioRenderCategory: usize,
    #[cfg(feature = "Media_Render")]
    pub SetAudioRenderCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Render::AudioRenderCategory) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    SetAudioRenderCategory: usize,
    pub DesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
    pub SetDesiredRenderDeviceAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphSettings2 {
    type Vtable = IAudioGraphSettings2_Vtbl;
}
impl ::core::clone::Clone for IAudioGraphSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraphSettings2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72919787_4dab_46e3_b4c9_d8e1a2636062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettings2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetMaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub MaxPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphSettingsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphSettingsFactory {
    type Vtable = IAudioGraphSettingsFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioGraphSettingsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraphSettingsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5d91cc6_c2eb_4a61_a214_1d66d75f83da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphSettingsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Render")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiorendercategory: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphStatics {
    type Vtable = IAudioGraphStatics_Vtbl;
}
impl ::core::clone::Clone for IAudioGraphStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraphStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76ec3132_e159_4ab7_a82a_17beb4b31e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioGraphUnrecoverableErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3d9cbe0_3ff6_4fb3_b262_50d435c55423);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphUnrecoverableError) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioInputNode(::windows::core::IUnknown);
impl IAudioInputNode {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IAudioInputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IAudioNode> for IAudioInputNode {}
#[cfg(feature = "Foundation")]
impl windows::core::CanTryInto<super::super::Foundation::IClosable> for IAudioInputNode {}
impl ::core::cmp::PartialEq for IAudioInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputNode {}
impl ::core::fmt::Debug for IAudioInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAudioInputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d148005c-8428-4784-b7fd-a99d468c5d20}");
}
unsafe impl ::windows::core::Interface for IAudioInputNode {
    type Vtable = IAudioInputNode_Vtbl;
}
impl ::core::clone::Clone for IAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioInputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd148005c_8428_4784_b7fd_a99d468c5d20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub OutgoingConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutgoingConnections: usize,
    pub AddOutgoingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddOutgoingConnectionWithGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, gain: f64) -> ::windows::core::HRESULT,
    pub RemoveOutgoingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioInputNode2(::windows::core::IUnknown);
impl IAudioInputNode2 {
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).Emitter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IAudioInputNode2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IAudioInputNode> for IAudioInputNode2 {}
impl windows::core::CanTryInto<IAudioNode> for IAudioInputNode2 {}
#[cfg(feature = "Foundation")]
impl windows::core::CanTryInto<super::super::Foundation::IClosable> for IAudioInputNode2 {}
impl ::core::cmp::PartialEq for IAudioInputNode2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputNode2 {}
impl ::core::fmt::Debug for IAudioInputNode2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputNode2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAudioInputNode2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2}");
}
unsafe impl ::windows::core::Interface for IAudioInputNode2 {
    type Vtable = IAudioInputNode2_Vtbl;
}
impl ::core::clone::Clone for IAudioInputNode2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioInputNode2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905156b7_ca68_4c6d_a8bc_e3ee17fe3fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputNode2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Emitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioNode(::windows::core::IUnknown);
impl IAudioNode {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IAudioNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl windows::core::CanTryInto<super::super::Foundation::IClosable> for IAudioNode {}
impl ::core::cmp::PartialEq for IAudioNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioNode {}
impl ::core::fmt::Debug for IAudioNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAudioNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{15389d7f-dbd8-4819-bf03-668e9357cd6d}");
}
unsafe impl ::windows::core::Interface for IAudioNode {
    type Vtable = IAudioNode_Vtbl;
}
impl ::core::clone::Clone for IAudioNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15389d7f_dbd8_4819_bf03_668e9357cd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub EffectDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    EffectDefinitions: usize,
    pub SetOutgoingGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub OutgoingGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub ConsumeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetConsumeInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Effects")]
    pub DisableEffectsByDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    DisableEffectsByDefinition: usize,
    #[cfg(feature = "Media_Effects")]
    pub EnableEffectsByDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    EnableEffectsByDefinition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3676971d_880a_47b8_adf7_1323a9d965be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Direction: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDirection: usize,
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DecayModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DistanceScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDistanceScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DopplerScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDopplerScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
    pub IsDopplerDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitter2 {
    type Vtable = IAudioNodeEmitter2_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ab6eecb_ec29_47f8_818c_b6b660a5aeb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SpatialAudioModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpatialAudioModel) -> ::windows::core::HRESULT,
    pub SetSpatialAudioModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpatialAudioModel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterConeProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterConeProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterConeProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe99b2cee_02ca_4375_9326_0c6ae4bcdfb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterConeProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InnerAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub OuterAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub OuterAngleGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterDecayModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterDecayModel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1d5af7_0d53_4fa9_bd84_d5816a86f3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterDecayKind) -> ::windows::core::HRESULT,
    pub MinGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub NaturalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterDecayModelStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterDecayModelStatics {
    type Vtable = IAudioNodeEmitterDecayModelStatics_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterDecayModelStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterDecayModelStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7787ca8_f178_462f_bc81_8dd5cbe5dae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterDecayModelStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateNatural: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mingain: f64, maxgain: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterFactory {
    type Vtable = IAudioNodeEmitterFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdc8489a_6ad6_4ce4_b7f7_a99370df7ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateAudioNodeEmitter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shape: *mut ::core::ffi::c_void, decaymodel: *mut ::core::ffi::c_void, settings: AudioNodeEmitterSettings, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterNaturalDecayModelProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterNaturalDecayModelProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48934bcf_cf2c_4efc_9331_75bd22df1f0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterNaturalDecayModelProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnityGainDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub CutoffDistance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterShape(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterShape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterShape {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea0311c5_e73d_44bc_859c_45553bbc4828);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShape_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioNodeEmitterShapeKind) -> ::windows::core::HRESULT,
    pub ConeProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeEmitterShapeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeEmitterShapeStatics {
    type Vtable = IAudioNodeEmitterShapeStatics_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeEmitterShapeStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeEmitterShapeStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57bb2771_ffa5_4b86_a779_e264aeb9145f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeEmitterShapeStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateCone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, innerangle: f64, outerangle: f64, outeranglegain: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateOmnidirectional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioNodeListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeListener {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9722e16_0c0a_41da_b755_6c77835fb1eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeListener_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Orientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOrientation: usize,
    pub SpeedOfSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetSpeedOfSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DopplerVelocity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetDopplerVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetDopplerVelocity: usize,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioNodeWithListener(::windows::core::IUnknown);
impl IAudioNodeWithListener {
    pub fn SetListener(&self, value: &AudioNodeListener) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetListener)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Listener(&self) -> ::windows::core::Result<AudioNodeListener> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeListener>();
            (::windows::core::Interface::vtable(this).Listener)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IAudioNodeWithListener, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IAudioNode> for IAudioNodeWithListener {}
#[cfg(feature = "Foundation")]
impl windows::core::CanTryInto<super::super::Foundation::IClosable> for IAudioNodeWithListener {}
impl ::core::cmp::PartialEq for IAudioNodeWithListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioNodeWithListener {}
impl ::core::fmt::Debug for IAudioNodeWithListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioNodeWithListener").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAudioNodeWithListener {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{0e0f907c-79ff-4544-9eeb-01257b15105a}");
}
unsafe impl ::windows::core::Interface for IAudioNodeWithListener {
    type Vtable = IAudioNodeWithListener_Vtbl;
}
impl ::core::clone::Clone for IAudioNodeWithListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioNodeWithListener {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e0f907c_79ff_4544_9eeb_01257b15105a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioNodeWithListener_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Listener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
}
impl ::core::clone::Clone for IAudioPlaybackConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioPlaybackConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a4c1dea_cafc_50e7_8718_ea3f81cbfa51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionState) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnectionOpenResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
}
impl ::core::clone::Clone for IAudioPlaybackConnectionOpenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioPlaybackConnectionOpenResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e656aef_39f9_5fc9_a519_a5bbfd9fe921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionOpenResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioPlaybackConnectionOpenResultStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioPlaybackConnectionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioPlaybackConnectionStatics {
    type Vtable = IAudioPlaybackConnectionStatics_Vtbl;
}
impl ::core::clone::Clone for IAudioPlaybackConnectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioPlaybackConnectionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe60963a2_69e6_5ffc_9e13_824a85213daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPlaybackConnectionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TryCreateFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStateMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
}
impl ::core::clone::Clone for IAudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioStateMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d13d136_0199_4cdc_b84e_e72c2b581ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SoundLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSoundLevelChanged: usize,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SoundLevel) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStateMonitorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioStateMonitorStatics {
    type Vtable = IAudioStateMonitorStatics_Vtbl;
}
impl ::core::clone::Clone for IAudioStateMonitorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAudioStateMonitorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6374ea4c_1b3b_4001_94d9_dd225330fa40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateForRenderMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub CreateForRenderMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Devices", feature = "Media_Render")))]
    CreateForRenderMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Render")]
    pub CreateForRenderMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Render::AudioRenderCategory, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Render"))]
    CreateForRenderMonitoringWithCategoryAndDeviceId: usize,
    pub CreateForCaptureMonitoring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategory: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices")))]
    CreateForCaptureMonitoringWithCategoryAndDeviceRole: usize,
    #[cfg(feature = "Media_Capture")]
    pub CreateForCaptureMonitoringWithCategoryAndDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: super::Capture::MediaCategory, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    CreateForCaptureMonitoringWithCategoryAndDeviceId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioDeviceInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioDeviceInputNodeResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16eec7a8_1ca7_40ef_91a4_d346e0aa1bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT,
    pub DeviceInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceInputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceInputNodeResult2 {
    type Vtable = ICreateAudioDeviceInputNodeResult2_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioDeviceInputNodeResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioDeviceInputNodeResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x921c69ce_3f35_41c7_9622_79f608baedc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceInputNodeResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioDeviceOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioDeviceOutputNodeResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7776d27_1d9a_47f7_9cd4_2859cc1b7bff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceNodeCreationStatus) -> ::windows::core::HRESULT,
    pub DeviceOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioDeviceOutputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioDeviceOutputNodeResult2 {
    type Vtable = ICreateAudioDeviceOutputNodeResult2_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioDeviceOutputNodeResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioDeviceOutputNodeResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4864269f_bdce_4ab1_bd38_fbae93aedaca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioDeviceOutputNodeResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioFileInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioFileInputNodeResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce83d61c_e297_4c50_9ce7_1c7a69d6bd09);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT,
    pub FileInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileInputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileInputNodeResult2 {
    type Vtable = ICreateAudioFileInputNodeResult2_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioFileInputNodeResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioFileInputNodeResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9082020_3d80_4fe0_81c1_768fea7ca7e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileInputNodeResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioFileOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioFileOutputNodeResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47d6ba7b_e909_453f_866e_5540cda734ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioFileNodeCreationStatus) -> ::windows::core::HRESULT,
    pub FileOutputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioFileOutputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioFileOutputNodeResult2 {
    type Vtable = ICreateAudioFileOutputNodeResult2_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioFileOutputNodeResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioFileOutputNodeResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f01b50d_3318_47b3_a60a_1b492be7fc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioFileOutputNodeResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioGraphResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioGraphResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioGraphResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5453ef7e_7bde_4b76_bb5d_48f79cfc8c0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioGraphCreationStatus) -> ::windows::core::HRESULT,
    pub Graph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateAudioGraphResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateAudioGraphResult2 {
    type Vtable = ICreateAudioGraphResult2_Vtbl;
}
impl ::core::clone::Clone for ICreateAudioGraphResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateAudioGraphResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d738dfc_88c6_4fcb_a534_85cedd4050a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateAudioGraphResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
}
impl ::core::clone::Clone for ICreateMediaSourceAudioInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateMediaSourceAudioInputNodeResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46a658a3_53c0_4d59_9e51_cc1d1044a4c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceAudioInputNodeCreationStatus) -> ::windows::core::HRESULT,
    pub Node: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICreateMediaSourceAudioInputNodeResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICreateMediaSourceAudioInputNodeResult2 {
    type Vtable = ICreateMediaSourceAudioInputNodeResult2_Vtbl;
}
impl ::core::clone::Clone for ICreateMediaSourceAudioInputNodeResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICreateMediaSourceAudioInputNodeResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63514ce8_6a1a_49e3_97ec_28fd5be114e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreateMediaSourceAudioInputNodeResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEchoEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IEchoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEchoEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e4d3faa_36b8_4c91_b9da_11f44a8a6610);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Feedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEchoEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEchoEffectDefinitionFactory {
    type Vtable = IEchoEffectDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for IEchoEffectDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEchoEffectDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d4e2257_aaf2_4e86_a54c_fb79db8f6c12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEchoEffectDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerBand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
}
impl ::core::clone::Clone for IEqualizerBand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEqualizerBand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc00a5a6a_262d_4b85_9bb7_43280b62ed0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerBand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Bandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub FrequencyCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetFrequencyCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IEqualizerEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEqualizerEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x023f6f1f_83fe_449a_a822_c696442d16b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Bands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bands: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEqualizerEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEqualizerEffectDefinitionFactory {
    type Vtable = IEqualizerEffectDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for IEqualizerEffectDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEqualizerEffectDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2876fc4_d410_4eb5_9e69_c9aa1277eaf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEqualizerEffectDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameInputNodeQuantumStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IFrameInputNodeQuantumStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFrameInputNodeQuantumStartedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9bd498_a306_4f06_bd9f_e9efc8226304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameInputNodeQuantumStartedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequiredSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimiterEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for ILimiterEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILimiterEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b755d19_2603_47ba_bdeb_39055e3486dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLoudness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Loudness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILimiterEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILimiterEffectDefinitionFactory {
    type Vtable = ILimiterEffectDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for ILimiterEffectDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILimiterEffectDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecbae6f1_61ff_45ef_b8f5_48659a57c72d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILimiterEffectDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAudioInputNode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaSourceAudioInputNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d8983b_a88a_4041_8e4f_ddbac0c91fd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAudioInputNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub PlaybackSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Seek: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub LoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub SetLoopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLoopCount: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Media_Core")]
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    MediaSource: usize,
    #[cfg(feature = "Foundation")]
    pub MediaSourceCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaSourceCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaSourceCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaSourceCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReverbEffectDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IReverbEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IReverbEffectDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4606aa89_f563_4d0a_8f6e_f0cddff35d84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub WetDryMix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReflectionsDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ReflectionsDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetReverbDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub ReverbDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetRearDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub RearDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionMatrixLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionMatrixLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetPositionMatrixRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub PositionMatrixRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetEarlyDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub EarlyDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetLateDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub LateDiffusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetLowEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub LowEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetLowEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub LowEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetHighEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub HighEQGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetHighEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub HighEQCutoff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub SetRoomFilterFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomFilterFreq: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoomFilterMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomFilterMain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoomFilterHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomFilterHF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReflectionsGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ReflectionsGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetReverbGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ReverbGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDecayTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DecayTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDensity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Density: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetRoomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RoomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDisableLateField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisableLateField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReverbEffectDefinitionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReverbEffectDefinitionFactory {
    type Vtable = IReverbEffectDefinitionFactory_Vtbl;
}
impl ::core::clone::Clone for IReverbEffectDefinitionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IReverbEffectDefinitionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d5cbfe_100b_4ff0_9da6_dc4e05a759f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReverbEffectDefinitionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiograph: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISetDefaultSpatialAudioFormatResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
}
impl ::core::clone::Clone for ISetDefaultSpatialAudioFormatResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISetDefaultSpatialAudioFormatResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2aa511_1400_5e70_9ea9_ae151241e8ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISetDefaultSpatialAudioFormatResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SetDefaultSpatialAudioFormatStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioDeviceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioDeviceConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee830034_61cf_5749_9da4_10f0fe028199);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsSpatialAudioSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSpatialAudioFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ActiveSpatialAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DefaultSpatialAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultSpatialAudioFormatAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultSpatialAudioFormatAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConfigurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConfigurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioDeviceConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioDeviceConfigurationStatics {
    type Vtable = ISpatialAudioDeviceConfigurationStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioDeviceConfigurationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioDeviceConfigurationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ec37f7b_936d_4e04_9728_2827d9f758c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioDeviceConfigurationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioFormatConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioFormatConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32df09a8_50f0_5395_9923_7d44ca71ed6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReportLicenseChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportLicenseChangedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportConfigurationChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportConfigurationChangedAsync: usize,
    pub MixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT,
    pub SetMixedRealityExclusiveModePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatConfigurationStatics {
    type Vtable = ISpatialAudioFormatConfigurationStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioFormatConfigurationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioFormatConfigurationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b5fef71_67c9_4e5f_a35b_41680711f8c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatConfigurationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatSubtypeStatics {
    type Vtable = ISpatialAudioFormatSubtypeStatics_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioFormatSubtypeStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioFormatSubtypeStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3de8a47_83ee_4266_a945_bedf507afeed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub WindowsSonic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DolbyAtmosForHeadphones: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DolbyAtmosForHomeTheater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DolbyAtmosForSpeakers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DTSHeadphoneX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DTSXUltra: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialAudioFormatSubtypeStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialAudioFormatSubtypeStatics2 {
    type Vtable = ISpatialAudioFormatSubtypeStatics2_Vtbl;
}
impl ::core::clone::Clone for ISpatialAudioFormatSubtypeStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpatialAudioFormatSubtypeStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4565e6cb_d95b_5621_b6af_0e8849c57c80);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioFormatSubtypeStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DTSXForHomeTheater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioDeviceInputNode(::windows::core::IUnknown);
impl AudioDeviceInputNode {
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DeviceInformation>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).Emitter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioDeviceInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceInputNode {}
impl ::core::fmt::Debug for AudioDeviceInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceInputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioDeviceInputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceInputNode;{b01b6be1-6f4e-49e2-ac01-559d62beb3a9})");
}
impl ::core::clone::Clone for AudioDeviceInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioDeviceInputNode {
    type Vtable = IAudioDeviceInputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioDeviceInputNode {
    const IID: ::windows::core::GUID = <IAudioDeviceInputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceInputNode";
}
::windows::imp::interface_hierarchy!(AudioDeviceInputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioInputNode> for AudioDeviceInputNode {}
impl ::windows::core::CanTryInto<IAudioInputNode2> for AudioDeviceInputNode {}
impl ::windows::core::CanTryInto<IAudioNode> for AudioDeviceInputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioDeviceInputNode {}
unsafe impl ::core::marker::Send for AudioDeviceInputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioDeviceOutputNode(::windows::core::IUnknown);
impl AudioDeviceOutputNode {
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Device(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DeviceInformation>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    pub fn SetListener(&self, value: &AudioNodeListener) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNodeWithListener>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetListener)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Listener(&self) -> ::windows::core::Result<AudioNodeListener> {
        let this = &::windows::core::ComInterface::cast::<IAudioNodeWithListener>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeListener>();
            (::windows::core::Interface::vtable(this).Listener)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioDeviceOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioDeviceOutputNode {}
impl ::core::fmt::Debug for AudioDeviceOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceOutputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioDeviceOutputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioDeviceOutputNode;{362edbff-ff1c-4434-9e0f-bd2ef522ac82})");
}
impl ::core::clone::Clone for AudioDeviceOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioDeviceOutputNode {
    type Vtable = IAudioDeviceOutputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioDeviceOutputNode {
    const IID: ::windows::core::GUID = <IAudioDeviceOutputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioDeviceOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
}
::windows::imp::interface_hierarchy!(AudioDeviceOutputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioNode> for AudioDeviceOutputNode {}
impl ::windows::core::CanTryInto<IAudioNodeWithListener> for AudioDeviceOutputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioDeviceOutputNode {}
unsafe impl ::core::marker::Send for AudioDeviceOutputNode {}
unsafe impl ::core::marker::Sync for AudioDeviceOutputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFileInputNode(::windows::core::IUnknown);
impl AudioFileInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).PlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
    pub fn Seek(&self, position: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::windows::core::Interface::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).EndTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<i32>>();
            (::windows::core::Interface::vtable(this).LoopCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLoopCount)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFile>();
            (::windows::core::Interface::vtable(this).SourceFile)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<AudioFileInputNode, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).FileCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).Emitter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioFileInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFileInputNode {}
impl ::core::fmt::Debug for AudioFileInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileInputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioFileInputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileInputNode;{905b67c8-6f65-4cd4-8890-4694843c276d})");
}
impl ::core::clone::Clone for AudioFileInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioFileInputNode {
    type Vtable = IAudioFileInputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioFileInputNode {
    const IID: ::windows::core::GUID = <IAudioFileInputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioFileInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
}
::windows::imp::interface_hierarchy!(AudioFileInputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioInputNode> for AudioFileInputNode {}
impl ::windows::core::CanTryInto<IAudioInputNode2> for AudioFileInputNode {}
impl ::windows::core::CanTryInto<IAudioNode> for AudioFileInputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioFileInputNode {}
unsafe impl ::core::marker::Send for AudioFileInputNode {}
unsafe impl ::core::marker::Sync for AudioFileInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFileOutputNode(::windows::core::IUnknown);
impl AudioFileOutputNode {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::IStorageFile>();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FileEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::MediaEncodingProfile>();
            (::windows::core::Interface::vtable(this).FileEncodingProfile)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Transcoding\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding"))]
    pub fn FinalizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::Transcoding::TranscodeFailureReason>>();
            (::windows::core::Interface::vtable(this).FinalizeAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioFileOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFileOutputNode {}
impl ::core::fmt::Debug for AudioFileOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileOutputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioFileOutputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFileOutputNode;{50e01980-5166-4093-80f8-ada00089e9cf})");
}
impl ::core::clone::Clone for AudioFileOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioFileOutputNode {
    type Vtable = IAudioFileOutputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioFileOutputNode {
    const IID: ::windows::core::GUID = <IAudioFileOutputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioFileOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFileOutputNode";
}
::windows::imp::interface_hierarchy!(AudioFileOutputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioNode> for AudioFileOutputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioFileOutputNode {}
unsafe impl ::core::marker::Send for AudioFileOutputNode {}
unsafe impl ::core::marker::Sync for AudioFileOutputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFrameCompletedEventArgs(::windows::core::IUnknown);
impl AudioFrameCompletedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AudioFrame>();
            (::windows::core::Interface::vtable(this).Frame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioFrameCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameCompletedEventArgs {}
impl ::core::fmt::Debug for AudioFrameCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioFrameCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameCompletedEventArgs;{dc7c829e-0208-4504-a5a8-f0f268920a65})");
}
impl ::core::clone::Clone for AudioFrameCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioFrameCompletedEventArgs {
    type Vtable = IAudioFrameCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioFrameCompletedEventArgs {
    const IID: ::windows::core::GUID = <IAudioFrameCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrameCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(AudioFrameCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioFrameCompletedEventArgs {}
unsafe impl ::core::marker::Sync for AudioFrameCompletedEventArgs {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFrameInputNode(::windows::core::IUnknown);
impl AudioFrameInputNode {
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).PlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddFrame(&self, frame: &super::AudioFrame) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddFrame)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(frame)).ok() }
    }
    pub fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DiscardQueuedFrames)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn QueuedSampleCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).QueuedSampleCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<AudioFrameInputNode, AudioFrameCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AudioFrameCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioFrameCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted(&self, handler: &super::super::Foundation::TypedEventHandler<AudioFrameInputNode, FrameInputNodeQuantumStartedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).QuantumStarted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveQuantumStarted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).Emitter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioFrameInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameInputNode {}
impl ::core::fmt::Debug for AudioFrameInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameInputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioFrameInputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameInputNode;{01b266c7-fd96-4ff5-a3c5-d27a9bf44237})");
}
impl ::core::clone::Clone for AudioFrameInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioFrameInputNode {
    type Vtable = IAudioFrameInputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioFrameInputNode {
    const IID: ::windows::core::GUID = <IAudioFrameInputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrameInputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameInputNode";
}
::windows::imp::interface_hierarchy!(AudioFrameInputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioInputNode> for AudioFrameInputNode {}
impl ::windows::core::CanTryInto<IAudioInputNode2> for AudioFrameInputNode {}
impl ::windows::core::CanTryInto<IAudioNode> for AudioFrameInputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioFrameInputNode {}
unsafe impl ::core::marker::Send for AudioFrameInputNode {}
unsafe impl ::core::marker::Sync for AudioFrameInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioFrameOutputNode(::windows::core::IUnknown);
impl AudioFrameOutputNode {
    pub fn GetFrame(&self) -> ::windows::core::Result<super::AudioFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AudioFrame>();
            (::windows::core::Interface::vtable(this).GetFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioFrameOutputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrameOutputNode {}
impl ::core::fmt::Debug for AudioFrameOutputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrameOutputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioFrameOutputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioFrameOutputNode;{b847371b-3299-45f5-88b3-c9d12a3f1cc8})");
}
impl ::core::clone::Clone for AudioFrameOutputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioFrameOutputNode {
    type Vtable = IAudioFrameOutputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioFrameOutputNode {
    const IID: ::windows::core::GUID = <IAudioFrameOutputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrameOutputNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioFrameOutputNode";
}
::windows::imp::interface_hierarchy!(AudioFrameOutputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioNode> for AudioFrameOutputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioFrameOutputNode {}
unsafe impl ::core::marker::Send for AudioFrameOutputNode {}
unsafe impl ::core::marker::Sync for AudioFrameOutputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraph(::windows::core::IUnknown);
impl AudioGraph {
    pub fn CreateFrameInputNode(&self) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFrameInputNode>();
            (::windows::core::Interface::vtable(this).CreateFrameInputNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormat(&self, encodingproperties: &super::MediaProperties::AudioEncodingProperties) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFrameInputNode>();
            (::windows::core::Interface::vtable(this).CreateFrameInputNodeWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture"))]
    pub fn CreateDeviceInputNodeAsync(&self, category: super::Capture::MediaCategory) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeAsync)(::windows::core::Interface::as_raw(this), category, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &super::MediaProperties::AudioEncodingProperties) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAsync)(::windows::core::Interface::as_raw(this), category, ::core::mem::transmute_copy(encodingproperties), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatOnDeviceAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &super::MediaProperties::AudioEncodingProperties, device: &super::super::Devices::Enumeration::DeviceInformation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeWithFormatOnDeviceAsync)(::windows::core::Interface::as_raw(this), category, ::core::mem::transmute_copy(encodingproperties), ::core::mem::transmute_copy(device), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFrameOutputNode(&self) -> ::windows::core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFrameOutputNode>();
            (::windows::core::Interface::vtable(this).CreateFrameOutputNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameOutputNodeWithFormat(&self, encodingproperties: &super::MediaProperties::AudioEncodingProperties) -> ::windows::core::Result<AudioFrameOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFrameOutputNode>();
            (::windows::core::Interface::vtable(this).CreateFrameOutputNodeWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDeviceOutputNodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateDeviceOutputNodeAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeAsync<P0>(&self, file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateFileInputNodeAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileOutputNodeAsync<P0>(&self, file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateFileOutputNodeAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CreateFileOutputNodeWithFileProfileAsync<P0>(&self, file: P0, fileencodingprofile: &super::MediaProperties::MediaEncodingProfile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioFileOutputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateFileOutputNodeWithFileProfileAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(fileencodingprofile), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateSubmixNode(&self) -> ::windows::core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioSubmixNode>();
            (::windows::core::Interface::vtable(this).CreateSubmixNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormat(&self, encodingproperties: &super::MediaProperties::AudioEncodingProperties) -> ::windows::core::Result<AudioSubmixNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioSubmixNode>();
            (::windows::core::Interface::vtable(this).CreateSubmixNodeWithFormat)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ResetAllNodes(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResetAllNodes)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QuantumStarted(&self, handler: &super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).QuantumStarted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveQuantumStarted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn QuantumProcessed(&self, handler: &super::super::Foundation::TypedEventHandler<AudioGraph, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).QuantumProcessed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveQuantumProcessed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveQuantumProcessed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnrecoverableErrorOccurred(&self, handler: &super::super::Foundation::TypedEventHandler<AudioGraph, AudioGraphUnrecoverableErrorOccurredEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).UnrecoverableErrorOccurred)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnrecoverableErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnrecoverableErrorOccurred)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn CompletedQuantumCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).CompletedQuantumCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LatencyInSamples(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).LatencyInSamples)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DeviceInformation>();
            (::windows::core::Interface::vtable(this).PrimaryRenderDevice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AudioProcessing>();
            (::windows::core::Interface::vtable(this).RenderDeviceAudioProcessing)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SamplesPerQuantum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).SamplesPerQuantum)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateFrameInputNodeWithFormatAndEmitter(&self, encodingproperties: &super::MediaProperties::AudioEncodingProperties, emitter: &AudioNodeEmitter) -> ::windows::core::Result<AudioFrameInputNode> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFrameInputNode>();
            (::windows::core::Interface::vtable(this).CreateFrameInputNodeWithFormatAndEmitter)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties), ::core::mem::transmute_copy(emitter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`, `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Media_Capture", feature = "Media_MediaProperties"))]
    pub fn CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(&self, category: super::Capture::MediaCategory, encodingproperties: &super::MediaProperties::AudioEncodingProperties, device: &super::super::Devices::Enumeration::DeviceInformation, emitter: &AudioNodeEmitter) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioDeviceInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync)(::windows::core::Interface::as_raw(this), category, ::core::mem::transmute_copy(encodingproperties), ::core::mem::transmute_copy(device), ::core::mem::transmute_copy(emitter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFileInputNodeWithEmitterAsync<P0>(&self, file: P0, emitter: &AudioNodeEmitter) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateFileInputNodeWithEmitterAsync)(::windows::core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(emitter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateSubmixNodeWithFormatAndEmitter(&self, encodingproperties: &super::MediaProperties::AudioEncodingProperties, emitter: &AudioNodeEmitter) -> ::windows::core::Result<AudioSubmixNode> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioSubmixNode>();
            (::windows::core::Interface::vtable(this).CreateSubmixNodeWithFormatAndEmitter)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(encodingproperties), ::core::mem::transmute_copy(emitter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateBatchUpdater(&self) -> ::windows::core::Result<AudioGraphBatchUpdater> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioGraphBatchUpdater>();
            (::windows::core::Interface::vtable(this).CreateBatchUpdater)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeAsync(&self, mediasource: &super::Core::MediaSource) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateMediaSourceAudioInputNodeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(mediasource), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateMediaSourceAudioInputNodeWithEmitterAsync(&self, mediasource: &super::Core::MediaSource, emitter: &AudioNodeEmitter) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraph3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateMediaSourceAudioInputNodeResult>>();
            (::windows::core::Interface::vtable(this).CreateMediaSourceAudioInputNodeWithEmitterAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(mediasource), ::core::mem::transmute_copy(emitter), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(settings: &AudioGraphSettings) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>> {
        Self::IAudioGraphStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>>();
            (::windows::core::Interface::vtable(this).CreateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(settings), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAudioGraphStatics<R, F: FnOnce(&IAudioGraphStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioGraph, IAudioGraphStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioGraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraph {}
impl ::core::fmt::Debug for AudioGraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraph").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioGraph {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraph;{1ad46eed-e48c-4e14-9660-2c4f83e9cdd8})");
}
impl ::core::clone::Clone for AudioGraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioGraph {
    type Vtable = IAudioGraph_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioGraph {
    const IID: ::windows::core::GUID = <IAudioGraph as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraph {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
}
::windows::imp::interface_hierarchy!(AudioGraph, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioGraph {}
unsafe impl ::core::marker::Send for AudioGraph {}
unsafe impl ::core::marker::Sync for AudioGraph {}
#[doc = "*Required features: `\"Media_Audio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct AudioGraphBatchUpdater(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl AudioGraphBatchUpdater {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for AudioGraphBatchUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for AudioGraphBatchUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphBatchUpdater").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeType for AudioGraphBatchUpdater {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphBatchUpdater;{30d5a829-7fa4-4026-83bb-d75bae4ea99e})");
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for AudioGraphBatchUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for AudioGraphBatchUpdater {
    type Vtable = super::super::Foundation::IClosable_Vtbl;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::ComInterface for AudioGraphBatchUpdater {
    const IID: ::windows::core::GUID = <super::super::Foundation::IClosable as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for AudioGraphBatchUpdater {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphBatchUpdater";
}
#[cfg(feature = "Foundation")]
::windows::imp::interface_hierarchy!(AudioGraphBatchUpdater, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for AudioGraphBatchUpdater {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for AudioGraphBatchUpdater {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphConnection(::windows::core::IUnknown);
impl AudioGraphConnection {
    pub fn Destination(&self) -> ::windows::core::Result<IAudioNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IAudioNode>();
            (::windows::core::Interface::vtable(this).Destination)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioGraphConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphConnection {}
impl ::core::fmt::Debug for AudioGraphConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphConnection").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioGraphConnection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphConnection;{763070ed-d04e-4fac-b233-600b42edd469})");
}
impl ::core::clone::Clone for AudioGraphConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioGraphConnection {
    type Vtable = IAudioGraphConnection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioGraphConnection {
    const IID: ::windows::core::GUID = <IAudioGraphConnection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraphConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphConnection";
}
::windows::imp::interface_hierarchy!(AudioGraphConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioGraphConnection {}
unsafe impl ::core::marker::Sync for AudioGraphConnection {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphSettings(::windows::core::IUnknown);
impl AudioGraphSettings {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetEncodingProperties(&self, value: &super::MediaProperties::AudioEncodingProperties) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncodingProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn PrimaryRenderDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DeviceInformation>();
            (::windows::core::Interface::vtable(this).PrimaryRenderDevice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetPrimaryRenderDevice(&self, value: &super::super::Devices::Enumeration::DeviceInformation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrimaryRenderDevice)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn QuantumSizeSelectionMode(&self) -> ::windows::core::Result<QuantumSizeSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<QuantumSizeSelectionMode>();
            (::windows::core::Interface::vtable(this).QuantumSizeSelectionMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetQuantumSizeSelectionMode(&self, value: QuantumSizeSelectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetQuantumSizeSelectionMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredSamplesPerQuantum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).DesiredSamplesPerQuantum)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredSamplesPerQuantum)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn AudioRenderCategory(&self) -> ::windows::core::Result<super::Render::AudioRenderCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Render::AudioRenderCategory>();
            (::windows::core::Interface::vtable(this).AudioRenderCategory)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn SetAudioRenderCategory(&self, value: super::Render::AudioRenderCategory) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioRenderCategory)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredRenderDeviceAudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::AudioProcessing>();
            (::windows::core::Interface::vtable(this).DesiredRenderDeviceAudioProcessing)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredRenderDeviceAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredRenderDeviceAudioProcessing)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraphSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxPlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxPlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioGraphSettings2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MaxPlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn Create(audiorendercategory: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioGraphSettings> {
        Self::IAudioGraphSettingsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioGraphSettings>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), audiorendercategory, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioGraphSettingsFactory<R, F: FnOnce(&IAudioGraphSettingsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioGraphSettings, IAudioGraphSettingsFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioGraphSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphSettings {}
impl ::core::fmt::Debug for AudioGraphSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphSettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioGraphSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphSettings;{1d59647f-e6fe-4628-84f8-9d8bdba25785})");
}
impl ::core::clone::Clone for AudioGraphSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioGraphSettings {
    type Vtable = IAudioGraphSettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioGraphSettings {
    const IID: ::windows::core::GUID = <IAudioGraphSettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraphSettings {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
}
::windows::imp::interface_hierarchy!(AudioGraphSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioGraphSettings {}
unsafe impl ::core::marker::Sync for AudioGraphSettings {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioGraphUnrecoverableErrorOccurredEventArgs(::windows::core::IUnknown);
impl AudioGraphUnrecoverableErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<AudioGraphUnrecoverableError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioGraphUnrecoverableError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioGraphUnrecoverableErrorOccurredEventArgs {}
impl ::core::fmt::Debug for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableErrorOccurredEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs;{c3d9cbe0-3ff6-4fb3-b262-50d435c55423})");
}
impl ::core::clone::Clone for AudioGraphUnrecoverableErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    type Vtable = IAudioGraphUnrecoverableErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = <IAudioGraphUnrecoverableErrorOccurredEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioGraphUnrecoverableErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
}
::windows::imp::interface_hierarchy!(AudioGraphUnrecoverableErrorOccurredEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioGraphUnrecoverableErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for AudioGraphUnrecoverableErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitter(::windows::core::IUnknown);
impl AudioNodeEmitter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioNodeEmitter, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Vector3>();
            (::windows::core::Interface::vtable(this).Position)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition(&self, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Direction(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Vector3>();
            (::windows::core::Interface::vtable(this).Direction)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDirection(&self, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDirection)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Shape(&self) -> ::windows::core::Result<AudioNodeEmitterShape> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterShape>();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DecayModel(&self) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterDecayModel>();
            (::windows::core::Interface::vtable(this).DecayModel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DistanceScale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).DistanceScale)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDistanceScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDistanceScale)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DopplerScale(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).DopplerScale)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDopplerScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDopplerScale)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Vector3>();
            (::windows::core::Interface::vtable(this).DopplerVelocity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity(&self, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDopplerVelocity)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDopplerDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDopplerDisabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SpatialAudioModel(&self) -> ::windows::core::Result<SpatialAudioModel> {
        let this = &::windows::core::ComInterface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpatialAudioModel>();
            (::windows::core::Interface::vtable(this).SpatialAudioModel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpatialAudioModel(&self, value: SpatialAudioModel) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNodeEmitter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSpatialAudioModel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAudioNodeEmitter(shape: &AudioNodeEmitterShape, decaymodel: &AudioNodeEmitterDecayModel, settings: AudioNodeEmitterSettings) -> ::windows::core::Result<AudioNodeEmitter> {
        Self::IAudioNodeEmitterFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).CreateAudioNodeEmitter)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(shape), ::core::mem::transmute_copy(decaymodel), settings, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterFactory<R, F: FnOnce(&IAudioNodeEmitterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioNodeEmitter, IAudioNodeEmitterFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitter {}
impl ::core::fmt::Debug for AudioNodeEmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitter;{3676971d-880a-47b8-adf7-1323a9d965be})");
}
impl ::core::clone::Clone for AudioNodeEmitter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitter {
    type Vtable = IAudioNodeEmitter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioNodeEmitter {
    const IID: ::windows::core::GUID = <IAudioNodeEmitter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitter {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitter";
}
::windows::imp::interface_hierarchy!(AudioNodeEmitter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioNodeEmitter {}
unsafe impl ::core::marker::Sync for AudioNodeEmitter {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterConeProperties(::windows::core::IUnknown);
impl AudioNodeEmitterConeProperties {
    pub fn InnerAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).InnerAngle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OuterAngle(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OuterAngle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OuterAngleGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OuterAngleGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterConeProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterConeProperties {}
impl ::core::fmt::Debug for AudioNodeEmitterConeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterConeProperties").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterConeProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterConeProperties;{e99b2cee-02ca-4375-9326-0c6ae4bcdfb5})");
}
impl ::core::clone::Clone for AudioNodeEmitterConeProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterConeProperties {
    type Vtable = IAudioNodeEmitterConeProperties_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioNodeEmitterConeProperties {
    const IID: ::windows::core::GUID = <IAudioNodeEmitterConeProperties as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterConeProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterConeProperties";
}
::windows::imp::interface_hierarchy!(AudioNodeEmitterConeProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioNodeEmitterConeProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterConeProperties {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterDecayModel(::windows::core::IUnknown);
impl AudioNodeEmitterDecayModel {
    pub fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterDecayKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterDecayKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MinGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MaxGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NaturalProperties(&self) -> ::windows::core::Result<AudioNodeEmitterNaturalDecayModelProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterNaturalDecayModelProperties>();
            (::windows::core::Interface::vtable(this).NaturalProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateNatural(mingain: f64, maxgain: f64, unitygaindistance: f64, cutoffdistance: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterDecayModel>();
            (::windows::core::Interface::vtable(this).CreateNatural)(::windows::core::Interface::as_raw(this), mingain, maxgain, unitygaindistance, cutoffdistance, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateCustom(mingain: f64, maxgain: f64) -> ::windows::core::Result<AudioNodeEmitterDecayModel> {
        Self::IAudioNodeEmitterDecayModelStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterDecayModel>();
            (::windows::core::Interface::vtable(this).CreateCustom)(::windows::core::Interface::as_raw(this), mingain, maxgain, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterDecayModelStatics<R, F: FnOnce(&IAudioNodeEmitterDecayModelStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioNodeEmitterDecayModel, IAudioNodeEmitterDecayModelStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterDecayModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterDecayModel {}
impl ::core::fmt::Debug for AudioNodeEmitterDecayModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayModel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterDecayModel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterDecayModel;{1d1d5af7-0d53-4fa9-bd84-d5816a86f3ff})");
}
impl ::core::clone::Clone for AudioNodeEmitterDecayModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterDecayModel {
    type Vtable = IAudioNodeEmitterDecayModel_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioNodeEmitterDecayModel {
    const IID: ::windows::core::GUID = <IAudioNodeEmitterDecayModel as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterDecayModel {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterDecayModel";
}
::windows::imp::interface_hierarchy!(AudioNodeEmitterDecayModel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioNodeEmitterDecayModel {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterDecayModel {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterNaturalDecayModelProperties(::windows::core::IUnknown);
impl AudioNodeEmitterNaturalDecayModelProperties {
    pub fn UnityGainDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).UnityGainDistance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CutoffDistance(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).CutoffDistance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterNaturalDecayModelProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterNaturalDecayModelProperties {}
impl ::core::fmt::Debug for AudioNodeEmitterNaturalDecayModelProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterNaturalDecayModelProperties").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterNaturalDecayModelProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties;{48934bcf-cf2c-4efc-9331-75bd22df1f0c})");
}
impl ::core::clone::Clone for AudioNodeEmitterNaturalDecayModelProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterNaturalDecayModelProperties {
    type Vtable = IAudioNodeEmitterNaturalDecayModelProperties_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioNodeEmitterNaturalDecayModelProperties {
    const IID: ::windows::core::GUID = <IAudioNodeEmitterNaturalDecayModelProperties as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterNaturalDecayModelProperties {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
}
::windows::imp::interface_hierarchy!(AudioNodeEmitterNaturalDecayModelProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioNodeEmitterNaturalDecayModelProperties {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterNaturalDecayModelProperties {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeEmitterShape(::windows::core::IUnknown);
impl AudioNodeEmitterShape {
    pub fn Kind(&self) -> ::windows::core::Result<AudioNodeEmitterShapeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterShapeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConeProperties(&self) -> ::windows::core::Result<AudioNodeEmitterConeProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterConeProperties>();
            (::windows::core::Interface::vtable(this).ConeProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateCone(innerangle: f64, outerangle: f64, outeranglegain: f64) -> ::windows::core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterShape>();
            (::windows::core::Interface::vtable(this).CreateCone)(::windows::core::Interface::as_raw(this), innerangle, outerangle, outeranglegain, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateOmnidirectional() -> ::windows::core::Result<AudioNodeEmitterShape> {
        Self::IAudioNodeEmitterShapeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitterShape>();
            (::windows::core::Interface::vtable(this).CreateOmnidirectional)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioNodeEmitterShapeStatics<R, F: FnOnce(&IAudioNodeEmitterShapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioNodeEmitterShape, IAudioNodeEmitterShapeStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioNodeEmitterShape {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeEmitterShape {}
impl ::core::fmt::Debug for AudioNodeEmitterShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShape").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterShape {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeEmitterShape;{ea0311c5-e73d-44bc-859c-45553bbc4828})");
}
impl ::core::clone::Clone for AudioNodeEmitterShape {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioNodeEmitterShape {
    type Vtable = IAudioNodeEmitterShape_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioNodeEmitterShape {
    const IID: ::windows::core::GUID = <IAudioNodeEmitterShape as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeEmitterShape {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeEmitterShape";
}
::windows::imp::interface_hierarchy!(AudioNodeEmitterShape, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioNodeEmitterShape {}
unsafe impl ::core::marker::Sync for AudioNodeEmitterShape {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioNodeListener(::windows::core::IUnknown);
impl AudioNodeListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioNodeListener, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Vector3>();
            (::windows::core::Interface::vtable(this).Position)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPosition(&self, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Orientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Quaternion>();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOrientation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpeedOfSound(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).SpeedOfSound)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpeedOfSound(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSpeedOfSound)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DopplerVelocity(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Vector3>();
            (::windows::core::Interface::vtable(this).DopplerVelocity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetDopplerVelocity(&self, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDopplerVelocity)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioNodeListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioNodeListener {}
impl ::core::fmt::Debug for AudioNodeListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeListener").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeListener {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioNodeListener;{d9722e16-0c0a-41da-b755-6c77835fb1eb})");
}
impl ::core::clone::Clone for AudioNodeListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioNodeListener {
    type Vtable = IAudioNodeListener_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioNodeListener {
    const IID: ::windows::core::GUID = <IAudioNodeListener as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioNodeListener {
    const NAME: &'static str = "Windows.Media.Audio.AudioNodeListener";
}
::windows::imp::interface_hierarchy!(AudioNodeListener, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioNodeListener {}
unsafe impl ::core::marker::Sync for AudioNodeListener {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioPlaybackConnection(::windows::core::IUnknown);
impl AudioPlaybackConnection {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).StartAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<AudioPlaybackConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioPlaybackConnectionState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Open(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioPlaybackConnectionOpenResult>();
            (::windows::core::Interface::vtable(this).Open)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<AudioPlaybackConnectionOpenResult>>();
            (::windows::core::Interface::vtable(this).OpenAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AudioPlaybackConnection, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StateChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TryCreateFromId(id: &::windows::core::HSTRING) -> ::windows::core::Result<AudioPlaybackConnection> {
        Self::IAudioPlaybackConnectionStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioPlaybackConnection>();
            (::windows::core::Interface::vtable(this).TryCreateFromId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAudioPlaybackConnectionStatics<R, F: FnOnce(&IAudioPlaybackConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioPlaybackConnection, IAudioPlaybackConnectionStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioPlaybackConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioPlaybackConnection {}
impl ::core::fmt::Debug for AudioPlaybackConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnection").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioPlaybackConnection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnection;{1a4c1dea-cafc-50e7-8718-ea3f81cbfa51})");
}
impl ::core::clone::Clone for AudioPlaybackConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioPlaybackConnection {
    type Vtable = IAudioPlaybackConnection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioPlaybackConnection {
    const IID: ::windows::core::GUID = <IAudioPlaybackConnection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioPlaybackConnection {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnection";
}
::windows::imp::interface_hierarchy!(AudioPlaybackConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioPlaybackConnection {}
unsafe impl ::core::marker::Send for AudioPlaybackConnection {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnection {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioPlaybackConnectionOpenResult(::windows::core::IUnknown);
impl AudioPlaybackConnectionOpenResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioPlaybackConnectionOpenResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioPlaybackConnectionOpenResultStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioPlaybackConnectionOpenResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioPlaybackConnectionOpenResult {}
impl ::core::fmt::Debug for AudioPlaybackConnectionOpenResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioPlaybackConnectionOpenResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioPlaybackConnectionOpenResult;{4e656aef-39f9-5fc9-a519-a5bbfd9fe921})");
}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioPlaybackConnectionOpenResult {
    type Vtable = IAudioPlaybackConnectionOpenResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioPlaybackConnectionOpenResult {
    const IID: ::windows::core::GUID = <IAudioPlaybackConnectionOpenResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioPlaybackConnectionOpenResult {
    const NAME: &'static str = "Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
}
::windows::imp::interface_hierarchy!(AudioPlaybackConnectionOpenResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioPlaybackConnectionOpenResult {}
unsafe impl ::core::marker::Sync for AudioPlaybackConnectionOpenResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioStateMonitor(::windows::core::IUnknown);
impl AudioStateMonitor {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AudioStateMonitor, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SoundLevelChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn SoundLevel(&self) -> ::windows::core::Result<super::SoundLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::SoundLevel>();
            (::windows::core::Interface::vtable(this).SoundLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateForRenderMonitoring() -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoring)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategory(category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoringWithCategory)(::windows::core::Interface::as_raw(this), category, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Devices\"`, `\"Media_Render\"`*"]
    #[cfg(all(feature = "Media_Devices", feature = "Media_Render"))]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceRole(category: super::Render::AudioRenderCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceRole)(::windows::core::Interface::as_raw(this), category, role, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Render\"`*"]
    #[cfg(feature = "Media_Render")]
    pub fn CreateForRenderMonitoringWithCategoryAndDeviceId(category: super::Render::AudioRenderCategory, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForRenderMonitoringWithCategoryAndDeviceId)(::windows::core::Interface::as_raw(this), category, ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateForCaptureMonitoring() -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoring)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategory(category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoringWithCategory)(::windows::core::Interface::as_raw(this), category, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices"))]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceRole(category: super::Capture::MediaCategory, role: super::Devices::AudioDeviceRole) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceRole)(::windows::core::Interface::as_raw(this), category, role, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn CreateForCaptureMonitoringWithCategoryAndDeviceId(category: super::Capture::MediaCategory, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioStateMonitor> {
        Self::IAudioStateMonitorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioStateMonitor>();
            (::windows::core::Interface::vtable(this).CreateForCaptureMonitoringWithCategoryAndDeviceId)(::windows::core::Interface::as_raw(this), category, ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAudioStateMonitorStatics<R, F: FnOnce(&IAudioStateMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AudioStateMonitor, IAudioStateMonitorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioStateMonitor {}
impl ::core::fmt::Debug for AudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitor").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioStateMonitor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioStateMonitor;{1d13d136-0199-4cdc-b84e-e72c2b581ece})");
}
impl ::core::clone::Clone for AudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioStateMonitor {
    const IID: ::windows::core::GUID = <IAudioStateMonitor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioStateMonitor {
    const NAME: &'static str = "Windows.Media.Audio.AudioStateMonitor";
}
::windows::imp::interface_hierarchy!(AudioStateMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AudioStateMonitor {}
unsafe impl ::core::marker::Sync for AudioStateMonitor {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct AudioSubmixNode(::windows::core::IUnknown);
impl AudioSubmixNode {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).Emitter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for AudioSubmixNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioSubmixNode {}
impl ::core::fmt::Debug for AudioSubmixNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSubmixNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioSubmixNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.AudioSubmixNode;{d148005c-8428-4784-b7fd-a99d468c5d20})");
}
impl ::core::clone::Clone for AudioSubmixNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AudioSubmixNode {
    type Vtable = IAudioInputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AudioSubmixNode {
    const IID: ::windows::core::GUID = <IAudioInputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AudioSubmixNode {
    const NAME: &'static str = "Windows.Media.Audio.AudioSubmixNode";
}
::windows::imp::interface_hierarchy!(AudioSubmixNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioInputNode> for AudioSubmixNode {}
impl ::windows::core::CanTryInto<IAudioInputNode2> for AudioSubmixNode {}
impl ::windows::core::CanTryInto<IAudioNode> for AudioSubmixNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for AudioSubmixNode {}
unsafe impl ::core::marker::Send for AudioSubmixNode {}
unsafe impl ::core::marker::Sync for AudioSubmixNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioDeviceInputNodeResult(::windows::core::IUnknown);
impl CreateAudioDeviceInputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioDeviceNodeCreationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceInputNode(&self) -> ::windows::core::Result<AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioDeviceInputNode>();
            (::windows::core::Interface::vtable(this).DeviceInputNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<ICreateAudioDeviceInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateAudioDeviceInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioDeviceInputNodeResult {}
impl ::core::fmt::Debug for CreateAudioDeviceInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioDeviceInputNodeResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateAudioDeviceInputNodeResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceInputNodeResult;{16eec7a8-1ca7-40ef-91a4-d346e0aa1bba})");
}
impl ::core::clone::Clone for CreateAudioDeviceInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateAudioDeviceInputNodeResult {
    type Vtable = ICreateAudioDeviceInputNodeResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateAudioDeviceInputNodeResult {
    const IID: ::windows::core::GUID = <ICreateAudioDeviceInputNodeResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioDeviceInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
}
::windows::imp::interface_hierarchy!(CreateAudioDeviceInputNodeResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateAudioDeviceInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceInputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioDeviceOutputNodeResult(::windows::core::IUnknown);
impl CreateAudioDeviceOutputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioDeviceNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioDeviceNodeCreationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceOutputNode(&self) -> ::windows::core::Result<AudioDeviceOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioDeviceOutputNode>();
            (::windows::core::Interface::vtable(this).DeviceOutputNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateAudioDeviceOutputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioDeviceOutputNodeResult {}
impl ::core::fmt::Debug for CreateAudioDeviceOutputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioDeviceOutputNodeResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateAudioDeviceOutputNodeResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioDeviceOutputNodeResult;{f7776d27-1d9a-47f7-9cd4-2859cc1b7bff})");
}
impl ::core::clone::Clone for CreateAudioDeviceOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateAudioDeviceOutputNodeResult {
    type Vtable = ICreateAudioDeviceOutputNodeResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateAudioDeviceOutputNodeResult {
    const IID: ::windows::core::GUID = <ICreateAudioDeviceOutputNodeResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioDeviceOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
}
::windows::imp::interface_hierarchy!(CreateAudioDeviceOutputNodeResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateAudioDeviceOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioDeviceOutputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioFileInputNodeResult(::windows::core::IUnknown);
impl CreateAudioFileInputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFileNodeCreationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FileInputNode(&self) -> ::windows::core::Result<AudioFileInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFileInputNode>();
            (::windows::core::Interface::vtable(this).FileInputNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<ICreateAudioFileInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateAudioFileInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioFileInputNodeResult {}
impl ::core::fmt::Debug for CreateAudioFileInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioFileInputNodeResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateAudioFileInputNodeResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileInputNodeResult;{ce83d61c-e297-4c50-9ce7-1c7a69d6bd09})");
}
impl ::core::clone::Clone for CreateAudioFileInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateAudioFileInputNodeResult {
    type Vtable = ICreateAudioFileInputNodeResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateAudioFileInputNodeResult {
    const IID: ::windows::core::GUID = <ICreateAudioFileInputNodeResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioFileInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
}
::windows::imp::interface_hierarchy!(CreateAudioFileInputNodeResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateAudioFileInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileInputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioFileOutputNodeResult(::windows::core::IUnknown);
impl CreateAudioFileOutputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioFileNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFileNodeCreationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FileOutputNode(&self) -> ::windows::core::Result<AudioFileOutputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioFileOutputNode>();
            (::windows::core::Interface::vtable(this).FileOutputNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<ICreateAudioFileOutputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateAudioFileOutputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioFileOutputNodeResult {}
impl ::core::fmt::Debug for CreateAudioFileOutputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioFileOutputNodeResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateAudioFileOutputNodeResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioFileOutputNodeResult;{47d6ba7b-e909-453f-866e-5540cda734ff})");
}
impl ::core::clone::Clone for CreateAudioFileOutputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateAudioFileOutputNodeResult {
    type Vtable = ICreateAudioFileOutputNodeResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateAudioFileOutputNodeResult {
    const IID: ::windows::core::GUID = <ICreateAudioFileOutputNodeResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioFileOutputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileOutputNodeResult";
}
::windows::imp::interface_hierarchy!(CreateAudioFileOutputNodeResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateAudioFileOutputNodeResult {}
unsafe impl ::core::marker::Sync for CreateAudioFileOutputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateAudioGraphResult(::windows::core::IUnknown);
impl CreateAudioGraphResult {
    pub fn Status(&self) -> ::windows::core::Result<AudioGraphCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioGraphCreationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Graph(&self) -> ::windows::core::Result<AudioGraph> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioGraph>();
            (::windows::core::Interface::vtable(this).Graph)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<ICreateAudioGraphResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateAudioGraphResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateAudioGraphResult {}
impl ::core::fmt::Debug for CreateAudioGraphResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateAudioGraphResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateAudioGraphResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateAudioGraphResult;{5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b})");
}
impl ::core::clone::Clone for CreateAudioGraphResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateAudioGraphResult {
    type Vtable = ICreateAudioGraphResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateAudioGraphResult {
    const IID: ::windows::core::GUID = <ICreateAudioGraphResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateAudioGraphResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
}
::windows::imp::interface_hierarchy!(CreateAudioGraphResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateAudioGraphResult {}
unsafe impl ::core::marker::Sync for CreateAudioGraphResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct CreateMediaSourceAudioInputNodeResult(::windows::core::IUnknown);
impl CreateMediaSourceAudioInputNodeResult {
    pub fn Status(&self) -> ::windows::core::Result<MediaSourceAudioInputNodeCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MediaSourceAudioInputNodeCreationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Node(&self) -> ::windows::core::Result<MediaSourceAudioInputNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MediaSourceAudioInputNode>();
            (::windows::core::Interface::vtable(this).Node)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<ICreateMediaSourceAudioInputNodeResult2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CreateMediaSourceAudioInputNodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateMediaSourceAudioInputNodeResult {}
impl ::core::fmt::Debug for CreateMediaSourceAudioInputNodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateMediaSourceAudioInputNodeResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CreateMediaSourceAudioInputNodeResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult;{46a658a3-53c0-4d59-9e51-cc1d1044a4c4})");
}
impl ::core::clone::Clone for CreateMediaSourceAudioInputNodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CreateMediaSourceAudioInputNodeResult {
    type Vtable = ICreateMediaSourceAudioInputNodeResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CreateMediaSourceAudioInputNodeResult {
    const IID: ::windows::core::GUID = <ICreateMediaSourceAudioInputNodeResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CreateMediaSourceAudioInputNodeResult {
    const NAME: &'static str = "Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
}
::windows::imp::interface_hierarchy!(CreateMediaSourceAudioInputNodeResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CreateMediaSourceAudioInputNodeResult {}
unsafe impl ::core::marker::Sync for CreateMediaSourceAudioInputNodeResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct EchoEffectDefinition(::windows::core::IUnknown);
impl EchoEffectDefinition {
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWetDryMix)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).WetDryMix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFeedback(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFeedback)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Feedback(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Feedback)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDelay(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Delay(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Delay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(audiograph: &AudioGraph) -> ::windows::core::Result<EchoEffectDefinition> {
        Self::IEchoEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EchoEffectDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(audiograph), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEchoEffectDefinitionFactory<R, F: FnOnce(&IEchoEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EchoEffectDefinition, IEchoEffectDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EchoEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EchoEffectDefinition {}
impl ::core::fmt::Debug for EchoEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EchoEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EchoEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EchoEffectDefinition;{0e4d3faa-36b8-4c91-b9da-11f44a8a6610})");
}
impl ::core::clone::Clone for EchoEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EchoEffectDefinition {
    type Vtable = IEchoEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EchoEffectDefinition {
    const IID: ::windows::core::GUID = <IEchoEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EchoEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EchoEffectDefinition";
}
::windows::imp::interface_hierarchy!(EchoEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows::core::CanTryInto<super::Effects::IAudioEffectDefinition> for EchoEffectDefinition {}
unsafe impl ::core::marker::Send for EchoEffectDefinition {}
unsafe impl ::core::marker::Sync for EchoEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct EqualizerBand(::windows::core::IUnknown);
impl EqualizerBand {
    pub fn Bandwidth(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Bandwidth)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBandwidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBandwidth)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn FrequencyCenter(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).FrequencyCenter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFrequencyCenter(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFrequencyCenter)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Gain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Gain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for EqualizerBand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EqualizerBand {}
impl ::core::fmt::Debug for EqualizerBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EqualizerBand").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EqualizerBand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerBand;{c00a5a6a-262d-4b85-9bb7-43280b62ed0c})");
}
impl ::core::clone::Clone for EqualizerBand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EqualizerBand {
    type Vtable = IEqualizerBand_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EqualizerBand {
    const IID: ::windows::core::GUID = <IEqualizerBand as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EqualizerBand {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerBand";
}
::windows::imp::interface_hierarchy!(EqualizerBand, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EqualizerBand {}
unsafe impl ::core::marker::Sync for EqualizerBand {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct EqualizerEffectDefinition(::windows::core::IUnknown);
impl EqualizerEffectDefinition {
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EqualizerBand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<EqualizerBand>>();
            (::windows::core::Interface::vtable(this).Bands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(audiograph: &AudioGraph) -> ::windows::core::Result<EqualizerEffectDefinition> {
        Self::IEqualizerEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EqualizerEffectDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(audiograph), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEqualizerEffectDefinitionFactory<R, F: FnOnce(&IEqualizerEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EqualizerEffectDefinition, IEqualizerEffectDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EqualizerEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EqualizerEffectDefinition {}
impl ::core::fmt::Debug for EqualizerEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EqualizerEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EqualizerEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.EqualizerEffectDefinition;{023f6f1f-83fe-449a-a822-c696442d16b0})");
}
impl ::core::clone::Clone for EqualizerEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EqualizerEffectDefinition {
    type Vtable = IEqualizerEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EqualizerEffectDefinition {
    const IID: ::windows::core::GUID = <IEqualizerEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EqualizerEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.EqualizerEffectDefinition";
}
::windows::imp::interface_hierarchy!(EqualizerEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows::core::CanTryInto<super::Effects::IAudioEffectDefinition> for EqualizerEffectDefinition {}
unsafe impl ::core::marker::Send for EqualizerEffectDefinition {}
unsafe impl ::core::marker::Sync for EqualizerEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct FrameInputNodeQuantumStartedEventArgs(::windows::core::IUnknown);
impl FrameInputNodeQuantumStartedEventArgs {
    pub fn RequiredSamples(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).RequiredSamples)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FrameInputNodeQuantumStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameInputNodeQuantumStartedEventArgs {}
impl ::core::fmt::Debug for FrameInputNodeQuantumStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameInputNodeQuantumStartedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FrameInputNodeQuantumStartedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs;{3d9bd498-a306-4f06-bd9f-e9efc8226304})");
}
impl ::core::clone::Clone for FrameInputNodeQuantumStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FrameInputNodeQuantumStartedEventArgs {
    type Vtable = IFrameInputNodeQuantumStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FrameInputNodeQuantumStartedEventArgs {
    const IID: ::windows::core::GUID = <IFrameInputNodeQuantumStartedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FrameInputNodeQuantumStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
}
::windows::imp::interface_hierarchy!(FrameInputNodeQuantumStartedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for FrameInputNodeQuantumStartedEventArgs {}
unsafe impl ::core::marker::Sync for FrameInputNodeQuantumStartedEventArgs {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct LimiterEffectDefinition(::windows::core::IUnknown);
impl LimiterEffectDefinition {
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRelease(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelease)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Release(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Release)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLoudness(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLoudness)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Loudness(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Loudness)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(audiograph: &AudioGraph) -> ::windows::core::Result<LimiterEffectDefinition> {
        Self::ILimiterEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LimiterEffectDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(audiograph), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILimiterEffectDefinitionFactory<R, F: FnOnce(&ILimiterEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LimiterEffectDefinition, ILimiterEffectDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LimiterEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LimiterEffectDefinition {}
impl ::core::fmt::Debug for LimiterEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LimiterEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LimiterEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.LimiterEffectDefinition;{6b755d19-2603-47ba-bdeb-39055e3486dc})");
}
impl ::core::clone::Clone for LimiterEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LimiterEffectDefinition {
    type Vtable = ILimiterEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LimiterEffectDefinition {
    const IID: ::windows::core::GUID = <ILimiterEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LimiterEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.LimiterEffectDefinition";
}
::windows::imp::interface_hierarchy!(LimiterEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows::core::CanTryInto<super::Effects::IAudioEffectDefinition> for LimiterEffectDefinition {}
unsafe impl ::core::marker::Send for LimiterEffectDefinition {}
unsafe impl ::core::marker::Sync for LimiterEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct MediaSourceAudioInputNode(::windows::core::IUnknown);
impl MediaSourceAudioInputNode {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutgoingConnections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AudioGraphConnection>>();
            (::windows::core::Interface::vtable(this).OutgoingConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn AddOutgoingConnectionWithGain<P0>(&self, destination: P0, gain: f64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddOutgoingConnectionWithGain)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi(), gain).ok() }
    }
    pub fn RemoveOutgoingConnection<P0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IAudioNode>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOutgoingConnection)(::windows::core::Interface::as_raw(this), destination.try_into_param()?.abi()).ok() }
    }
    pub fn Emitter(&self) -> ::windows::core::Result<AudioNodeEmitter> {
        let this = &::windows::core::ComInterface::cast::<IAudioInputNode2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AudioNodeEmitter>();
            (::windows::core::Interface::vtable(this).Emitter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn EffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>();
            (::windows::core::Interface::vtable(this).EffectDefinitions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutgoingGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutgoingGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).OutgoingGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::MediaProperties::AudioEncodingProperties>();
            (::windows::core::Interface::vtable(this).EncodingProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConsumeInput(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ConsumeInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetConsumeInput(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetConsumeInput)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn DisableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DisableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn EnableEffectsByDefinition<P0>(&self, definition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::Effects::IAudioEffectDefinition>,
    {
        let this = &::windows::core::ComInterface::cast::<IAudioNode>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).EnableEffectsByDefinition)(::windows::core::Interface::as_raw(this), definition.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).PlaybackSpeedFactor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
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
    pub fn Seek(&self, position: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::windows::core::Interface::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).EndTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoopCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<i32>>();
            (::windows::core::Interface::vtable(this).LoopCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLoopCount<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLoopCount)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn MediaSource(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Core::MediaSource>();
            (::windows::core::Interface::vtable(this).MediaSource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaSourceCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<MediaSourceAudioInputNode, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MediaSourceCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaSourceCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMediaSourceCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaSourceAudioInputNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceAudioInputNode {}
impl ::core::fmt::Debug for MediaSourceAudioInputNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaSourceAudioInputNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.MediaSourceAudioInputNode;{99d8983b-a88a-4041-8e4f-ddbac0c91fd3})");
}
impl ::core::clone::Clone for MediaSourceAudioInputNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MediaSourceAudioInputNode {
    type Vtable = IMediaSourceAudioInputNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MediaSourceAudioInputNode {
    const IID: ::windows::core::GUID = <IMediaSourceAudioInputNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MediaSourceAudioInputNode {
    const NAME: &'static str = "Windows.Media.Audio.MediaSourceAudioInputNode";
}
::windows::imp::interface_hierarchy!(MediaSourceAudioInputNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IAudioInputNode> for MediaSourceAudioInputNode {}
impl ::windows::core::CanTryInto<IAudioInputNode2> for MediaSourceAudioInputNode {}
impl ::windows::core::CanTryInto<IAudioNode> for MediaSourceAudioInputNode {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for MediaSourceAudioInputNode {}
unsafe impl ::core::marker::Send for MediaSourceAudioInputNode {}
unsafe impl ::core::marker::Sync for MediaSourceAudioInputNode {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct ReverbEffectDefinition(::windows::core::IUnknown);
impl ReverbEffectDefinition {
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActivatableClassId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::ComInterface::cast::<super::Effects::IAudioEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWetDryMix(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWetDryMix)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn WetDryMix(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).WetDryMix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReflectionsDelay(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReflectionsDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectionsDelay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReflectionsDelay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReverbDelay(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReverbDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverbDelay(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).ReverbDelay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRearDelay(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRearDelay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RearDelay(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).RearDelay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionLeft(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionLeft)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionLeft(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).PositionLeft)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionRight(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionRight)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionRight(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).PositionRight)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionMatrixLeft(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionMatrixLeft)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionMatrixLeft(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).PositionMatrixLeft)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPositionMatrixRight(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPositionMatrixRight)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PositionMatrixRight(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).PositionMatrixRight)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEarlyDiffusion(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEarlyDiffusion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EarlyDiffusion(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).EarlyDiffusion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLateDiffusion(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLateDiffusion)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn LateDiffusion(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).LateDiffusion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLowEQGain(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLowEQGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowEQGain(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).LowEQGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLowEQCutoff(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLowEQCutoff)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn LowEQCutoff(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).LowEQCutoff)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHighEQGain(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHighEQGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighEQGain(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).HighEQGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHighEQCutoff(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHighEQCutoff)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HighEQCutoff(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).HighEQCutoff)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoomFilterFreq(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomFilterFreq)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterFreq(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).RoomFilterFreq)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoomFilterMain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomFilterMain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterMain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).RoomFilterMain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoomFilterHF(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomFilterHF)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomFilterHF(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).RoomFilterHF)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReflectionsGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReflectionsGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectionsGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ReflectionsGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReverbGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReverbGain)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReverbGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ReverbGain)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDecayTime(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDecayTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecayTime(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).DecayTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDensity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDensity)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Density(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Density)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoomSize(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRoomSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoomSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).RoomSize)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisableLateField(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisableLateField)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisableLateField(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DisableLateField)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(audiograph: &AudioGraph) -> ::windows::core::Result<ReverbEffectDefinition> {
        Self::IReverbEffectDefinitionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ReverbEffectDefinition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(audiograph), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IReverbEffectDefinitionFactory<R, F: FnOnce(&IReverbEffectDefinitionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ReverbEffectDefinition, IReverbEffectDefinitionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ReverbEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReverbEffectDefinition {}
impl ::core::fmt::Debug for ReverbEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReverbEffectDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ReverbEffectDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.ReverbEffectDefinition;{4606aa89-f563-4d0a-8f6e-f0cddff35d84})");
}
impl ::core::clone::Clone for ReverbEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ReverbEffectDefinition {
    type Vtable = IReverbEffectDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ReverbEffectDefinition {
    const IID: ::windows::core::GUID = <IReverbEffectDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ReverbEffectDefinition {
    const NAME: &'static str = "Windows.Media.Audio.ReverbEffectDefinition";
}
::windows::imp::interface_hierarchy!(ReverbEffectDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows::core::CanTryInto<super::Effects::IAudioEffectDefinition> for ReverbEffectDefinition {}
unsafe impl ::core::marker::Send for ReverbEffectDefinition {}
unsafe impl ::core::marker::Sync for ReverbEffectDefinition {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SetDefaultSpatialAudioFormatResult(::windows::core::IUnknown);
impl SetDefaultSpatialAudioFormatResult {
    pub fn Status(&self) -> ::windows::core::Result<SetDefaultSpatialAudioFormatStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SetDefaultSpatialAudioFormatStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SetDefaultSpatialAudioFormatResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SetDefaultSpatialAudioFormatResult {}
impl ::core::fmt::Debug for SetDefaultSpatialAudioFormatResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SetDefaultSpatialAudioFormatResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SetDefaultSpatialAudioFormatResult;{1c2aa511-1400-5e70-9ea9-ae151241e8ea})");
}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SetDefaultSpatialAudioFormatResult {
    type Vtable = ISetDefaultSpatialAudioFormatResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SetDefaultSpatialAudioFormatResult {
    const IID: ::windows::core::GUID = <ISetDefaultSpatialAudioFormatResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SetDefaultSpatialAudioFormatResult {
    const NAME: &'static str = "Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
}
::windows::imp::interface_hierarchy!(SetDefaultSpatialAudioFormatResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SetDefaultSpatialAudioFormatResult {}
unsafe impl ::core::marker::Sync for SetDefaultSpatialAudioFormatResult {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SpatialAudioDeviceConfiguration(::windows::core::IUnknown);
impl SpatialAudioDeviceConfiguration {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSpatialAudioSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSpatialAudioSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSpatialAudioFormatSupported(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSpatialAudioFormatSupported)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), &mut result__).from_abi(result__)
        }
    }
    pub fn ActiveSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActiveSpatialAudioFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultSpatialAudioFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DefaultSpatialAudioFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultSpatialAudioFormatAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SetDefaultSpatialAudioFormatResult>>();
            (::windows::core::Interface::vtable(this).SetDefaultSpatialAudioFormatAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConfigurationChanged(&self, handler: &super::super::Foundation::TypedEventHandler<SpatialAudioDeviceConfiguration, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ConfigurationChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConfigurationChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForDeviceId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<SpatialAudioDeviceConfiguration> {
        Self::ISpatialAudioDeviceConfigurationStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpatialAudioDeviceConfiguration>();
            (::windows::core::Interface::vtable(this).GetForDeviceId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioDeviceConfigurationStatics<R, F: FnOnce(&ISpatialAudioDeviceConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpatialAudioDeviceConfiguration, ISpatialAudioDeviceConfigurationStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpatialAudioDeviceConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAudioDeviceConfiguration {}
impl ::core::fmt::Debug for SpatialAudioDeviceConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioDeviceConfiguration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpatialAudioDeviceConfiguration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioDeviceConfiguration;{ee830034-61cf-5749-9da4-10f0fe028199})");
}
impl ::core::clone::Clone for SpatialAudioDeviceConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpatialAudioDeviceConfiguration {
    type Vtable = ISpatialAudioDeviceConfiguration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpatialAudioDeviceConfiguration {
    const IID: ::windows::core::GUID = <ISpatialAudioDeviceConfiguration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAudioDeviceConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioDeviceConfiguration";
}
::windows::imp::interface_hierarchy!(SpatialAudioDeviceConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAudioDeviceConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioDeviceConfiguration {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
pub struct SpatialAudioFormatConfiguration(::windows::core::IUnknown);
impl SpatialAudioFormatConfiguration {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportLicenseChangedAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ReportLicenseChangedAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportConfigurationChangedAsync(&self, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ReportConfigurationChangedAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), &mut result__).from_abi(result__)
        }
    }
    pub fn MixedRealityExclusiveModePolicy(&self) -> ::windows::core::Result<MixedRealitySpatialAudioFormatPolicy> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MixedRealitySpatialAudioFormatPolicy>();
            (::windows::core::Interface::vtable(this).MixedRealityExclusiveModePolicy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMixedRealityExclusiveModePolicy(&self, value: MixedRealitySpatialAudioFormatPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMixedRealityExclusiveModePolicy)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<SpatialAudioFormatConfiguration> {
        Self::ISpatialAudioFormatConfigurationStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SpatialAudioFormatConfiguration>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatConfigurationStatics<R, F: FnOnce(&ISpatialAudioFormatConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpatialAudioFormatConfiguration, ISpatialAudioFormatConfigurationStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpatialAudioFormatConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAudioFormatConfiguration {}
impl ::core::fmt::Debug for SpatialAudioFormatConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioFormatConfiguration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpatialAudioFormatConfiguration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Audio.SpatialAudioFormatConfiguration;{32df09a8-50f0-5395-9923-7d44ca71ed6d})");
}
impl ::core::clone::Clone for SpatialAudioFormatConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpatialAudioFormatConfiguration {
    type Vtable = ISpatialAudioFormatConfiguration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpatialAudioFormatConfiguration {
    const IID: ::windows::core::GUID = <ISpatialAudioFormatConfiguration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpatialAudioFormatConfiguration {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatConfiguration";
}
::windows::imp::interface_hierarchy!(SpatialAudioFormatConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialAudioFormatConfiguration {}
unsafe impl ::core::marker::Sync for SpatialAudioFormatConfiguration {}
#[doc = "*Required features: `\"Media_Audio\"`*"]
pub struct SpatialAudioFormatSubtype;
impl SpatialAudioFormatSubtype {
    pub fn WindowsSonic() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WindowsSonic)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DolbyAtmosForHeadphones() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DolbyAtmosForHeadphones)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DolbyAtmosForHomeTheater() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DolbyAtmosForHomeTheater)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DolbyAtmosForSpeakers() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DolbyAtmosForSpeakers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DTSHeadphoneX() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DTSHeadphoneX)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DTSXUltra() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DTSXUltra)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DTSXForHomeTheater() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ISpatialAudioFormatSubtypeStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DTSXForHomeTheater)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatSubtypeStatics<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialAudioFormatSubtypeStatics2<R, F: FnOnce(&ISpatialAudioFormatSubtypeStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpatialAudioFormatSubtype, ISpatialAudioFormatSubtypeStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for SpatialAudioFormatSubtype {
    const NAME: &'static str = "Windows.Media.Audio.SpatialAudioFormatSubtype";
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioDeviceNodeCreationStatus(pub i32);
impl AudioDeviceNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioDeviceNodeCreationStatus {}
impl ::core::clone::Clone for AudioDeviceNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDeviceNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioDeviceNodeCreationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioDeviceNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDeviceNodeCreationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioDeviceNodeCreationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioDeviceNodeCreationStatus;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioFileNodeCreationStatus(pub i32);
impl AudioFileNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FileNotFound: Self = Self(1i32);
    pub const InvalidFileType: Self = Self(2i32);
    pub const FormatNotSupported: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AudioFileNodeCreationStatus {}
impl ::core::clone::Clone for AudioFileNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioFileNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioFileNodeCreationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioFileNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFileNodeCreationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioFileNodeCreationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioFileNodeCreationStatus;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioGraphCreationStatus(pub i32);
impl AudioGraphCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const DeviceNotAvailable: Self = Self(1i32);
    pub const FormatNotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphCreationStatus {}
impl ::core::clone::Clone for AudioGraphCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioGraphCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioGraphCreationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioGraphCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphCreationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioGraphCreationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphCreationStatus;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioGraphUnrecoverableError(pub i32);
impl AudioGraphUnrecoverableError {
    pub const None: Self = Self(0i32);
    pub const AudioDeviceLost: Self = Self(1i32);
    pub const AudioSessionDisconnected: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioGraphUnrecoverableError {}
impl ::core::clone::Clone for AudioGraphUnrecoverableError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioGraphUnrecoverableError {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioGraphUnrecoverableError {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioGraphUnrecoverableError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioGraphUnrecoverableError").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioGraphUnrecoverableError {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioGraphUnrecoverableError;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioNodeEmitterDecayKind(pub i32);
impl AudioNodeEmitterDecayKind {
    pub const Natural: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterDecayKind {}
impl ::core::clone::Clone for AudioNodeEmitterDecayKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterDecayKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioNodeEmitterDecayKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioNodeEmitterDecayKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterDecayKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterDecayKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterDecayKind;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioNodeEmitterSettings(pub u32);
impl AudioNodeEmitterSettings {
    pub const None: Self = Self(0u32);
    pub const DisableDoppler: Self = Self(1u32);
}
impl ::core::marker::Copy for AudioNodeEmitterSettings {}
impl ::core::clone::Clone for AudioNodeEmitterSettings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterSettings {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioNodeEmitterSettings {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioNodeEmitterSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterSettings").field(&self.0).finish()
    }
}
impl AudioNodeEmitterSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioNodeEmitterSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioNodeEmitterSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioNodeEmitterSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioNodeEmitterSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterSettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterSettings;u4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioNodeEmitterShapeKind(pub i32);
impl AudioNodeEmitterShapeKind {
    pub const Omnidirectional: Self = Self(0i32);
    pub const Cone: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioNodeEmitterShapeKind {}
impl ::core::clone::Clone for AudioNodeEmitterShapeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioNodeEmitterShapeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioNodeEmitterShapeKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioNodeEmitterShapeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioNodeEmitterShapeKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioNodeEmitterShapeKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioNodeEmitterShapeKind;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioPlaybackConnectionOpenResultStatus(pub i32);
impl AudioPlaybackConnectionOpenResultStatus {
    pub const Success: Self = Self(0i32);
    pub const RequestTimedOut: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionOpenResultStatus {}
impl ::core::clone::Clone for AudioPlaybackConnectionOpenResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioPlaybackConnectionOpenResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioPlaybackConnectionOpenResultStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioPlaybackConnectionOpenResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionOpenResultStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioPlaybackConnectionOpenResultStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioPlaybackConnectionState(pub i32);
impl AudioPlaybackConnectionState {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioPlaybackConnectionState {}
impl ::core::clone::Clone for AudioPlaybackConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioPlaybackConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AudioPlaybackConnectionState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AudioPlaybackConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioPlaybackConnectionState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AudioPlaybackConnectionState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.AudioPlaybackConnectionState;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaSourceAudioInputNodeCreationStatus(pub i32);
impl MediaSourceAudioInputNodeCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaSourceAudioInputNodeCreationStatus {}
impl ::core::clone::Clone for MediaSourceAudioInputNodeCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceAudioInputNodeCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MediaSourceAudioInputNodeCreationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MediaSourceAudioInputNodeCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAudioInputNodeCreationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaSourceAudioInputNodeCreationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MixedRealitySpatialAudioFormatPolicy(pub i32);
impl MixedRealitySpatialAudioFormatPolicy {
    pub const UseMixedRealityDefaultSpatialAudioFormat: Self = Self(0i32);
    pub const UseDeviceConfigurationDefaultSpatialAudioFormat: Self = Self(1i32);
}
impl ::core::marker::Copy for MixedRealitySpatialAudioFormatPolicy {}
impl ::core::clone::Clone for MixedRealitySpatialAudioFormatPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MixedRealitySpatialAudioFormatPolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MixedRealitySpatialAudioFormatPolicy {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MixedRealitySpatialAudioFormatPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MixedRealitySpatialAudioFormatPolicy").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MixedRealitySpatialAudioFormatPolicy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QuantumSizeSelectionMode(pub i32);
impl QuantumSizeSelectionMode {
    pub const SystemDefault: Self = Self(0i32);
    pub const LowestLatency: Self = Self(1i32);
    pub const ClosestToDesired: Self = Self(2i32);
}
impl ::core::marker::Copy for QuantumSizeSelectionMode {}
impl ::core::clone::Clone for QuantumSizeSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QuantumSizeSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for QuantumSizeSelectionMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for QuantumSizeSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuantumSizeSelectionMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for QuantumSizeSelectionMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.QuantumSizeSelectionMode;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SetDefaultSpatialAudioFormatStatus(pub i32);
impl SetDefaultSpatialAudioFormatStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const LicenseExpired: Self = Self(2i32);
    pub const LicenseNotValidForAudioEndpoint: Self = Self(3i32);
    pub const NotSupportedOnAudioEndpoint: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for SetDefaultSpatialAudioFormatStatus {}
impl ::core::clone::Clone for SetDefaultSpatialAudioFormatStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SetDefaultSpatialAudioFormatStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SetDefaultSpatialAudioFormatStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SetDefaultSpatialAudioFormatStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SetDefaultSpatialAudioFormatStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SetDefaultSpatialAudioFormatStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus;i4)");
}
#[doc = "*Required features: `\"Media_Audio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpatialAudioModel(pub i32);
impl SpatialAudioModel {
    pub const ObjectBased: Self = Self(0i32);
    pub const FoldDown: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAudioModel {}
impl ::core::clone::Clone for SpatialAudioModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioModel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpatialAudioModel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpatialAudioModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioModel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpatialAudioModel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Audio.SpatialAudioModel;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
