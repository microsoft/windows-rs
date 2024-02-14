#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
::windows_core::imp::com_interface!(IAudioStreamDescriptor, IAudioStreamDescriptor_Vtbl, 0x1e3692e4_4027_4847_a70b_df1d9a2a7b04);
#[repr(C)]
pub struct IAudioStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
::windows_core::imp::com_interface!(IAudioStreamDescriptor2, IAudioStreamDescriptor2_Vtbl, 0x2e68f1f6_a448_497b_8840_85082665acf9);
#[repr(C)]
pub struct IAudioStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetLeadingEncoderPadding: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LeadingEncoderPadding: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTrailingEncoderPadding: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TrailingEncoderPadding: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioStreamDescriptor3, IAudioStreamDescriptor3_Vtbl, 0x4d220da1_8e83_44ef_8973_2f63e993f36b);
#[repr(C)]
pub struct IAudioStreamDescriptor3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioStreamDescriptorFactory, IAudioStreamDescriptorFactory_Vtbl, 0x4a86ce9e_4cb1_4380_8e0c_83504b7f5bf3);
#[repr(C)]
pub struct IAudioStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
::windows_core::imp::com_interface!(IAudioTrack, IAudioTrack_Vtbl, 0xf23b6e77_3ef7_40de_b943_068b1321701d);
#[repr(C)]
pub struct IAudioTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpenFailed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenFailed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetEncodingProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioTrackOpenFailedEventArgs, IAudioTrackOpenFailedEventArgs_Vtbl, 0xeeddb9b9_bb7c_4112_bf76_9384676f824b);
#[repr(C)]
pub struct IAudioTrackOpenFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioTrackSupportInfo, IAudioTrackSupportInfo_Vtbl, 0x178beff7_cc39_44a6_b951_4a5653f073fa);
#[repr(C)]
pub struct IAudioTrackSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DecoderStatus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaDecoderStatus) -> ::windows_core::HRESULT,
    pub Degradation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut AudioDecoderDegradation) -> ::windows_core::HRESULT,
    pub DegradationReason: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut AudioDecoderDegradationReason) -> ::windows_core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaSourceStatus) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IChapterCue, IChapterCue_Vtbl, 0x72a98001_d38a_4c0a_8fa6_75cddaf4664c);
#[repr(C)]
pub struct IChapterCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTitle: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICodecInfo, ICodecInfo_Vtbl, 0x51e89f85_ea97_499c_86ac_4ce5e73f3a42);
#[repr(C)]
pub struct ICodecInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut CodecKind) -> ::windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut CodecCategory) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Subtypes: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Subtypes: usize,
    pub DisplayName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsTrusted: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ICodecQuery, ICodecQuery_Vtbl, 0x222a953a_af61_4e04_808a_a4634e2f3ac4);
#[repr(C)]
pub struct ICodecQuery_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, CodecKind, CodecCategory, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
::windows_core::imp::com_interface!(ICodecSubtypesStatics, ICodecSubtypesStatics_Vtbl, 0xa66ac4f2_888b_4224_8cf6_2a8d4eb02382);
#[repr(C)]
pub struct ICodecSubtypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoFormatDV25: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDV50: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvc: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvh1: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvhD: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvsd: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvsl: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH263: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH264: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH265: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH264ES: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatHevc: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatHevcES: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatM4S2: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMjpg: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP43: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP4S: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP4V: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMpeg2: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatVP80: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatVP90: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMpg1: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMss1: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMss2: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv1: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv2: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv3: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWvc1: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormat420O: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAac: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAdts: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAlac: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrNB: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrWB: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrWP: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyAC3: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyAC3Spdif: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyDDPlus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDrm: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDts: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatFlac: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatFloat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMP3: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMPeg: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMsp1: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatOpus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatPcm: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWmaSpdif: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioLossless: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioV8: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioV9: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IDataCue, IDataCue_Vtbl, 0x7c7f676d_1fbc_4e2d_9a87_ee38bd1dc637);
#[repr(C)]
pub struct IDataCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
::windows_core::imp::com_interface!(IDataCue2, IDataCue2_Vtbl, 0xbc561b15_95f2_49e8_96f1_8dd5dac68d93);
#[repr(C)]
pub struct IDataCue2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
::windows_core::imp::com_interface!(IFaceDetectedEventArgs, IFaceDetectedEventArgs_Vtbl, 0x19918426_c65b_46ba_85f8_13880576c90a);
#[repr(C)]
pub struct IFaceDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResultFrame: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFaceDetectionEffect, IFaceDetectionEffect_Vtbl, 0xae15ebd2_0542_42a9_bc90_f283a29f46c1);
#[repr(C)]
pub struct IFaceDetectionEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetDesiredDetectionInterval: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DesiredDetectionInterval: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub FaceDetected: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFaceDetected: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFaceDetectionEffectDefinition, IFaceDetectionEffectDefinition_Vtbl, 0x43dca081_b848_4f33_b702_1fd2624fb016);
#[repr(C)]
pub struct IFaceDetectionEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDetectionMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, FaceDetectionMode) -> ::windows_core::HRESULT,
    pub DetectionMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut FaceDetectionMode) -> ::windows_core::HRESULT,
    pub SetSynchronousDetectionEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub SynchronousDetectionEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IFaceDetectionEffectFrame, IFaceDetectionEffectFrame_Vtbl, 0x8ab08993_5dc8_447b_a247_5270bd802ece);
#[repr(C)]
pub struct IFaceDetectionEffectFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub DetectedFaces: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis")))]
    DetectedFaces: usize,
}
::windows_core::imp::com_interface!(IHighDynamicRangeControl, IHighDynamicRangeControl_Vtbl, 0x55f1a7ae_d957_4dc9_9d1c_8553a82a7d99);
#[repr(C)]
pub struct IHighDynamicRangeControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IHighDynamicRangeOutput, IHighDynamicRangeOutput_Vtbl, 0x0f57806b_253b_4119_bb40_3a90e51384f7);
#[repr(C)]
pub struct IHighDynamicRangeOutput_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Certainty: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut f64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))]
    pub FrameControllers: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Devices_Core")))]
    FrameControllers: usize,
}
::windows_core::imp::com_interface!(IImageCue, IImageCue_Vtbl, 0x52828282_367b_440b_9116_3c84570dd270);
#[repr(C)]
pub struct IImageCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextPoint) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextPoint) -> ::windows_core::HRESULT,
    pub Extent: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextSize) -> ::windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextSize) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetSoftwareBitmap: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
::windows_core::imp::com_interface!(IInitializeMediaStreamSourceRequestedEventArgs, IInitializeMediaStreamSourceRequestedEventArgs_Vtbl, 0x25bc45e1_9b08_4c2e_a855_4542f1a75deb);
#[repr(C)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RandomAccessStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RandomAccessStream: usize,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ILowLightFusionResult, ILowLightFusionResult_Vtbl, 0x78edbe35_27a0_42e0_9cd3_738d2089de9c);
#[repr(C)]
pub struct ILowLightFusionResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Frame: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Frame: usize,
}
::windows_core::imp::com_interface!(ILowLightFusionStatics, ILowLightFusionStatics_Vtbl, 0x5305016d_c29e_40e2_87a9_9e1fd2f192f5);
#[repr(C)]
pub struct ILowLightFusionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub SupportedBitmapPixelFormats: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    SupportedBitmapPixelFormats: usize,
    pub MaxSupportedFrameCount: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub FuseAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    FuseAsync: usize,
}
::windows_core::imp::com_interface!(IMediaBinder, IMediaBinder_Vtbl, 0x2b7e40aa_de07_424f_83f1_f1de46c4fa2e);
#[repr(C)]
pub struct IMediaBinder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Binding: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveBinding: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Token: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetToken: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaBindingEventArgs, IMediaBindingEventArgs_Vtbl, 0xb61cb25a_1b6d_4630_a86d_2f0837f712e5);
#[repr(C)]
pub struct IMediaBindingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Canceled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCanceled: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub MediaBinder: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetUri: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStreamReference: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStreamReference: usize,
}
::windows_core::imp::com_interface!(IMediaBindingEventArgs2, IMediaBindingEventArgs2_Vtbl, 0x0464cceb_bb5a_482f_b8ba_f0284c696567);
#[repr(C)]
pub struct IMediaBindingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub SetAdaptiveMediaSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    SetAdaptiveMediaSource: usize,
    #[cfg(feature = "Storage")]
    pub SetStorageFile: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetStorageFile: usize,
}
::windows_core::imp::com_interface!(IMediaBindingEventArgs3, IMediaBindingEventArgs3_Vtbl, 0xf8eb475e_19be_44fc_a5ed_7aba315037f9);
#[repr(C)]
pub struct IMediaBindingEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub SetDownloadOperation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    SetDownloadOperation: usize,
}
::windows_core::imp::com_interface!(IMediaCue, IMediaCue_Vtbl, 0xc7d15e5d_59dc_431f_a0ee_27744323b36d);
::windows_core::imp::interface_hierarchy!(IMediaCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl IMediaCue {
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for IMediaCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetStartTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaCueEventArgs, IMediaCueEventArgs_Vtbl, 0xd12f47f7_5fa4_4e68_9fe5_32160dcee57e);
#[repr(C)]
pub struct IMediaCueEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cue: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSource, IMediaSource_Vtbl, 0xe7bfb599_a09d_4c21_bcdf_20af4f86b3d9);
::windows_core::imp::interface_hierarchy!(IMediaSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl IMediaSource {}
impl ::windows_core::RuntimeType for IMediaSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
::windows_core::imp::com_interface!(IMediaSource2, IMediaSource2_Vtbl, 0x2eb61048_655f_4c37_b813_b4e45dfa0abe);
#[repr(C)]
pub struct IMediaSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpenOperationCompleted: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenOperationCompleted: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub Duration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExternalTimedTextSources: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExternalTimedTextSources: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExternalTimedMetadataTracks: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExternalTimedMetadataTracks: usize,
}
::windows_core::imp::com_interface!(IMediaSource3, IMediaSource3_Vtbl, 0xb59f0d9b_4b6e_41ed_bbb4_7c7509a994ad);
#[repr(C)]
pub struct IMediaSource3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StateChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaSourceState) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSource4, IMediaSource4_Vtbl, 0xbdafad57_8eff_4c63_85a6_84de0ae3e4f2);
#[repr(C)]
pub struct IMediaSource4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub AdaptiveMediaSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    AdaptiveMediaSource: usize,
    pub MediaStreamSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MseStreamSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSource5, IMediaSource5_Vtbl, 0x331a22ae_ed2e_4a22_94c8_b743a92b3022);
#[repr(C)]
pub struct IMediaSource5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub DownloadOperation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    DownloadOperation: usize,
}
::windows_core::imp::com_interface!(IMediaSourceAppServiceConnection, IMediaSourceAppServiceConnection_Vtbl, 0x61e1ea97_1916_4810_b7f4_b642be829596);
#[repr(C)]
pub struct IMediaSourceAppServiceConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InitializeMediaStreamSourceRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveInitializeMediaStreamSourceRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSourceAppServiceConnectionFactory, IMediaSourceAppServiceConnectionFactory_Vtbl, 0x65b912eb_80b9_44f9_9c1e_e120f6d92838);
#[repr(C)]
pub struct IMediaSourceAppServiceConnectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub Create: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    Create: usize,
}
::windows_core::imp::com_interface!(IMediaSourceError, IMediaSourceError_Vtbl, 0x5c0a8965_37c5_4e9d_8d21_1cdee90cecc6);
#[repr(C)]
pub struct IMediaSourceError_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSourceOpenOperationCompletedEventArgs, IMediaSourceOpenOperationCompletedEventArgs_Vtbl, 0xfc682ceb_e281_477c_a8e0_1acd654114c8);
#[repr(C)]
pub struct IMediaSourceOpenOperationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSourceStateChangedEventArgs, IMediaSourceStateChangedEventArgs_Vtbl, 0x0a30af82_9071_4bac_bc39_ca2a93b717a9);
#[repr(C)]
pub struct IMediaSourceStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldState: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaSourceState) -> ::windows_core::HRESULT,
    pub NewState: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaSourceState) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSourceStatics, IMediaSourceStatics_Vtbl, 0xf77d6fa4_4652_410e_b1d8_e9a5e245a45c);
