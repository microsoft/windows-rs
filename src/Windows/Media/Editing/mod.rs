#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BackgroundAudioTrack(pub ::windows::core::IInspectable);
impl BackgroundAudioTrack {
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<BackgroundAudioTrack> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundAudioTrack>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    pub fn CreateFromEmbeddedAudioTrack<'a, Param0: ::windows::core::IntoParam<'a, EmbeddedAudioTrack>>(embeddedaudiotrack: Param0) -> ::windows::core::Result<BackgroundAudioTrack> {
        Self::IBackgroundAudioTrackStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), embeddedaudiotrack.into_param().abi(), &mut result__).from_abi::<BackgroundAudioTrack>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>> {
        Self::IBackgroundAudioTrackStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>>(result__)
        })
    }
    pub fn IBackgroundAudioTrackStatics<R, F: FnOnce(&IBackgroundAudioTrackStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundAudioTrack, IBackgroundAudioTrackStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundAudioTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.BackgroundAudioTrack;{4b91b3bd-9e21-4266-a9c2-67dd011a2357})");
}
unsafe impl ::windows::core::Interface for BackgroundAudioTrack {
    type Vtable = IBackgroundAudioTrack_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b91b3bd_9e21_4266_a9c2_67dd011a2357);
}
impl ::windows::core::RuntimeName for BackgroundAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.BackgroundAudioTrack";
}
impl ::core::convert::From<BackgroundAudioTrack> for ::windows::core::IUnknown {
    fn from(value: BackgroundAudioTrack) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BackgroundAudioTrack> for ::windows::core::IUnknown {
    fn from(value: &BackgroundAudioTrack) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BackgroundAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BackgroundAudioTrack> for ::windows::core::IInspectable {
    fn from(value: BackgroundAudioTrack) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BackgroundAudioTrack> for ::windows::core::IInspectable {
    fn from(value: &BackgroundAudioTrack) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BackgroundAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BackgroundAudioTrack {}
unsafe impl ::core::marker::Sync for BackgroundAudioTrack {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmbeddedAudioTrack(pub ::windows::core::IInspectable);
impl EmbeddedAudioTrack {
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetAudioEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmbeddedAudioTrack {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.EmbeddedAudioTrack;{55ee5a7a-2d30-3fba-a190-4f1a6454f88f})");
}
unsafe impl ::windows::core::Interface for EmbeddedAudioTrack {
    type Vtable = IEmbeddedAudioTrack_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ee5a7a_2d30_3fba_a190_4f1a6454f88f);
}
impl ::windows::core::RuntimeName for EmbeddedAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.EmbeddedAudioTrack";
}
impl ::core::convert::From<EmbeddedAudioTrack> for ::windows::core::IUnknown {
    fn from(value: EmbeddedAudioTrack) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmbeddedAudioTrack> for ::windows::core::IUnknown {
    fn from(value: &EmbeddedAudioTrack) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmbeddedAudioTrack> for ::windows::core::IInspectable {
    fn from(value: EmbeddedAudioTrack) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmbeddedAudioTrack> for ::windows::core::IInspectable {
    fn from(value: &EmbeddedAudioTrack) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmbeddedAudioTrack {}
unsafe impl ::core::marker::Sync for EmbeddedAudioTrack {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundAudioTrack(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackgroundAudioTrack {
    type Vtable = IBackgroundAudioTrack_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b91b3bd_9e21_4266_a9c2_67dd011a2357);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundAudioTrack_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundAudioTrackStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackgroundAudioTrackStatics {
    type Vtable = IBackgroundAudioTrackStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b1c0d7_d018_42a8_a559_cb4d9e97e664);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundAudioTrackStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, embeddedaudiotrack: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmbeddedAudioTrack(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmbeddedAudioTrack {
    type Vtable = IEmbeddedAudioTrack_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ee5a7a_2d30_3fba_a190_4f1a6454f88f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmbeddedAudioTrack_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaClip(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaClip {
    type Vtable = IMediaClip_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f25366_5fba_3ea4_8693_24761811140a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClip_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaClipStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaClipStatics {
    type Vtable = IMediaClipStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa402b68_928f_43c4_bc6e_783a1a359656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClipStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: super::super::UI::Color, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaClipStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaClipStatics2 {
    type Vtable = IMediaClipStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b1dd7b3_854e_4d9b_877d_4774a556cd12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClipStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaComposition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaComposition {
    type Vtable = IMediaComposition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e06e605_dc71_41d6_b837_2d2bc14a2947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaComposition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timefromstart: super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timesfromstart: ::windows::core::RawPtr, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, trimmingpreference: MediaTrimmingPreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, destination: ::windows::core::RawPtr, trimmingpreference: MediaTrimmingPreference, encodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage")))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingprofile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaComposition2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaComposition2 {
    type Vtable = IMediaComposition2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa59e5372_2366_492c_bec8_e6dfba6d0281);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaComposition2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCompositionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaCompositionStatics {
    type Vtable = IMediaCompositionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a08f04_e32a_45ce_8f66_a30df0766224);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCompositionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlay(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaOverlay {
    type Vtable = IMediaOverlay_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa902ae5d_7869_4830_8ab1_94dc01c05fa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlay_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlayFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaOverlayFactory {
    type Vtable = IMediaOverlayFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb584828a_6188_4f8f_a2e0_aa552d598e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: ::windows::core::RawPtr, position: super::super::Foundation::Rect, opacity: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlayLayer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaOverlayLayer {
    type Vtable = IMediaOverlayLayer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6d9ba57_eeda_46c6_bbe5_e398c84168ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayLayer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Media_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlayLayerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMediaOverlayLayerFactory {
    type Vtable = IMediaOverlayLayerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947cb473_a39e_4362_abbf_9f8b5070a062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayLayerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositordefinition: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaClip(pub ::windows::core::IInspectable);
impl MediaClip {
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrimTimeFromEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTrimTimeFromEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn OriginalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrimmedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaClip> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaClip>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EndTimeInComposition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EmbeddedAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>>(result__)
        }
    }
    pub fn SelectedEmbeddedAudioTrackIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetSelectedEmbeddedAudioTrackIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetVideoEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn AudioEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn VideoEffectDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn CreateFromColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(color: Param0, originalduration: Param1) -> ::windows::core::Result<MediaClip> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), color.into_param().abi(), originalduration.into_param().abi(), &mut result__).from_abi::<MediaClip>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaClip>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateFromImageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(file: Param0, originalduration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaClip>> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), file.into_param().abi(), originalduration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaClip>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateFromSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(surface: Param0, originalduration: Param1) -> ::windows::core::Result<MediaClip> {
        Self::IMediaClipStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), surface.into_param().abi(), originalduration.into_param().abi(), &mut result__).from_abi::<MediaClip>(result__)
        })
    }
    pub fn IMediaClipStatics<R, F: FnOnce(&IMediaClipStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaClip, IMediaClipStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaClipStatics2<R, F: FnOnce(&IMediaClipStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaClip, IMediaClipStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaClip {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaClip;{53f25366-5fba-3ea4-8693-24761811140a})");
}
unsafe impl ::windows::core::Interface for MediaClip {
    type Vtable = IMediaClip_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f25366_5fba_3ea4_8693_24761811140a);
}
impl ::windows::core::RuntimeName for MediaClip {
    const NAME: &'static str = "Windows.Media.Editing.MediaClip";
}
impl ::core::convert::From<MediaClip> for ::windows::core::IUnknown {
    fn from(value: MediaClip) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaClip> for ::windows::core::IUnknown {
    fn from(value: &MediaClip) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaClip> for ::windows::core::IInspectable {
    fn from(value: MediaClip) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaClip> for ::windows::core::IInspectable {
    fn from(value: &MediaClip) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaClip {}
unsafe impl ::core::marker::Sync for MediaClip {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaComposition(pub ::windows::core::IInspectable);
impl MediaComposition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaComposition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clips(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaClip>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<MediaClip>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BackgroundAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaComposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaComposition>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timefromstart: Param0, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), timefromstart.into_param().abi(), scaledwidth, scaledheight, frameprecision, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    pub fn GetThumbnailsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan>>>(&self, timesfromstart: Param0, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), timesfromstart.into_param().abi(), scaledwidth, scaledheight, frameprecision, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub fn RenderToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, destination: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    pub fn RenderToFileWithTrimmingPreferenceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, destination: Param0, trimmingpreference: MediaTrimmingPreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), destination.into_param().abi(), trimmingpreference, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))]
    pub fn RenderToFileWithProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, destination: Param0, trimmingpreference: MediaTrimmingPreference, encodingprofile: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), destination.into_param().abi(), trimmingpreference, encodingprofile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    pub fn CreateDefaultEncodingProfile(&self) -> ::windows::core::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn GenerateMediaStreamSource(&self) -> ::windows::core::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::MediaStreamSource>(result__)
        }
    }
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))]
    pub fn GenerateMediaStreamSourceWithProfile<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, encodingprofile: Param0) -> ::windows::core::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), &mut result__).from_abi::<super::Core::MediaStreamSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    pub fn GeneratePreviewMediaStreamSource(&self, scaledwidth: i32, scaledheight: i32) -> ::windows::core::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), scaledwidth, scaledheight, &mut result__).from_abi::<super::Core::MediaStreamSource>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OverlayLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlayLayer>> {
        let this = &::windows::core::Interface::cast::<IMediaComposition2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<MediaOverlayLayer>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaComposition>> {
        Self::IMediaCompositionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaComposition>>(result__)
        })
    }
    pub fn IMediaCompositionStatics<R, F: FnOnce(&IMediaCompositionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaComposition, IMediaCompositionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaComposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaComposition;{2e06e605-dc71-41d6-b837-2d2bc14a2947})");
}
unsafe impl ::windows::core::Interface for MediaComposition {
    type Vtable = IMediaComposition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e06e605_dc71_41d6_b837_2d2bc14a2947);
}
impl ::windows::core::RuntimeName for MediaComposition {
    const NAME: &'static str = "Windows.Media.Editing.MediaComposition";
}
impl ::core::convert::From<MediaComposition> for ::windows::core::IUnknown {
    fn from(value: MediaComposition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaComposition> for ::windows::core::IUnknown {
    fn from(value: &MediaComposition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaComposition> for ::windows::core::IInspectable {
    fn from(value: MediaComposition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaComposition> for ::windows::core::IInspectable {
    fn from(value: &MediaComposition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaComposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaComposition {}
unsafe impl ::core::marker::Sync for MediaComposition {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaOverlay(pub ::windows::core::IInspectable);
impl MediaOverlay {
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaOverlay> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaOverlay>(result__)
        }
    }
    pub fn Clip(&self) -> ::windows::core::Result<MediaClip> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaClip>(result__)
        }
    }
    pub fn AudioEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAudioEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, MediaClip>>(clip: Param0) -> ::windows::core::Result<MediaOverlay> {
        Self::IMediaOverlayFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), clip.into_param().abi(), &mut result__).from_abi::<MediaOverlay>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWithPositionAndOpacity<'a, Param0: ::windows::core::IntoParam<'a, MediaClip>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(clip: Param0, position: Param1, opacity: f64) -> ::windows::core::Result<MediaOverlay> {
        Self::IMediaOverlayFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), clip.into_param().abi(), position.into_param().abi(), opacity, &mut result__).from_abi::<MediaOverlay>(result__)
        })
    }
    pub fn IMediaOverlayFactory<R, F: FnOnce(&IMediaOverlayFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaOverlay, IMediaOverlayFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaOverlay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaOverlay;{a902ae5d-7869-4830-8ab1-94dc01c05fa4})");
}
unsafe impl ::windows::core::Interface for MediaOverlay {
    type Vtable = IMediaOverlay_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa902ae5d_7869_4830_8ab1_94dc01c05fa4);
}
impl ::windows::core::RuntimeName for MediaOverlay {
    const NAME: &'static str = "Windows.Media.Editing.MediaOverlay";
}
impl ::core::convert::From<MediaOverlay> for ::windows::core::IUnknown {
    fn from(value: MediaOverlay) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaOverlay> for ::windows::core::IUnknown {
    fn from(value: &MediaOverlay) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaOverlay> for ::windows::core::IInspectable {
    fn from(value: MediaOverlay) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaOverlay> for ::windows::core::IInspectable {
    fn from(value: &MediaOverlay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaOverlay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaOverlay {}
unsafe impl ::core::marker::Sync for MediaOverlay {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MediaOverlayLayer(pub ::windows::core::IInspectable);
impl MediaOverlayLayer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaOverlayLayer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Clone(&self) -> ::windows::core::Result<MediaOverlayLayer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaOverlayLayer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Overlays(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<MediaOverlay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<MediaOverlay>>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn CustomCompositorDefinition(&self) -> ::windows::core::Result<super::Effects::IVideoCompositorDefinition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Effects::IVideoCompositorDefinition>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    pub fn CreateWithCompositorDefinition<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IVideoCompositorDefinition>>(compositordefinition: Param0) -> ::windows::core::Result<MediaOverlayLayer> {
        Self::IMediaOverlayLayerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositordefinition.into_param().abi(), &mut result__).from_abi::<MediaOverlayLayer>(result__)
        })
    }
    pub fn IMediaOverlayLayerFactory<R, F: FnOnce(&IMediaOverlayLayerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaOverlayLayer, IMediaOverlayLayerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MediaOverlayLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaOverlayLayer;{a6d9ba57-eeda-46c6-bbe5-e398c84168ac})");
}
unsafe impl ::windows::core::Interface for MediaOverlayLayer {
    type Vtable = IMediaOverlayLayer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6d9ba57_eeda_46c6_bbe5_e398c84168ac);
}
impl ::windows::core::RuntimeName for MediaOverlayLayer {
    const NAME: &'static str = "Windows.Media.Editing.MediaOverlayLayer";
}
impl ::core::convert::From<MediaOverlayLayer> for ::windows::core::IUnknown {
    fn from(value: MediaOverlayLayer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MediaOverlayLayer> for ::windows::core::IUnknown {
    fn from(value: &MediaOverlayLayer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaOverlayLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaOverlayLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MediaOverlayLayer> for ::windows::core::IInspectable {
    fn from(value: MediaOverlayLayer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MediaOverlayLayer> for ::windows::core::IInspectable {
    fn from(value: &MediaOverlayLayer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaOverlayLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaOverlayLayer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MediaOverlayLayer {}
unsafe impl ::core::marker::Sync for MediaOverlayLayer {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: MediaTrimmingPreference = MediaTrimmingPreference(0i32);
    pub const Precise: MediaTrimmingPreference = MediaTrimmingPreference(1i32);
}
impl ::core::convert::From<i32> for MediaTrimmingPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MediaTrimmingPreference {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MediaTrimmingPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Editing.MediaTrimmingPreference;i4)");
}
impl ::windows::core::DefaultType for MediaTrimmingPreference {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: VideoFramePrecision = VideoFramePrecision(0i32);
    pub const NearestKeyFrame: VideoFramePrecision = VideoFramePrecision(1i32);
}
impl ::core::convert::From<i32> for VideoFramePrecision {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VideoFramePrecision {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VideoFramePrecision {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Editing.VideoFramePrecision;i4)");
}
impl ::windows::core::DefaultType for VideoFramePrecision {
    type DefaultType = Self;
}
