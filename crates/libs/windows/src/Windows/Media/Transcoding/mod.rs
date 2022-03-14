#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTranscoder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaTranscoder {
    type Vtable = IMediaTranscoder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x190c99d2_a0aa_4d34_86bc_eed1b12c2f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTranscoder_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetTrimStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub TrimStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrimStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrimStopTime: usize,
    #[cfg(feature = "Foundation")]
    pub TrimStopTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrimStopTime: usize,
    pub SetAlwaysReencode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AlwaysReencode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHardwareAccelerationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub HardwareAccelerationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffectWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffectWithSettings: usize,
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffectWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffectWithSettings: usize,
    pub ClearEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareFileTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareFileTranscodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareStreamTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareStreamTranscodeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTranscoder2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaTranscoder2 {
    type Vtable = IMediaTranscoder2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40531d74_35e0_4f04_8574_ca8bc4e5a082);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTranscoder2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareMediaStreamSourceTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareMediaStreamSourceTranscodeAsync: usize,
    pub SetVideoProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT,
    pub VideoProcessingAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaVideoProcessingAlgorithm) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrepareTranscodeResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05f25dce_994f_4a34_9d68_97ccce1730d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareTranscodeResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanTranscode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub FailureReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TranscodeFailureReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TranscodeAsync: usize,
}
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
pub struct MediaTranscoder(::windows::core::IUnknown);
impl MediaTranscoder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaTranscoder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrimStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrimStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStopTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrimStopTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimStopTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrimStopTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn SetAlwaysReencode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlwaysReencode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn AlwaysReencode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlwaysReencode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHardwareAccelerationEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn HardwareAccelerationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HardwareAccelerationEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn AddAudioEffect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddAudioEffect)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffectWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectrequired: bool, configuration: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddAudioEffectWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectrequired, configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn AddVideoEffect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddVideoEffect)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffectWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectrequired: bool, configuration: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddVideoEffectWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectrequired, configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn ClearEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearEffects)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareFileTranscodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareFileTranscodeAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareStreamTranscodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareStreamTranscodeAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`, `\"Media_Core\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareMediaStreamSourceTranscodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Core::IMediaSource>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = &::windows::core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareMediaStreamSourceTranscodeAsync)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoProcessingAlgorithm)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn VideoProcessingAlgorithm(&self) -> ::windows::core::Result<MediaVideoProcessingAlgorithm> {
        let this = &::windows::core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__: MediaVideoProcessingAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoProcessingAlgorithm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaVideoProcessingAlgorithm>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaTranscoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaTranscoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaTranscoder {}
impl ::core::fmt::Debug for MediaTranscoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTranscoder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaTranscoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Transcoding.MediaTranscoder;{190c99d2-a0aa-4d34-86bc-eed1b12c2f5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaTranscoder {
    type Vtable = IMediaTranscoder_Vtbl;
    const IID: ::windows::core::GUID = <IMediaTranscoder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.MediaTranscoder";
}
impl ::core::convert::From<MediaTranscoder> for ::windows::core::IUnknown {
    fn from(value: MediaTranscoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTranscoder> for ::windows::core::IUnknown {
    fn from(value: &MediaTranscoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaTranscoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaTranscoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaTranscoder> for ::windows::core::IInspectable {
    fn from(value: MediaTranscoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTranscoder> for ::windows::core::IInspectable {
    fn from(value: &MediaTranscoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaTranscoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaTranscoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaTranscoder {}
unsafe impl ::core::marker::Sync for MediaTranscoder {}
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: Self = Self(0i32);
    pub const MrfCrf444: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaVideoProcessingAlgorithm {}
impl ::core::clone::Clone for MediaVideoProcessingAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaVideoProcessingAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaVideoProcessingAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaVideoProcessingAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaVideoProcessingAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaVideoProcessingAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.MediaVideoProcessingAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
pub struct PrepareTranscodeResult(::windows::core::IUnknown);
impl PrepareTranscodeResult {
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn CanTranscode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanTranscode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`*"]
    pub fn FailureReason(&self) -> ::windows::core::Result<TranscodeFailureReason> {
        let this = self;
        unsafe {
            let mut result__: TranscodeFailureReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FailureReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TranscodeFailureReason>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Transcoding\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TranscodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TranscodeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrepareTranscodeResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrepareTranscodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrepareTranscodeResult {}
impl ::core::fmt::Debug for PrepareTranscodeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrepareTranscodeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrepareTranscodeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Transcoding.PrepareTranscodeResult;{05f25dce-994f-4a34-9d68-97ccce1730d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_Vtbl;
    const IID: ::windows::core::GUID = <IPrepareTranscodeResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.PrepareTranscodeResult";
}
impl ::core::convert::From<PrepareTranscodeResult> for ::windows::core::IUnknown {
    fn from(value: PrepareTranscodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrepareTranscodeResult> for ::windows::core::IUnknown {
    fn from(value: &PrepareTranscodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrepareTranscodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrepareTranscodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrepareTranscodeResult> for ::windows::core::IInspectable {
    fn from(value: PrepareTranscodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrepareTranscodeResult> for ::windows::core::IInspectable {
    fn from(value: &PrepareTranscodeResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrepareTranscodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrepareTranscodeResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrepareTranscodeResult {}
unsafe impl ::core::marker::Sync for PrepareTranscodeResult {}
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidProfile: Self = Self(2i32);
    pub const CodecNotFound: Self = Self(3i32);
}
impl ::core::marker::Copy for TranscodeFailureReason {}
impl ::core::clone::Clone for TranscodeFailureReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TranscodeFailureReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TranscodeFailureReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for TranscodeFailureReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TranscodeFailureReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TranscodeFailureReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.TranscodeFailureReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
