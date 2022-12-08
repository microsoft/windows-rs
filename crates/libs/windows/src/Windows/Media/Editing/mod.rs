#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundAudioTrack(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBackgroundAudioTrack {
    type Vtable = IBackgroundAudioTrack_Vtbl;
}
unsafe impl ::windows::core::Interface for IBackgroundAudioTrack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b91b3bd_9e21_4266_a9c2_67dd011a2357);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundAudioTrack_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub OriginalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub TrimmedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimmedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserData: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetAudioEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetAudioEncodingProperties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub AudioEffectDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    AudioEffectDefinitions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundAudioTrackStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBackgroundAudioTrackStatics {
    type Vtable = IBackgroundAudioTrackStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBackgroundAudioTrackStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b1c0d7_d018_42a8_a559_cb4d9e97e664);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundAudioTrackStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromEmbeddedAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddedaudiotrack: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmbeddedAudioTrack(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmbeddedAudioTrack {
    type Vtable = IEmbeddedAudioTrack_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmbeddedAudioTrack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ee5a7a_2d30_3fba_a190_4f1a6454f88f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmbeddedAudioTrack_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetAudioEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetAudioEncodingProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaClip(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaClip {
    type Vtable = IMediaClip_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaClip {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f25366_5fba_3ea4_8693_24761811140a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClip_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromStart: usize,
    #[cfg(feature = "Foundation")]
    pub TrimTimeFromEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimTimeFromEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimTimeFromEnd: usize,
    #[cfg(feature = "Foundation")]
    pub OriginalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub TrimmedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimmedDuration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserData: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTimeInComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTimeInComposition: usize,
    #[cfg(feature = "Foundation")]
    pub EndTimeInComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTimeInComposition: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub EmbeddedAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EmbeddedAudioTracks: usize,
    pub SelectedEmbeddedAudioTrackIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetSelectedEmbeddedAudioTrackIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetVideoEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetVideoEncodingProperties: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub AudioEffectDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    AudioEffectDefinitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub VideoEffectDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))]
    VideoEffectDefinitions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaClipStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaClipStatics {
    type Vtable = IMediaClipStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaClipStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa402b68_928f_43c4_bc6e_783a1a359656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClipStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub CreateFromColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: super::super::UI::Color, originalduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    CreateFromColor: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateFromImageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, originalduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateFromImageFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaClipStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaClipStatics2 {
    type Vtable = IMediaClipStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaClipStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b1dd7b3_854e_4d9b_877d_4774a556cd12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClipStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateFromSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, originalduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateFromSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaComposition(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaComposition {
    type Vtable = IMediaComposition_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaComposition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e06e605_dc71_41d6_b837_2d2bc14a2947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaComposition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Clips: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Clips: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BackgroundAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BackgroundAudioTracks: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserData: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timefromstart: super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub GetThumbnailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timesfromstart: *mut ::core::ffi::c_void, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams")))]
    GetThumbnailsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub RenderToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))]
    RenderToFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub RenderToFileWithTrimmingPreferenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, trimmingpreference: MediaTrimmingPreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))]
    RenderToFileWithTrimmingPreferenceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))]
    pub RenderToFileWithProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, trimmingpreference: MediaTrimmingPreference, encodingprofile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage")))]
    RenderToFileWithProfileAsync: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub CreateDefaultEncodingProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    CreateDefaultEncodingProfile: usize,
    #[cfg(feature = "Media_Core")]
    pub GenerateMediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    GenerateMediaStreamSource: usize,
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))]
    pub GenerateMediaStreamSourceWithProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Media_MediaProperties")))]
    GenerateMediaStreamSourceWithProfile: usize,
    #[cfg(feature = "Media_Core")]
    pub GeneratePreviewMediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaledwidth: i32, scaledheight: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    GeneratePreviewMediaStreamSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaComposition2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaComposition2 {
    type Vtable = IMediaComposition2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaComposition2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa59e5372_2366_492c_bec8_e6dfba6d0281);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaComposition2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub OverlayLayers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OverlayLayers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCompositionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaCompositionStatics {
    type Vtable = IMediaCompositionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaCompositionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a08f04_e32a_45ce_8f66_a30df0766224);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCompositionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaOverlay(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaOverlay {
    type Vtable = IMediaOverlay_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaOverlay {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa902ae5d_7869_4830_8ab1_94dc01c05fa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlay_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    #[cfg(feature = "Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    pub Opacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AudioEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAudioEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaOverlayFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaOverlayFactory {
    type Vtable = IMediaOverlayFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaOverlayFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb584828a_6188_4f8f_a2e0_aa552d598e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWithPositionAndOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::core::ffi::c_void, position: super::super::Foundation::Rect, opacity: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPositionAndOpacity: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaOverlayLayer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaOverlayLayer {
    type Vtable = IMediaOverlayLayer_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaOverlayLayer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6d9ba57_eeda_46c6_bbe5_e398c84168ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayLayer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Overlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Overlays: usize,
    #[cfg(feature = "Media_Effects")]
    pub CustomCompositorDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    CustomCompositorDefinition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaOverlayLayerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaOverlayLayerFactory {
    type Vtable = IMediaOverlayLayerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaOverlayLayerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947cb473_a39e_4362_abbf_9f8b5070a062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayLayerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Effects")]
    pub CreateWithCompositorDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositordefinition: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))]
    CreateWithCompositorDefinition: usize,
}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct BackgroundAudioTrack(::windows::core::IUnknown);
impl BackgroundAudioTrack {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrimTimeFromStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromStart(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTrimTimeFromStart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrimTimeFromEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromEnd(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTrimTimeFromEnd)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrimmedDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDelay(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDelay)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVolume)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Volume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<BackgroundAudioTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioEncodingProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioEffectDefinitions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFromEmbeddedAudioTrack(embeddedaudiotrack: &EmbeddedAudioTrack) -> ::windows::core::Result<BackgroundAudioTrack> {
        Self::IBackgroundAudioTrackStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromEmbeddedAudioTrack)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(embeddedaudiotrack), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromFileAsync<P0, E0>(file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBackgroundAudioTrackStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromFileAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundAudioTrackStatics<R, F: FnOnce(&IBackgroundAudioTrackStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BackgroundAudioTrack, IBackgroundAudioTrackStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BackgroundAudioTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundAudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundAudioTrack {}
impl ::core::fmt::Debug for BackgroundAudioTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundAudioTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundAudioTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.BackgroundAudioTrack;{4b91b3bd-9e21-4266-a9c2-67dd011a2357})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BackgroundAudioTrack {
    type Vtable = IBackgroundAudioTrack_Vtbl;
}
unsafe impl ::windows::core::Interface for BackgroundAudioTrack {
    const IID: ::windows::core::GUID = <IBackgroundAudioTrack as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.BackgroundAudioTrack";
}
::windows::core::interface_hierarchy!(BackgroundAudioTrack, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundAudioTrack {}
unsafe impl ::core::marker::Sync for BackgroundAudioTrack {}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct EmbeddedAudioTrack(::windows::core::IUnknown);
impl EmbeddedAudioTrack {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAudioEncodingProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmbeddedAudioTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmbeddedAudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmbeddedAudioTrack {}
impl ::core::fmt::Debug for EmbeddedAudioTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmbeddedAudioTrack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmbeddedAudioTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.EmbeddedAudioTrack;{55ee5a7a-2d30-3fba-a190-4f1a6454f88f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmbeddedAudioTrack {
    type Vtable = IEmbeddedAudioTrack_Vtbl;
}
unsafe impl ::windows::core::Interface for EmbeddedAudioTrack {
    const IID: ::windows::core::GUID = <IEmbeddedAudioTrack as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmbeddedAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.EmbeddedAudioTrack";
}
::windows::core::interface_hierarchy!(EmbeddedAudioTrack, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmbeddedAudioTrack {}
unsafe impl ::core::marker::Sync for EmbeddedAudioTrack {}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct MediaClip(::windows::core::IUnknown);
impl MediaClip {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrimTimeFromStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromStart(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTrimTimeFromStart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrimTimeFromEnd)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromEnd(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTrimTimeFromEnd)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TrimmedDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaClip> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTimeInComposition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EndTimeInComposition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmbeddedAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmbeddedAudioTracks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SelectedEmbeddedAudioTrackIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedEmbeddedAudioTrackIndex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSelectedEmbeddedAudioTrackIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelectedEmbeddedAudioTrackIndex)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVolume)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Volume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetVideoEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetVideoEncodingProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioEffectDefinitions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn VideoEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoEffectDefinitions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn CreateFromColor(color: super::super::UI::Color, originalduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromColor)(::windows::core::Vtable::as_raw(this), color, originalduration, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromFileAsync<P0, E0>(file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromFileAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromImageFileAsync<P0, E0>(file: P0, originalduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromImageFileAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), originalduration, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateFromSurface<P0, E0>(surface: P0, originalduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaClip>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMediaClipStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromSurface)(::windows::core::Vtable::as_raw(this), surface.try_into().map_err(|e| e.into())?.abi(), originalduration, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaClipStatics<R, F: FnOnce(&IMediaClipStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaClip, IMediaClipStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaClipStatics2<R, F: FnOnce(&IMediaClipStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaClip, IMediaClipStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaClip {}
impl ::core::fmt::Debug for MediaClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaClip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaClip {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaClip;{53f25366-5fba-3ea4-8693-24761811140a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaClip {
    type Vtable = IMediaClip_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaClip {
    const IID: ::windows::core::GUID = <IMediaClip as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaClip {
    const NAME: &'static str = "Windows.Media.Editing.MediaClip";
}
::windows::core::interface_hierarchy!(MediaClip, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaClip {}
unsafe impl ::core::marker::Sync for MediaClip {}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct MediaComposition(::windows::core::IUnknown);
impl MediaComposition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaComposition, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clips(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaClip>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clips)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BackgroundAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundAudioTracks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaComposition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveAsync<P0, E0>(&self, file: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Imaging\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, timefromstart: super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailAsync)(::windows::core::Vtable::as_raw(this), timefromstart, scaledwidth, scaledheight, frameprecision, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub fn GetThumbnailsAsync<P0, E0>(&self, timesfromstart: P0, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailsAsync)(::windows::core::Vtable::as_raw(this), timesfromstart.try_into().map_err(|e| e.into())?.abi(), scaledwidth, scaledheight, frameprecision, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Transcoding\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub fn RenderToFileAsync<P0, E0>(&self, destination: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderToFileAsync)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Transcoding\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub fn RenderToFileWithTrimmingPreferenceAsync<P0, E0>(&self, destination: P0, trimmingpreference: MediaTrimmingPreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderToFileWithTrimmingPreferenceAsync)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi(), trimmingpreference, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Media_Transcoding\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))]
    pub fn RenderToFileWithProfileAsync<P0, E0>(&self, destination: P0, trimmingpreference: MediaTrimmingPreference, encodingprofile: &super::MediaProperties::MediaEncodingProfile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderToFileWithProfileAsync)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi(), trimmingpreference, ::core::mem::transmute_copy(encodingprofile), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateDefaultEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDefaultEncodingProfile)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn GenerateMediaStreamSource(&self) -> ::windows::core::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateMediaStreamSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))]
    pub fn GenerateMediaStreamSourceWithProfile(&self, encodingprofile: &super::MediaProperties::MediaEncodingProfile) -> ::windows::core::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GenerateMediaStreamSourceWithProfile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(encodingprofile), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn GeneratePreviewMediaStreamSource(&self, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeneratePreviewMediaStreamSource)(::windows::core::Vtable::as_raw(this), scaledwidth, scaledheight, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OverlayLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlayLayer>> {
        let this = &::windows::core::Interface::cast::<IMediaComposition2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OverlayLayers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadAsync(file: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaComposition>> {
        Self::IMediaCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(file), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaCompositionStatics<R, F: FnOnce(&IMediaCompositionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaComposition, IMediaCompositionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaComposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaComposition {}
impl ::core::fmt::Debug for MediaComposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaComposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaComposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaComposition;{2e06e605-dc71-41d6-b837-2d2bc14a2947})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaComposition {
    type Vtable = IMediaComposition_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaComposition {
    const IID: ::windows::core::GUID = <IMediaComposition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaComposition {
    const NAME: &'static str = "Windows.Media.Editing.MediaComposition";
}
::windows::core::interface_hierarchy!(MediaComposition, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaComposition {}
unsafe impl ::core::marker::Sync for MediaComposition {}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct MediaOverlay(::windows::core::IUnknown);
impl MediaOverlay {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDelay(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDelay)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Delay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOpacity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaOverlay> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Clip(&self) -> ::windows::core::Result<MediaClip> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clip)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AudioEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAudioEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(clip: &MediaClip) -> ::windows::core::Result<MediaOverlay> {
        Self::IMediaOverlayFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(clip), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithPositionAndOpacity(clip: &MediaClip, position: super::super::Foundation::Rect, opacity: f64) -> ::windows::core::Result<MediaOverlay> {
        Self::IMediaOverlayFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithPositionAndOpacity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(clip), position, opacity, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaOverlayFactory<R, F: FnOnce(&IMediaOverlayFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaOverlay, IMediaOverlayFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaOverlay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaOverlay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaOverlay {}
impl ::core::fmt::Debug for MediaOverlay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaOverlay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaOverlay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaOverlay;{a902ae5d-7869-4830-8ab1-94dc01c05fa4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaOverlay {
    type Vtable = IMediaOverlay_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaOverlay {
    const IID: ::windows::core::GUID = <IMediaOverlay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaOverlay {
    const NAME: &'static str = "Windows.Media.Editing.MediaOverlay";
}
::windows::core::interface_hierarchy!(MediaOverlay, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaOverlay {}
unsafe impl ::core::marker::Sync for MediaOverlay {}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
pub struct MediaOverlayLayer(::windows::core::IUnknown);
impl MediaOverlayLayer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaOverlayLayer, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaOverlayLayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Overlays(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlay>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Overlays)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn CustomCompositorDefinition(&self) -> ::windows::core::Result<super::Effects::IVideoCompositorDefinition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomCompositorDefinition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn CreateWithCompositorDefinition<P0, E0>(compositordefinition: P0) -> ::windows::core::Result<MediaOverlayLayer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Effects::IVideoCompositorDefinition>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMediaOverlayLayerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithCompositorDefinition)(::windows::core::Vtable::as_raw(this), compositordefinition.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaOverlayLayerFactory<R, F: FnOnce(&IMediaOverlayLayerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaOverlayLayer, IMediaOverlayLayerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaOverlayLayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaOverlayLayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaOverlayLayer {}
impl ::core::fmt::Debug for MediaOverlayLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaOverlayLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaOverlayLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaOverlayLayer;{a6d9ba57-eeda-46c6-bbe5-e398c84168ac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaOverlayLayer {
    type Vtable = IMediaOverlayLayer_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaOverlayLayer {
    const IID: ::windows::core::GUID = <IMediaOverlayLayer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaOverlayLayer {
    const NAME: &'static str = "Windows.Media.Editing.MediaOverlayLayer";
}
::windows::core::interface_hierarchy!(MediaOverlayLayer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaOverlayLayer {}
unsafe impl ::core::marker::Sync for MediaOverlayLayer {}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: Self = Self(0i32);
    pub const Precise: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaTrimmingPreference {}
impl ::core::clone::Clone for MediaTrimmingPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaTrimmingPreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaTrimmingPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaTrimmingPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTrimmingPreference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaTrimmingPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Editing.MediaTrimmingPreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Editing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: Self = Self(0i32);
    pub const NearestKeyFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for VideoFramePrecision {}
impl ::core::clone::Clone for VideoFramePrecision {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoFramePrecision {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VideoFramePrecision {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoFramePrecision {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoFramePrecision").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoFramePrecision {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Editing.VideoFramePrecision;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
