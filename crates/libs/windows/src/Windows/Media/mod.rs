#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AudioBuffer(::windows::core::IUnknown);
impl AudioBuffer {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capacity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLength)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateReference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IMemoryBufferReference>(result__)
        }
    }
}
impl ::core::clone::Clone for AudioBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AudioBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AudioBuffer;{35175827-724b-4c6a-b130-f6537f9ae0d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
    const IID: ::windows::core::GUID = <IAudioBuffer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioBuffer {
    const NAME: &'static str = "Windows.Media.AudioBuffer";
}
impl ::core::convert::From<AudioBuffer> for ::windows::core::IUnknown {
    fn from(value: AudioBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioBuffer> for ::windows::core::IUnknown {
    fn from(value: &AudioBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioBuffer> for ::windows::core::IInspectable {
    fn from(value: AudioBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioBuffer> for ::windows::core::IInspectable {
    fn from(value: &AudioBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioBuffer> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioBuffer> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioBuffer> for super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioBuffer> for super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IMemoryBuffer> for AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IMemoryBuffer> for &AudioBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioBuffer {}
unsafe impl ::core::marker::Sync for AudioBuffer {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AudioBufferAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioBufferAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioBufferAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioBufferAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.AudioBufferAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AudioFrame(::windows::core::IUnknown);
impl AudioFrame {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn LockBuffer(&self, mode: AudioBufferAccessMode) -> ::windows::core::Result<AudioBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LockBuffer)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<AudioBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Create(capacity: u32) -> ::windows::core::Result<AudioFrame> {
        Self::IAudioFrameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), capacity, &mut result__).from_abi::<AudioFrame>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRelativeTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSystemRelativeTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemRelativeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDiscontinuous)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsDiscontinuous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDiscontinuous)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAudioFrameFactory<R, F: FnOnce(&IAudioFrameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AudioFrame, IAudioFrameFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AudioFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AudioFrame;{e36ac304-aab2-4277-9ed0-43cedf8e29c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AudioFrame {
    type Vtable = IAudioFrame_Vtbl;
    const IID: ::windows::core::GUID = <IAudioFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AudioFrame {
    const NAME: &'static str = "Windows.Media.AudioFrame";
}
impl ::core::convert::From<AudioFrame> for ::windows::core::IUnknown {
    fn from(value: AudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrame> for ::windows::core::IUnknown {
    fn from(value: &AudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AudioFrame> for ::windows::core::IInspectable {
    fn from(value: AudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AudioFrame> for ::windows::core::IInspectable {
    fn from(value: &AudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AudioFrame> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AudioFrame> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AudioFrame> for IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: AudioFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AudioFrame> for IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &AudioFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaFrame> for AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaFrame> for &AudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaFrame> {
        ::core::convert::TryInto::<IMediaFrame>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AudioFrame {}
unsafe impl ::core::marker::Sync for AudioFrame {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AudioProcessing {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioProcessing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AudioProcessing {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.AudioProcessing;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct AutoRepeatModeChangeRequestedEventArgs(::windows::core::IUnknown);
impl AutoRepeatModeChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RequestedAutoRepeatMode(&self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackAutoRepeatMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedAutoRepeatMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackAutoRepeatMode>(result__)
        }
    }
}
impl ::core::clone::Clone for AutoRepeatModeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AutoRepeatModeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AutoRepeatModeChangeRequestedEventArgs;{ea137efa-d852-438e-882b-c990109a78f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAutoRepeatModeChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AutoRepeatModeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
}
impl ::core::convert::From<AutoRepeatModeChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AutoRepeatModeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutoRepeatModeChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AutoRepeatModeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutoRepeatModeChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AutoRepeatModeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutoRepeatModeChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AutoRepeatModeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AutoRepeatModeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutoRepeatModeChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AutoRepeatModeChangeRequestedEventArgs {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioBuffer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioBuffer {
    type Vtable = IAudioBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35175827_724b_4c6a_b130_f6537f9ae0d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBuffer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrame {
    type Vtable = IAudioFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe36ac304_aab2_4277_9ed0_43cedf8e29c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LockBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: AudioBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioFrameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAudioFrameFactory {
    type Vtable = IAudioFrameFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91a90ade_2422_40a6_b9ad_30d02404317d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutoRepeatModeChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutoRepeatModeChangeRequestedEventArgs {
    type Vtable = IAutoRepeatModeChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea137efa_d852_438e_882b_c990109a78f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoRepeatModeChangeRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestedAutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageDisplayProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd0bc7ef_54e7_411f_9933_f0e98b0a96d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDisplayProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IMediaControl(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IMediaControl {
    type Vtable = IMediaControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98f1fbe1_7a8d_42cb_b6fe_8fe698264f13);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SoundLevelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSoundLevelChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlayPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlayPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PausePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PausePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePausePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePausePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StopPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StopPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlayPauseTogglePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlayPauseTogglePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlayPauseTogglePressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlayPauseTogglePressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RecordPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RecordPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRecordPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRecordPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub NextTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    NextTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveNextTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveNextTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PreviousTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PreviousTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePreviousTrackPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePreviousTrackPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FastForwardPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FastForwardPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveFastForwardPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveFastForwardPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RewindPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RewindPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRewindPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRewindPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ChannelUpPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ChannelUpPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveChannelUpPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveChannelUpPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ChannelDownPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ChannelDownPressed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveChannelDownPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveChannelDownPressed: usize,
    #[cfg(feature = "deprecated")]
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoundLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SetTrackName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTrackName: usize,
    #[cfg(feature = "deprecated")]
    pub TrackName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrackName: usize,
    #[cfg(feature = "deprecated")]
    pub SetArtistName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub ArtistName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ArtistName: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub IsPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsPlaying: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetAlbumArt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetAlbumArt: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AlbumArt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AlbumArt: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaExtension(::windows::core::IUnknown);
impl IMediaExtension {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, configuration: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProperties)(::core::mem::transmute_copy(this), configuration.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IMediaExtension> for ::windows::core::IUnknown {
    fn from(value: IMediaExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaExtension> for ::windows::core::IUnknown {
    fn from(value: &IMediaExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaExtension> for ::windows::core::IInspectable {
    fn from(value: IMediaExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaExtension> for ::windows::core::IInspectable {
    fn from(value: &IMediaExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaExtension {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IMediaExtension {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{07915118-45df-442b-8a3f-f7826a6370ab}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IMediaExtension {
    type Vtable = IMediaExtension_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07915118_45df_442b_8a3f_f7826a6370ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtension_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaExtensionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a25eaf5_242d_4dfb_97f4_69b7c42576ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RegisterSchemeHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterSchemeHandlerWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterSchemeHandlerWithSettings: usize,
    pub RegisterByteStreamHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterByteStreamHandlerWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterByteStreamHandlerWithSettings: usize,
    pub RegisterAudioDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioDecoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioDecoderWithSettings: usize,
    pub RegisterAudioEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAudioEncoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAudioEncoderWithSettings: usize,
    pub RegisterVideoDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoDecoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoDecoderWithSettings: usize,
    pub RegisterVideoEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterVideoEncoderWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, inputsubtype: ::windows::core::GUID, outputsubtype: ::windows::core::GUID, configuration: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterVideoEncoderWithSettings: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaExtensionManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaExtensionManager2 {
    type Vtable = IMediaExtensionManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bcebf47_4043_4fed_acaf_54ec29dfb1f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaExtensionManager2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub RegisterMediaExtensionForAppService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extension: ::windows::core::RawPtr, connection: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    RegisterMediaExtensionForAppService: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaFrame(::windows::core::IUnknown);
impl IMediaFrame {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelativeTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSystemRelativeTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemRelativeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDiscontinuous)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsDiscontinuous(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDiscontinuous)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IMediaFrame> for ::windows::core::IUnknown {
    fn from(value: IMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaFrame> for ::windows::core::IUnknown {
    fn from(value: &IMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaFrame> for ::windows::core::IInspectable {
    fn from(value: IMediaFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaFrame> for ::windows::core::IInspectable {
    fn from(value: &IMediaFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IMediaFrame> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IMediaFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IMediaFrame> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMediaFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for IMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &IMediaFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IMediaFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IMediaFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bfb52f8c-5943-47d8-8e10-05308aa5fbd0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IMediaFrame {
    type Vtable = IMediaFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb52f8c_5943_47d8_8e10_05308aa5fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetIsDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaMarker(::windows::core::IUnknown);
impl IMediaMarker {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Time)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaMarkerType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IMediaMarker> for ::windows::core::IUnknown {
    fn from(value: IMediaMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarker> for ::windows::core::IUnknown {
    fn from(value: &IMediaMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaMarker> for ::windows::core::IInspectable {
    fn from(value: IMediaMarker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarker> for ::windows::core::IInspectable {
    fn from(value: &IMediaMarker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaMarker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IMediaMarker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1803def8-dca5-4b6f-9c20-e3d3c0643625}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IMediaMarker {
    type Vtable = IMediaMarker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1803def8_dca5_4b6f_9c20_e3d3c0643625);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarker_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaMarkerTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaMarkerTypesStatics {
    type Vtable = IMediaMarkerTypesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb198040_482f_4743_8832_45853821ece0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkerTypesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Bookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct IMediaMarkers(::windows::core::IUnknown);
impl IMediaMarkers {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Markers(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<IMediaMarker>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Markers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<IMediaMarker>>(result__)
        }
    }
}
impl ::core::convert::From<IMediaMarkers> for ::windows::core::IUnknown {
    fn from(value: IMediaMarkers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarkers> for ::windows::core::IUnknown {
    fn from(value: &IMediaMarkers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMediaMarkers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMediaMarkers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMediaMarkers> for ::windows::core::IInspectable {
    fn from(value: IMediaMarkers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMediaMarkers> for ::windows::core::IInspectable {
    fn from(value: &IMediaMarkers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMediaMarkers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMediaMarkers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMediaMarkers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IMediaMarkers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{afeab189-f8dd-466e-aa10-920b52353fdf}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IMediaMarkers {
    type Vtable = IMediaMarkers_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafeab189_f8dd_466e_aa10_920b52353fdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaMarkers_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Markers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Markers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProcessingTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb8564ac_a351_4f4e_b4f0_9bf2408993db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ed361f3_0b78_4360_bf71_0c841999ea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    pub ClockRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetClockRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaTimelineControllerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positionchangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statechangedeventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaTimelineController2 {
    type Vtable = IMediaTimelineController2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef74ea38_9e72_4df9_8355_6e90c81bbadd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineController2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
    #[cfg(feature = "Foundation")]
    pub Ended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Ended: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnded: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaTimelineControllerFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8821f81d_3e77_43fb_be26_4fc87a044834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTimelineControllerFailedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bbf0c59_d0a0_4d26_92a0_f978e1d18e7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAlbumArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Artist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetArtist: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMusicDisplayProperties2 {
    type Vtable = IMusicDisplayProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00368462_97d3_44b9_b00f_008afcefaf18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAlbumTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMusicDisplayProperties3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMusicDisplayProperties3 {
    type Vtable = IMusicDisplayProperties3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4db51ac1_0681_4e8c_9401_b8159d9eefc7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMusicDisplayProperties3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetAlbumTrackCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackPositionChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4493f88_eb28_4961_9c14_335e44f3e125);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackPositionChangeRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestedPlaybackPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedPlaybackPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce2c41f_3cd6_4f77_9ba7_eb27c26a2140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShuffleEnabledChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b593fe_4fd0_4666_a314_c0e01940d302);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShuffleEnabledChangeRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestedShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControls(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99fa3ff4_1742_42a6_902e_087d41f965ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackStatus) -> ::windows::core::HRESULT,
    pub SetPlaybackStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackStatus) -> ::windows::core::HRESULT,
    pub DisplayUpdater: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SoundLevel) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPlayEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsStopEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPauseEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRecordEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsFastForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRewindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPreviousEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsNextEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsChannelUpEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsChannelDownEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveButtonPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveButtonPressed: usize,
    #[cfg(feature = "Foundation")]
    pub PropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertyChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControls2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControls2 {
    type Vtable = ISystemMediaTransportControls2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea98d2f6_7f3c_4af2_a586_72889808efb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControls2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT,
    pub SetAutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub UpdateTimelineProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timelineproperties: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackPositionChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackPositionChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackPositionChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShuffleEnabledChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShuffleEnabledChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShuffleEnabledChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatModeChangeRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoRepeatModeChangeRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoRepeatModeChangeRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7f47116_a56f_4dc8_9e11_92031f4a87c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Button: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsButton) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsDisplayUpdater(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8abbc53e_fa55_4ecf_ad8e_c984e5dd1550);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsDisplayUpdater_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlaybackType) -> ::windows::core::HRESULT,
    pub AppMediaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAppMediaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CopyFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: MediaPlaybackType, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CopyFromFileAsync: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0ca0936_339b_4cb3_8eeb_737607f56e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemMediaTransportControlsProperty) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsStatics {
    type Vtable = ISystemMediaTransportControlsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43ba380a_eca4_4832_91ab_d415fae484c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemMediaTransportControlsTimelineProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5125316a_c3a2_475b_8507_93534dc88f15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsTimelineProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    #[cfg(feature = "Foundation")]
    pub MinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMinSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMinSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxSeekTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSeekTime: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDisplayProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5609fdb1_5d2d_4872_8170_45dee5bc2f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoDisplayProperties2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoDisplayProperties2 {
    type Vtable = IVideoDisplayProperties2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb410e1ce_ab52_41ab_a486_cc10fab152f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoDisplayProperties2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Genres: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Genres: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoEffectsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoEffectsStatics {
    type Vtable = IVideoEffectsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fcda5e8_baf1_4521_980c_3bcebb44cf38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEffectsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VideoStabilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoFrame {
    type Vtable = IVideoFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cc06625_90fc_4c92_bd95_7ded21819d1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
    #[cfg(feature = "Foundation")]
    pub CopyToAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyToAsync: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3DSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3DSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrame2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoFrame2 {
    type Vtable = IVideoFrame2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3837840d_336c_4366_8d46_060798736c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrame2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub CopyToWithBoundsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: ::windows::core::RawPtr, sourcebounds: ::windows::core::RawPtr, destinationbounds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))]
    CopyToWithBoundsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoFrameFactory {
    type Vtable = IVideoFrameFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x014b6d69_2228_4c92_92ff_50c380d3e776);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Create: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithAlpha: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoFrameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoFrameStatics {
    type Vtable = IVideoFrameStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab2a556f_6111_4b33_8ec3_2b209a02e17a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateAsDirect3D11SurfaceBacked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateAsDirect3D11SurfaceBacked: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateAsDirect3D11SurfaceBackedWithDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateAsDirect3D11SurfaceBackedWithDevice: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateWithSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateWithSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateWithDirect3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateWithDirect3D11Surface: usize,
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct ImageDisplayProperties(::windows::core::IUnknown);
impl ImageDisplayProperties {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ImageDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ImageDisplayProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ImageDisplayProperties;{cd0bc7ef-54e7-411f-9933-f0e98b0a96d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ImageDisplayProperties {
    type Vtable = IImageDisplayProperties_Vtbl;
    const IID: ::windows::core::GUID = <IImageDisplayProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ImageDisplayProperties {
    const NAME: &'static str = "Windows.Media.ImageDisplayProperties";
}
impl ::core::convert::From<ImageDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: ImageDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: &ImageDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ImageDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: ImageDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ImageDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: &ImageDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ImageDisplayProperties {}
unsafe impl ::core::marker::Sync for ImageDisplayProperties {}
#[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct MediaControl {}
#[cfg(feature = "deprecated")]
impl MediaControl {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSoundLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveSoundLevelChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PlayPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlayPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePlayPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePlayPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PausePressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PausePressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePausePressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePausePressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StopPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStopPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveStopPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PlayPauseTogglePressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlayPauseTogglePressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePlayPauseTogglePressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePlayPauseTogglePressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RecordPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRecordPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRecordPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn NextTrackPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NextTrackPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveNextTrackPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveNextTrackPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PreviousTrackPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviousTrackPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePreviousTrackPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePreviousTrackPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn FastForwardPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FastForwardPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveFastForwardPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveFastForwardPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RewindPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RewindPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRewindPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveRewindPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ChannelUpPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelUpPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveChannelUpPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveChannelUpPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ChannelDownPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChannelDownPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveChannelDownPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(cookie: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveChannelDownPressed)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SoundLevel() -> ::windows::core::Result<SoundLevel> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: SoundLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SoundLevel>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTrackName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).SetTrackName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TrackName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetArtistName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).SetArtistName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ArtistName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ArtistName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetIsPlaying(value: bool) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).SetIsPlaying)(::core::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsPlaying() -> ::windows::core::Result<bool> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPlaying)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetAlbumArt<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Uri>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IMediaControl(|this| unsafe { (::windows::core::Interface::vtable(this).SetAlbumArt)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn AlbumArt() -> ::windows::core::Result<super::Foundation::Uri> {
        Self::IMediaControl(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumArt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Uri>(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IMediaControl<R, F: FnOnce(&IMediaControl) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaControl, IMediaControl> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for MediaControl {
    const NAME: &'static str = "Windows.Media.MediaControl";
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaExtensionManager(::windows::core::IUnknown);
impl MediaExtensionManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaExtensionManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RegisterSchemeHandler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activatableclassid: Param0, scheme: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterSchemeHandler)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), scheme.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterSchemeHandlerWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, scheme: Param1, configuration: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterSchemeHandlerWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), scheme.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RegisterByteStreamHandler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activatableclassid: Param0, fileextension: Param1, mimetype: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterByteStreamHandler)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), fileextension.into_param().abi(), mimetype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterByteStreamHandlerWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, fileextension: Param1, mimetype: Param2, configuration: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterByteStreamHandlerWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), fileextension.into_param().abi(), mimetype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RegisterAudioDecoder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterAudioDecoder)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioDecoderWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterAudioDecoderWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RegisterAudioEncoder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterAudioEncoder)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAudioEncoderWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterAudioEncoderWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RegisterVideoDecoder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterVideoDecoder)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoDecoderWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterVideoDecoderWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RegisterVideoEncoder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterVideoEncoder)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterVideoEncoderWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param3: ::windows::core::IntoParam<'a, super::Foundation::Collections::IPropertySet>>(&self, activatableclassid: Param0, inputsubtype: Param1, outputsubtype: Param2, configuration: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterVideoEncoderWithSettings)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), inputsubtype.into_param().abi(), outputsubtype.into_param().abi(), configuration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn RegisterMediaExtensionForAppService<'a, Param0: ::windows::core::IntoParam<'a, IMediaExtension>, Param1: ::windows::core::IntoParam<'a, super::ApplicationModel::AppService::AppServiceConnection>>(&self, extension: Param0, connection: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaExtensionManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RegisterMediaExtensionForAppService)(::core::mem::transmute_copy(this), extension.into_param().abi(), connection.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaExtensionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaExtensionManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaExtensionManager;{4a25eaf5-242d-4dfb-97f4-69b7c42576ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaExtensionManager {
    type Vtable = IMediaExtensionManager_Vtbl;
    const IID: ::windows::core::GUID = <IMediaExtensionManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaExtensionManager {
    const NAME: &'static str = "Windows.Media.MediaExtensionManager";
}
impl ::core::convert::From<MediaExtensionManager> for ::windows::core::IUnknown {
    fn from(value: MediaExtensionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaExtensionManager> for ::windows::core::IUnknown {
    fn from(value: &MediaExtensionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaExtensionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaExtensionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaExtensionManager> for ::windows::core::IInspectable {
    fn from(value: MediaExtensionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaExtensionManager> for ::windows::core::IInspectable {
    fn from(value: &MediaExtensionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaExtensionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaExtensionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaExtensionManager {}
unsafe impl ::core::marker::Sync for MediaExtensionManager {}
#[doc = "*Required features: `\"Media\"`*"]
pub struct MediaMarkerTypes {}
impl MediaMarkerTypes {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Bookmark() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMediaMarkerTypesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bookmark)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaMarkerTypesStatics<R, F: FnOnce(&IMediaMarkerTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaMarkerTypes, IMediaMarkerTypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MediaMarkerTypes {
    const NAME: &'static str = "Windows.Media.MediaMarkerTypes";
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaPlaybackAutoRepeatMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackAutoRepeatMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackAutoRepeatMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackAutoRepeatMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackAutoRepeatMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaPlaybackStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaPlaybackType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaPlaybackType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaProcessingTriggerDetails(::windows::core::IUnknown);
impl MediaProcessingTriggerDetails {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaProcessingTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaProcessingTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaProcessingTriggerDetails;{eb8564ac-a351-4f4e-b4f0-9bf2408993db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaProcessingTriggerDetails {
    type Vtable = IMediaProcessingTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IMediaProcessingTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaProcessingTriggerDetails {
    const NAME: &'static str = "Windows.Media.MediaProcessingTriggerDetails";
}
impl ::core::convert::From<MediaProcessingTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: MediaProcessingTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProcessingTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &MediaProcessingTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaProcessingTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: MediaProcessingTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaProcessingTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &MediaProcessingTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaProcessingTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaProcessingTriggerDetails {}
unsafe impl ::core::marker::Sync for MediaProcessingTriggerDetails {}
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
unsafe impl ::windows::core::Abi for MediaTimeRange {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for MediaTimeRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.MediaTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for MediaTimeRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MediaTimeRange>()) == 0 }
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
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaTimelineController(::windows::core::IUnknown);
impl MediaTimelineController {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaTimelineController, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Resume)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Pause)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn ClockRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClockRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetClockRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClockRate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn State(&self) -> ::windows::core::Result<MediaTimelineControllerState> {
        let this = self;
        unsafe {
            let mut result__: MediaTimelineControllerState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaTimelineControllerState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>>(&self, positionchangedeventhandler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PositionChanged)(::core::mem::transmute_copy(this), positionchangedeventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePositionChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>>(&self, statechangedeventhandler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::core::mem::transmute_copy(this), statechangedeventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLoopingEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsLoopingEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Failed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, MediaTimelineControllerFailedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Failed)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFailed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Ended<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<MediaTimelineController, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ended)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnded<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaTimelineController2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaTimelineController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaTimelineController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineController;{8ed361f3-0b78-4360-bf71-0c841999ea1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaTimelineController {
    type Vtable = IMediaTimelineController_Vtbl;
    const IID: ::windows::core::GUID = <IMediaTimelineController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaTimelineController {
    const NAME: &'static str = "Windows.Media.MediaTimelineController";
}
impl ::core::convert::From<MediaTimelineController> for ::windows::core::IUnknown {
    fn from(value: MediaTimelineController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineController> for ::windows::core::IUnknown {
    fn from(value: &MediaTimelineController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaTimelineController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaTimelineController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaTimelineController> for ::windows::core::IInspectable {
    fn from(value: MediaTimelineController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineController> for ::windows::core::IInspectable {
    fn from(value: &MediaTimelineController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaTimelineController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaTimelineController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaTimelineController {}
unsafe impl ::core::marker::Sync for MediaTimelineController {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MediaTimelineControllerFailedEventArgs(::windows::core::IUnknown);
impl MediaTimelineControllerFailedEventArgs {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaTimelineControllerFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaTimelineControllerFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MediaTimelineControllerFailedEventArgs;{8821f81d-3e77-43fb-be26-4fc87a044834})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaTimelineControllerFailedEventArgs {
    type Vtable = IMediaTimelineControllerFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMediaTimelineControllerFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaTimelineControllerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.MediaTimelineControllerFailedEventArgs";
}
impl ::core::convert::From<MediaTimelineControllerFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaTimelineControllerFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineControllerFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaTimelineControllerFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaTimelineControllerFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaTimelineControllerFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaTimelineControllerFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaTimelineControllerFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaTimelineControllerFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaTimelineControllerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaTimelineControllerFailedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaTimelineControllerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaTimelineControllerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTimelineControllerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaTimelineControllerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.MediaTimelineControllerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct MusicDisplayProperties(::windows::core::IUnknown);
impl MusicDisplayProperties {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn AlbumArtist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumArtist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetAlbumArtist<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlbumArtist)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Artist(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Artist)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetArtist<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetArtist)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn AlbumTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetAlbumTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAlbumTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn TrackNumber(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrackNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetTrackNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTrackNumber)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Genres)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn AlbumTrackCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlbumTrackCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetAlbumTrackCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMusicDisplayProperties3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAlbumTrackCount)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for MusicDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MusicDisplayProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.MusicDisplayProperties;{6bbf0c59-d0a0-4d26-92a0-f978e1d18e7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MusicDisplayProperties {
    type Vtable = IMusicDisplayProperties_Vtbl;
    const IID: ::windows::core::GUID = <IMusicDisplayProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MusicDisplayProperties {
    const NAME: &'static str = "Windows.Media.MusicDisplayProperties";
}
impl ::core::convert::From<MusicDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: MusicDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MusicDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: &MusicDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MusicDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MusicDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MusicDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: MusicDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MusicDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: &MusicDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MusicDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MusicDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MusicDisplayProperties {}
unsafe impl ::core::marker::Sync for MusicDisplayProperties {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct PlaybackPositionChangeRequestedEventArgs(::windows::core::IUnknown);
impl PlaybackPositionChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedPlaybackPosition(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedPlaybackPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackPositionChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PlaybackPositionChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackPositionChangeRequestedEventArgs;{b4493f88-eb28-4961-9c14-335e44f3e125})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlaybackPositionChangeRequestedEventArgs {
    type Vtable = IPlaybackPositionChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPlaybackPositionChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackPositionChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackPositionChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackPositionChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlaybackPositionChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackPositionChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlaybackPositionChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackPositionChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlaybackPositionChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackPositionChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlaybackPositionChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaybackPositionChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackPositionChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackPositionChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows::core::IUnknown);
impl PlaybackRateChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RequestedPlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedPlaybackRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlaybackRateChangeRequestedEventArgs;{2ce2c41f-3cd6-4f77-9ba7-eb27c26a2140})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPlaybackRateChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlaybackRateChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlaybackRateChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackRateChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct ShuffleEnabledChangeRequestedEventArgs(::windows::core::IUnknown);
impl ShuffleEnabledChangeRequestedEventArgs {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn RequestedShuffleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestedShuffleEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ShuffleEnabledChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ShuffleEnabledChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.ShuffleEnabledChangeRequestedEventArgs;{49b593fe-4fd0-4666-a314-c0e01940d302})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ShuffleEnabledChangeRequestedEventArgs {
    type Vtable = IShuffleEnabledChangeRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IShuffleEnabledChangeRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ShuffleEnabledChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
}
impl ::core::convert::From<ShuffleEnabledChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShuffleEnabledChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShuffleEnabledChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShuffleEnabledChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShuffleEnabledChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShuffleEnabledChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShuffleEnabledChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShuffleEnabledChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ShuffleEnabledChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShuffleEnabledChangeRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ShuffleEnabledChangeRequestedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for SoundLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for SoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SoundLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SoundLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SoundLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControls(::windows::core::IUnknown);
impl SystemMediaTransportControls {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn PlaybackStatus(&self) -> ::windows::core::Result<MediaPlaybackStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetPlaybackStatus(&self, value: MediaPlaybackStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn DisplayUpdater(&self) -> ::windows::core::Result<SystemMediaTransportControlsDisplayUpdater> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayUpdater)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControlsDisplayUpdater>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SoundLevel(&self) -> ::windows::core::Result<SoundLevel> {
        let this = self;
        unsafe {
            let mut result__: SoundLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoundLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SoundLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsPlayEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPlayEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsPlayEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPlayEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsStopEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStopEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsStopEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsStopEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsPauseEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPauseEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsPauseEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPauseEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsRecordEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRecordEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsRecordEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRecordEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsFastForwardEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFastForwardEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsFastForwardEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsFastForwardEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsRewindEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRewindEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsRewindEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRewindEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsPreviousEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPreviousEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsPreviousEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPreviousEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsNextEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsNextEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsNextEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsNextEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsChannelUpEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsChannelUpEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsChannelUpEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsChannelUpEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsChannelDownEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsChannelDownEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsChannelDownEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsChannelDownEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsButtonPressedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ButtonPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveButtonPressed<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveButtonPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, SystemMediaTransportControlsPropertyChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PropertyChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertyChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn AutoRepeatMode(&self) -> ::windows::core::Result<MediaPlaybackAutoRepeatMode> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: MediaPlaybackAutoRepeatMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoRepeatMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackAutoRepeatMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetAutoRepeatMode(&self, value: MediaPlaybackAutoRepeatMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoRepeatMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn ShuffleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShuffleEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetShuffleEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPlaybackRate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn UpdateTimelineProperties<'a, Param0: ::windows::core::IntoParam<'a, SystemMediaTransportControlsTimelineProperties>>(&self, timelineproperties: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).UpdateTimelineProperties)(::core::mem::transmute_copy(this), timelineproperties.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackPositionChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackPositionChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackPositionChangeRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackPositionChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePlaybackPositionChangeRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, PlaybackRateChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackRateChangeRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePlaybackRateChangeRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShuffleEnabledChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, ShuffleEnabledChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShuffleEnabledChangeRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShuffleEnabledChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShuffleEnabledChangeRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatModeChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TypedEventHandler<SystemMediaTransportControls, AutoRepeatModeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe {
            let mut result__: super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoRepeatModeChangeRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoRepeatModeChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISystemMediaTransportControls2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAutoRepeatModeChangeRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<SystemMediaTransportControls> {
        Self::ISystemMediaTransportControlsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControls>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemMediaTransportControlsStatics<R, F: FnOnce(&ISystemMediaTransportControlsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemMediaTransportControls, ISystemMediaTransportControlsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControls {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControls;{99fa3ff4-1742-42a6-902e-087d41f965ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMediaTransportControls {
    type Vtable = ISystemMediaTransportControls_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMediaTransportControls as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMediaTransportControls {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControls";
}
impl ::core::convert::From<SystemMediaTransportControls> for ::windows::core::IUnknown {
    fn from(value: SystemMediaTransportControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControls> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaTransportControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaTransportControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaTransportControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControls> for ::windows::core::IInspectable {
    fn from(value: SystemMediaTransportControls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControls> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaTransportControls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaTransportControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaTransportControls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControls {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControls {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for SystemMediaTransportControlsButton {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemMediaTransportControlsButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControlsButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsButton;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsButtonPressedEventArgs(::windows::core::IUnknown);
impl SystemMediaTransportControlsButtonPressedEventArgs {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Button(&self) -> ::windows::core::Result<SystemMediaTransportControlsButton> {
        let this = self;
        unsafe {
            let mut result__: SystemMediaTransportControlsButton = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Button)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControlsButton>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsButtonPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControlsButtonPressedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs;{b7f47116-a56f-4dc8-9e11-92031f4a87c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMediaTransportControlsButtonPressedEventArgs {
    type Vtable = ISystemMediaTransportControlsButtonPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMediaTransportControlsButtonPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMediaTransportControlsButtonPressedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
}
impl ::core::convert::From<SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsButtonPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaTransportControlsButtonPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaTransportControlsButtonPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsButtonPressedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsButtonPressedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsDisplayUpdater(::windows::core::IUnknown);
impl SystemMediaTransportControlsDisplayUpdater {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__: MediaPlaybackType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaPlaybackType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetType(&self, value: MediaPlaybackType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn AppMediaId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppMediaId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetAppMediaId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppMediaId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Storage::Streams::RandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::Storage::Streams::RandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnail)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn MusicProperties(&self) -> ::windows::core::Result<MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MusicDisplayProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn VideoProperties(&self) -> ::windows::core::Result<VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoDisplayProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn ImageProperties(&self) -> ::windows::core::Result<ImageDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ImageDisplayProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CopyFromFileAsync<'a, Param1: ::windows::core::IntoParam<'a, super::Storage::StorageFile>>(&self, r#type: MediaPlaybackType, source: Param1) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyFromFileAsync)(::core::mem::transmute_copy(this), r#type, source.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn ClearAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearAll)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Update(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Update)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsDisplayUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControlsDisplayUpdater {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsDisplayUpdater;{8abbc53e-fa55-4ecf-ad8e-c984e5dd1550})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMediaTransportControlsDisplayUpdater {
    type Vtable = ISystemMediaTransportControlsDisplayUpdater_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMediaTransportControlsDisplayUpdater as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMediaTransportControlsDisplayUpdater {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsDisplayUpdater";
}
impl ::core::convert::From<SystemMediaTransportControlsDisplayUpdater> for ::windows::core::IUnknown {
    fn from(value: SystemMediaTransportControlsDisplayUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsDisplayUpdater> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaTransportControlsDisplayUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsDisplayUpdater> for ::windows::core::IInspectable {
    fn from(value: SystemMediaTransportControlsDisplayUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsDisplayUpdater> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaTransportControlsDisplayUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaTransportControlsDisplayUpdater {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsDisplayUpdater {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsDisplayUpdater {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for SystemMediaTransportControlsProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemMediaTransportControlsProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMediaTransportControlsProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControlsProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.SystemMediaTransportControlsProperty;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsPropertyChangedEventArgs(::windows::core::IUnknown);
impl SystemMediaTransportControlsPropertyChangedEventArgs {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Property(&self) -> ::windows::core::Result<SystemMediaTransportControlsProperty> {
        let this = self;
        unsafe {
            let mut result__: SystemMediaTransportControlsProperty = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Property)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemMediaTransportControlsProperty>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControlsPropertyChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs;{d0ca0936-339b-4cb3-8eeb-737607f56e08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMediaTransportControlsPropertyChangedEventArgs {
    type Vtable = ISystemMediaTransportControlsPropertyChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMediaTransportControlsPropertyChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMediaTransportControlsPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
}
impl ::core::convert::From<SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsPropertyChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaTransportControlsPropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaTransportControlsPropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsPropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsPropertyChangedEventArgs {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct SystemMediaTransportControlsTimelineProperties(::windows::core::IUnknown);
impl SystemMediaTransportControlsTimelineProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemMediaTransportControlsTimelineProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSeekTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinSeekTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMinSeekTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMinSeekTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSeekTime(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxSeekTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSeekTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxSeekTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SystemMediaTransportControlsTimelineProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SystemMediaTransportControlsTimelineProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.SystemMediaTransportControlsTimelineProperties;{5125316a-c3a2-475b-8507-93534dc88f15})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemMediaTransportControlsTimelineProperties {
    type Vtable = ISystemMediaTransportControlsTimelineProperties_Vtbl;
    const IID: ::windows::core::GUID = <ISystemMediaTransportControlsTimelineProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemMediaTransportControlsTimelineProperties {
    const NAME: &'static str = "Windows.Media.SystemMediaTransportControlsTimelineProperties";
}
impl ::core::convert::From<SystemMediaTransportControlsTimelineProperties> for ::windows::core::IUnknown {
    fn from(value: SystemMediaTransportControlsTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsTimelineProperties> for ::windows::core::IUnknown {
    fn from(value: &SystemMediaTransportControlsTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemMediaTransportControlsTimelineProperties> for ::windows::core::IInspectable {
    fn from(value: SystemMediaTransportControlsTimelineProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemMediaTransportControlsTimelineProperties> for ::windows::core::IInspectable {
    fn from(value: &SystemMediaTransportControlsTimelineProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemMediaTransportControlsTimelineProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemMediaTransportControlsTimelineProperties {}
unsafe impl ::core::marker::Sync for SystemMediaTransportControlsTimelineProperties {}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct VideoDisplayProperties(::windows::core::IUnknown);
impl VideoDisplayProperties {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Subtitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetSubtitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubtitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Genres(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IVideoDisplayProperties2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Genres)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VideoDisplayProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.VideoDisplayProperties;{5609fdb1-5d2d-4872-8170-45dee5bc2f5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VideoDisplayProperties {
    type Vtable = IVideoDisplayProperties_Vtbl;
    const IID: ::windows::core::GUID = <IVideoDisplayProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoDisplayProperties {
    const NAME: &'static str = "Windows.Media.VideoDisplayProperties";
}
impl ::core::convert::From<VideoDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: VideoDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDisplayProperties> for ::windows::core::IUnknown {
    fn from(value: &VideoDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: VideoDisplayProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoDisplayProperties> for ::windows::core::IInspectable {
    fn from(value: &VideoDisplayProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoDisplayProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoDisplayProperties {}
unsafe impl ::core::marker::Sync for VideoDisplayProperties {}
#[doc = "*Required features: `\"Media\"`*"]
pub struct VideoEffects {}
impl VideoEffects {
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn VideoStabilization() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IVideoEffectsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoStabilization)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoEffectsStatics<R, F: FnOnce(&IVideoEffectsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoEffects, IVideoEffectsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for VideoEffects {
    const NAME: &'static str = "Windows.Media.VideoEffects";
}
#[doc = "*Required features: `\"Media\"`*"]
#[repr(transparent)]
pub struct VideoFrame(::windows::core::IUnknown);
impl VideoFrame {
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRelativeTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSystemRelativeTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemRelativeTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IReference<super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDiscontinuous)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media\"`*"]
    pub fn IsDiscontinuous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDiscontinuous)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<IMediaFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows::core::Result<super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoftwareBitmap)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyToAsync<'a, Param0: ::windows::core::IntoParam<'a, VideoFrame>>(&self, frame: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyToAsync)(::core::mem::transmute_copy(this), frame.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3DSurface(&self) -> ::windows::core::Result<super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direct3DSurface)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Graphics::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    pub fn CopyToWithBoundsAsync<'a, Param0: ::windows::core::IntoParam<'a, VideoFrame>, Param1: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>, Param2: ::windows::core::IntoParam<'a, super::Foundation::IReference<super::Graphics::Imaging::BitmapBounds>>>(&self, frame: Param0, sourcebounds: Param1, destinationbounds: Param2) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IVideoFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CopyToWithBoundsAsync)(::core::mem::transmute_copy(this), frame.into_param().abi(), sourcebounds.into_param().abi(), destinationbounds.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Create(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), format, width, height, &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithAlpha(format: super::Graphics::Imaging::BitmapPixelFormat, width: i32, height: i32, alpha: super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<VideoFrame> {
        Self::IVideoFrameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithAlpha)(::core::mem::transmute_copy(this), format, width, height, alpha, &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateAsDirect3D11SurfaceBacked(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32) -> ::windows::core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsDirect3D11SurfaceBacked)(::core::mem::transmute_copy(this), format, width, height, &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateAsDirect3D11SurfaceBackedWithDevice<'a, Param3: ::windows::core::IntoParam<'a, super::Graphics::DirectX::Direct3D11::IDirect3DDevice>>(format: super::Graphics::DirectX::DirectXPixelFormat, width: i32, height: i32, device: Param3) -> ::windows::core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsDirect3D11SurfaceBackedWithDevice)(::core::mem::transmute_copy(this), format, width, height, device.into_param().abi(), &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateWithSoftwareBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::Graphics::Imaging::SoftwareBitmap>>(bitmap: Param0) -> ::windows::core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithSoftwareBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[doc = "*Required features: `\"Media\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateWithDirect3D11Surface<'a, Param0: ::windows::core::IntoParam<'a, super::Graphics::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0) -> ::windows::core::Result<VideoFrame> {
        Self::IVideoFrameStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithDirect3D11Surface)(::core::mem::transmute_copy(this), surface.into_param().abi(), &mut result__).from_abi::<VideoFrame>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoFrameFactory<R, F: FnOnce(&IVideoFrameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoFrame, IVideoFrameFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IVideoFrameStatics<R, F: FnOnce(&IVideoFrameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VideoFrame, IVideoFrameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VideoFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.VideoFrame;{0cc06625-90fc-4c92-bd95-7ded21819d1c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VideoFrame {
    type Vtable = IVideoFrame_Vtbl;
    const IID: ::windows::core::GUID = <IVideoFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoFrame {
    const NAME: &'static str = "Windows.Media.VideoFrame";
}
impl ::core::convert::From<VideoFrame> for ::windows::core::IUnknown {
    fn from(value: VideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoFrame> for ::windows::core::IUnknown {
    fn from(value: &VideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoFrame> for ::windows::core::IInspectable {
    fn from(value: VideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoFrame> for ::windows::core::IInspectable {
    fn from(value: &VideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<VideoFrame> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&VideoFrame> for super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IClosable> for &VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VideoFrame> for IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: VideoFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VideoFrame> for IMediaFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &VideoFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaFrame> for VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMediaFrame> for &VideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IMediaFrame> {
        ::core::convert::TryInto::<IMediaFrame>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VideoFrame {}
unsafe impl ::core::marker::Sync for VideoFrame {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
