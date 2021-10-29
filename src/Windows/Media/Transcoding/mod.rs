#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaTranscoder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTranscoder {
    type Vtable = IMediaTranscoder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(420256210, 41130, 19764, [134, 188, 238, 209, 177, 44, 47, 91]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, effectrequired: bool, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, effectrequired: bool, configuration: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, profile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaTranscoder2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaTranscoder2 {
    type Vtable = IMediaTranscoder2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1079188852, 13792, 20228, [133, 116, 202, 139, 196, 229, 160, 130]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPrepareTranscodeResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(99769806, 39247, 18996, [157, 104, 151, 204, 206, 23, 48, 214]);
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
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaTranscoder(::windows::runtime::IInspectable);
impl MediaTranscoder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaTranscoder, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStartTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrimStartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStopTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrimStopTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn SetAlwaysReencode(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysReencode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn HardwareAccelerationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AddAudioEffect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffectWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectrequired: bool, configuration: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectrequired, configuration.into_param().abi()).ok() }
    }
    pub fn AddVideoEffect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffectWithSettings<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, effectrequired: bool, configuration: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), effectrequired, configuration.into_param().abi()).ok() }
    }
    pub fn ClearEffects(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareFileTranscodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareStreamTranscodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareMediaStreamSourceTranscodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Core::IMediaSource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, source: Param0, destination: Param1, profile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>> {
        let this = &::windows::runtime::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), source.into_param().abi(), destination.into_param().abi(), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    pub fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn VideoProcessingAlgorithm(&self) -> ::windows::runtime::Result<MediaVideoProcessingAlgorithm> {
        let this = &::windows::runtime::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__: MediaVideoProcessingAlgorithm = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaVideoProcessingAlgorithm>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaTranscoder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Transcoding.MediaTranscoder;{190c99d2-a0aa-4d34-86bc-eed1b12c2f5b})");
}
unsafe impl ::windows::runtime::Interface for MediaTranscoder {
    type Vtable = IMediaTranscoder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(420256210, 41130, 19764, [134, 188, 238, 209, 177, 44, 47, 91]);
}
impl ::windows::runtime::RuntimeName for MediaTranscoder {
    const NAME: &'static str = "Windows.Media.Transcoding.MediaTranscoder";
}
impl ::std::convert::From<MediaTranscoder> for ::windows::runtime::IUnknown {
    fn from(value: MediaTranscoder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaTranscoder> for ::windows::runtime::IUnknown {
    fn from(value: &MediaTranscoder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaTranscoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaTranscoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaTranscoder> for ::windows::runtime::IInspectable {
    fn from(value: MediaTranscoder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaTranscoder> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for MediaTranscoder {}
unsafe impl ::std::marker::Sync for MediaTranscoder {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaVideoProcessingAlgorithm(pub i32);
impl MediaVideoProcessingAlgorithm {
    pub const Default: MediaVideoProcessingAlgorithm = MediaVideoProcessingAlgorithm(0i32);
    pub const MrfCrf444: MediaVideoProcessingAlgorithm = MediaVideoProcessingAlgorithm(1i32);
}
impl ::std::convert::From<i32> for MediaVideoProcessingAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaVideoProcessingAlgorithm {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaVideoProcessingAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.MediaVideoProcessingAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PrepareTranscodeResult(::windows::runtime::IInspectable);
impl PrepareTranscodeResult {
    pub fn CanTranscode(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn FailureReason(&self) -> ::windows::runtime::Result<TranscodeFailureReason> {
        let this = self;
        unsafe {
            let mut result__: TranscodeFailureReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TranscodeFailureReason>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TranscodeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncActionWithProgress<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncActionWithProgress<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrepareTranscodeResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Transcoding.PrepareTranscodeResult;{05f25dce-994f-4a34-9d68-97ccce1730d6})");
}
unsafe impl ::windows::runtime::Interface for PrepareTranscodeResult {
    type Vtable = IPrepareTranscodeResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(99769806, 39247, 18996, [157, 104, 151, 204, 206, 23, 48, 214]);
}
impl ::windows::runtime::RuntimeName for PrepareTranscodeResult {
    const NAME: &'static str = "Windows.Media.Transcoding.PrepareTranscodeResult";
}
impl ::std::convert::From<PrepareTranscodeResult> for ::windows::runtime::IUnknown {
    fn from(value: PrepareTranscodeResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PrepareTranscodeResult> for ::windows::runtime::IUnknown {
    fn from(value: &PrepareTranscodeResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrepareTranscodeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PrepareTranscodeResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PrepareTranscodeResult> for ::windows::runtime::IInspectable {
    fn from(value: PrepareTranscodeResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PrepareTranscodeResult> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for PrepareTranscodeResult {}
unsafe impl ::std::marker::Sync for PrepareTranscodeResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TranscodeFailureReason(pub i32);
impl TranscodeFailureReason {
    pub const None: TranscodeFailureReason = TranscodeFailureReason(0i32);
    pub const Unknown: TranscodeFailureReason = TranscodeFailureReason(1i32);
    pub const InvalidProfile: TranscodeFailureReason = TranscodeFailureReason(2i32);
    pub const CodecNotFound: TranscodeFailureReason = TranscodeFailureReason(3i32);
}
impl ::std::convert::From<i32> for TranscodeFailureReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TranscodeFailureReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TranscodeFailureReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Transcoding.TranscodeFailureReason;i4)");
}
