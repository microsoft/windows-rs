#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_Editing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundAudioTrack(::windows::runtime::IInspectable);
impl BackgroundAudioTrack {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn TrimTimeFromStart(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetTrimTimeFromStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn TrimTimeFromEnd(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetTrimTimeFromEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn OriginalDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn TrimmedDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn UserData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn SetVolume(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Volume(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<BackgroundAudioTrack> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundAudioTrack>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Editing`, `Media_MediaProperties`*"]
    pub fn GetAudioEncodingProperties(&self) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`, `Media_Effects`*"]
    pub fn AudioEffectDefinitions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn CreateFromEmbeddedAudioTrack<'a, Param0: ::windows::runtime::IntoParam<'a, EmbeddedAudioTrack>>(embeddedaudiotrack: Param0) -> ::windows::runtime::Result<BackgroundAudioTrack> {
        Self::IBackgroundAudioTrackStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), embeddedaudiotrack.into_param().abi(), &mut result__).from_abi::<BackgroundAudioTrack>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Storage`*"]
    pub fn CreateFromFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>> {
        Self::IBackgroundAudioTrackStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BackgroundAudioTrack>>(result__)
        })
    }
    pub fn IBackgroundAudioTrackStatics<R, F: FnOnce(&IBackgroundAudioTrackStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundAudioTrack, IBackgroundAudioTrackStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundAudioTrack {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.BackgroundAudioTrack;{4b91b3bd-9e21-4266-a9c2-67dd011a2357})");
}
unsafe impl ::windows::runtime::Interface for BackgroundAudioTrack {
    type Vtable = IBackgroundAudioTrack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1267839933, 40481, 16998, [169, 194, 103, 221, 1, 26, 35, 87]);
}
impl ::windows::runtime::RuntimeName for BackgroundAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.BackgroundAudioTrack";
}
impl ::std::convert::From<BackgroundAudioTrack> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundAudioTrack) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundAudioTrack> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundAudioTrack) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundAudioTrack> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundAudioTrack) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundAudioTrack> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundAudioTrack) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundAudioTrack {}
unsafe impl ::std::marker::Sync for BackgroundAudioTrack {}
#[doc = "*Required features: `Media_Editing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmbeddedAudioTrack(::windows::runtime::IInspectable);
impl EmbeddedAudioTrack {
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Editing`, `Media_MediaProperties`*"]
    pub fn GetAudioEncodingProperties(&self) -> ::windows::runtime::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::AudioEncodingProperties>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmbeddedAudioTrack {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.EmbeddedAudioTrack;{55ee5a7a-2d30-3fba-a190-4f1a6454f88f})");
}
unsafe impl ::windows::runtime::Interface for EmbeddedAudioTrack {
    type Vtable = IEmbeddedAudioTrack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1441684090, 11568, 16314, [161, 144, 79, 26, 100, 84, 248, 143]);
}
impl ::windows::runtime::RuntimeName for EmbeddedAudioTrack {
    const NAME: &'static str = "Windows.Media.Editing.EmbeddedAudioTrack";
}
impl ::std::convert::From<EmbeddedAudioTrack> for ::windows::runtime::IUnknown {
    fn from(value: EmbeddedAudioTrack) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmbeddedAudioTrack> for ::windows::runtime::IUnknown {
    fn from(value: &EmbeddedAudioTrack) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmbeddedAudioTrack> for ::windows::runtime::IInspectable {
    fn from(value: EmbeddedAudioTrack) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmbeddedAudioTrack> for ::windows::runtime::IInspectable {
    fn from(value: &EmbeddedAudioTrack) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmbeddedAudioTrack {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmbeddedAudioTrack {}
unsafe impl ::std::marker::Sync for EmbeddedAudioTrack {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundAudioTrack(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundAudioTrack {
    type Vtable = IBackgroundAudioTrack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1267839933, 40481, 16998, [169, 194, 103, 221, 1, 26, 35, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundAudioTrack_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBackgroundAudioTrackStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundAudioTrackStatics {
    type Vtable = IBackgroundAudioTrackStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3652305111, 53272, 17064, [165, 89, 203, 77, 158, 151, 230, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundAudioTrackStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, embeddedaudiotrack: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmbeddedAudioTrack(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmbeddedAudioTrack {
    type Vtable = IEmbeddedAudioTrack_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1441684090, 11568, 16314, [161, 144, 79, 26, 100, 84, 248, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmbeddedAudioTrack_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaClip(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaClip {
    type Vtable = IMediaClip_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1408389990, 24506, 16036, [134, 147, 36, 118, 24, 17, 20, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClip_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Effects")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaClipStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaClipStatics {
    type Vtable = IMediaClipStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4198509416, 37519, 17348, [188, 110, 120, 58, 26, 53, 150, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClipStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, color: super::super::UI::Color, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaClipStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaClipStatics2 {
    type Vtable = IMediaClipStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1528682419, 34126, 19867, [135, 125, 71, 116, 165, 86, 205, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaClipStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, surface: ::windows::runtime::RawPtr, originalduration: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaComposition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaComposition {
    type Vtable = IMediaComposition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(772204037, 56433, 16854, [184, 55, 45, 43, 193, 74, 41, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaComposition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timefromstart: super::super::Foundation::TimeSpan, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timesfromstart: ::windows::runtime::RawPtr, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, trimmingpreference: MediaTrimmingPreference, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destination: ::windows::runtime::RawPtr, trimmingpreference: MediaTrimmingPreference, encodingprofile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage")))] usize,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, encodingprofile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "Media_MediaProperties")))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scaledwidth: i32, scaledheight: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaComposition2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaComposition2 {
    type Vtable = IMediaComposition2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2778616690, 9062, 18732, [190, 200, 230, 223, 186, 109, 2, 129]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaComposition2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaCompositionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaCompositionStatics {
    type Vtable = IMediaCompositionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275446532, 58154, 17870, [143, 102, 163, 13, 240, 118, 98, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCompositionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlay(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaOverlay {
    type Vtable = IMediaOverlay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2835525213, 30825, 18480, [138, 177, 148, 220, 1, 192, 95, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlay_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlayFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaOverlayFactory {
    type Vtable = IMediaOverlayFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3045360266, 24968, 20367, [162, 224, 170, 85, 45, 89, 142, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clip: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clip: ::windows::runtime::RawPtr, position: super::super::Foundation::Rect, opacity: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlayLayer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaOverlayLayer {
    type Vtable = IMediaOverlayLayer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2799286871, 61146, 18118, [187, 229, 227, 152, 200, 65, 104, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayLayer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Media_Effects")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMediaOverlayLayerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaOverlayLayerFactory {
    type Vtable = IMediaOverlayLayerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2491200627, 41886, 17250, [171, 191, 159, 139, 80, 112, 160, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaOverlayLayerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Effects")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositordefinition: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Effects"))] usize,
);
#[doc = "*Required features: `Media_Editing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaClip(::windows::runtime::IInspectable);
impl MediaClip {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn TrimTimeFromStart(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetTrimTimeFromStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn TrimTimeFromEnd(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetTrimTimeFromEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn OriginalDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn TrimmedDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn UserData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<MediaClip> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaClip>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn StartTimeInComposition(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn EndTimeInComposition(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn EmbeddedAudioTracks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<EmbeddedAudioTrack>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn SelectedEmbeddedAudioTrackIndex(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn SetSelectedEmbeddedAudioTrackIndex(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn SetVolume(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Volume(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Editing`, `Media_MediaProperties`*"]
    pub fn GetVideoEncodingProperties(&self) -> ::windows::runtime::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`, `Media_Effects`*"]
    pub fn AudioEffectDefinitions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IAudioEffectDefinition>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`, `Media_Effects`*"]
    pub fn VideoEffectDefinitions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Effects::IVideoEffectDefinition>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `UI`*"]
    pub fn CreateFromColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(color: Param0, originalduration: Param1) -> ::windows::runtime::Result<MediaClip> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), color.into_param().abi(), originalduration.into_param().abi(), &mut result__).from_abi::<MediaClip>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Storage`*"]
    pub fn CreateFromFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaClip>> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaClip>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Storage`*"]
    pub fn CreateFromImageFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(file: Param0, originalduration: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaClip>> {
        Self::IMediaClipStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), file.into_param().abi(), originalduration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaClip>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Graphics_DirectX_Direct3D11`*"]
    pub fn CreateFromSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(surface: Param0, originalduration: Param1) -> ::windows::runtime::Result<MediaClip> {
        Self::IMediaClipStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), surface.into_param().abi(), originalduration.into_param().abi(), &mut result__).from_abi::<MediaClip>(result__)
        })
    }
    pub fn IMediaClipStatics<R, F: FnOnce(&IMediaClipStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaClip, IMediaClipStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMediaClipStatics2<R, F: FnOnce(&IMediaClipStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaClip, IMediaClipStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaClip {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaClip;{53f25366-5fba-3ea4-8693-24761811140a})");
}
unsafe impl ::windows::runtime::Interface for MediaClip {
    type Vtable = IMediaClip_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1408389990, 24506, 16036, [134, 147, 36, 118, 24, 17, 20, 10]);
}
impl ::windows::runtime::RuntimeName for MediaClip {
    const NAME: &'static str = "Windows.Media.Editing.MediaClip";
}
impl ::std::convert::From<MediaClip> for ::windows::runtime::IUnknown {
    fn from(value: MediaClip) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaClip> for ::windows::runtime::IUnknown {
    fn from(value: &MediaClip) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaClip> for ::windows::runtime::IInspectable {
    fn from(value: MediaClip) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaClip> for ::windows::runtime::IInspectable {
    fn from(value: &MediaClip) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaClip {}
unsafe impl ::std::marker::Sync for MediaClip {}
#[doc = "*Required features: `Media_Editing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaComposition(::windows::runtime::IInspectable);
impl MediaComposition {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaComposition, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn Clips(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<MediaClip>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<MediaClip>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn BackgroundAudioTracks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<BackgroundAudioTrack>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn UserData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<MediaComposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaComposition>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Storage`*"]
    pub fn SaveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Graphics_Imaging`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timefromstart: Param0, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), timefromstart.into_param().abi(), scaledwidth, scaledheight, frameprecision, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Graphics::Imaging::ImageStream>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Foundation_Collections`, `Graphics_Imaging`, `Storage_Streams`*"]
    pub fn GetThumbnailsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::TimeSpan>>>(&self, timesfromstart: Param0, scaledwidth: i32, scaledheight: i32, frameprecision: VideoFramePrecision) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), timesfromstart.into_param().abi(), scaledwidth, scaledheight, frameprecision, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::ImageStream>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Media_Transcoding`, `Storage`*"]
    pub fn RenderToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, destination: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Transcoding", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Media_Transcoding`, `Storage`*"]
    pub fn RenderToFileWithTrimmingPreferenceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, destination: Param0, trimmingpreference: MediaTrimmingPreference) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), destination.into_param().abi(), trimmingpreference, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Transcoding", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Media_MediaProperties`, `Media_Transcoding`, `Storage`*"]
    pub fn RenderToFileWithProfileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, destination: Param0, trimmingpreference: MediaTrimmingPreference, encodingprofile: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), destination.into_param().abi(), trimmingpreference, encodingprofile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::Transcoding::TranscodeFailureReason, f64>>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `Media_Editing`, `Media_MediaProperties`*"]
    pub fn CreateDefaultEncodingProfile(&self) -> ::windows::runtime::Result<super::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Editing`, `Media_Core`*"]
    pub fn GenerateMediaStreamSource(&self) -> ::windows::runtime::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::MediaStreamSource>(result__)
        }
    }
    #[cfg(all(feature = "Media_Core", feature = "Media_MediaProperties"))]
    #[doc = "*Required features: `Media_Editing`, `Media_Core`, `Media_MediaProperties`*"]
    pub fn GenerateMediaStreamSourceWithProfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>>(&self, encodingprofile: Param0) -> ::windows::runtime::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), encodingprofile.into_param().abi(), &mut result__).from_abi::<super::Core::MediaStreamSource>(result__)
        }
    }
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Editing`, `Media_Core`*"]
    pub fn GeneratePreviewMediaStreamSource(&self, scaledwidth: i32, scaledheight: i32) -> ::windows::runtime::Result<super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), scaledwidth, scaledheight, &mut result__).from_abi::<super::Core::MediaStreamSource>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn OverlayLayers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<MediaOverlayLayer>> {
        let this = &::windows::runtime::Interface::cast::<IMediaComposition2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<MediaOverlayLayer>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Media_Editing`, `Foundation`, `Storage`*"]
    pub fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MediaComposition>> {
        Self::IMediaCompositionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaComposition>>(result__)
        })
    }
    pub fn IMediaCompositionStatics<R, F: FnOnce(&IMediaCompositionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaComposition, IMediaCompositionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaComposition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaComposition;{2e06e605-dc71-41d6-b837-2d2bc14a2947})");
}
unsafe impl ::windows::runtime::Interface for MediaComposition {
    type Vtable = IMediaComposition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(772204037, 56433, 16854, [184, 55, 45, 43, 193, 74, 41, 71]);
}
impl ::windows::runtime::RuntimeName for MediaComposition {
    const NAME: &'static str = "Windows.Media.Editing.MediaComposition";
}
impl ::std::convert::From<MediaComposition> for ::windows::runtime::IUnknown {
    fn from(value: MediaComposition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaComposition> for ::windows::runtime::IUnknown {
    fn from(value: &MediaComposition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaComposition> for ::windows::runtime::IInspectable {
    fn from(value: MediaComposition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaComposition> for ::windows::runtime::IInspectable {
    fn from(value: &MediaComposition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaComposition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaComposition {}
unsafe impl ::std::marker::Sync for MediaComposition {}
#[doc = "*Required features: `Media_Editing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaOverlay(::windows::runtime::IInspectable);
impl MediaOverlay {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn SetDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Opacity(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<MediaOverlay> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaOverlay>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Clip(&self) -> ::windows::runtime::Result<MediaClip> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaClip>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn AudioEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn SetAudioEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, MediaClip>>(clip: Param0) -> ::windows::runtime::Result<MediaOverlay> {
        Self::IMediaOverlayFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), clip.into_param().abi(), &mut result__).from_abi::<MediaOverlay>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Editing`, `Foundation`*"]
    pub fn CreateWithPositionAndOpacity<'a, Param0: ::windows::runtime::IntoParam<'a, MediaClip>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(clip: Param0, position: Param1, opacity: f64) -> ::windows::runtime::Result<MediaOverlay> {
        Self::IMediaOverlayFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), clip.into_param().abi(), position.into_param().abi(), opacity, &mut result__).from_abi::<MediaOverlay>(result__)
        })
    }
    pub fn IMediaOverlayFactory<R, F: FnOnce(&IMediaOverlayFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaOverlay, IMediaOverlayFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaOverlay {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaOverlay;{a902ae5d-7869-4830-8ab1-94dc01c05fa4})");
}
unsafe impl ::windows::runtime::Interface for MediaOverlay {
    type Vtable = IMediaOverlay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2835525213, 30825, 18480, [138, 177, 148, 220, 1, 192, 95, 164]);
}
impl ::windows::runtime::RuntimeName for MediaOverlay {
    const NAME: &'static str = "Windows.Media.Editing.MediaOverlay";
}
impl ::std::convert::From<MediaOverlay> for ::windows::runtime::IUnknown {
    fn from(value: MediaOverlay) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaOverlay> for ::windows::runtime::IUnknown {
    fn from(value: &MediaOverlay) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaOverlay> for ::windows::runtime::IInspectable {
    fn from(value: MediaOverlay) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaOverlay> for ::windows::runtime::IInspectable {
    fn from(value: &MediaOverlay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaOverlay {}
unsafe impl ::std::marker::Sync for MediaOverlay {}
#[doc = "*Required features: `Media_Editing`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaOverlayLayer(::windows::runtime::IInspectable);
impl MediaOverlayLayer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaOverlayLayer, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Editing`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<MediaOverlayLayer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaOverlayLayer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Editing`, `Foundation_Collections`*"]
    pub fn Overlays(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<MediaOverlay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<MediaOverlay>>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    #[doc = "*Required features: `Media_Editing`, `Media_Effects`*"]
    pub fn CustomCompositorDefinition(&self) -> ::windows::runtime::Result<super::Effects::IVideoCompositorDefinition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Effects::IVideoCompositorDefinition>(result__)
        }
    }
    #[cfg(feature = "Media_Effects")]
    #[doc = "*Required features: `Media_Editing`, `Media_Effects`*"]
    pub fn CreateWithCompositorDefinition<'a, Param0: ::windows::runtime::IntoParam<'a, super::Effects::IVideoCompositorDefinition>>(compositordefinition: Param0) -> ::windows::runtime::Result<MediaOverlayLayer> {
        Self::IMediaOverlayLayerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), compositordefinition.into_param().abi(), &mut result__).from_abi::<MediaOverlayLayer>(result__)
        })
    }
    pub fn IMediaOverlayLayerFactory<R, F: FnOnce(&IMediaOverlayLayerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaOverlayLayer, IMediaOverlayLayerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaOverlayLayer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Editing.MediaOverlayLayer;{a6d9ba57-eeda-46c6-bbe5-e398c84168ac})");
}
unsafe impl ::windows::runtime::Interface for MediaOverlayLayer {
    type Vtable = IMediaOverlayLayer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2799286871, 61146, 18118, [187, 229, 227, 152, 200, 65, 104, 172]);
}
impl ::windows::runtime::RuntimeName for MediaOverlayLayer {
    const NAME: &'static str = "Windows.Media.Editing.MediaOverlayLayer";
}
impl ::std::convert::From<MediaOverlayLayer> for ::windows::runtime::IUnknown {
    fn from(value: MediaOverlayLayer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaOverlayLayer> for ::windows::runtime::IUnknown {
    fn from(value: &MediaOverlayLayer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaOverlayLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaOverlayLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaOverlayLayer> for ::windows::runtime::IInspectable {
    fn from(value: MediaOverlayLayer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaOverlayLayer> for ::windows::runtime::IInspectable {
    fn from(value: &MediaOverlayLayer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaOverlayLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaOverlayLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaOverlayLayer {}
unsafe impl ::std::marker::Sync for MediaOverlayLayer {}
#[doc = "*Required features: `Media_Editing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MediaTrimmingPreference(pub i32);
impl MediaTrimmingPreference {
    pub const Fast: MediaTrimmingPreference = MediaTrimmingPreference(0i32);
    pub const Precise: MediaTrimmingPreference = MediaTrimmingPreference(1i32);
}
impl ::std::convert::From<i32> for MediaTrimmingPreference {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MediaTrimmingPreference {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MediaTrimmingPreference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Editing.MediaTrimmingPreference;i4)");
}
impl ::windows::runtime::DefaultType for MediaTrimmingPreference {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Editing`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VideoFramePrecision(pub i32);
impl VideoFramePrecision {
    pub const NearestFrame: VideoFramePrecision = VideoFramePrecision(0i32);
    pub const NearestKeyFrame: VideoFramePrecision = VideoFramePrecision(1i32);
}
impl ::std::convert::From<i32> for VideoFramePrecision {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VideoFramePrecision {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VideoFramePrecision {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Editing.VideoFramePrecision;i4)");
}
impl ::windows::runtime::DefaultType for VideoFramePrecision {
    type DefaultType = Self;
}
