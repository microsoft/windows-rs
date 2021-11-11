#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaTranscoder(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTranscoder {
    type Vtable = IMediaTranscoder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x190c99d2_a0aa_4d34_86bc_eed1b12c2f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTranscoder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, effectrequired: bool, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, effectrequired: bool, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaTranscoder2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTranscoder2 {
    type Vtable = IMediaTranscoder2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x40531d74_35e0_4f04_8574_ca8bc4e5a082);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTranscoder2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MediaVideoProcessingAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MediaVideoProcessingAlgorithm) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrepareTranscodeResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05f25dce_994f_4a34_9d68_97ccce1730d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrepareTranscodeResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TranscodeFailureReason) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `Media_Transcoding`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaTranscoder(pub ::windows::runtime::IInspectable);
impl MediaTranscoder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaTranscoder, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`*"]
    pub fn SetTrimStartTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`*"]
    pub fn TrimStartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`*"]
    pub fn SetTrimStopTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`*"]
    pub fn TrimStopTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn SetAlwaysReencode(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn AlwaysReencode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn HardwareAccelerationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn AddAudioEffect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation_Collections`*"]
    pub fn AddAudioEffectWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectrequired: bool, configuration: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectrequired, configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn AddVideoEffect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation_Collections`*"]
    pub fn AddVideoEffectWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectrequired: bool, configuration: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectrequired, configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn ClearEffects(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`, `Media_MediaProperties`, `Storage`*"]
    pub fn PrepareFileTranscodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`, `Media_MediaProperties`, `Storage_Streams`*"]
    pub fn PrepareStreamTranscodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`, `Media_Core`, `Media_MediaProperties`, `Storage_Streams`*"]
    pub fn PrepareMediaStreamSourceTranscodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Core::IMediaSource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = &::windows::runtime::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn VideoProcessingAlgorithm(&self) -> ::windows::runtime::Result<MediaVideoProcessingAlgorithm> {
        let this = &::windows::runtime::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__: MediaVideoProcessingAlgorithm = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaVideoProcessingAlgorithm>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaTranscoder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Transcoding.MediaTranscoder;{190c99d2-a0aa-4d34-86bc-eed1b12c2f5b})");
}
unsafe impl ::windows::runtime::Interface for MediaTranscoder {
    type Vtable = IMediaTranscoder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x190c99d2_a0aa_4d34_86bc_eed1b12c2f5b);
}
impl ::windows::runtime::RuntimeName for MediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.MediaTranscoder";
}
impl ::core::convert::From<MediaTranscoder> for ::windows::runtime::IUnknown {
    fn from(value: MediaTranscoder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaTranscoder> for ::windows::runtime::IUnknown {
    fn from(value: &MediaTranscoder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaTranscoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a MediaTranscoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaTranscoder> for ::windows::runtime::IInspectable {
    fn from(value: MediaTranscoder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaTranscoder> for ::windows::runtime::IInspectable {
    fn from(value: &MediaTranscoder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaTranscoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaTranscoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaTranscoder {}
unsafe impl ::core::marker::Sync for MediaTranscoder {}
#[doc = "*Required features: `Media_Transcoding`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: MediaVideoProcessingAlgorithm = MediaVideoProcessingAlgorithm(0i32);
    pub const MrfCrf444: MediaVideoProcessingAlgorithm = MediaVideoProcessingAlgorithm(1i32);
}
impl ::core::convert::From<i32> for MediaVideoProcessingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaVideoProcessingAlgorithm {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaVideoProcessingAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.MediaVideoProcessingAlgorithm;i4)");
}
impl ::windows::runtime::DefaultType for MediaVideoProcessingAlgorithm {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Transcoding`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrepareTranscodeResult(pub ::windows::runtime::IInspectable);
impl PrepareTranscodeResult {
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn CanTranscode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Transcoding`*"]
    pub fn FailureReason(&self) -> ::windows::runtime::Result<TranscodeFailureReason> {
        let this = self;
        unsafe {
            let mut result__: TranscodeFailureReason = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TranscodeFailureReason>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Transcoding`, `Foundation`*"]
    pub fn TranscodeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrepareTranscodeResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Transcoding.PrepareTranscodeResult;{05f25dce-994f-4a34-9d68-97ccce1730d6})");
}
unsafe impl ::windows::runtime::Interface for PrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05f25dce_994f_4a34_9d68_97ccce1730d6);
}
impl ::windows::runtime::RuntimeName for PrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.PrepareTranscodeResult";
}
impl ::core::convert::From<PrepareTranscodeResult> for ::windows::runtime::IUnknown {
    fn from(value: PrepareTranscodeResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrepareTranscodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &PrepareTranscodeResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrepareTranscodeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrepareTranscodeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrepareTranscodeResult> for ::windows::runtime::IInspectable {
    fn from(value: PrepareTranscodeResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrepareTranscodeResult> for ::windows::runtime::IInspectable {
    fn from(value: &PrepareTranscodeResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrepareTranscodeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrepareTranscodeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrepareTranscodeResult {}
unsafe impl ::core::marker::Sync for PrepareTranscodeResult {}
#[doc = "*Required features: `Media_Transcoding`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: TranscodeFailureReason = TranscodeFailureReason(0i32);
    pub const Unknown: TranscodeFailureReason = TranscodeFailureReason(1i32);
    pub const InvalidProfile: TranscodeFailureReason = TranscodeFailureReason(2i32);
    pub const CodecNotFound: TranscodeFailureReason = TranscodeFailureReason(3i32);
}
impl ::core::convert::From<i32> for TranscodeFailureReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TranscodeFailureReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TranscodeFailureReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.TranscodeFailureReason;i4)");
}
impl ::windows::runtime::DefaultType for TranscodeFailureReason {
    type DefaultType = Self;
}
