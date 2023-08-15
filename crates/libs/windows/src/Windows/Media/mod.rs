#[cfg(feature = "Media_AppBroadcasting")]
pub mod AppBroadcasting;
#[cfg(feature = "Media_AppRecording")]
pub mod AppRecording;
#[cfg(feature = "Media_Audio")]
pub mod Audio;
#[cfg(feature = "Media_Capture")]
pub mod Capture;
#[cfg(feature = "Media_Casting")]
pub mod Casting;
#[cfg(feature = "Media_ClosedCaptioning")]
pub mod ClosedCaptioning;
#[cfg(feature = "Media_ContentRestrictions")]
pub mod ContentRestrictions;
#[cfg(feature = "Media_Control")]
pub mod Control;
#[cfg(feature = "Media_Core")]
pub mod Core;
#[cfg(feature = "Media_Devices")]
pub mod Devices;
#[cfg(feature = "Media_DialProtocol")]
pub mod DialProtocol;
#[cfg(feature = "Media_Editing")]
pub mod Editing;
#[cfg(feature = "Media_Effects")]
pub mod Effects;
#[cfg(feature = "Media_FaceAnalysis")]
pub mod FaceAnalysis;
#[cfg(feature = "Media_Import")]
pub mod Import;
#[cfg(feature = "Media_MediaProperties")]
pub mod MediaProperties;
#[cfg(feature = "Media_Miracast")]
pub mod Miracast;
#[cfg(feature = "Media_Ocr")]
pub mod Ocr;
#[cfg(feature = "Media_PlayTo")]
pub mod PlayTo;
#[cfg(feature = "Media_Playback")]
pub mod Playback;
#[cfg(feature = "Media_Playlists")]
pub mod Playlists;
#[cfg(feature = "Media_Protection")]
pub mod Protection;
#[cfg(feature = "Media_Render")]
pub mod Render;
#[cfg(feature = "Media_SpeechRecognition")]
pub mod SpeechRecognition;
#[cfg(feature = "Media_SpeechSynthesis")]
pub mod SpeechSynthesis;
#[cfg(feature = "Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Media_Transcoding")]
pub mod Transcoding;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
}
impl ::core::clone::Clone for IAudioBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35175827_724b_4c6a_b130_f6537f9ae0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrame {
    type Vtable = IAudioFrame_Vtbl;
}
impl ::core::clone::Clone for IAudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe36ac304_aab2_4277_9ed0_43cedf8e29c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LockBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: AudioBufferAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioFrameFactory {
    type Vtable = IAudioFrameFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioFrameFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioFrameFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91a90ade_2422_40a6_b9ad_30d02404317d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoRepeatModeChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAutoRepeatModeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAutoRepeatModeChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea137efa_d852_438e_882b_c990109a78f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestedAutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
}
impl ::core::clone::Clone for IImageDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageDisplayProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IMediaControl(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IMediaControl {
    type Vtable = IMediaControl_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IMediaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IMediaControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98f1fbe1_7a8d_42cb_b6fe_8fe698264f13);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SoundLevelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSoundLevelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlayPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlayPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PausePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PausePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePausePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePausePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StopPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StopPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayPauseTogglePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayPauseTogglePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlayPauseTogglePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlayPauseTogglePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecordPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecordPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecordPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecordPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub NextTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    NextTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveNextTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveNextTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PreviousTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PreviousTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePreviousTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePreviousTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FastForwardPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FastForwardPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFastForwardPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFastForwardPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RewindPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RewindPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRewindPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRewindPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ChannelUpPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ChannelUpPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveChannelUpPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveChannelUpPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ChannelDownPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ChannelDownPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveChannelDownPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SetTrackName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTrackName: usize,
    #[cfg(feature = "deprecated")]
    pub TrackName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrackName: usize,
    #[cfg(feature = "deprecated")]
    pub SetArtistName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub ArtistName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub IsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPlaying: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetAlbumArt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetAlbumArt: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AlbumArt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AlbumArt: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaExtension(::windows_core::IUnknown);
impl IMediaExtension {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaExtension, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaExtension {}
impl ::core::fmt::Debug for IMediaExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaExtension").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaExtension {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{07915118-45df-442b-8a3f-f7826a6370ab}");
}
unsafe impl ::windows_core::Interface for IMediaExtension {
    type Vtable = IMediaExtension_Vtbl;
}
impl ::core::clone::Clone for IMediaExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaExtension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07915118_45df_442b_8a3f_f7826a6370ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtension_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaExtensionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
}
impl ::core::clone::Clone for IMediaExtensionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaExtensionManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a25eaf5_242d_4dfb_97f4_69b7c42576ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RegisterSchemeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterSchemeHandlerWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, scheme: ::std::mem::MaybeUninit<::windows_core::HSTRING>, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterSchemeHandlerWithSettings: usize,
    pub RegisterByteStreamHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fileextension: ::std::mem::MaybeUninit<::windows_core::HSTRING>, mimetype: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterByteStreamHandlerWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fileextension: ::std::mem::MaybeUninit<::windows_core::HSTRING>, mimetype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterByteStreamHandlerWithSettings: usize,
    pub RegisterAudioDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioDecoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioDecoderWithSettings: usize,
    pub RegisterAudioEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioEncoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioEncoderWithSettings: usize,
    pub RegisterVideoDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoDecoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoDecoderWithSettings: usize,
    pub RegisterVideoEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoEncoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoEncoderWithSettings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaExtensionManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaExtensionManager2 {
    type Vtable = IMediaExtensionManager2_Vtbl;
}
impl ::core::clone::Clone for IMediaExtensionManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaExtensionManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bcebf47_4043_4fed_acaf_54ec29dfb1f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub RegisterMediaExtensionForAppService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: *mut ::core::ffi::c_void, connection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    RegisterMediaExtensionForAppService: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaFrame(::windows_core::IUnknown);
impl IMediaFrame {
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::Foundation::IClosable> for IMediaFrame {}
impl ::core::cmp::PartialEq for IMediaFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaFrame {}
impl ::core::fmt::Debug for IMediaFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{bfb52f8c-5943-47d8-8e10-05308aa5fbd0}");
}
unsafe impl ::windows_core::Interface for IMediaFrame {
    type Vtable = IMediaFrame_Vtbl;
}
impl ::core::clone::Clone for IMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfb52f8c_5943_47d8_8e10_05308aa5fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetIsDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaMarker(::windows_core::IUnknown);
impl IMediaMarker {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Time)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaMarkerType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaMarkerType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaMarker, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaMarker {}
impl ::core::fmt::Debug for IMediaMarker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaMarker").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaMarker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1803def8-dca5-4b6f-9c20-e3d3c0643625}");
}
unsafe impl ::windows_core::Interface for IMediaMarker {
    type Vtable = IMediaMarker_Vtbl;
}
impl ::core::clone::Clone for IMediaMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaMarker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1803def8_dca5_4b6f_9c20_e3d3c0643625);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMarkerTypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaMarkerTypesStatics {
    type Vtable = IMediaMarkerTypesStatics_Vtbl;
}
impl ::core::clone::Clone for IMediaMarkerTypesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaMarkerTypesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb198040_482f_4743_8832_45853821ece0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkerTypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Bookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaMarkers(::windows_core::IUnknown);
impl IMediaMarkers {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Markers(&self) -> ::windows_core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Markers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaMarkers, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaMarkers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaMarkers {}
impl ::core::fmt::Debug for IMediaMarkers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaMarkers").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaMarkers {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{afeab189-f8dd-466e-aa10-920b52353fdf}");
}
unsafe impl ::windows_core::Interface for IMediaMarkers {
    type Vtable = IMediaMarkers_Vtbl;
}
impl ::core::clone::Clone for IMediaMarkers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaMarkers {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xafeab189_f8dd_466e_aa10_920b52353fdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkers_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProcessingTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IMediaProcessingTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaProcessingTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb8564ac_a351_4f4e_b4f0_9bf2408993db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineController(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
}
impl ::core::clone::Clone for IMediaTimelineController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaTimelineController {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ed361f3_0b78_4360_bf71_0c841999ea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    pub ClockRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetClockRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaTimelineControllerState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positionchangedeventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statechangedeventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineController2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTimelineController2 {
    type Vtable = IMediaTimelineController2_Vtbl;
}
impl ::core::clone::Clone for IMediaTimelineController2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaTimelineController2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef74ea38_9e72_4df9_8355_6e90c81bbadd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
    #[cfg(feature = "Foundation")]
    pub Ended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Ended: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnded: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineControllerFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaTimelineControllerFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaTimelineControllerFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8821f81d_3e77_43fb_be26_4fc87a044834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineControllerFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
}
impl ::core::clone::Clone for IMusicDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMusicDisplayProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMusicDisplayProperties2 {
    type Vtable = IMusicDisplayProperties2_Vtbl;
}
impl ::core::clone::Clone for IMusicDisplayProperties2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMusicDisplayProperties2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00368462_97d3_44b9_b00f_008afcefaf18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMusicDisplayProperties3 {
    type Vtable = IMusicDisplayProperties3_Vtbl;
}
impl ::core::clone::Clone for IMusicDisplayProperties3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMusicDisplayProperties3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4db51ac1_0681_4e8c_9401_b8159d9eefc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetAlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackPositionChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPlaybackPositionChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaybackPositionChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4493f88_eb28_4961_9c14_335e44f3e125);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackPositionChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestedPlaybackPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedPlaybackPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPlaybackRateChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShuffleEnabledChangeRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IShuffleEnabledChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IShuffleEnabledChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49b593fe_4fd0_4666_a314_c0e01940d302);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShuffleEnabledChangeRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestedShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControls(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControls {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99fa3ff4_1742_42a6_902e_087d41f965ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackStatus) -> ::windows_core::HRESULT,
    pub SetPlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackStatus) -> ::windows_core::HRESULT,
    pub DisplayUpdater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertyChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControls2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControls2 {
    type Vtable = ISystemMediaTransportControls2_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControls2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControls2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea98d2f6_7f3c_4af2_a586_72889808efb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
    pub SetAutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackAutoRepeatMode) -> ::windows_core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub UpdateTimelineProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timelineproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackPositionChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackPositionChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShuffleEnabledChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShuffleEnabledChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatModeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoRepeatModeChangeRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControlsButtonPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControlsButtonPressedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7f47116_a56f_4dc8_9e11_92031f4a87c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Button: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsButton) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsDisplayUpdater(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControlsDisplayUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControlsDisplayUpdater {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsDisplayUpdater_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackType) -> ::windows_core::HRESULT,
    pub AppMediaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppMediaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CopyFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: MediaPlaybackType, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CopyFromFileAsync: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControlsPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControlsPropertyChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0ca0936_339b_4cb3_8eeb_737607f56e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsProperty) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsStatics {
    type Vtable = ISystemMediaTransportControlsStatics_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControlsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControlsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43ba380a_eca4_4832_91ab_d415fae484c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsTimelineProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
}
impl ::core::clone::Clone for ISystemMediaTransportControlsTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISystemMediaTransportControlsTimelineProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5125316a_c3a2_475b_8507_93534dc88f15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsTimelineProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub MinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDisplayProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
}
impl ::core::clone::Clone for IVideoDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoDisplayProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDisplayProperties2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoDisplayProperties2 {
    type Vtable = IVideoDisplayProperties2_Vtbl;
}
impl ::core::clone::Clone for IVideoDisplayProperties2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoDisplayProperties2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb410e1ce_ab52_41ab_a486_cc10fab152f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEffectsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoEffectsStatics {
    type Vtable = IVideoEffectsStatics_Vtbl;
}
impl ::core::clone::Clone for IVideoEffectsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoEffectsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1fcda5e8_baf1_4521_980c_3bcebb44cf38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoStabilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrame {
    type Vtable = IVideoFrame_Vtbl;
}
impl ::core::clone::Clone for IVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cc06625_90fc_4c92_bd95_7ded21819d1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    #[cfg(feature = "Foundation")]
    pub CopyToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyToAsync: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrame2 {
    type Vtable = IVideoFrame2_Vtbl;
}
impl ::core::clone::Clone for IVideoFrame2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoFrame2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3837840d_336c_4366_8d46_060798736c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub CopyToWithBoundsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut ::core::ffi::c_void, sourcebounds: *mut ::core::ffi::c_void, destinationbounds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    CopyToWithBoundsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrameFactory {
    type Vtable = IVideoFrameFactory_Vtbl;
}
impl ::core::clone::Clone for IVideoFrameFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoFrameFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x014b6d69_2228_4c92_92ff_50c380d3e776);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Create: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithAlpha: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoFrameStatics {
    type Vtable = IVideoFrameStatics_Vtbl;
}
impl ::core::clone::Clone for IVideoFrameStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoFrameStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2a556f_6111_4b33_8ec3_2b209a02e17a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateAsDirect3D11SurfaceBacked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateAsDirect3D11SurfaceBacked: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateAsDirect3D11SurfaceBackedWithDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateAsDirect3D11SurfaceBackedWithDevice: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateWithDirect3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateWithDirect3D11Surface: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AudioBuffer(::windows_core::IUnknown);
impl AudioBuffer {
    pub fn Capacity(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Capacity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioBuffer {}
impl ::core::fmt::Debug for AudioBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioBuffer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AudioBuffer;{35175827-724b-4c6a-b130-f6537f9ae0d0})");
}
impl ::core::clone::Clone for AudioBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioBuffer {
    const IID: ::windows_core::GUID = <IAudioBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioBuffer {
    const NAME: &'static str = "Windows.Media.AudioBuffer";
}
::windows_core::imp::interface_hierarchy!(AudioBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::Foundation::IClosable> for AudioBuffer {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::Foundation::IMemoryBuffer> for AudioBuffer {}
unsafe impl ::core::marker::Send for AudioBuffer {}
unsafe impl ::core::marker::Sync for AudioBuffer {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AudioFrame(::windows_core::IUnknown);
impl AudioFrame {
    pub fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows_core::Result<AudioBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LockBuffer)(::windows_core::Interface::as_raw(this), mode, &mut result__).from_abi(result__)
        }
    }
    pub fn Create(capacity: u32) -> ::windows_core::Result<AudioFrame> {
        Self::IAudioFrameFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), capacity, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAudioFrameFactory<R, F: FnOnce(&IAudioFrameFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioFrame, IAudioFrameFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioFrame {}
impl ::core::fmt::Debug for AudioFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AudioFrame;{e36ac304-aab2-4277-9ed0-43cedf8e29c6})");
}
impl ::core::clone::Clone for AudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioFrame {
    type Vtable = IAudioFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioFrame {
    const IID: ::windows_core::GUID = <IAudioFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioFrame {
    const NAME: &'static str = "Windows.Media.AudioFrame";
}
::windows_core::imp::interface_hierarchy!(AudioFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::Foundation::IClosable> for AudioFrame {}
impl ::windows_core::CanTryInto<IMediaFrame> for AudioFrame {}
unsafe impl ::core::marker::Send for AudioFrame {}
unsafe impl ::core::marker::Sync for AudioFrame {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AutoRepeatModeChangeRequestedEventArgs(::windows_core::IUnknown);
impl AutoRepeatModeChangeRequestedEventArgs {
    pub fn RequestedAutoRepeatMode(&self) -> ::windows_core::Result<MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedAutoRepeatMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AutoRepeatModeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutoRepeatModeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for AutoRepeatModeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoRepeatModeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutoRepeatModeChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AutoRepeatModeChangeRequestedEventArgs;{ea137efa-d852-438e-882b-c990109a78f4})");
}
impl ::core::clone::Clone for AutoRepeatModeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutoRepeatModeChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IAutoRepeatModeChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AutoRepeatModeChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AutoRepeatModeChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AutoRepeatModeChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct ImageDisplayProperties(::windows_core::IUnknown);
impl ImageDisplayProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for ImageDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageDisplayProperties {}
impl ::core::fmt::Debug for ImageDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageDisplayProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageDisplayProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.ImageDisplayProperties;{cd0bc7ef-54e7-411f-9933-f0e98b0a96d2})");
}
impl ::core::clone::Clone for ImageDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageDisplayProperties {
    const IID: ::windows_core::GUID = <IImageDisplayProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.ImageDisplayProperties";
}
::windows_core::imp::interface_hierarchy!(ImageDisplayProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ImageDisplayProperties {}
unsafe impl ::core::marker::Sync for ImageDisplayProperties {}
#[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct MediaControl;
#[cfg(feature = "deprecated")]
impl MediaControl {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SoundLevelChanged<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevelChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSoundLevelChanged(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSoundLevelChanged)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PlayPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlayPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePlayPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePlayPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PausePressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PausePressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePausePressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePausePressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StopPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStopPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveStopPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PlayPauseTogglePressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlayPauseTogglePressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePlayPauseTogglePressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePlayPauseTogglePressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecordPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecordPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRecordPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn NextTrackPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextTrackPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveNextTrackPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNextTrackPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PreviousTrackPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousTrackPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePreviousTrackPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePreviousTrackPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FastForwardPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FastForwardPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFastForwardPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveFastForwardPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RewindPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RewindPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRewindPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveRewindPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ChannelUpPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelUpPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveChannelUpPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveChannelUpPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ChannelDownPressed<P0>(handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelDownPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveChannelDownPressed(cookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveChannelDownPressed)(::windows_core::Interface::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SoundLevel() -> ::windows_core::Result<SoundLevel> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTrackName(value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetTrackName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TrackName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetArtistName(value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetArtistName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ArtistName() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ArtistName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsPlaying(value: bool) -> ::windows_core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsPlaying)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsPlaying() -> ::windows_core::Result<bool> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlaying)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetAlbumArt<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Foundation::Uri>,
    {
        Self::IMediaControl(|this| unsafe { (::windows_core::Interface::vtable(this).SetAlbumArt)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AlbumArt() -> ::windows_core::Result<super::Foundation::Uri> {
        Self::IMediaControl(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IMediaControl<R, F: FnOnce(&IMediaControl) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaControl, IMediaControl> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for MediaControl {
    const NAME: &'static str = "Windows.Media.MediaControl";
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaExtensionManager(::windows_core::IUnknown);
impl MediaExtensionManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaExtensionManager, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn RegisterSchemeHandler(&self, activatableclassid: &::windows_core::HSTRING, scheme: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterSchemeHandler)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), ::core::mem::transmute_copy(scheme)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterSchemeHandlerWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, scheme: &::windows_core::HSTRING, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterSchemeHandlerWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), ::core::mem::transmute_copy(scheme), configuration.try_into_param()?.abi()).ok() }
    }
    pub fn RegisterByteStreamHandler(&self, activatableclassid: &::windows_core::HSTRING, fileextension: &::windows_core::HSTRING, mimetype: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterByteStreamHandler)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), ::core::mem::transmute_copy(fileextension), ::core::mem::transmute_copy(mimetype)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterByteStreamHandlerWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, fileextension: &::windows_core::HSTRING, mimetype: &::windows_core::HSTRING, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterByteStreamHandlerWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), ::core::mem::transmute_copy(fileextension), ::core::mem::transmute_copy(mimetype), configuration.try_into_param()?.abi()).ok() }
    }
    pub fn RegisterAudioDecoder(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioDecoder)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioDecoderWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioDecoderWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.try_into_param()?.abi()).ok() }
    }
    pub fn RegisterAudioEncoder(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioEncoder)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioEncoderWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterAudioEncoderWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.try_into_param()?.abi()).ok() }
    }
    pub fn RegisterVideoDecoder(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoDecoder)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoDecoderWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoDecoderWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.try_into_param()?.abi()).ok() }
    }
    pub fn RegisterVideoEncoder(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoEncoder)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoEncoderWithSettings<P0>(&self, activatableclassid: &::windows_core::HSTRING, inputsubtype: ::windows_core::GUID, outputsubtype: ::windows_core::GUID, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterVideoEncoderWithSettings)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), inputsubtype, outputsubtype, configuration.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn RegisterMediaExtensionForAppService<P0, P1>(&self, extension: P0, connection: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMediaExtension>,
        P1: ::windows_core::IntoParam<super::ApplicationModel::AppService::AppServiceConnection>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaExtensionManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RegisterMediaExtensionForAppService)(::windows_core::Interface::as_raw(this), extension.try_into_param()?.abi(), connection.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaExtensionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaExtensionManager {}
impl ::core::fmt::Debug for MediaExtensionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaExtensionManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaExtensionManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.MediaExtensionManager;{4a25eaf5-242d-4dfb-97f4-69b7c42576ff})");
}
impl ::core::clone::Clone for MediaExtensionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaExtensionManager {
    const IID: ::windows_core::GUID = <IMediaExtensionManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaExtensionManager {
    const NAME: &'static str = "Windows.Media.MediaExtensionManager";
}
::windows_core::imp::interface_hierarchy!(MediaExtensionManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaExtensionManager {}
unsafe impl ::core::marker::Sync for MediaExtensionManager {}
#[doc = "*Required features: `\"Media\"`*"]
pub struct MediaMarkerTypes;
impl MediaMarkerTypes {
    pub fn Bookmark() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaMarkerTypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bookmark)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaMarkerTypesStatics<R, F: FnOnce(&IMediaMarkerTypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaMarkerTypes, IMediaMarkerTypesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MediaMarkerTypes {
    const NAME: &'static str = "Windows.Media.MediaMarkerTypes";
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaProcessingTriggerDetails(::windows_core::IUnknown);
impl MediaProcessingTriggerDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> ::windows_core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaProcessingTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProcessingTriggerDetails {}
impl ::core::fmt::Debug for MediaProcessingTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProcessingTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaProcessingTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProcessingTriggerDetails;{eb8564ac-a351-4f4e-b4f0-9bf2408993db})");
}
impl ::core::clone::Clone for MediaProcessingTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaProcessingTriggerDetails {
    const IID: ::windows_core::GUID = <IMediaProcessingTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.MediaProcessingTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(MediaProcessingTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaProcessingTriggerDetails {}
unsafe impl ::core::marker::Sync for MediaProcessingTriggerDetails {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaTimelineController(::windows_core::IUnknown);
impl MediaTimelineController {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaTimelineController, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClockRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClockRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetClockRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClockRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<MediaTimelineControllerState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<P0>(&self, positionchangedeventhandler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PositionChanged)(::windows_core::Interface::as_raw(this), positionchangedeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged(&self, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePositionChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, statechangedeventhandler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), statechangedeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsLoopingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLoopingEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLoopingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Failed<P0>(&self, eventhandler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Failed)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Ended<P0>(&self, eventhandler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<MediaTimelineController, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ended)(::windows_core::Interface::as_raw(this), eventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnded(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaTimelineController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTimelineController {}
impl ::core::fmt::Debug for MediaTimelineController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineController").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaTimelineController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineController;{8ed361f3-0b78-4360-bf71-0c841999ea1b})");
}
impl ::core::clone::Clone for MediaTimelineController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaTimelineController {
    const IID: ::windows_core::GUID = <IMediaTimelineController as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaTimelineController {
    const NAME: &'static str = "Windows.Media.MediaTimelineController";
}
::windows_core::imp::interface_hierarchy!(MediaTimelineController, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaTimelineController {}
unsafe impl ::core::marker::Sync for MediaTimelineController {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaTimelineControllerFailedEventArgs(::windows_core::IUnknown);
impl MediaTimelineControllerFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaTimelineControllerFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTimelineControllerFailedEventArgs {}
impl ::core::fmt::Debug for MediaTimelineControllerFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineControllerFailedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaTimelineControllerFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineControllerFailedEventArgs;{8821f81d-3e77-43fb-be26-4fc87a044834})");
}
impl ::core::clone::Clone for MediaTimelineControllerFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaTimelineControllerFailedEventArgs {
    const IID: ::windows_core::GUID = <IMediaTimelineControllerFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.MediaTimelineControllerFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaTimelineControllerFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaTimelineControllerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTimelineControllerFailedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MusicDisplayProperties(::windows_core::IUnknown);
impl MusicDisplayProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AlbumArtist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumArtist)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlbumArtist(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlbumArtist)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Artist(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Artist)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetArtist(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetArtist)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AlbumTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlbumTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlbumTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TrackNumber(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTrackNumber(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTrackNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlbumTrackCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlbumTrackCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlbumTrackCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlbumTrackCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for MusicDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MusicDisplayProperties {}
impl ::core::fmt::Debug for MusicDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MusicDisplayProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MusicDisplayProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.MusicDisplayProperties;{6bbf0c59-d0a0-4d26-92a0-f978e1d18e7b})");
}
impl ::core::clone::Clone for MusicDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MusicDisplayProperties {
    const IID: ::windows_core::GUID = <IMusicDisplayProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.MusicDisplayProperties";
}
::windows_core::imp::interface_hierarchy!(MusicDisplayProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MusicDisplayProperties {}
unsafe impl ::core::marker::Sync for MusicDisplayProperties {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct PlaybackPositionChangeRequestedEventArgs(::windows_core::IUnknown);
impl PlaybackPositionChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedPlaybackPosition(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedPlaybackPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PlaybackPositionChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackPositionChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackPositionChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackPositionChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlaybackPositionChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackPositionChangeRequestedEventArgs;{b4493f88-eb28-4961-9c14-335e44f3e125})");
}
impl ::core::clone::Clone for PlaybackPositionChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaybackPositionChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPlaybackPositionChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackPositionChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PlaybackPositionChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PlaybackPositionChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackPositionChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows_core::IUnknown);
impl PlaybackRateChangeRequestedEventArgs {
    pub fn RequestedPlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedPlaybackRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PlaybackRateChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackRateChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackRateChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackRateChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackRateChangeRequestedEventArgs;{2ce2c41f-3cd6-4f77-9ba7-eb27c26a2140})");
}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlaybackRateChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPlaybackRateChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackRateChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PlaybackRateChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PlaybackRateChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackRateChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct ShuffleEnabledChangeRequestedEventArgs(::windows_core::IUnknown);
impl ShuffleEnabledChangeRequestedEventArgs {
    pub fn RequestedShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedShuffleEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ShuffleEnabledChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShuffleEnabledChangeRequestedEventArgs {}
impl ::core::fmt::Debug for ShuffleEnabledChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShuffleEnabledChangeRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ShuffleEnabledChangeRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.ShuffleEnabledChangeRequestedEventArgs;{49b593fe-4fd0-4666-a314-c0e01940d302})");
}
impl ::core::clone::Clone for ShuffleEnabledChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ShuffleEnabledChangeRequestedEventArgs {
    const IID: ::windows_core::GUID = <IShuffleEnabledChangeRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ShuffleEnabledChangeRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ShuffleEnabledChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ShuffleEnabledChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControls(::windows_core::IUnknown);
impl SystemMediaTransportControls {
    pub fn PlaybackStatus(&self) -> ::windows_core::Result<MediaPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayUpdater(&self) -> ::windows_core::Result<SystemMediaTransportControlsDisplayUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayUpdater)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SoundLevel(&self) -> ::windows_core::Result<SoundLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoundLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPlayEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPlayEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPlayEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStopEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStopEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsStopEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsStopEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPauseEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPauseEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPauseEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPauseEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRecordEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRecordEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRecordEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRecordEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFastForwardEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFastForwardEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsFastForwardEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRewindEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRewindEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRewindEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsRewindEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPreviousEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPreviousEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsPreviousEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsPreviousEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsNextEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNextEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsNextEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsNextEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelUpEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelUpEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsChannelUpEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsChannelUpEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChannelDownEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsChannelDownEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsChannelDownEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsChannelDownEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressed<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ButtonPressed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveButtonPressed(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveButtonPressed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertyChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PropertyChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertyChanged(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePropertyChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoRepeatMode(&self) -> ::windows_core::Result<MediaPlaybackAutoRepeatMode> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoRepeatMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShuffleEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PlaybackRate(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaybackRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UpdateTimelineProperties<P0>(&self, timelineproperties: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SystemMediaTransportControlsTimelineProperties>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).UpdateTimelineProperties)(::windows_core::Interface::as_raw(this), timelineproperties.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackPositionChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackPositionChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackPositionChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackPositionChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackRateChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePlaybackRateChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShuffleEnabledChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShuffleEnabledChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShuffleEnabledChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveShuffleEnabledChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatModeChangeRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoRepeatModeChangeRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoRepeatModeChangeRequested(&self, token: super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAutoRepeatModeChangeRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<SystemMediaTransportControls> {
        Self::ISystemMediaTransportControlsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemMediaTransportControlsStatics<R, F: FnOnce(&ISystemMediaTransportControlsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemMediaTransportControls, ISystemMediaTransportControlsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControls {}
impl ::core::fmt::Debug for SystemMediaTransportControls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControls").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControls {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControls;{99fa3ff4-1742-42a6-902e-087d41f965ec})");
}
impl ::core::clone::Clone for SystemMediaTransportControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemMediaTransportControls {
    const IID: ::windows_core::GUID = <ISystemMediaTransportControls as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControls";
}
::windows_core::imp::interface_hierarchy!(SystemMediaTransportControls, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemMediaTransportControls {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControls {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(::windows_core::IUnknown);
impl SystemMediaTransportControlsButtonPressedEventArgs {
    pub fn Button(&self) -> ::windows_core::Result<SystemMediaTransportControlsButton> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Button)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsButtonPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsButtonPressedEventArgs {}
impl ::core::fmt::Debug for SystemMediaTransportControlsButtonPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsButtonPressedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControlsButtonPressedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs;{b7f47116-a56f-4dc8-9e11-92031f4a87c2})");
}
impl ::core::clone::Clone for SystemMediaTransportControlsButtonPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemMediaTransportControlsButtonPressedEventArgs {
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsButtonPressedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsButtonPressedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemMediaTransportControlsButtonPressedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsButtonPressedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsDisplayUpdater(::windows_core::IUnknown);
impl SystemMediaTransportControlsDisplayUpdater {
    pub fn Type(&self) -> ::windows_core::Result<MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: MediaPlaybackType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppMediaId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppMediaId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppMediaId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppMediaId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Storage::Streams::RandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MusicProperties(&self) -> ::windows_core::Result<MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoProperties(&self) -> ::windows_core::Result<VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ImageProperties(&self) -> ::windows_core::Result<ImageDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CopyFromFileAsync<P0>(&self, r#type: MediaPlaybackType, source: P0) -> ::windows_core::Result<super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyFromFileAsync)(::windows_core::Interface::as_raw(this), r#type, source.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ClearAll(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearAll)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Update(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsDisplayUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsDisplayUpdater {}
impl ::core::fmt::Debug for SystemMediaTransportControlsDisplayUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsDisplayUpdater").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControlsDisplayUpdater {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsDisplayUpdater;{8abbc53e-fa55-4ecf-ad8e-c984e5dd1550})");
}
impl ::core::clone::Clone for SystemMediaTransportControlsDisplayUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemMediaTransportControlsDisplayUpdater {
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsDisplayUpdater as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsDisplayUpdater";
}
::windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsDisplayUpdater, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemMediaTransportControlsDisplayUpdater {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsDisplayUpdater {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(::windows_core::IUnknown);
impl SystemMediaTransportControlsPropertyChangedEventArgs {
    pub fn Property(&self) -> ::windows_core::Result<SystemMediaTransportControlsProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Property)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsPropertyChangedEventArgs {}
impl ::core::fmt::Debug for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsPropertyChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControlsPropertyChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs;{d0ca0936-339b-4cb3-8eeb-737607f56e08})");
}
impl ::core::clone::Clone for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemMediaTransportControlsPropertyChangedEventArgs {
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsPropertyChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsPropertyChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemMediaTransportControlsPropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsPropertyChangedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsTimelineProperties(::windows_core::IUnknown);
impl SystemMediaTransportControlsTimelineProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemMediaTransportControlsTimelineProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSeekTime(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinSeekTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMinSeekTime(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinSeekTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSeekTime(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSeekTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSeekTime(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSeekTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows_core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition(&self, value: super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SystemMediaTransportControlsTimelineProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMediaTransportControlsTimelineProperties {}
impl ::core::fmt::Debug for SystemMediaTransportControlsTimelineProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsTimelineProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControlsTimelineProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsTimelineProperties;{5125316a-c3a2-475b-8507-93534dc88f15})");
}
impl ::core::clone::Clone for SystemMediaTransportControlsTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemMediaTransportControlsTimelineProperties {
    const IID: ::windows_core::GUID = <ISystemMediaTransportControlsTimelineProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsTimelineProperties";
}
::windows_core::imp::interface_hierarchy!(SystemMediaTransportControlsTimelineProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SystemMediaTransportControlsTimelineProperties {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsTimelineProperties {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct VideoDisplayProperties(::windows_core::IUnknown);
impl VideoDisplayProperties {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows_core::Result<super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IVideoDisplayProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Genres)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoDisplayProperties {}
impl ::core::fmt::Debug for VideoDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDisplayProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoDisplayProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.VideoDisplayProperties;{5609fdb1-5d2d-4872-8170-45dee5bc2f5c})");
}
impl ::core::clone::Clone for VideoDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoDisplayProperties {
    const IID: ::windows_core::GUID = <IVideoDisplayProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.VideoDisplayProperties";
}
::windows_core::imp::interface_hierarchy!(VideoDisplayProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoDisplayProperties {}
unsafe impl ::core::marker::Sync for VideoDisplayProperties {}
#[doc = "*Required features: `\"Media\"`*"]
pub struct VideoEffects;
impl VideoEffects {
    pub fn VideoStabilization() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IVideoEffectsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoStabilization)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoEffectsStatics<R, F: FnOnce(&IVideoEffectsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoEffects, IVideoEffectsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for VideoEffects {
    const NAME: &'static str = "Windows.Media.VideoEffects";
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct VideoFrame(::windows_core::IUnknown);
impl VideoFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyToAsync<P0>(&self, frame: P0) -> ::windows_core::Result<super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<VideoFrame>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyToAsync)(::windows_core::Interface::as_raw(this), frame.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> ::windows_core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direct3DSurface)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn CopyToWithBoundsAsync<P0, P1, P2>(&self, frame: P0, sourcebounds: P1, destinationbounds: P2) -> ::windows_core::Result<super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<VideoFrame>,
        P1: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>,
        P2: ::windows_core::TryIntoParam<super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVideoFrame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyToWithBoundsAsync)(::windows_core::Interface::as_raw(this), frame.into_param().abi(), sourcebounds.try_into_param()?.abi(), destinationbounds.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Create(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), format, width, height, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithAlpha(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAlpha)(::windows_core::Interface::as_raw(this), format, width, height, alpha, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateAsDirect3D11SurfaceBacked(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows_core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsDirect3D11SurfaceBacked)(::windows_core::Interface::as_raw(this), format, width, height, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateAsDirect3D11SurfaceBackedWithDevice<P0>(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: P0) -> ::windows_core::Result<VideoFrame>
    where
        P0: ::windows_core::TryIntoParam<super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsDirect3D11SurfaceBackedWithDevice)(::windows_core::Interface::as_raw(this), format, width, height, device.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithSoftwareBitmap<P0>(bitmap: P0) -> ::windows_core::Result<VideoFrame>
    where
        P0: ::windows_core::IntoParam<super::Graphics::Imaging::SoftwareBitmap>,
    {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithSoftwareBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateWithDirect3D11Surface<P0>(surface: P0) -> ::windows_core::Result<VideoFrame>
    where
        P0: ::windows_core::TryIntoParam<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithDirect3D11Surface)(::windows_core::Interface::as_raw(this), surface.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoFrameFactory<R, F: FnOnce(&IVideoFrameFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoFrame, IVideoFrameFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IVideoFrameStatics<R, F: FnOnce(&IVideoFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoFrame, IVideoFrameStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VideoFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoFrame {}
impl ::core::fmt::Debug for VideoFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.VideoFrame;{0cc06625-90fc-4c92-bd95-7ded21819d1c})");
}
impl ::core::clone::Clone for VideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoFrame {
    type Vtable = IVideoFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoFrame {
    const IID: ::windows_core::GUID = <IVideoFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoFrame {
    const NAME: &'static str = "Windows.Media.VideoFrame";
}
::windows_core::imp::interface_hierarchy!(VideoFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::Foundation::IClosable> for VideoFrame {}
impl ::windows_core::CanTryInto<IMediaFrame> for VideoFrame {}
unsafe impl ::core::marker::Send for VideoFrame {}
unsafe impl ::core::marker::Sync for VideoFrame {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioBufferAccessMode(pub i32);
impl AudioBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioBufferAccessMode {}
impl ::core::clone::Clone for AudioBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioBufferAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioBufferAccessMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioBufferAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioBufferAccessMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioBufferAccessMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.AudioBufferAccessMode;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioProcessing(pub i32);
impl AudioProcessing {
    pub const Default: Self = Self(0i32);
    pub const Raw: Self = Self(1i32);
}
impl ::core::marker::Copy for AudioProcessing {}
impl ::core::clone::Clone for AudioProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioProcessing {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioProcessing {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioProcessing").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioProcessing {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.AudioProcessing;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackAutoRepeatMode(pub i32);
impl MediaPlaybackAutoRepeatMode {
    pub const None: Self = Self(0i32);
    pub const Track: Self = Self(1i32);
    pub const List: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlaybackAutoRepeatMode {}
impl ::core::clone::Clone for MediaPlaybackAutoRepeatMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackAutoRepeatMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaPlaybackAutoRepeatMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaPlaybackAutoRepeatMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackAutoRepeatMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaPlaybackAutoRepeatMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackAutoRepeatMode;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackStatus(pub i32);
impl MediaPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Changing: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackStatus {}
impl ::core::clone::Clone for MediaPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaPlaybackStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaPlaybackStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackStatus;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackType(pub i32);
impl MediaPlaybackType {
    pub const Unknown: Self = Self(0i32);
    pub const Music: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackType {}
impl ::core::clone::Clone for MediaPlaybackType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaPlaybackType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaPlaybackType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaPlaybackType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackType;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaTimelineControllerState(pub i32);
impl MediaTimelineControllerState {
    pub const Paused: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Stalled: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaTimelineControllerState {}
impl ::core::clone::Clone for MediaTimelineControllerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaTimelineControllerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaTimelineControllerState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaTimelineControllerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineControllerState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaTimelineControllerState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaTimelineControllerState;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SoundLevel(pub i32);
impl SoundLevel {
    pub const Muted: Self = Self(0i32);
    pub const Low: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
}
impl ::core::marker::Copy for SoundLevel {}
impl ::core::clone::Clone for SoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SoundLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SoundLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SoundLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SoundLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SoundLevel;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemMediaTransportControlsButton(pub i32);
impl SystemMediaTransportControlsButton {
    pub const Play: Self = Self(0i32);
    pub const Pause: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Record: Self = Self(3i32);
    pub const FastForward: Self = Self(4i32);
    pub const Rewind: Self = Self(5i32);
    pub const Next: Self = Self(6i32);
    pub const Previous: Self = Self(7i32);
    pub const ChannelUp: Self = Self(8i32);
    pub const ChannelDown: Self = Self(9i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsButton {}
impl ::core::clone::Clone for SystemMediaTransportControlsButton {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemMediaTransportControlsButton {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SystemMediaTransportControlsButton {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemMediaTransportControlsButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsButton").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControlsButton {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsButton;i4)");
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemMediaTransportControlsProperty(pub i32);
impl SystemMediaTransportControlsProperty {
    pub const SoundLevel: Self = Self(0i32);
}
impl ::core::marker::Copy for SystemMediaTransportControlsProperty {}
impl ::core::clone::Clone for SystemMediaTransportControlsProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemMediaTransportControlsProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SystemMediaTransportControlsProperty {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemMediaTransportControlsProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsProperty").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemMediaTransportControlsProperty {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsProperty;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct MediaTimeRange {
    pub Start: super::Foundation::TimeSpan,
    pub End: super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MediaTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MediaTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for MediaTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MediaTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows_core::TypeKind for MediaTimeRange {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeType for MediaTimeRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.MediaTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for MediaTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.Start == other.Start && self.End == other.End
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for MediaTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for MediaTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
