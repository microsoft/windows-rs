#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {
    fn SetBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<u32>;
    fn SetChannelCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn ChannelCount(&self) -> ::windows::core::Result<u32>;
    fn SetSampleRate(&self, value: u32) -> ::windows::core::Result<()>;
    fn SampleRate(&self) -> ::windows::core::Result<u32>;
    fn SetBitsPerSample(&self, value: u32) -> ::windows::core::Result<()>;
    fn BitsPerSample(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingProperties2Impl: Sized {
    fn IsSpatial(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingProperties3Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesStaticsImpl: Sized {
    fn CreateAac(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateAacAdts(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateMp3(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreatePcm(&self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateWma(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesStatics2Impl: Sized {
    fn CreateAlac(&self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateFlac(&self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesWithFormatUserDataImpl: Sized {
    fn SetFormatUserData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerEncodingProperties2Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<ContainerEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IH264ProfileIdsStaticsImpl: Sized {
    fn ConstrainedBaseline(&self) -> ::windows::core::Result<i32>;
    fn Baseline(&self) -> ::windows::core::Result<i32>;
    fn Extended(&self) -> ::windows::core::Result<i32>;
    fn Main(&self) -> ::windows::core::Result<i32>;
    fn High(&self) -> ::windows::core::Result<i32>;
    fn High10(&self) -> ::windows::core::Result<i32>;
    fn High422(&self) -> ::windows::core::Result<i32>;
    fn High444(&self) -> ::windows::core::Result<i32>;
    fn StereoHigh(&self) -> ::windows::core::Result<i32>;
    fn MultiviewHigh(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {
    fn SetWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn SetHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingProperties2Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStaticsImpl: Sized {
    fn CreateJpeg(&self) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreatePng(&self) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreateJpegXR(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics2Impl: Sized {
    fn CreateUncompressed(&self, format: MediaPixelFormat) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreateBmp(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics3Impl: Sized {
    fn CreateHeif(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileImpl: Sized {
    fn SetAudio(&self, value: &::core::option::Option<AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn Audio(&self) -> ::windows::core::Result<AudioEncodingProperties>;
    fn SetVideo(&self, value: &::core::option::Option<VideoEncodingProperties>) -> ::windows::core::Result<()>;
    fn Video(&self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn SetContainer(&self, value: &::core::option::Option<ContainerEncodingProperties>) -> ::windows::core::Result<()>;
    fn Container(&self) -> ::windows::core::Result<ContainerEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfile2Impl: Sized {
    fn SetAudioTracks(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>>;
    fn SetVideoTracks(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetVideoTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfile3Impl: Sized {
    fn SetTimedMetadataTracks(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetTimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStaticsImpl: Sized {
    fn CreateM4a(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateMp3(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateWma(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateMp4(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateWmv(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>;
    fn CreateFromStreamAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStatics2Impl: Sized {
    fn CreateWav(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateAvi(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStatics3Impl: Sized {
    fn CreateAlac(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateFlac(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateHevc(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
}
pub trait IMediaEncodingPropertiesImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<MediaPropertySet>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtype(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStaticsImpl: Sized {
    fn Aac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AacAdts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ac3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmrNb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmrWb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Argb32(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Asf(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Avi(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bgra8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bmp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Eac3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Float(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H263(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H264(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H264Es(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hevc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HevcEs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Iyuv(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Jpeg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JpegXr(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mjpg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mp3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nv12(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pcm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Png(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rgb24(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rgb32(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tiff(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wave(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wma8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wma9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wmv3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wvc1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Yuy2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Yv12(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics2Impl: Sized {
    fn Vp9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn L8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn L16(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn D16(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics3Impl: Sized {
    fn Alac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Flac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics4Impl: Sized {
    fn P010(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics5Impl: Sized {
    fn Heif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics6Impl: Sized {
    fn Pgs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Srt(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ssa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VobSub(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaRatioImpl: Sized {
    fn SetNumerator(&self, value: u32) -> ::windows::core::Result<()>;
    fn Numerator(&self) -> ::windows::core::Result<u32>;
    fn SetDenominator(&self, value: u32) -> ::windows::core::Result<()>;
    fn Denominator(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMpeg2ProfileIdsStaticsImpl: Sized {
    fn Simple(&self) -> ::windows::core::Result<i32>;
    fn Main(&self) -> ::windows::core::Result<i32>;
    fn SignalNoiseRatioScalable(&self) -> ::windows::core::Result<i32>;
    fn SpatiallyScalable(&self) -> ::windows::core::Result<i32>;
    fn High(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataEncodingPropertiesImpl: Sized {
    fn SetFormatUserData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn Copy(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataEncodingPropertiesStaticsImpl: Sized {
    fn CreatePgs(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateSrt(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateSsa(&self, formatuserdata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateVobSub(&self, formatuserdata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {
    fn SetBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<u32>;
    fn SetWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn SetHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn FrameRate(&self) -> ::windows::core::Result<MediaRatio>;
    fn PixelAspectRatio(&self) -> ::windows::core::Result<MediaRatio>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties2Impl: Sized {
    fn SetFormatUserData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetProfileId(&self, value: i32) -> ::windows::core::Result<()>;
    fn ProfileId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties3Impl: Sized {
    fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<StereoscopicVideoPackingMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties4Impl: Sized {
    fn SphericalVideoFrameFormat(&self) -> ::windows::core::Result<SphericalVideoFrameFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties5Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesStaticsImpl: Sized {
    fn CreateH264(&self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn CreateMpeg2(&self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn CreateUncompressed(&self, subtype: &::windows::core::HSTRING, width: u32, height: u32) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesStatics2Impl: Sized {
    fn CreateHevc(&self) -> ::windows::core::Result<VideoEncodingProperties>;
}
