#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledVoicesStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInstalledVoicesStatic {
    type Vtable = IInstalledVoicesStatic_Vtbl;
}
impl ::core::clone::Clone for IInstalledVoicesStatic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInstalledVoicesStatic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d526ecc_7533_4c3f_85be_888c2baeebdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllVoices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllVoices: usize,
    pub DefaultVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledVoicesStatic2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInstalledVoicesStatic2 {
    type Vtable = IInstalledVoicesStatic2_Vtbl;
}
impl ::core::clone::Clone for IInstalledVoicesStatic2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInstalledVoicesStatic2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64255f2e_358d_4058_be9a_fd3fcb423530);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledVoicesStatic2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TrySetDefaultVoiceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetDefaultVoiceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesisStream(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechSynthesisStream {
    type Vtable = ISpeechSynthesisStream_Vtbl;
}
impl ::core::clone::Clone for ISpeechSynthesisStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechSynthesisStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83e46e93_244c_4622_ba0b_6229c4d0d65d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesisStream_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechSynthesizer {
    type Vtable = ISpeechSynthesizer_Vtbl;
}
impl ::core::clone::Clone for ISpeechSynthesizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechSynthesizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce9f7c76_97f4_4ced_ad68_d51c458e45c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SynthesizeTextToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SynthesizeTextToStreamAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SynthesizeSsmlToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ssml: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SynthesizeSsmlToStreamAsync: usize,
    pub SetVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Voice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechSynthesizer2 {
    type Vtable = ISpeechSynthesizer2_Vtbl;
}
impl ::core::clone::Clone for ISpeechSynthesizer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechSynthesizer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7c5ecb2_4339_4d6a_bbf8_c7a4f1544c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechSynthesizerOptions {
    type Vtable = ISpeechSynthesizerOptions_Vtbl;
}
impl ::core::clone::Clone for ISpeechSynthesizerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechSynthesizerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e23871_cc3d_43c9_91b1_ee185324d83d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeWordBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeSentenceBoundaryMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechSynthesizerOptions2 {
    type Vtable = ISpeechSynthesizerOptions2_Vtbl;
}
impl ::core::clone::Clone for ISpeechSynthesizerOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechSynthesizerOptions2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cbef60e_119c_4bed_b118_d250c3a25793);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AudioVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetAudioVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SpeakingRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetSpeakingRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub AudioPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetAudioPitch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechSynthesizerOptions3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpeechSynthesizerOptions3 {
    type Vtable = ISpeechSynthesizerOptions3_Vtbl;
}
impl ::core::clone::Clone for ISpeechSynthesizerOptions3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechSynthesizerOptions3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401ed877_902c_4814_a582_a5d0c0769fa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechSynthesizerOptions3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppendedSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechAppendedSilence) -> ::windows::core::HRESULT,
    pub SetAppendedSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechAppendedSilence) -> ::windows::core::HRESULT,
    pub PunctuationSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SpeechPunctuationSilence) -> ::windows::core::HRESULT,
    pub SetPunctuationSilence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SpeechPunctuationSilence) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVoiceInformation {
    type Vtable = IVoiceInformation_Vtbl;
}
impl ::core::clone::Clone for IVoiceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVoiceInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb127d6a4_1291_4604_aa9c_83134083352c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoiceGender) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct SpeechSynthesisStream(::windows::core::IUnknown);
impl SpeechSynthesisStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>();
            (::windows::core::Interface::vtable(this).ReadAsync)(::windows::core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>();
            (::windows::core::Interface::vtable(this).WriteAsync)(::windows::core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).FlushAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSize)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IInputStream>();
            (::windows::core::Interface::vtable(this).GetInputStreamAt)(::windows::core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IOutputStream>();
            (::windows::core::Interface::vtable(this).GetOutputStreamAt)(::windows::core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).Position)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::windows::core::Interface::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStream>();
            (::windows::core::Interface::vtable(this).CloneStream)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanRead)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanWrite)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Markers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::IMediaMarker>>();
            (::windows::core::Interface::vtable(this).Markers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn TimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
        let this = &::windows::core::ComInterface::cast::<super::Core::ITimedMetadataTrackProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>>();
            (::windows::core::Interface::vtable(this).TimedMetadataTracks)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesisStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesisStream {}
impl ::core::fmt::Debug for SpeechSynthesisStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesisStream").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechSynthesisStream {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesisStream;{83e46e93-244c-4622-ba0b-6229c4d0d65d})");
}
impl ::core::clone::Clone for SpeechSynthesisStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechSynthesisStream {
    type Vtable = ISpeechSynthesisStream_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechSynthesisStream {
    const IID: ::windows::core::GUID = <ISpeechSynthesisStream as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechSynthesisStream {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesisStream";
}
::windows::imp::interface_hierarchy!(SpeechSynthesisStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for SpeechSynthesisStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::super::Storage::Streams::IContentTypeProvider> for SpeechSynthesisStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::super::Storage::Streams::IInputStream> for SpeechSynthesisStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::super::Storage::Streams::IOutputStream> for SpeechSynthesisStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::super::Storage::Streams::IRandomAccessStream> for SpeechSynthesisStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::CanTryInto<super::super::Storage::Streams::IRandomAccessStreamWithContentType> for SpeechSynthesisStream {}
#[cfg(feature = "Media_Core")]
impl ::windows::core::CanTryInto<super::Core::ITimedMetadataTrackProvider> for SpeechSynthesisStream {}
unsafe impl ::core::marker::Send for SpeechSynthesisStream {}
unsafe impl ::core::marker::Sync for SpeechSynthesisStream {}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct SpeechSynthesizer(::windows::core::IUnknown);
impl SpeechSynthesizer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechSynthesizer, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllVoices() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<VoiceInformation>> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<VoiceInformation>>();
            (::windows::core::Interface::vtable(this).AllVoices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DefaultVoice() -> ::windows::core::Result<VoiceInformation> {
        Self::IInstalledVoicesStatic(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<VoiceInformation>();
            (::windows::core::Interface::vtable(this).DefaultVoice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetDefaultVoiceAsync(voice: &VoiceInformation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IInstalledVoicesStatic2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TrySetDefaultVoiceAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(voice), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SynthesizeTextToStreamAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>();
            (::windows::core::Interface::vtable(this).SynthesizeTextToStreamAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SynthesizeSsmlToStreamAsync(&self, ssml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<SpeechSynthesisStream>>();
            (::windows::core::Interface::vtable(this).SynthesizeSsmlToStreamAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(ssml), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVoice(&self, value: &VoiceInformation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVoice)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Voice(&self) -> ::windows::core::Result<VoiceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<VoiceInformation>();
            (::windows::core::Interface::vtable(this).Voice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<SpeechSynthesizerOptions> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizer2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechSynthesizerOptions>();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IInstalledVoicesStatic<R, F: FnOnce(&IInstalledVoicesStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInstalledVoicesStatic2<R, F: FnOnce(&IInstalledVoicesStatic2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SpeechSynthesizer, IInstalledVoicesStatic2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesizer {}
impl ::core::fmt::Debug for SpeechSynthesizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesizer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechSynthesizer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesizer;{ce9f7c76-97f4-4ced-ad68-d51c458e45c6})");
}
impl ::core::clone::Clone for SpeechSynthesizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechSynthesizer {
    type Vtable = ISpeechSynthesizer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechSynthesizer {
    const IID: ::windows::core::GUID = <ISpeechSynthesizer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechSynthesizer {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizer";
}
::windows::imp::interface_hierarchy!(SpeechSynthesizer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for SpeechSynthesizer {}
unsafe impl ::core::marker::Send for SpeechSynthesizer {}
unsafe impl ::core::marker::Sync for SpeechSynthesizer {}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct SpeechSynthesizerOptions(::windows::core::IUnknown);
impl SpeechSynthesizerOptions {
    pub fn IncludeWordBoundaryMetadata(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IncludeWordBoundaryMetadata)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncludeWordBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeWordBoundaryMetadata)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeSentenceBoundaryMetadata(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IncludeSentenceBoundaryMetadata)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncludeSentenceBoundaryMetadata(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeSentenceBoundaryMetadata)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioVolume(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AudioVolume)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioVolume)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpeakingRate(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).SpeakingRate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpeakingRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSpeakingRate)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AudioPitch(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).AudioPitch)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAudioPitch(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioPitch)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendedSilence(&self) -> ::windows::core::Result<SpeechAppendedSilence> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechAppendedSilence>();
            (::windows::core::Interface::vtable(this).AppendedSilence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppendedSilence(&self, value: SpeechAppendedSilence) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAppendedSilence)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PunctuationSilence(&self) -> ::windows::core::Result<SpeechPunctuationSilence> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SpeechPunctuationSilence>();
            (::windows::core::Interface::vtable(this).PunctuationSilence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPunctuationSilence(&self, value: SpeechPunctuationSilence) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ISpeechSynthesizerOptions3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPunctuationSilence)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SpeechSynthesizerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechSynthesizerOptions {}
impl ::core::fmt::Debug for SpeechSynthesizerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSynthesizerOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechSynthesizerOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions;{a0e23871-cc3d-43c9-91b1-ee185324d83d})");
}
impl ::core::clone::Clone for SpeechSynthesizerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SpeechSynthesizerOptions {
    type Vtable = ISpeechSynthesizerOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SpeechSynthesizerOptions {
    const IID: ::windows::core::GUID = <ISpeechSynthesizerOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SpeechSynthesizerOptions {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.SpeechSynthesizerOptions";
}
::windows::imp::interface_hierarchy!(SpeechSynthesizerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpeechSynthesizerOptions {}
unsafe impl ::core::marker::Sync for SpeechSynthesizerOptions {}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
pub struct VoiceInformation(::windows::core::IUnknown);
impl VoiceInformation {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Gender(&self) -> ::windows::core::Result<VoiceGender> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<VoiceGender>();
            (::windows::core::Interface::vtable(this).Gender)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceInformation {}
impl ::core::fmt::Debug for VoiceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceInformation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VoiceInformation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.SpeechSynthesis.VoiceInformation;{b127d6a4-1291-4604-aa9c-83134083352c})");
}
impl ::core::clone::Clone for VoiceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VoiceInformation {
    type Vtable = IVoiceInformation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VoiceInformation {
    const IID: ::windows::core::GUID = <IVoiceInformation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VoiceInformation {
    const NAME: &'static str = "Windows.Media.SpeechSynthesis.VoiceInformation";
}
::windows::imp::interface_hierarchy!(VoiceInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VoiceInformation {}
unsafe impl ::core::marker::Sync for VoiceInformation {}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechAppendedSilence(pub i32);
impl SpeechAppendedSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechAppendedSilence {}
impl ::core::clone::Clone for SpeechAppendedSilence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechAppendedSilence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechAppendedSilence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechAppendedSilence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechAppendedSilence").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechAppendedSilence {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechAppendedSilence;i4)");
}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechPunctuationSilence(pub i32);
impl SpeechPunctuationSilence {
    pub const Default: Self = Self(0i32);
    pub const Min: Self = Self(1i32);
}
impl ::core::marker::Copy for SpeechPunctuationSilence {}
impl ::core::clone::Clone for SpeechPunctuationSilence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechPunctuationSilence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechPunctuationSilence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechPunctuationSilence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechPunctuationSilence").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SpeechPunctuationSilence {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.SpeechPunctuationSilence;i4)");
}
#[doc = "*Required features: `\"Media_SpeechSynthesis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoiceGender(pub i32);
impl VoiceGender {
    pub const Male: Self = Self(0i32);
    pub const Female: Self = Self(1i32);
}
impl ::core::marker::Copy for VoiceGender {}
impl ::core::clone::Clone for VoiceGender {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoiceGender {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VoiceGender {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VoiceGender {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceGender").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VoiceGender {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.SpeechSynthesis.VoiceGender;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