#[repr(C)]
pub struct IMediaSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub CreateFromAdaptiveMediaSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    CreateFromAdaptiveMediaSource: usize,
    pub CreateFromMediaStreamSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromMseStreamSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromIMediaSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CreateFromStorageFile: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFromStorageFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamReference: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamReference: usize,
    pub CreateFromUri: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSourceStatics2, IMediaSourceStatics2_Vtbl, 0xeee161a4_7f13_4896_b8cb_df0de5bcb9f1);
#[repr(C)]
pub struct IMediaSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromMediaBinder: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaSourceStatics3, IMediaSourceStatics3_Vtbl, 0x453a30d6_2bea_4122_9f73_eace04526e35);
#[repr(C)]
pub struct IMediaSourceStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFromMediaFrameSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFromMediaFrameSource: usize,
}
::windows_core::imp::com_interface!(IMediaSourceStatics4, IMediaSourceStatics4_Vtbl, 0x281b3bfc_e50a_4428_a500_9c4ed918d3f0);
#[repr(C)]
pub struct IMediaSourceStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub CreateFromDownloadOperation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    CreateFromDownloadOperation: usize,
}
::windows_core::imp::com_interface!(IMediaStreamDescriptor, IMediaStreamDescriptor_Vtbl, 0x80f16e6e_92f7_451e_97d2_afd80742da70);
::windows_core::imp::interface_hierarchy!(IMediaStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl IMediaStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for IMediaStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSelected: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamDescriptor2, IMediaStreamDescriptor2_Vtbl, 0x5073010f_e8b2_4071_b00b_ebf337a76b58);
::windows_core::imp::interface_hierarchy!(IMediaStreamDescriptor2, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(IMediaStreamDescriptor2, IMediaStreamDescriptor);
impl IMediaStreamDescriptor2 {
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for IMediaStreamDescriptor2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetLabel: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSample, IMediaStreamSample_Vtbl, 0x5c8db627_4b80_4361_9837_6cb7481ad9d6);
#[repr(C)]
pub struct IMediaStreamSample_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Processed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveProcessed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub Timestamp: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub Protection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDecodeTimestamp: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DecodeTimestamp: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetKeyFrame: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub KeyFrame: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetDiscontinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub Discontinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSample2, IMediaStreamSample2_Vtbl, 0x45078691_fce8_4746_a1c8_10c25d3d7cd3);
#[repr(C)]
pub struct IMediaStreamSample2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Surface: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Surface: usize,
}
::windows_core::imp::com_interface!(IMediaStreamSampleProtectionProperties, IMediaStreamSampleProtectionProperties_Vtbl, 0x4eb88292_ecdf_493e_841d_dd4add7caca2);
#[repr(C)]
pub struct IMediaStreamSampleProtectionProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetKeyIdentifier: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const u8) -> ::windows_core::HRESULT,
    pub GetKeyIdentifier: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32, *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetInitializationVector: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const u8) -> ::windows_core::HRESULT,
    pub GetInitializationVector: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32, *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetSubSampleMapping: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const u8) -> ::windows_core::HRESULT,
    pub GetSubSampleMapping: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32, *mut *mut u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSampleStatics, IMediaStreamSampleStatics_Vtbl, 0xdfdf218f_a6cf_4579_be41_73dd941ad972);
#[repr(C)]
pub struct IMediaStreamSampleStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, u32, super::super::Foundation::TimeSpan, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamAsync: usize,
}
::windows_core::imp::com_interface!(IMediaStreamSampleStatics2, IMediaStreamSampleStatics2_Vtbl, 0x9efe9521_6d46_494c_a2f8_d662922e2dd7);
#[repr(C)]
pub struct IMediaStreamSampleStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Surface: usize,
}
::windows_core::imp::com_interface!(IMediaStreamSource, IMediaStreamSource_Vtbl, 0x3712d543_45eb_4138_aa62_c01e26f3843f);
#[repr(C)]
pub struct IMediaStreamSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Closed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Starting: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStarting: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Paused: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePaused: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SampleRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSampleRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SwitchStreamsRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSwitchStreamsRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub NotifyError: unsafe extern "system" fn(*mut ::core::ffi::c_void, MediaStreamSourceErrorStatus) -> ::windows_core::HRESULT,
    pub AddStreamDescriptor: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Protection")]
    pub SetMediaProtectionManager: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetMediaProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub MediaProtectionManager: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    MediaProtectionManager: usize,
    pub SetDuration: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetCanSeek: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetBufferTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub BufferTime: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetBufferedRange: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    pub AddProtectionKey: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, u32, *const u8, u32, *const u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSource2, IMediaStreamSource2_Vtbl, 0xec55d0ad_2e6a_4f74_adbb_b562d1533849);
#[repr(C)]
pub struct IMediaStreamSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SampleRendered: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSampleRendered: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSource3, IMediaStreamSource3_Vtbl, 0x6a2a2746_3ddd_4ddf_a121_94045ecf9440);
#[repr(C)]
pub struct IMediaStreamSource3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetMaxSupportedPlaybackRate: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MaxSupportedPlaybackRate: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSource4, IMediaStreamSource4_Vtbl, 0x1d0cfcab_830d_417c_a3a9_2454fd6415c7);
#[repr(C)]
pub struct IMediaStreamSource4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsLive: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub IsLive: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceClosedEventArgs, IMediaStreamSourceClosedEventArgs_Vtbl, 0xcd8c7eb2_4816_4e24_88f0_491ef7386406);
#[repr(C)]
pub struct IMediaStreamSourceClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceClosedRequest, IMediaStreamSourceClosedRequest_Vtbl, 0x907c00e9_18a3_4951_887a_2c1eebd5c69e);
#[repr(C)]
pub struct IMediaStreamSourceClosedRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaStreamSourceClosedReason) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceFactory, IMediaStreamSourceFactory_Vtbl, 0xef77e0d9_d158_4b7a_863f_203342fbfd41);
#[repr(C)]
pub struct IMediaStreamSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromDescriptor: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromDescriptors: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSampleRenderedEventArgs, IMediaStreamSourceSampleRenderedEventArgs_Vtbl, 0x9d697b05_d4f2_4c7a_9dfe_8d6cd0b3ee84);
#[repr(C)]
pub struct IMediaStreamSourceSampleRenderedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SampleLag: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSampleRequest, IMediaStreamSourceSampleRequest_Vtbl, 0x4db341a9_3501_4d9b_83f9_8f235c822532);
#[repr(C)]
pub struct IMediaStreamSourceSampleRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StreamDescriptor: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSample: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Sample: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportSampleProgress: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSampleRequestDeferral, IMediaStreamSourceSampleRequestDeferral_Vtbl, 0x7895cc02_f982_43c8_9d16_c62d999319be);
#[repr(C)]
pub struct IMediaStreamSourceSampleRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSampleRequestedEventArgs, IMediaStreamSourceSampleRequestedEventArgs_Vtbl, 0x10f9bb9e_71c5_492f_847f_0da1f35e81f8);
#[repr(C)]
pub struct IMediaStreamSourceSampleRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceStartingEventArgs, IMediaStreamSourceStartingEventArgs_Vtbl, 0xf41468f2_c274_4940_a5bb_28a572452fa7);
#[repr(C)]
pub struct IMediaStreamSourceStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceStartingRequest, IMediaStreamSourceStartingRequest_Vtbl, 0x2a9093e4_35c4_4b1b_a791_0d99db56dd1d);
#[repr(C)]
pub struct IMediaStreamSourceStartingRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartPosition: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetActualStartPosition: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceStartingRequestDeferral, IMediaStreamSourceStartingRequestDeferral_Vtbl, 0x3f1356a5_6340_4dc4_9910_068ed9f598f8);
#[repr(C)]
pub struct IMediaStreamSourceStartingRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSwitchStreamsRequest, IMediaStreamSourceSwitchStreamsRequest_Vtbl, 0x41b8808e_38a9_4ec3_9ba0_b69b85501e90);
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldStreamDescriptor: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NewStreamDescriptor: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSwitchStreamsRequestDeferral, IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl, 0xbee3d835_a505_4f9a_b943_2b8cb1b4bbd9);
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaStreamSourceSwitchStreamsRequestedEventArgs, IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl, 0x42202b72_6ea1_4677_981e_350a0da412aa);
#[repr(C)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaTrack, IMediaTrack_Vtbl, 0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
::windows_core::imp::interface_hierarchy!(IMediaTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl IMediaTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for IMediaTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMediaTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackKind: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaTrackKind) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMseSourceBuffer, IMseSourceBuffer_Vtbl, 0x0c1aa3e3_df8d_4079_a3fe_6849184b4e2f);
#[repr(C)]
pub struct IMseSourceBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UpdateStarting: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdateStarting: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Updated: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub UpdateEnded: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUpdateEnded: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ErrorOccurred: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveErrorOccurred: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Aborted: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAborted: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Mode: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MseAppendMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, MseAppendMode) -> ::windows_core::HRESULT,
    pub IsUpdating: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Buffered: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Buffered: usize,
    pub TimestampOffset: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetTimestampOffset: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AppendWindowStart: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetAppendWindowStart: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub AppendWindowEnd: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAppendWindowEnd: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AppendBuffer: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendStreamMaxSize: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendStreamMaxSize: usize,
    pub Abort: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMseSourceBufferList, IMseSourceBufferList_Vtbl, 0x95fae8e7_a8e7_4ebf_8927_145e940ba511);
#[repr(C)]
pub struct IMseSourceBufferList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourceBufferAdded: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSourceBufferAdded: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SourceBufferRemoved: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSourceBufferRemoved: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Buffers: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Buffers: usize,
}
::windows_core::imp::com_interface!(IMseStreamSource, IMseStreamSource_Vtbl, 0xb0b4198d_02f4_4923_88dd_81bc3f360ffa);
#[repr(C)]
pub struct IMseStreamSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Opened: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Ended: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnded: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SourceBuffers: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ActiveSourceBuffers: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReadyState: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MseReadyState) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddSourceBuffer: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveSourceBuffer: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndOfStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, MseEndOfStreamStatus) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMseStreamSource2, IMseStreamSource2_Vtbl, 0x66f57d37_f9e7_418a_9cde_a020e956552b);
#[repr(C)]
pub struct IMseStreamSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LiveSeekableRange: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLiveSeekableRange: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMseStreamSourceStatics, IMseStreamSourceStatics_Vtbl, 0x465c679d_d570_43ce_ba21_0bff5f3fbd0a);
#[repr(C)]
pub struct IMseStreamSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsContentTypeSupported: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISceneAnalysisEffect, ISceneAnalysisEffect_Vtbl, 0xc04ba319_ca41_4813_bffd_7b08b0ed2557);
#[repr(C)]
pub struct ISceneAnalysisEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HighDynamicRangeAnalyzer: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDesiredAnalysisInterval: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub DesiredAnalysisInterval: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SceneAnalyzed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSceneAnalyzed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISceneAnalysisEffectFrame, ISceneAnalysisEffectFrame_Vtbl, 0xd8b10e4c_7fd9_42e1_85eb_6572c297c987);
#[repr(C)]
pub struct ISceneAnalysisEffectFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture")]
    pub FrameControlValues: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    FrameControlValues: usize,
    pub HighDynamicRange: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISceneAnalysisEffectFrame2, ISceneAnalysisEffectFrame2_Vtbl, 0x2d4e29be_061f_47ae_9915_02524b5f9a5f);
