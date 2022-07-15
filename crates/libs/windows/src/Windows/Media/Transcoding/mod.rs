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
    pub base__: ::windows::core::IInspectableVtbl,
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
    pub AddAudioEffectWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffectWithSettings: usize,
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffectWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectrequired: bool, configuration: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffectWithSettings: usize,
    pub ClearEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareFileTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareFileTranscodeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareStreamTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareMediaStreamSourceTranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    pub base__: ::windows::core::IInspectableVtbl,
    pub CanTranscode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub FailureReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TranscodeFailureReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TranscodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaTranscoder, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrimStartTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrimStartTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrimStopTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrimStopTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrimStopTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrimStopTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn SetAlwaysReencode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlwaysReencode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysReencode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AlwaysReencode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetHardwareAccelerationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHardwareAccelerationEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HardwareAccelerationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareAccelerationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AddAudioEffect(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddAudioEffect)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffectWithSettings<'a, P0, E0>(&self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddAudioEffectWithSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), effectrequired, configuration.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn AddVideoEffect(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddVideoEffect)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffectWithSettings<'a, P0, E0>(&self, activatableclassid: &::windows::core::HSTRING, effectrequired: bool, configuration: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddVideoEffectWithSettings)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(activatableclassid), effectrequired, configuration.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn ClearEffects(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearEffects)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareFileTranscodeAsync<'a, P0, E0, P1, E1, P2>(&self, source: P0, destination: P1, profile: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFile>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::MediaProperties::MediaEncodingProfile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrepareFileTranscodeAsync)(::windows::core::Interface::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), destination.try_into().map_err(|e| e.into())?.abi(), profile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareStreamTranscodeAsync<'a, P0, E0, P1, E1, P2>(&self, source: P0, destination: P1, profile: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::MediaProperties::MediaEncodingProfile>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrepareStreamTranscodeAsync)(::windows::core::Interface::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), destination.try_into().map_err(|e| e.into())?.abi(), profile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareMediaStreamSourceTranscodeAsync<'a, P0, E0, P1, E1, P2>(&self, source: P0, destination: P1, profile: P2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Core::IMediaSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::MediaProperties::MediaEncodingProfile>>,
    {
        let this = &::windows::core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrepareMediaStreamSourceTranscodeAsync)(::windows::core::Interface::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), destination.try_into().map_err(|e| e.into())?.abi(), profile.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PrepareTranscodeResult>>(result__)
        }
    }
    pub fn SetVideoProcessingAlgorithm(&self, value: MediaVideoProcessingAlgorithm) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoProcessingAlgorithm)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn VideoProcessingAlgorithm(&self) -> ::windows::core::Result<MediaVideoProcessingAlgorithm> {
        let this = &::windows::core::Interface::cast::<IMediaTranscoder2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProcessingAlgorithm)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<MediaVideoProcessingAlgorithm>(result__)
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
impl ::core::convert::From<&MediaTranscoder> for &::windows::core::IUnknown {
    fn from(value: &MediaTranscoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&MediaTranscoder> for &::windows::core::IInspectable {
    fn from(value: &MediaTranscoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for MediaTranscoder {}
unsafe impl ::core::marker::Sync for MediaTranscoder {}
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
    pub fn CanTranscode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanTranscode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn FailureReason(&self) -> ::windows::core::Result<TranscodeFailureReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FailureReason)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TranscodeFailureReason>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TranscodeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncActionWithProgress<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TranscodeAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncActionWithProgress<f64>>(result__)
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
impl ::core::convert::From<&PrepareTranscodeResult> for &::windows::core::IUnknown {
    fn from(value: &PrepareTranscodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl ::core::convert::From<&PrepareTranscodeResult> for &::windows::core::IInspectable {
    fn from(value: &PrepareTranscodeResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrepareTranscodeResult {}
unsafe impl ::core::marker::Sync for PrepareTranscodeResult {}
#[doc = "*Required features: `\"Media_Transcoding\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
