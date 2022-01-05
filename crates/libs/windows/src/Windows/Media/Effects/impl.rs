#[cfg(feature = "implement_exclusive")]
pub trait IAudioCaptureEffectsManagerImpl: Sized {
    fn AudioCaptureEffectsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioCaptureEffectsManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioCaptureEffectsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAudioCaptureEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEffectImpl: Sized {
    fn AudioEffectType(&self) -> ::windows::core::Result<AudioEffectType>;
}
pub trait IAudioEffectDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<AudioEffectDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<AudioEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEffectsManagerStaticsImpl: Sized {
    fn CreateAudioRenderEffectsManager(&self, deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory) -> ::windows::core::Result<AudioRenderEffectsManager>;
    fn CreateAudioRenderEffectsManagerWithMode(&self, deviceid: &::windows::core::HSTRING, category: super::Render::AudioRenderCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioRenderEffectsManager>;
    fn CreateAudioCaptureEffectsManager(&self, deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory) -> ::windows::core::Result<AudioCaptureEffectsManager>;
    fn CreateAudioCaptureEffectsManagerWithMode(&self, deviceid: &::windows::core::HSTRING, category: super::Capture::MediaCategory, mode: super::AudioProcessing) -> ::windows::core::Result<AudioCaptureEffectsManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioRenderEffectsManagerImpl: Sized {
    fn AudioRenderEffectsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioRenderEffectsManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAudioRenderEffectsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAudioRenderEffects(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AudioEffect>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAudioRenderEffectsManager2Impl: Sized {
    fn EffectsProviderThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn EffectsProviderSettingsLabel(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowSettingsUI(&self) -> ::windows::core::Result<()>;
}
pub trait IBasicAudioEffectImpl: Sized + IMediaExtensionImpl {
    fn UseInputFrameForOutput(&self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::AudioEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&self, context: &::core::option::Option<ProcessAudioFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
pub trait IBasicVideoEffectImpl: Sized + IMediaExtensionImpl {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn SupportedMemoryTypes(&self) -> ::windows::core::Result<MediaMemoryTypes>;
    fn TimeIndependent(&self) -> ::windows::core::Result<bool>;
    fn SupportedEncodingProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::VideoEncodingProperties>>;
    fn SetEncodingProperties(&self, encodingproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn ProcessFrame(&self, context: &::core::option::Option<ProcessVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeVideoFrameContextImpl: Sized {
    fn SurfacesToOverlay(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>;
    fn BackgroundFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn GetOverlayForSurface(&self, surfacetooverlay: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<super::Editing::MediaOverlay>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessAudioFrameContextImpl: Sized {
    fn InputFrame(&self) -> ::windows::core::Result<super::AudioFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessVideoFrameContextImpl: Sized {
    fn InputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
    fn OutputFrame(&self) -> ::windows::core::Result<super::VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlowMotionEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn TimeStretchRate(&self) -> ::windows::core::Result<f64>;
    fn SetTimeStretchRate(&self, value: f64) -> ::windows::core::Result<()>;
}
pub trait IVideoCompositorImpl: Sized + IMediaExtensionImpl {
    fn TimeIndependent(&self) -> ::windows::core::Result<bool>;
    fn SetEncodingProperties(&self, backgroundproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>, device: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CompositeFrame(&self, context: &::core::option::Option<CompositeVideoFrameContext>) -> ::windows::core::Result<()>;
    fn Close(&self, reason: MediaEffectClosedReason) -> ::windows::core::Result<()>;
    fn DiscardQueuedFrames(&self) -> ::windows::core::Result<()>;
}
pub trait IVideoCompositorDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoCompositorDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoCompositorDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<VideoCompositorDefinition>;
}
pub trait IVideoEffectDefinitionImpl: Sized {
    fn ActivatableClassId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEffectDefinitionFactoryImpl: Sized {
    fn Create(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<VideoEffectDefinition>;
    fn CreateWithProperties(&self, activatableclassid: &::windows::core::HSTRING, props: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<VideoEffectDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTransformEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn PaddingColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetPaddingColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn OutputSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetOutputSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CropRectangle(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetCropRectangle(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation>;
    fn SetRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()>;
    fn Mirror(&self) -> ::windows::core::Result<super::MediaProperties::MediaMirroringOptions>;
    fn SetMirror(&self, value: super::MediaProperties::MediaMirroringOptions) -> ::windows::core::Result<()>;
    fn SetProcessingAlgorithm(&self, value: super::Transcoding::MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()>;
    fn ProcessingAlgorithm(&self) -> ::windows::core::Result<super::Transcoding::MediaVideoProcessingAlgorithm>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTransformEffectDefinition2Impl: Sized {
    fn SphericalProjection(&self) -> ::windows::core::Result<VideoTransformSphericalProjection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTransformSphericalProjectionImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat>;
    fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()>;
    fn ProjectionMode(&self) -> ::windows::core::Result<super::Playback::SphericalVideoProjectionMode>;
    fn SetProjectionMode(&self, value: super::Playback::SphericalVideoProjectionMode) -> ::windows::core::Result<()>;
    fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
    fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion>;
    fn SetViewOrientation(&self, value: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
}