#[repr(C)]
pub struct ISceneAnalysisEffectFrame2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AnalysisRecommendation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut SceneAnalysisRecommendation) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISceneAnalyzedEventArgs, ISceneAnalyzedEventArgs_Vtbl, 0x146b9588_2851_45e4_ad55_44cf8df8db4d);
#[repr(C)]
pub struct ISceneAnalyzedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResultFrame: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISingleSelectMediaTrackList, ISingleSelectMediaTrackList_Vtbl, 0x77206f1f_c34f_494f_8077_2bad9ff4ecf1);
::windows_core::imp::interface_hierarchy!(ISingleSelectMediaTrackList, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ISingleSelectMediaTrackList {
    pub fn SelectedIndexChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSelectedIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for ISingleSelectMediaTrackList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISingleSelectMediaTrackList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedIndexChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSelectedIndexChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, i32) -> ::windows_core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISpeechCue, ISpeechCue_Vtbl, 0xaee254dc_1725_4bad_8043_a98499b017a2);
#[repr(C)]
pub struct ISpeechCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartPositionInInput: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStartPositionInInput: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndPositionInInput: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetEndPositionInInput: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataStreamDescriptor, ITimedMetadataStreamDescriptor_Vtbl, 0x133336bf_296a_463e_9ff9_01cd25691408);
#[repr(C)]
pub struct ITimedMetadataStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub Copy: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataStreamDescriptorFactory, ITimedMetadataStreamDescriptorFactory_Vtbl, 0xc027de30_7362_4ff9_98b1_2dfd0b8d1cae);
#[repr(C)]
pub struct ITimedMetadataStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
::windows_core::imp::com_interface!(ITimedMetadataTrack, ITimedMetadataTrack_Vtbl, 0x9e6aed9e_f67a_49a9_b330_cf03b0e9cf07);
#[repr(C)]
pub struct ITimedMetadataTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CueEntered: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCueEntered: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CueExited: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCueExited: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TrackFailed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveTrackFailed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Cues: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Cues: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActiveCues: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActiveCues: usize,
    pub TimedMetadataKind: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedMetadataKind) -> ::windows_core::HRESULT,
    pub DispatchType: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddCue: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveCue: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataTrack2, ITimedMetadataTrack2_Vtbl, 0x21b4b648_9f9d_40ba_a8f3_1a92753aef0b);
#[repr(C)]
pub struct ITimedMetadataTrack2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataTrackError, ITimedMetadataTrackError_Vtbl, 0xb3767915_4114_4819_b9d9_dd76089e72f8);
#[repr(C)]
pub struct ITimedMetadataTrackError_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedMetadataTrackErrorCode) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataTrackFactory, ITimedMetadataTrackFactory_Vtbl, 0x8dd57611_97b3_4e1f_852c_0f482c81ad26);
#[repr(C)]
pub struct ITimedMetadataTrackFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, ::std::mem::MaybeUninit<::windows_core::HSTRING>, TimedMetadataKind, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataTrackFailedEventArgs, ITimedMetadataTrackFailedEventArgs_Vtbl, 0xa57fc9d1_6789_4d4d_b07f_84b4f31acb70);
#[repr(C)]
pub struct ITimedMetadataTrackFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataTrackProvider, ITimedMetadataTrackProvider_Vtbl, 0x3b7f2024_f74e_4ade_93c5_219da05b6856);
::windows_core::imp::interface_hierarchy!(ITimedMetadataTrackProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ITimedMetadataTrackProvider {
    #[cfg(feature = "Foundation_Collections")]
    pub fn TimedMetadataTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataTracks)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for ITimedMetadataTrackProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITimedMetadataTrackProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracks: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracks: usize,
}
::windows_core::imp::com_interface!(ITimedTextBouten, ITimedTextBouten_Vtbl, 0xd9062783_5597_5092_820c_8f738e0f774a);
#[repr(C)]
pub struct ITimedTextBouten_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextBoutenType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextBoutenType) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    pub Position: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextBoutenPosition) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextBoutenPosition) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextCue, ITimedTextCue_Vtbl, 0x51c79e51_3b86_494d_b359_bb2ea7aca9a9);
#[repr(C)]
pub struct ITimedTextCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CueRegion: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCueRegion: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CueStyle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCueStyle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
}
::windows_core::imp::com_interface!(ITimedTextLine, ITimedTextLine_Vtbl, 0x978d7ce2_7308_4c66_be50_65777289f5df);
#[repr(C)]
pub struct ITimedTextLine_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Subformats: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Subformats: usize,
}
::windows_core::imp::com_interface!(ITimedTextRegion, ITimedTextRegion_Vtbl, 0x1ed0881f_8a06_4222_9f59_b21bf40124b4);
#[repr(C)]
pub struct ITimedTextRegion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextPoint) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextPoint) -> ::windows_core::HRESULT,
    pub Extent: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextSize) -> ::windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextSize) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Background: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Background: usize,
    #[cfg(feature = "UI")]
    pub SetBackground: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackground: usize,
    pub WritingMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextWritingMode) -> ::windows_core::HRESULT,
    pub SetWritingMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextWritingMode) -> ::windows_core::HRESULT,
    pub DisplayAlignment: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextDisplayAlignment) -> ::windows_core::HRESULT,
    pub SetDisplayAlignment: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextDisplayAlignment) -> ::windows_core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextDouble) -> ::windows_core::HRESULT,
    pub IsOverflowClipped: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOverflowClipped: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub Padding: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextPadding) -> ::windows_core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextPadding) -> ::windows_core::HRESULT,
    pub TextWrapping: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextWrapping) -> ::windows_core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextWrapping) -> ::windows_core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, i32) -> ::windows_core::HRESULT,
    pub ScrollMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextScrollMode) -> ::windows_core::HRESULT,
    pub SetScrollMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextScrollMode) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextRuby, ITimedTextRuby_Vtbl, 0x10335c29_5b3c_5693_9959_d05a0bd24628);
#[repr(C)]
pub struct ITimedTextRuby_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextRubyPosition) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextRubyPosition) -> ::windows_core::HRESULT,
    pub Align: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextRubyAlign) -> ::windows_core::HRESULT,
    pub SetAlign: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextRubyAlign) -> ::windows_core::HRESULT,
    pub Reserve: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextRubyReserve) -> ::windows_core::HRESULT,
    pub SetReserve: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextRubyReserve) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextSource, ITimedTextSource_Vtbl, 0xc4ed9ba6_101f_404d_a949_82f33fcd93b7);
#[repr(C)]
pub struct ITimedTextSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Resolved: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveResolved: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextSourceResolveResultEventArgs, ITimedTextSourceResolveResultEventArgs_Vtbl, 0x48907c9c_dcd8_4c33_9ad3_6cdce7b1c566);
#[repr(C)]
pub struct ITimedTextSourceResolveResultEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Tracks: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tracks: usize,
}
::windows_core::imp::com_interface!(ITimedTextSourceStatics, ITimedTextSourceStatics_Vtbl, 0x7e311853_9aba_4ac4_bb98_2fb176c3bfdd);
#[repr(C)]
pub struct ITimedTextSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    pub CreateFromUri: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithLanguage: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithLanguage: usize,
    pub CreateFromUriWithLanguage: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextSourceStatics2, ITimedTextSourceStatics2_Vtbl, 0xb66b7602_923e_43fa_9633_587075812db5);
#[repr(C)]
pub struct ITimedTextSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndex: usize,
    pub CreateFromUriWithIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndexAndLanguage: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndexAndLanguage: usize,
    pub CreateFromUriWithIndexAndLanguage: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextStyle, ITimedTextStyle_Vtbl, 0x1bb2384d_a825_40c2_a7f5_281eaedf3b55);
#[repr(C)]
pub struct ITimedTextStyle_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FontFamily: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFontFamily: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextDouble) -> ::windows_core::HRESULT,
    pub FontWeight: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextWeight) -> ::windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextWeight) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Foreground: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Foreground: usize,
    #[cfg(feature = "UI")]
    pub SetForeground: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForeground: usize,
    #[cfg(feature = "UI")]
    pub Background: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Background: usize,
    #[cfg(feature = "UI")]
    pub SetBackground: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackground: usize,
    pub IsBackgroundAlwaysShown: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetIsBackgroundAlwaysShown: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextFlowDirection) -> ::windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextFlowDirection) -> ::windows_core::HRESULT,
    pub LineAlignment: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextLineAlignment) -> ::windows_core::HRESULT,
    pub SetLineAlignment: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextLineAlignment) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub OutlineColor: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    OutlineColor: usize,
    #[cfg(feature = "UI")]
    pub SetOutlineColor: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetOutlineColor: usize,
    pub OutlineThickness: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetOutlineThickness: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextDouble) -> ::windows_core::HRESULT,
    pub OutlineRadius: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetOutlineRadius: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextDouble) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextStyle2, ITimedTextStyle2_Vtbl, 0x655f492d_6111_4787_89cc_686fece57e14);
#[repr(C)]
pub struct ITimedTextStyle2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FontStyle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut TimedTextFontStyle) -> ::windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(*mut ::core::ffi::c_void, TimedTextFontStyle) -> ::windows_core::HRESULT,
    pub IsUnderlineEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetIsUnderlineEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub IsLineThroughEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLineThroughEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub IsOverlineEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOverlineEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextStyle3, ITimedTextStyle3_Vtbl, 0xf803f93b_3e99_595e_bbb7_78a2fa13c270);
#[repr(C)]
pub struct ITimedTextStyle3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Ruby: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Bouten: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsTextCombined: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub SetIsTextCombined: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub FontAngleInDegrees: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut f64) -> ::windows_core::HRESULT,
    pub SetFontAngleInDegrees: unsafe extern "system" fn(*mut ::core::ffi::c_void, f64) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedTextSubformat, ITimedTextSubformat_Vtbl, 0xd713502f_3261_4722_a0c2_b937b2390f14);
#[repr(C)]
pub struct ITimedTextSubformat_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    pub SetStartIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, i32) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut i32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut ::core::ffi::c_void, i32) -> ::windows_core::HRESULT,
    pub SubformatStyle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSubformatStyle: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoStabilizationEffect, IVideoStabilizationEffect_Vtbl, 0x0808a650_9698_4e57_877b_bd7cb2ee0f8a);
#[repr(C)]
pub struct IVideoStabilizationEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub EnabledChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveEnabledChanged: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))]
    pub GetRecommendedStreamConfiguration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties")))]
    GetRecommendedStreamConfiguration: usize,
}
::windows_core::imp::com_interface!(IVideoStabilizationEffectEnabledChangedEventArgs, IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl, 0x187eff28_67bb_4713_b900_4168da164529);
#[repr(C)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoStreamDescriptor, IVideoStreamDescriptor_Vtbl, 0x12ee0d55_9c2b_4440_8057_2c7a90f0cbec);
#[repr(C)]
pub struct IVideoStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
::windows_core::imp::com_interface!(IVideoStreamDescriptor2, IVideoStreamDescriptor2_Vtbl, 0x8b306e10_453e_4088_832d_c36fa4f94af3);
#[repr(C)]
pub struct IVideoStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoStreamDescriptorFactory, IVideoStreamDescriptorFactory_Vtbl, 0x494ef6d1_bb75_43d2_9e5e_7b79a3afced4);
#[repr(C)]
pub struct IVideoStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
::windows_core::imp::com_interface!(IVideoTrack, IVideoTrack_Vtbl, 0x99f3b7f3_e298_4396_bb6a_a51be6a2a20a);
#[repr(C)]
pub struct IVideoTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OpenFailed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpenFailed: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetEncodingProperties: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoTrackOpenFailedEventArgs, IVideoTrackOpenFailedEventArgs_Vtbl, 0x7679e231_04f9_4c82_a4ee_8602c8bb4754);
#[repr(C)]
pub struct IVideoTrackOpenFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoTrackSupportInfo, IVideoTrackSupportInfo_Vtbl, 0x4bb534a0_fc5f_450d_8ff0_778d590486de);
#[repr(C)]
pub struct IVideoTrackSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DecoderStatus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaDecoderStatus) -> ::windows_core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MediaSourceStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AudioStreamDescriptor(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AudioStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(AudioStreamDescriptor, IMediaStreamDescriptor, IMediaStreamDescriptor2);
impl AudioStreamDescriptor {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLeadingEncoderPadding<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLeadingEncoderPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LeadingEncoderPadding(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeadingEncoderPadding)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTrailingEncoderPadding<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTrailingEncoderPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TrailingEncoderPadding(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrailingEncoderPadding)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<AudioStreamDescriptor> {
        let this = &::windows_core::Interface::cast::<IAudioStreamDescriptor3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<P0>(encodingproperties: P0) -> ::windows_core::Result<AudioStreamDescriptor>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::AudioEncodingProperties>,
    {
        Self::IAudioStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IAudioStreamDescriptorFactory<R, F: FnOnce(&IAudioStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioStreamDescriptor, IAudioStreamDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AudioStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <IAudioStreamDescriptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.AudioStreamDescriptor";
}
unsafe impl ::core::marker::Send for AudioStreamDescriptor {}
unsafe impl ::core::marker::Sync for AudioStreamDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AudioTrack(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AudioTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(AudioTrack, IMediaTrack);
impl AudioTrack {
    pub fn OpenFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>,
    {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOpenFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetEncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportInfo(&self) -> ::windows_core::Result<AudioTrackSupportInfo> {
        let this = &::windows_core::Interface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportInfo)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for AudioTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AudioTrack {
    type Vtable = IMediaTrack_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTrack as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrack {
    const NAME: &'static str = "Windows.Media.Core.AudioTrack";
}
unsafe impl ::core::marker::Send for AudioTrack {}
unsafe impl ::core::marker::Sync for AudioTrack {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AudioTrackOpenFailedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AudioTrackOpenFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AudioTrackOpenFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for AudioTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAudioTrackOpenFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackOpenFailedEventArgs";
}
unsafe impl ::core::marker::Send for AudioTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for AudioTrackOpenFailedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AudioTrackSupportInfo(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(AudioTrackSupportInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl AudioTrackSupportInfo {
    pub fn DecoderStatus(&self) -> ::windows_core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecoderStatus)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Degradation(&self) -> ::windows_core::Result<AudioDecoderDegradation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Degradation)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DegradationReason(&self) -> ::windows_core::Result<AudioDecoderDegradationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DegradationReason)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MediaSourceStatus(&self) -> ::windows_core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceStatus)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for AudioTrackSupportInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = <IAudioTrackSupportInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackSupportInfo";
}
unsafe impl ::core::marker::Send for AudioTrackSupportInfo {}
unsafe impl ::core::marker::Sync for AudioTrackSupportInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ChapterCue(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(ChapterCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(ChapterCue, IMediaCue);
impl ChapterCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChapterCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for ChapterCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ChapterCue {
    type Vtable = IChapterCue_Vtbl;
    const IID: ::windows_core::GUID = <IChapterCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ChapterCue {
    const NAME: &'static str = "Windows.Media.Core.ChapterCue";
}
unsafe impl ::core::marker::Send for ChapterCue {}
unsafe impl ::core::marker::Sync for ChapterCue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CodecInfo(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(CodecInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl CodecInfo {
    pub fn Kind(&self) -> ::windows_core::Result<CodecKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<CodecCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Subtypes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtypes)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsTrusted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTrusted)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for CodecInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CodecInfo {
    type Vtable = ICodecInfo_Vtbl;
    const IID: ::windows_core::GUID = <ICodecInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CodecInfo {
    const NAME: &'static str = "Windows.Media.Core.CodecInfo";
}
unsafe impl ::core::marker::Send for CodecInfo {}
unsafe impl ::core::marker::Sync for CodecInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CodecQuery(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(CodecQuery, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl CodecQuery {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CodecQuery, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync(&self, kind: CodecKind, category: CodecCategory, subtype: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), kind, category, ::core::mem::transmute_copy(subtype), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for CodecQuery {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for CodecQuery {
    type Vtable = ICodecQuery_Vtbl;
    const IID: ::windows_core::GUID = <ICodecQuery as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CodecQuery {
    const NAME: &'static str = "Windows.Media.Core.CodecQuery";
}
unsafe impl ::core::marker::Send for CodecQuery {}
unsafe impl ::core::marker::Sync for CodecQuery {}
pub struct CodecSubtypes;
impl CodecSubtypes {
    pub fn VideoFormatDV25() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDV25)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatDV50() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDV50)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatDvc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvc)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatDvh1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvh1)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatDvhD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvhD)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatDvsd() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvsd)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatDvsl() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvsl)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatH263() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH263)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatH264() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH264)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatH265() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH265)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatH264ES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH264ES)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatHevc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatHevc)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatHevcES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatHevcES)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatM4S2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatM4S2)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMjpg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMjpg)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMP43() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP43)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMP4S() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP4S)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMP4V() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP4V)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMpeg2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMpeg2)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatVP80() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatVP80)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatVP90() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatVP90)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMpg1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMpg1)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMss1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMss1)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatMss2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMss2)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatWmv1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv1)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatWmv2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv2)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatWmv3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv3)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormatWvc1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWvc1)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn VideoFormat420O() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormat420O)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatAac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAac)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatAdts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAdts)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatAlac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAlac)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatAmrNB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrNB)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatAmrWB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrWB)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatAmrWP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrWP)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatDolbyAC3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyAC3)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatDolbyAC3Spdif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyAC3Spdif)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatDolbyDDPlus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyDDPlus)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatDrm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDrm)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatDts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDts)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatFlac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatFlac)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatFloat() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatFloat)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatMP3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMP3)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatMPeg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMPeg)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatMsp1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMsp1)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatOpus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatOpus)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatPcm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatPcm)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatWmaSpdif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWmaSpdif)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatWMAudioLossless() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioLossless)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatWMAudioV8() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioV8)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AudioFormatWMAudioV9() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioV9)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ICodecSubtypesStatics<R, F: FnOnce(&ICodecSubtypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CodecSubtypes, ICodecSubtypesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CodecSubtypes {
    const NAME: &'static str = "Windows.Media.Core.CodecSubtypes";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DataCue(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(DataCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(DataCue, IMediaCue);
impl DataCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = &::windows_core::Interface::cast::<IDataCue2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for DataCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for DataCue {
    type Vtable = IDataCue_Vtbl;
    const IID: ::windows_core::GUID = <IDataCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DataCue {
    const NAME: &'static str = "Windows.Media.Core.DataCue";
}
unsafe impl ::core::marker::Send for DataCue {}
unsafe impl ::core::marker::Sync for DataCue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FaceDetectedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(FaceDetectedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl FaceDetectedEventArgs {
    pub fn ResultFrame(&self) -> ::windows_core::Result<FaceDetectionEffectFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultFrame)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for FaceDetectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetectedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectedEventArgs";
}
unsafe impl ::core::marker::Send for FaceDetectedEventArgs {}
unsafe impl ::core::marker::Sync for FaceDetectedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FaceDetectionEffect(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(FaceDetectionEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(FaceDetectionEffect, super::IMediaExtension);
impl FaceDetectionEffect {
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredDetectionInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDetectionInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredDetectionInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDetectionInterval)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FaceDetected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FaceDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveFaceDetected(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFaceDetected)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for FaceDetectionEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FaceDetectionEffect {
    type Vtable = IFaceDetectionEffect_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetectionEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectionEffect {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffect";
}
unsafe impl ::core::marker::Send for FaceDetectionEffect {}
unsafe impl ::core::marker::Sync for FaceDetectionEffect {}
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FaceDetectionEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "Media_Effects")]
::windows_core::imp::interface_hierarchy!(FaceDetectionEffectDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
::windows_core::imp::required_hierarchy!(FaceDetectionEffectDefinition, super::Effects::IVideoEffectDefinition);
#[cfg(feature = "Media_Effects")]
impl FaceDetectionEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FaceDetectionEffectDefinition, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDetectionMode(&self, value: FaceDetectionMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDetectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DetectionMode(&self) -> ::windows_core::Result<FaceDetectionMode> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetectionMode)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSynchronousDetectionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSynchronousDetectionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SynchronousDetectionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SynchronousDetectionEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeType for FaceDetectionEffectDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::Interface for FaceDetectionEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeName for FaceDetectionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for FaceDetectionEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for FaceDetectionEffectDefinition {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FaceDetectionEffectFrame(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(FaceDetectionEffectFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(FaceDetectionEffectFrame, super::super::Foundation::IClosable, super::IMediaFrame);
impl FaceDetectionEffectFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub fn DetectedFaces(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetectedFaces)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for FaceDetectionEffectFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for FaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrame_Vtbl;
    const IID: ::windows_core::GUID = <IFaceDetectionEffectFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectionEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectFrame";
}
unsafe impl ::core::marker::Send for FaceDetectionEffectFrame {}
unsafe impl ::core::marker::Sync for FaceDetectionEffectFrame {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HighDynamicRangeControl(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(HighDynamicRangeControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl HighDynamicRangeControl {
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for HighDynamicRangeControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControl_Vtbl;
    const IID: ::windows_core::GUID = <IHighDynamicRangeControl as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HighDynamicRangeControl {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeControl";
}
unsafe impl ::core::marker::Send for HighDynamicRangeControl {}
unsafe impl ::core::marker::Sync for HighDynamicRangeControl {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct HighDynamicRangeOutput(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(HighDynamicRangeOutput, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl HighDynamicRangeOutput {
    pub fn Certainty(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Certainty)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))]
    pub fn FrameControllers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameControllers)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for HighDynamicRangeOutput {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for HighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutput_Vtbl;
    const IID: ::windows_core::GUID = <IHighDynamicRangeOutput as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HighDynamicRangeOutput {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeOutput";
}
unsafe impl ::core::marker::Send for HighDynamicRangeOutput {}
unsafe impl ::core::marker::Sync for HighDynamicRangeOutput {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ImageCue(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(ImageCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(ImageCue, IMediaCue);
impl ImageCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextPoint) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Extent(&self) -> ::windows_core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extent)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtent(&self, value: TimedTextSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetSoftwareBitmap<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSoftwareBitmap)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for ImageCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ImageCue {
    type Vtable = IImageCue_Vtbl;
    const IID: ::windows_core::GUID = <IImageCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ImageCue {
    const NAME: &'static str = "Windows.Media.Core.ImageCue";
}
unsafe impl ::core::marker::Send for ImageCue {}
unsafe impl ::core::marker::Sync for ImageCue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InitializeMediaStreamSourceRequestedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(InitializeMediaStreamSourceRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl InitializeMediaStreamSourceRequestedEventArgs {
    pub fn Source(&self) -> ::windows_core::Result<MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn RandomAccessStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RandomAccessStream)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for InitializeMediaStreamSourceRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IInitializeMediaStreamSourceRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InitializeMediaStreamSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs";
}
unsafe impl ::core::marker::Send for InitializeMediaStreamSourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for InitializeMediaStreamSourceRequestedEventArgs {}
pub struct LowLightFusion;
impl LowLightFusion {
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn SupportedBitmapPixelFormats() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn MaxSupportedFrameCount() -> ::windows_core::Result<i32> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedFrameCount)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn FuseAsync<P0>(frameset: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap>>,
    {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FuseAsync)(::windows_core::Interface::as_raw(this), frameset.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILowLightFusionStatics<R, F: FnOnce(&ILowLightFusionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LowLightFusion, ILowLightFusionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for LowLightFusion {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusion";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LowLightFusionResult(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(LowLightFusionResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(LowLightFusionResult, super::super::Foundation::IClosable);
impl LowLightFusionResult {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Frame(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for LowLightFusionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for LowLightFusionResult {
    type Vtable = ILowLightFusionResult_Vtbl;
    const IID: ::windows_core::GUID = <ILowLightFusionResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LowLightFusionResult {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusionResult";
}
unsafe impl ::core::marker::Send for LowLightFusionResult {}
unsafe impl ::core::marker::Sync for LowLightFusionResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaBinder(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaBinder, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaBinder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaBinder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Binding<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Binding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBinding(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBinding)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Token(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Token)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetToken(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToken)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaBinder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaBinder {
    type Vtable = IMediaBinder_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBinder as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBinder {
    const NAME: &'static str = "Windows.Media.Core.MediaBinder";
}
unsafe impl ::core::marker::Send for MediaBinder {}
unsafe impl ::core::marker::Sync for MediaBinder {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaBindingEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaBindingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaBindingEventArgs {
    pub fn Canceled<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanceled(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaBinder(&self) -> ::windows_core::Result<MediaBinder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaBinder)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetUri<P0>(&self, uri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<P0>(&self, stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), ::core::mem::transmute_copy(contenttype)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStreamReference<P0>(&self, stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamReference)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), ::core::mem::transmute_copy(contenttype)).ok() }
    }
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn SetAdaptiveMediaSource<P0>(&self, mediasource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Streaming::Adaptive::AdaptiveMediaSource>,
    {
        let this = &::windows_core::Interface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAdaptiveMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn SetStorageFile<P0>(&self, file: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::IStorageFile>,
    {
        let this = &::windows_core::Interface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStorageFile)(::windows_core::Interface::as_raw(this), file.into_param().abi()).ok() }
    }
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn SetDownloadOperation<P0>(&self, downloadoperation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Networking::BackgroundTransfer::DownloadOperation>,
    {
        let this = &::windows_core::Interface::cast::<IMediaBindingEventArgs3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDownloadOperation)(::windows_core::Interface::as_raw(this), downloadoperation.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaBindingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaBindingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaBindingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaBindingEventArgs";
}
unsafe impl ::core::marker::Send for MediaBindingEventArgs {}
unsafe impl ::core::marker::Sync for MediaBindingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaCueEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaCueEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaCueEventArgs {
    pub fn Cue(&self) -> ::windows_core::Result<IMediaCue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cue)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaCueEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaCueEventArgs {
    type Vtable = IMediaCueEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaCueEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaCueEventArgs";
}
unsafe impl ::core::marker::Send for MediaCueEventArgs {}
unsafe impl ::core::marker::Sync for MediaCueEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaSource(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Playback")]
::windows_core::imp::required_hierarchy!(MediaSource, super::super::Foundation::IClosable, super::Playback::IMediaPlaybackSource);
impl MediaSource {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OpenOperationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenOperationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOpenOperationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenOperationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOpen)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExternalTimedTextSources(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IObservableVector<TimedTextSource>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExternalTimedTextSources)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExternalTimedMetadataTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExternalTimedMetadataTracks)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>,
    {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn AdaptiveMediaSource(&self) -> ::windows_core::Result<super::Streaming::Adaptive::AdaptiveMediaSource> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdaptiveMediaSource)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn MediaStreamSource(&self) -> ::windows_core::Result<MediaStreamSource> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaStreamSource)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn MseStreamSource(&self) -> ::windows_core::Result<MseStreamSource> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MseStreamSource)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn DownloadOperation(&self) -> ::windows_core::Result<super::super::Networking::BackgroundTransfer::DownloadOperation> {
        let this = &::windows_core::Interface::cast::<IMediaSource5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadOperation)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn CreateFromAdaptiveMediaSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::Streaming::Adaptive::AdaptiveMediaSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAdaptiveMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromMediaStreamSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<MediaStreamSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaStreamSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromMseStreamSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<MseStreamSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMseStreamSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromIMediaSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<IMediaSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn CreateFromStorageFile<P0>(file: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStorageFile)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<P0>(stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), ::core::mem::transmute_copy(contenttype), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamReference<P0>(stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamReference)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), ::core::mem::transmute_copy(contenttype), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUri<P0>(uri: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromMediaBinder<P0>(binder: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<MediaBinder>,
    {
        Self::IMediaSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaBinder)(::windows_core::Interface::as_raw(this), binder.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFromMediaFrameSource<P0>(framesource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::Capture::Frames::MediaFrameSource>,
    {
        Self::IMediaSourceStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaFrameSource)(::windows_core::Interface::as_raw(this), framesource.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn CreateFromDownloadOperation<P0>(downloadoperation: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Networking::BackgroundTransfer::DownloadOperation>,
    {
        Self::IMediaSourceStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDownloadOperation)(::windows_core::Interface::as_raw(this), downloadoperation.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics<R, F: FnOnce(&IMediaSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics2<R, F: FnOnce(&IMediaSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics3<R, F: FnOnce(&IMediaSourceStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics4<R, F: FnOnce(&IMediaSourceStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MediaSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaSource {
    type Vtable = IMediaSource2_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSource2 as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSource {
    const NAME: &'static str = "Windows.Media.Core.MediaSource";
}
unsafe impl ::core::marker::Send for MediaSource {}
unsafe impl ::core::marker::Sync for MediaSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaSourceAppServiceConnection(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaSourceAppServiceConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaSourceAppServiceConnection {
    pub fn InitializeMediaStreamSourceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitializeMediaStreamSourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveInitializeMediaStreamSourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInitializeMediaStreamSourceRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn Create<P0>(appserviceconnection: P0) -> ::windows_core::Result<MediaSourceAppServiceConnection>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::AppService::AppServiceConnection>,
    {
        Self::IMediaSourceAppServiceConnectionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), appserviceconnection.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaSourceAppServiceConnectionFactory<R, F: FnOnce(&IMediaSourceAppServiceConnectionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSourceAppServiceConnection, IMediaSourceAppServiceConnectionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MediaSourceAppServiceConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnection_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceAppServiceConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceAppServiceConnection {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceAppServiceConnection";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaSourceError(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaSourceError, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaSourceError {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaSourceError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaSourceError {
    type Vtable = IMediaSourceError_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceError {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceError";
}
unsafe impl ::core::marker::Send for MediaSourceError {}
unsafe impl ::core::marker::Sync for MediaSourceError {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaSourceOpenOperationCompletedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaSourceOpenOperationCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaSourceOpenOperationCompletedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<MediaSourceError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaSourceOpenOperationCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceOpenOperationCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs";
}
unsafe impl ::core::marker::Send for MediaSourceOpenOperationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceOpenOperationCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaSourceStateChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaSourceStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaSourceStateChangedEventArgs {
    pub fn OldState(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldState)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NewState(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewState)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaSourceStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaSourceStateChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceStateChangedEventArgs";
}
unsafe impl ::core::marker::Send for MediaSourceStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceStateChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSample(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSample, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSample {
    pub fn Processed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Processed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveProcessed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProcessed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<MediaStreamSamplePropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Protection(&self) -> ::windows_core::Result<MediaStreamSampleProtectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Protection)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDecodeTimestamp(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodeTimestamp)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DecodeTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodeTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyFrame(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyFrame)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyFrame(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrame)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Discontinuous(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Discontinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Surface(&self) -> ::windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSample2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Surface)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(buffer: P0, timestamp: super::super::Foundation::TimeSpan) -> ::windows_core::Result<MediaStreamSample>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi(), timestamp, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamAsync<P0>(stream: P0, count: u32, timestamp: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaStreamSample>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IInputStream>,
    {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), count, timestamp, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Surface<P0>(surface: P0, timestamp: super::super::Foundation::TimeSpan) -> ::windows_core::Result<MediaStreamSample>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::IMediaStreamSampleStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDirect3D11Surface)(::windows_core::Interface::as_raw(this), surface.into_param().abi(), timestamp, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaStreamSampleStatics<R, F: FnOnce(&IMediaStreamSampleStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaStreamSampleStatics2<R, F: FnOnce(&IMediaStreamSampleStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MediaStreamSample {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSample {
    type Vtable = IMediaStreamSample_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSample as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSample {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSample";
}
unsafe impl ::core::marker::Send for MediaStreamSample {}
unsafe impl ::core::marker::Sync for MediaStreamSample {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSamplePropertySet(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(MediaStreamSamplePropertySet, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::required_hierarchy!(MediaStreamSamplePropertySet, super::super::Foundation::Collections::IIterable::<super::super::Foundation::Collections::IKeyValuePair::<::windows_core::GUID, ::windows_core::IInspectable>>, super::super::Foundation::Collections::IMap::<::windows_core::GUID, ::windows_core::IInspectable>);
#[cfg(feature = "Foundation_Collections")]
impl MediaStreamSamplePropertySet {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: ::windows_core::GUID) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: ::windows_core::GUID, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key, value.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for MediaStreamSamplePropertySet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for MediaStreamSamplePropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows_core::GUID, ::windows_core::IInspectable>;
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for MediaStreamSamplePropertySet {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSamplePropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for MediaStreamSamplePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &MediaStreamSamplePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaStreamSamplePropertySet {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSampleProtectionProperties(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSampleProtectionProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSampleProtectionProperties {
    pub fn SetKeyIdentifier(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyIdentifier)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn GetKeyIdentifier(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetKeyIdentifier)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetInitializationVector(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitializationVector)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn GetInitializationVector(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetInitializationVector)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetSubSampleMapping(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubSampleMapping)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn GetSubSampleMapping(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetSubSampleMapping)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSampleProtectionProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionProperties_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSampleProtectionProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSampleProtectionProperties {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSampleProtectionProperties";
}
unsafe impl ::core::marker::Send for MediaStreamSampleProtectionProperties {}
unsafe impl ::core::marker::Sync for MediaStreamSampleProtectionProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSource(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(MediaStreamSource, IMediaSource);
impl MediaStreamSource {
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Starting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Starting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Paused<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Paused)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePaused(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePaused)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SampleRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSampleRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSampleRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SwitchStreamsRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SwitchStreamsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSwitchStreamsRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSwitchStreamsRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotifyError(&self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyError)(::windows_core::Interface::as_raw(this), errorstatus).ok() }
    }
    pub fn AddStreamDescriptor<P0>(&self, descriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStreamDescriptor)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Protection")]
    pub fn SetMediaProtectionManager<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Protection::MediaProtectionManager>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaProtectionManager)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Protection")]
    pub fn MediaProtectionManager(&self) -> ::windows_core::Result<super::Protection::MediaProtectionManager> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaProtectionManager)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanSeek(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanSeek)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBufferTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBufferTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BufferTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BufferTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBufferedRange(&self, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBufferedRange)(::windows_core::Interface::as_raw(this), startoffset, endoffset).ok() }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::super::Storage::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::super::Storage::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddProtectionKey<P0>(&self, streamdescriptor: P0, keyidentifier: &[u8], licensedata: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddProtectionKey)(::windows_core::Interface::as_raw(this), streamdescriptor.into_param().abi(), keyidentifier.len().try_into().unwrap(), keyidentifier.as_ptr(), licensedata.len().try_into().unwrap(), licensedata.as_ptr()).ok() }
    }
    pub fn SampleRendered<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>,
    {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleRendered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSampleRendered(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSampleRendered)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetMaxSupportedPlaybackRate<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<f64>>,
    {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSupportedPlaybackRate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MaxSupportedPlaybackRate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedPlaybackRate)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsLive(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLive)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLive(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLive)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateFromDescriptor<P0>(descriptor: P0) -> ::windows_core::Result<MediaStreamSource>
    where
        P0: ::windows_core::IntoParam<IMediaStreamDescriptor>,
    {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDescriptor)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromDescriptors<P0, P1>(descriptor: P0, descriptor2: P1) -> ::windows_core::Result<MediaStreamSource>
    where
        P0: ::windows_core::IntoParam<IMediaStreamDescriptor>,
        P1: ::windows_core::IntoParam<IMediaStreamDescriptor>,
    {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDescriptors)(::windows_core::Interface::as_raw(this), descriptor.into_param().abi(), descriptor2.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IMediaStreamSourceFactory<R, F: FnOnce(&IMediaStreamSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaStreamSource, IMediaStreamSourceFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MediaStreamSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSource {
    type Vtable = IMediaStreamSource_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSource";
}
unsafe impl ::core::marker::Send for MediaStreamSource {}
unsafe impl ::core::marker::Sync for MediaStreamSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceClosedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceClosedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceClosedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceClosedRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceClosedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceClosedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceClosedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedEventArgs";
}
unsafe impl ::core::marker::Send for MediaStreamSourceClosedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceClosedRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceClosedRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceClosedRequest {
    pub fn Reason(&self) -> ::windows_core::Result<MediaStreamSourceClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceClosedRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceClosedRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceClosedRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedRequest";
}
unsafe impl ::core::marker::Send for MediaStreamSourceClosedRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSampleRenderedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRenderedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSampleRenderedEventArgs {
    pub fn SampleLag(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleLag)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRenderedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRenderedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRenderedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRenderedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRenderedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSampleRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSampleRequest {
    pub fn StreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceSampleRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSample<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaStreamSample>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSample)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Sample(&self) -> ::windows_core::Result<MediaStreamSample> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Sample)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportSampleProgress(&self, progress: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportSampleProgress)(::windows_core::Interface::as_raw(this), progress).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequest";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSampleRequestDeferral(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSampleRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestDeferral";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSampleRequestedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSampleRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceSampleRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceStartingEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceStartingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceStartingEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceStartingRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingEventArgs";
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceStartingRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceStartingRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceStartingRequest {
    pub fn StartPosition(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPosition)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceStartingRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActualStartPosition(&self, position: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetActualStartPosition)(::windows_core::Interface::as_raw(this), position).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceStartingRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequest";
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceStartingRequestDeferral(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceStartingRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceStartingRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceStartingRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequestDeferral";
}
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequestDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSwitchStreamsRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSwitchStreamsRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSwitchStreamsRequest {
    pub fn OldStreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldStreamDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn NewStreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewStreamDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceSwitchStreamsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequest_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSwitchStreamsRequestDeferral(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSwitchStreamsRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSwitchStreamsRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequestDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaStreamSourceSwitchStreamsRequestedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSwitchStreamsRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MediaStreamSourceSwitchStreamsRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceSwitchStreamsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequestedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs";
}
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MseSourceBuffer(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MseSourceBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MseSourceBuffer {
    pub fn UpdateStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdateStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn UpdateEnded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateEnded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveUpdateEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateEnded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ErrorOccurred<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorOccurred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveErrorOccurred)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Aborted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Aborted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAborted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAborted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Mode(&self) -> ::windows_core::Result<MseAppendMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: MseAppendMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsUpdating(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUpdating)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Buffered(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MseTimeRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffered)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimestampOffset(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimestampOffset)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTimestampOffset(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestampOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendWindowStart(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendWindowStart)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAppendWindowStart(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendWindowStart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendWindowEnd(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendWindowEnd)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAppendWindowEnd<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendWindowEnd)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendStream<P0>(&self, stream: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendStreamMaxSize<P0>(&self, stream: P0, maxsize: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendStreamMaxSize)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), maxsize).ok() }
    }
    pub fn Abort(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Abort)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Remove<P0>(&self, start: super::super::Foundation::TimeSpan, end: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), start, end.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for MseSourceBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MseSourceBuffer {
    type Vtable = IMseSourceBuffer_Vtbl;
    const IID: ::windows_core::GUID = <IMseSourceBuffer as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MseSourceBuffer {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBuffer";
}
unsafe impl ::core::marker::Send for MseSourceBuffer {}
unsafe impl ::core::marker::Sync for MseSourceBuffer {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MseSourceBufferList(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MseSourceBufferList, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl MseSourceBufferList {
    pub fn SourceBufferAdded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceBufferAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceBufferAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBufferAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceBufferRemoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceBufferRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceBufferRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBufferRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Buffers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffers)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for MseSourceBufferList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MseSourceBufferList {
    type Vtable = IMseSourceBufferList_Vtbl;
    const IID: ::windows_core::GUID = <IMseSourceBufferList as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MseSourceBufferList {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBufferList";
}
unsafe impl ::core::marker::Send for MseSourceBufferList {}
unsafe impl ::core::marker::Sync for MseSourceBufferList {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MseStreamSource(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(MseStreamSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(MseStreamSource, IMediaSource);
impl MseStreamSource {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MseStreamSource, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Opened<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpened)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Ended<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ended)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceBuffers(&self) -> ::windows_core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceBuffers)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActiveSourceBuffers(&self) -> ::windows_core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActiveSourceBuffers)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadyState(&self) -> ::windows_core::Result<MseReadyState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadyState)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AddSourceBuffer(&self, mimetype: &::windows_core::HSTRING) -> ::windows_core::Result<MseSourceBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddSourceBuffer)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(mimetype), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveSourceBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MseSourceBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    pub fn EndOfStream(&self, status: MseEndOfStreamStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndOfStream)(::windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn LiveSeekableRange(&self) -> ::windows_core::Result<super::super::Foundation::IReference<MseTimeRange>> {
        let this = &::windows_core::Interface::cast::<IMseStreamSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LiveSeekableRange)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLiveSeekableRange<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<MseTimeRange>>,
    {
        let this = &::windows_core::Interface::cast::<IMseStreamSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLiveSeekableRange)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsContentTypeSupported(contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IMseStreamSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContentTypeSupported)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contenttype), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IMseStreamSourceStatics<R, F: FnOnce(&IMseStreamSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MseStreamSource, IMseStreamSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MseStreamSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MseStreamSource {
    type Vtable = IMseStreamSource_Vtbl;
    const IID: ::windows_core::GUID = <IMseStreamSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MseStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MseStreamSource";
}
unsafe impl ::core::marker::Send for MseStreamSource {}
unsafe impl ::core::marker::Sync for MseStreamSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SceneAnalysisEffect(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SceneAnalysisEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(SceneAnalysisEffect, super::IMediaExtension);
impl SceneAnalysisEffect {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
    pub fn HighDynamicRangeAnalyzer(&self) -> ::windows_core::Result<HighDynamicRangeControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighDynamicRangeAnalyzer)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDesiredAnalysisInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAnalysisInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredAnalysisInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAnalysisInterval)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SceneAnalyzed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SceneAnalyzed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSceneAnalyzed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSceneAnalyzed)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::windows_core::RuntimeType for SceneAnalysisEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffect_Vtbl;
    const IID: ::windows_core::GUID = <ISceneAnalysisEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalysisEffect {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffect";
}
unsafe impl ::core::marker::Send for SceneAnalysisEffect {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffect {}
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SceneAnalysisEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "Media_Effects")]
::windows_core::imp::interface_hierarchy!(SceneAnalysisEffectDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
::windows_core::imp::required_hierarchy!(SceneAnalysisEffectDefinition, super::Effects::IVideoEffectDefinition);
#[cfg(feature = "Media_Effects")]
impl SceneAnalysisEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneAnalysisEffectDefinition, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeType for SceneAnalysisEffectDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::Interface for SceneAnalysisEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeName for SceneAnalysisEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for SceneAnalysisEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for SceneAnalysisEffectDefinition {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SceneAnalysisEffectFrame(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SceneAnalysisEffectFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(SceneAnalysisEffectFrame, super::super::Foundation::IClosable, super::IMediaFrame);
impl SceneAnalysisEffectFrame {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::Interface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Capture")]
    pub fn FrameControlValues(&self) -> ::windows_core::Result<super::Capture::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameControlValues)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn HighDynamicRange(&self) -> ::windows_core::Result<HighDynamicRangeOutput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighDynamicRange)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AnalysisRecommendation(&self) -> ::windows_core::Result<SceneAnalysisRecommendation> {
        let this = &::windows_core::Interface::cast::<ISceneAnalysisEffectFrame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnalysisRecommendation)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for SceneAnalysisEffectFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrame_Vtbl;
    const IID: ::windows_core::GUID = <ISceneAnalysisEffectFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalysisEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectFrame";
}
unsafe impl ::core::marker::Send for SceneAnalysisEffectFrame {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffectFrame {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SceneAnalyzedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SceneAnalyzedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl SceneAnalyzedEventArgs {
    pub fn ResultFrame(&self) -> ::windows_core::Result<SceneAnalysisEffectFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultFrame)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for SceneAnalyzedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISceneAnalyzedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalyzedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalyzedEventArgs";
}
unsafe impl ::core::marker::Send for SceneAnalyzedEventArgs {}
unsafe impl ::core::marker::Sync for SceneAnalyzedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SpeechCue(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SpeechCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(SpeechCue, IMediaCue);
impl SpeechCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn StartPositionInInput(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPositionInInput)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetStartPositionInInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartPositionInInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EndPositionInInput(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndPositionInInput)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEndPositionInInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndPositionInInput)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for SpeechCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SpeechCue {
    type Vtable = ISpeechCue_Vtbl;
    const IID: ::windows_core::GUID = <ISpeechCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpeechCue {
    const NAME: &'static str = "Windows.Media.Core.SpeechCue";
}
unsafe impl ::core::marker::Send for SpeechCue {}
unsafe impl ::core::marker::Sync for SpeechCue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedMetadataStreamDescriptor(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedMetadataStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(TimedMetadataStreamDescriptor, IMediaStreamDescriptor, IMediaStreamDescriptor2);
impl TimedMetadataStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::TimedMetadataEncodingProperties> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<TimedMetadataStreamDescriptor> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<P0>(encodingproperties: P0) -> ::windows_core::Result<TimedMetadataStreamDescriptor>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::TimedMetadataEncodingProperties>,
    {
        Self::ITimedMetadataStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataStreamDescriptorFactory<R, F: FnOnce(&ITimedMetadataStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedMetadataStreamDescriptor, ITimedMetadataStreamDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TimedMetadataStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedMetadataStreamDescriptor {
    type Vtable = IMediaStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <IMediaStreamDescriptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataStreamDescriptor";
}
unsafe impl ::core::marker::Send for TimedMetadataStreamDescriptor {}
unsafe impl ::core::marker::Sync for TimedMetadataStreamDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedMetadataTrack(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedMetadataTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(TimedMetadataTrack, IMediaTrack);
impl TimedMetadataTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn CueEntered<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueEntered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCueEntered(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCueEntered)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CueExited<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueExited)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCueExited(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCueExited)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TrackFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTrackFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTrackFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Cues(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cues)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActiveCues(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActiveCues)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TimedMetadataKind(&self) -> ::windows_core::Result<TimedMetadataKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataKind)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DispatchType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatchType)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn AddCue<P0>(&self, cue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMediaCue>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddCue)(::windows_core::Interface::as_raw(this), cue.into_param().abi()).ok() }
    }
    pub fn RemoveCue<P0>(&self, cue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMediaCue>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCue)(::windows_core::Interface::as_raw(this), cue.into_param().abi()).ok() }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(id: &::windows_core::HSTRING, language: &::windows_core::HSTRING, kind: TimedMetadataKind) -> ::windows_core::Result<TimedMetadataTrack> {
        Self::ITimedMetadataTrackFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(language), kind, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataTrackFactory<R, F: FnOnce(&ITimedMetadataTrackFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedMetadataTrack, ITimedMetadataTrackFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedMetadataTrack {
    type Vtable = ITimedMetadataTrack_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataTrack as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrack";
}
unsafe impl ::core::marker::Send for TimedMetadataTrack {}
unsafe impl ::core::marker::Sync for TimedMetadataTrack {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedMetadataTrackError(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedMetadataTrackError, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedMetadataTrackError {
    pub fn ErrorCode(&self) -> ::windows_core::Result<TimedMetadataTrackErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrackError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackError_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataTrackError as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrackError {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackError";
}
unsafe impl ::core::marker::Send for TimedMetadataTrackError {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackError {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedMetadataTrackFailedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedMetadataTrackFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedMetadataTrackFailedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrackFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimedMetadataTrackFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackFailedEventArgs";
}
unsafe impl ::core::marker::Send for TimedMetadataTrackFailedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackFailedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextBouten(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextBouten, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextBouten {
    pub fn Type(&self) -> ::windows_core::Result<TimedTextBoutenType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetType(&self, value: TimedTextBoutenType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextBoutenPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextBoutenPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for TimedTextBouten {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextBouten {
    type Vtable = ITimedTextBouten_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextBouten as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextBouten {
    const NAME: &'static str = "Windows.Media.Core.TimedTextBouten";
}
unsafe impl ::core::marker::Send for TimedTextBouten {}
unsafe impl ::core::marker::Sync for TimedTextBouten {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextCue(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(TimedTextCue, IMediaCue);
impl TimedTextCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn CueRegion(&self) -> ::windows_core::Result<TimedTextRegion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueRegion)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCueRegion<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TimedTextRegion>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCueRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CueStyle(&self) -> ::windows_core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueStyle)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCueStyle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TimedTextStyle>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCueStyle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<TimedTextLine>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lines)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for TimedTextCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextCue {
    type Vtable = ITimedTextCue_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextCue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextCue {
    const NAME: &'static str = "Windows.Media.Core.TimedTextCue";
}
unsafe impl ::core::marker::Send for TimedTextCue {}
unsafe impl ::core::marker::Sync for TimedTextCue {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextLine(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextLine, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextLine {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextLine, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Subformats(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<TimedTextSubformat>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subformats)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for TimedTextLine {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextLine {
    type Vtable = ITimedTextLine_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextLine as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextLine {
    const NAME: &'static str = "Windows.Media.Core.TimedTextLine";
}
unsafe impl ::core::marker::Send for TimedTextLine {}
unsafe impl ::core::marker::Sync for TimedTextLine {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextRegion(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextRegion, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextRegion {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextRegion, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextPoint) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Extent(&self) -> ::windows_core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extent)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtent(&self, value: TimedTextSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn Background(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Background)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetBackground(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WritingMode(&self) -> ::windows_core::Result<TimedTextWritingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WritingMode)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWritingMode(&self, value: TimedTextWritingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWritingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayAlignment(&self) -> ::windows_core::Result<TimedTextDisplayAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAlignment)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDisplayAlignment(&self, value: TimedTextDisplayAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineHeight(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOverflowClipped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOverflowClipped)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsOverflowClipped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverflowClipped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> ::windows_core::Result<TimedTextPadding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Padding)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: TimedTextPadding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPadding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TextWrapping(&self) -> ::windows_core::Result<TimedTextWrapping> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextWrapping)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTextWrapping(&self, value: TimedTextWrapping) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextWrapping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScrollMode(&self) -> ::windows_core::Result<TimedTextScrollMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScrollMode)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScrollMode(&self, value: TimedTextScrollMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScrollMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for TimedTextRegion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextRegion {
    type Vtable = ITimedTextRegion_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextRegion as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextRegion {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRegion";
}
unsafe impl ::core::marker::Send for TimedTextRegion {}
unsafe impl ::core::marker::Sync for TimedTextRegion {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextRuby(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextRuby, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextRuby {
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextRubyPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextRubyPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Align(&self) -> ::windows_core::Result<TimedTextRubyAlign> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Align)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAlign(&self, value: TimedTextRubyAlign) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlign)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Reserve(&self) -> ::windows_core::Result<TimedTextRubyReserve> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reserve)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReserve(&self, value: TimedTextRubyReserve) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReserve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for TimedTextRuby {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextRuby {
    type Vtable = ITimedTextRuby_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextRuby as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextRuby {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRuby";
}
unsafe impl ::core::marker::Send for TimedTextRuby {}
unsafe impl ::core::marker::Sync for TimedTextRuby {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextSource(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextSource {
    pub fn Resolved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resolved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveResolved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResolved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<P0>(stream: P0) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUri<P0>(uri: P0) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithLanguage<P0>(stream: P0, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithLanguage)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUriWithLanguage<P0>(uri: P0, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithLanguage)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndex<P0, P1>(stream: P0, indexstream: P1) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithIndex)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), indexstream.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUriWithIndex<P0, P1>(uri: P0, indexuri: P1) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithIndex)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), indexuri.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndexAndLanguage<P0, P1>(stream: P0, indexstream: P1, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithIndexAndLanguage)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), indexstream.into_param().abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUriWithIndexAndLanguage<P0, P1>(uri: P0, indexuri: P1, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithIndexAndLanguage)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), indexuri.into_param().abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITimedTextSourceStatics<R, F: FnOnce(&ITimedTextSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextSource, ITimedTextSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITimedTextSourceStatics2<R, F: FnOnce(&ITimedTextSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextSource, ITimedTextSourceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TimedTextSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextSource {
    type Vtable = ITimedTextSource_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSource";
}
unsafe impl ::core::marker::Send for TimedTextSource {}
unsafe impl ::core::marker::Sync for TimedTextSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextSourceResolveResultEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextSourceResolveResultEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextSourceResolveResultEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tracks)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for TimedTextSourceResolveResultEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextSourceResolveResultEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSourceResolveResultEventArgs";
}
unsafe impl ::core::marker::Send for TimedTextSourceResolveResultEventArgs {}
unsafe impl ::core::marker::Sync for TimedTextSourceResolveResultEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextStyle(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextStyle, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextStyle {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextStyle, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FontFamily(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetFontFamily(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontFamily)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontWeight(&self) -> ::windows_core::Result<TimedTextWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: TimedTextWeight) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontWeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetForeground(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForeground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn Background(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Background)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetBackground(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsBackgroundAlwaysShown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBackgroundAlwaysShown)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsBackgroundAlwaysShown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBackgroundAlwaysShown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FlowDirection(&self) -> ::windows_core::Result<TimedTextFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowDirection)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFlowDirection(&self, value: TimedTextFlowDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFlowDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineAlignment(&self) -> ::windows_core::Result<TimedTextLineAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineAlignment)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLineAlignment(&self, value: TimedTextLineAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn OutlineColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutlineColor)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetOutlineColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutlineThickness(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutlineThickness)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutlineThickness(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineThickness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutlineRadius(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutlineRadius)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOutlineRadius(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineRadius)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> ::windows_core::Result<TimedTextFontStyle> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: TimedTextFontStyle) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFontStyle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsUnderlineEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsUnderlineEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsUnderlineEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLineThroughEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLineThroughEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLineThroughEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLineThroughEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOverlineEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOverlineEnabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsOverlineEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverlineEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ruby(&self) -> ::windows_core::Result<TimedTextRuby> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ruby)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Bouten(&self) -> ::windows_core::Result<TimedTextBouten> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bouten)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsTextCombined(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextCombined)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextCombined(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsTextCombined)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontAngleInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontAngleInDegrees)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontAngleInDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFontAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for TimedTextStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextStyle {
    type Vtable = ITimedTextStyle_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextStyle as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextStyle {
    const NAME: &'static str = "Windows.Media.Core.TimedTextStyle";
}
unsafe impl ::core::marker::Send for TimedTextStyle {}
unsafe impl ::core::marker::Sync for TimedTextStyle {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedTextSubformat(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(TimedTextSubformat, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl TimedTextSubformat {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextSubformat, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StartIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartIndex)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubformatStyle(&self) -> ::windows_core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubformatStyle)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetSubformatStyle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TimedTextStyle>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubformatStyle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for TimedTextSubformat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedTextSubformat {
    type Vtable = ITimedTextSubformat_Vtbl;
    const IID: ::windows_core::GUID = <ITimedTextSubformat as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSubformat {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSubformat";
}
unsafe impl ::core::marker::Send for TimedTextSubformat {}
unsafe impl ::core::marker::Sync for TimedTextSubformat {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoStabilizationEffect(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VideoStabilizationEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(VideoStabilizationEffect, super::IMediaExtension);
impl VideoStabilizationEffect {
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::Interface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.into_param().abi()).ok() }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EnabledChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveEnabledChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnabledChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))]
    pub fn GetRecommendedStreamConfiguration<P0, P1>(&self, controller: P0, desiredproperties: P1) -> ::windows_core::Result<super::Capture::VideoStreamConfiguration>
    where
        P0: ::windows_core::IntoParam<super::Devices::VideoDeviceController>,
        P1: ::windows_core::IntoParam<super::MediaProperties::VideoEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRecommendedStreamConfiguration)(::windows_core::Interface::as_raw(this), controller.into_param().abi(), desiredproperties.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for VideoStabilizationEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffect_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStabilizationEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStabilizationEffect {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffect";
}
unsafe impl ::core::marker::Send for VideoStabilizationEffect {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffect {}
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoStabilizationEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "Media_Effects")]
::windows_core::imp::interface_hierarchy!(VideoStabilizationEffectDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
::windows_core::imp::required_hierarchy!(VideoStabilizationEffectDefinition, super::Effects::IVideoEffectDefinition);
#[cfg(feature = "Media_Effects")]
impl VideoStabilizationEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoStabilizationEffectDefinition, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeType for VideoStabilizationEffectDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::Interface for VideoStabilizationEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::Interface>::IID;
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeName for VideoStabilizationEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for VideoStabilizationEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for VideoStabilizationEffectDefinition {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VideoStabilizationEffectEnabledChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl VideoStabilizationEffectEnabledChangedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<VideoStabilizationEffectEnabledChangedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for VideoStabilizationEffectEnabledChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStabilizationEffectEnabledChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStabilizationEffectEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs";
}
unsafe impl ::core::marker::Send for VideoStabilizationEffectEnabledChangedEventArgs {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffectEnabledChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoStreamDescriptor(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VideoStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(VideoStreamDescriptor, IMediaStreamDescriptor, IMediaStreamDescriptor2);
impl VideoStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<VideoStreamDescriptor> {
        let this = &::windows_core::Interface::cast::<IVideoStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<P0>(encodingproperties: P0) -> ::windows_core::Result<VideoStreamDescriptor>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::VideoEncodingProperties>,
    {
        Self::IVideoStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVideoStreamDescriptorFactory<R, F: FnOnce(&IVideoStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoStreamDescriptor, IVideoStreamDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for VideoStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptor_Vtbl;
    const IID: ::windows_core::GUID = <IVideoStreamDescriptor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.VideoStreamDescriptor";
}
unsafe impl ::core::marker::Send for VideoStreamDescriptor {}
unsafe impl ::core::marker::Sync for VideoStreamDescriptor {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoTrack(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VideoTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(VideoTrack, IMediaTrack);
impl VideoTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn OpenFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>,
    {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOpenFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetEncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SupportInfo(&self) -> ::windows_core::Result<VideoTrackSupportInfo> {
        let this = &::windows_core::Interface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportInfo)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for VideoTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoTrack {
    type Vtable = IMediaTrack_Vtbl;
    const IID: ::windows_core::GUID = <IMediaTrack as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrack {
    const NAME: &'static str = "Windows.Media.Core.VideoTrack";
}
unsafe impl ::core::marker::Send for VideoTrack {}
unsafe impl ::core::marker::Sync for VideoTrack {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoTrackOpenFailedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VideoTrackOpenFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl VideoTrackOpenFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for VideoTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IVideoTrackOpenFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackOpenFailedEventArgs";
}
unsafe impl ::core::marker::Send for VideoTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for VideoTrackOpenFailedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoTrackSupportInfo(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VideoTrackSupportInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl VideoTrackSupportInfo {
    pub fn DecoderStatus(&self) -> ::windows_core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecoderStatus)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MediaSourceStatus(&self) -> ::windows_core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceStatus)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for VideoTrackSupportInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfo_Vtbl;
    const IID: ::windows_core::GUID = <IVideoTrackSupportInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackSupportInfo";
}
unsafe impl ::core::marker::Send for VideoTrackSupportInfo {}
unsafe impl ::core::marker::Sync for VideoTrackSupportInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: Self = Self(0i32);
    pub const DownmixTo2Channels: Self = Self(1i32);
    pub const DownmixTo6Channels: Self = Self(2i32);
    pub const DownmixTo8Channels: Self = Self(3i32);
}
impl ::windows_core::TypeKind for AudioDecoderDegradation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioDecoderDegradation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDecoderDegradation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradation;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
}
impl ::windows_core::TypeKind for AudioDecoderDegradationReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioDecoderDegradationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradationReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDecoderDegradationReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradationReason;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
impl ::windows_core::TypeKind for CodecCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CodecCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CodecCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecCategory;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::windows_core::TypeKind for CodecKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CodecKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CodecKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
}
impl ::windows_core::TypeKind for FaceDetectionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FaceDetectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FaceDetectionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.FaceDetectionMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
}
impl ::windows_core::TypeKind for MediaDecoderStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaDecoderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDecoderStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaDecoderStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaDecoderStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
impl ::windows_core::TypeKind for MediaSourceState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaSourceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
}
impl ::windows_core::TypeKind for MediaSourceStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaStreamSourceClosedReason(pub i32);
impl MediaStreamSourceClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const AppReportedError: Self = Self(2i32);
    pub const UnsupportedProtectionSystem: Self = Self(3i32);
    pub const ProtectionSystemFailure: Self = Self(4i32);
    pub const UnsupportedEncodingFormat: Self = Self(5i32);
    pub const MissingSampleRequestedEventHandler: Self = Self(6i32);
}
impl ::windows_core::TypeKind for MediaStreamSourceClosedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaStreamSourceClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceClosedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceClosedReason;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: Self = Self(0i32);
    pub const OutOfMemory: Self = Self(1i32);
    pub const FailedToOpenFile: Self = Self(2i32);
    pub const FailedToConnectToServer: Self = Self(3i32);
    pub const ConnectionToServerLost: Self = Self(4i32);
    pub const UnspecifiedNetworkError: Self = Self(5i32);
    pub const DecodeError: Self = Self(6i32);
    pub const UnsupportedMediaFormat: Self = Self(7i32);
}
impl ::windows_core::TypeKind for MediaStreamSourceErrorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaStreamSourceErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceErrorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceErrorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceErrorStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
impl ::windows_core::TypeKind for MediaTrackKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaTrackKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTrackKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaTrackKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaTrackKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
impl ::windows_core::TypeKind for MseAppendMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MseAppendMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseAppendMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseAppendMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseAppendMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::windows_core::TypeKind for MseEndOfStreamStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MseEndOfStreamStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseEndOfStreamStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseEndOfStreamStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseEndOfStreamStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
impl ::windows_core::TypeKind for MseReadyState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MseReadyState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseReadyState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseReadyState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseReadyState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
impl ::windows_core::TypeKind for SceneAnalysisRecommendation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneAnalysisRecommendation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisRecommendation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAnalysisRecommendation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.SceneAnalysisRecommendation;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: Self = Self(0i32);
    pub const Chapter: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Data: Self = Self(3i32);
    pub const Description: Self = Self(4i32);
    pub const Subtitle: Self = Self(5i32);
    pub const ImageSubtitle: Self = Self(6i32);
    pub const Speech: Self = Self(7i32);
}
impl ::windows_core::TypeKind for TimedMetadataKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedMetadataKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
impl ::windows_core::TypeKind for TimedMetadataTrackErrorCode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedMetadataTrackErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackErrorCode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrackErrorCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataTrackErrorCode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::windows_core::TypeKind for TimedTextBoutenPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextBoutenPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenPosition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextBoutenPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenPosition;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const FilledCircle: Self = Self(2i32);
    pub const OpenCircle: Self = Self(3i32);
    pub const FilledDot: Self = Self(4i32);
    pub const OpenDot: Self = Self(5i32);
    pub const FilledSesame: Self = Self(6i32);
    pub const OpenSesame: Self = Self(7i32);
}
impl ::windows_core::TypeKind for TimedTextBoutenType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextBoutenType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextBoutenType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::windows_core::TypeKind for TimedTextDisplayAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextDisplayAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextDisplayAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextDisplayAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextDisplayAlignment;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::windows_core::TypeKind for TimedTextFlowDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextFlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFlowDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextFlowDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFlowDirection;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::windows_core::TypeKind for TimedTextFontStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextFontStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFontStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextFontStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFontStyle;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::windows_core::TypeKind for TimedTextLineAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextLineAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLineAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextLineAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextLineAlignment;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
    pub const SpaceAround: Self = Self(3i32);
    pub const SpaceBetween: Self = Self(4i32);
    pub const WithBase: Self = Self(5i32);
}
impl ::windows_core::TypeKind for TimedTextRubyAlign {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextRubyAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyAlign").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRubyAlign {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyAlign;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::windows_core::TypeKind for TimedTextRubyPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextRubyPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyPosition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRubyPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyPosition;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: Self = Self(0i32);
    pub const Before: Self = Self(1i32);
    pub const After: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const Outside: Self = Self(4i32);
}
impl ::windows_core::TypeKind for TimedTextRubyReserve {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextRubyReserve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyReserve").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRubyReserve {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyReserve;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
impl ::windows_core::TypeKind for TimedTextScrollMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextScrollMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextScrollMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextScrollMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextScrollMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::windows_core::TypeKind for TimedTextUnit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextUnit").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextUnit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextUnit;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
impl ::windows_core::TypeKind for TimedTextWeight {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextWeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWeight").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextWeight {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWeight;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
impl ::windows_core::TypeKind for TimedTextWrapping {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextWrapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWrapping").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextWrapping {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWrapping;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: Self = Self(0i32);
    pub const RightLeftTopBottom: Self = Self(1i32);
    pub const TopBottomRightLeft: Self = Self(2i32);
    pub const TopBottomLeftRight: Self = Self(3i32);
    pub const LeftRight: Self = Self(4i32);
    pub const RightLeft: Self = Self(5i32);
    pub const TopBottom: Self = Self(6i32);
}
impl ::windows_core::TypeKind for TimedTextWritingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextWritingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWritingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextWritingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWritingMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
impl ::windows_core::TypeKind for VideoStabilizationEffectEnabledChangedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoStabilizationEffectEnabledChangedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.VideoStabilizationEffectEnabledChangedReason;i4)");
}
#[repr(C)]
pub struct MseTimeRange {
    pub Start: super::super::Foundation::TimeSpan,
    pub End: super::super::Foundation::TimeSpan,
}
impl ::core::marker::Copy for MseTimeRange {}
impl ::core::clone::Clone for MseTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MseTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MseTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
impl ::windows_core::TypeKind for MseTimeRange {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for MseTimeRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.MseTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
}
impl ::core::cmp::PartialEq for MseTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.Start == other.Start && self.End == other.End
    }
}
impl ::core::cmp::Eq for MseTimeRange {}
impl ::core::default::Default for MseTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TimedTextDouble {
    pub Value: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextDouble {}
impl ::core::clone::Clone for TimedTextDouble {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextDouble").field("Value", &self.Value).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextDouble {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextDouble {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextDouble;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextDouble {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextDouble {}
impl ::core::default::Default for TimedTextDouble {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TimedTextPadding {
    pub Before: f64,
    pub After: f64,
    pub Start: f64,
    pub End: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPadding {}
impl ::core::clone::Clone for TimedTextPadding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPadding").field("Before", &self.Before).field("After", &self.After).field("Start", &self.Start).field("End", &self.End).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextPadding {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextPadding {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPadding;f8;f8;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextPadding {
    fn eq(&self, other: &Self) -> bool {
        self.Before == other.Before && self.After == other.After && self.Start == other.Start && self.End == other.End && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextPadding {}
impl ::core::default::Default for TimedTextPadding {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TimedTextPoint {
    pub X: f64,
    pub Y: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPoint {}
impl ::core::clone::Clone for TimedTextPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPoint").field("X", &self.X).field("Y", &self.Y).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextPoint {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextPoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPoint;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextPoint {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextPoint {}
impl ::core::default::Default for TimedTextPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TimedTextSize {
    pub Height: f64,
    pub Width: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextSize {}
impl ::core::clone::Clone for TimedTextSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextSize").field("Height", &self.Height).field("Width", &self.Width).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextSize;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextSize {
    fn eq(&self, other: &Self) -> bool {
        self.Height == other.Height && self.Width == other.Width && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextSize {}
impl ::core::default::Default for TimedTextSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
