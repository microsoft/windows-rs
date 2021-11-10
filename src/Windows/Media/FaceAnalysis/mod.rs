#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_FaceAnalysis`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DetectedFace(pub ::windows::runtime::IInspectable);
impl DetectedFace {
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn FaceBox(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapBounds = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapBounds>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DetectedFace {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.DetectedFace;{8200d454-66bc-34df-9410-e89400195414})");
}
unsafe impl ::windows::runtime::Interface for DetectedFace {
    type Vtable = IDetectedFace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8200d454_66bc_34df_9410_e89400195414);
}
impl ::windows::runtime::RuntimeName for DetectedFace {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.DetectedFace";
}
impl ::core::convert::From<DetectedFace> for ::windows::runtime::IUnknown {
    fn from(value: DetectedFace) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DetectedFace> for ::windows::runtime::IUnknown {
    fn from(value: &DetectedFace) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DetectedFace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DetectedFace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DetectedFace> for ::windows::runtime::IInspectable {
    fn from(value: DetectedFace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DetectedFace> for ::windows::runtime::IInspectable {
    fn from(value: &DetectedFace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DetectedFace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DetectedFace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DetectedFace {}
unsafe impl ::core::marker::Sync for DetectedFace {}
#[doc = "*Required features: `Media_FaceAnalysis`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FaceDetector(pub ::windows::runtime::IInspectable);
impl FaceDetector {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation`, `Foundation_Collections`, `Graphics_Imaging`*"]
    pub fn DetectFacesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>>(&self, image: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), image.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation`, `Foundation_Collections`, `Graphics_Imaging`*"]
    pub fn DetectFacesWithSearchAreaAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::SoftwareBitmap>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapBounds>>(&self, image: Param0, searcharea: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), image.into_param().abi(), searcharea.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn MinDetectableFaceSize(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn SetMinDetectableFaceSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn MaxDetectableFaceSize(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn SetMaxDetectableFaceSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation`*"]
    pub fn CreateAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FaceDetector>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FaceDetector>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation_Collections`, `Graphics_Imaging`*"]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::runtime::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), bitmappixelformat, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Media_FaceAnalysis`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IFaceDetectorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IFaceDetectorStatics<R, F: FnOnce(&IFaceDetectorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FaceDetector, IFaceDetectorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FaceDetector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceDetector;{16b672dc-fe6f-3117-8d95-c3f04d51630c})");
}
unsafe impl ::windows::runtime::Interface for FaceDetector {
    type Vtable = IFaceDetector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16b672dc_fe6f_3117_8d95_c3f04d51630c);
}
impl ::windows::runtime::RuntimeName for FaceDetector {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceDetector";
}
impl ::core::convert::From<FaceDetector> for ::windows::runtime::IUnknown {
    fn from(value: FaceDetector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FaceDetector> for ::windows::runtime::IUnknown {
    fn from(value: &FaceDetector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FaceDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FaceDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FaceDetector> for ::windows::runtime::IInspectable {
    fn from(value: FaceDetector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FaceDetector> for ::windows::runtime::IInspectable {
    fn from(value: &FaceDetector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FaceDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FaceDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FaceDetector {}
unsafe impl ::core::marker::Sync for FaceDetector {}
#[doc = "*Required features: `Media_FaceAnalysis`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FaceTracker(pub ::windows::runtime::IInspectable);
impl FaceTracker {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation`, `Foundation_Collections`*"]
    pub fn ProcessNextFrameAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::VideoFrame>>(&self, videoframe: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), videoframe.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<DetectedFace>>>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn MinDetectableFaceSize(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn SetMinDetectableFaceSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn MaxDetectableFaceSize(&self) -> ::windows::runtime::Result<super::super::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapSize>(result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn SetMaxDetectableFaceSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation`*"]
    pub fn CreateAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<FaceTracker>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<FaceTracker>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Foundation_Collections`, `Graphics_Imaging`*"]
    pub fn GetSupportedBitmapPixelFormats() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>(result__)
        })
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Media_FaceAnalysis`, `Graphics_Imaging`*"]
    pub fn IsBitmapPixelFormatSupported(bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::runtime::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), bitmappixelformat, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Media_FaceAnalysis`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IFaceTrackerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IFaceTrackerStatics<R, F: FnOnce(&IFaceTrackerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FaceTracker, IFaceTrackerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FaceTracker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.FaceAnalysis.FaceTracker;{6ba67d8c-a841-4420-93e6-2420a1884fcf})");
}
unsafe impl ::windows::runtime::Interface for FaceTracker {
    type Vtable = IFaceTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6ba67d8c_a841_4420_93e6_2420a1884fcf);
}
impl ::windows::runtime::RuntimeName for FaceTracker {
    const NAME: &'static str = "Windows.Media.FaceAnalysis.FaceTracker";
}
impl ::core::convert::From<FaceTracker> for ::windows::runtime::IUnknown {
    fn from(value: FaceTracker) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FaceTracker> for ::windows::runtime::IUnknown {
    fn from(value: &FaceTracker) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FaceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FaceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FaceTracker> for ::windows::runtime::IInspectable {
    fn from(value: FaceTracker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FaceTracker> for ::windows::runtime::IInspectable {
    fn from(value: &FaceTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FaceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FaceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for FaceTracker {}
unsafe impl ::core::marker::Sync for FaceTracker {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectedFace(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDetectedFace {
    type Vtable = IDetectedFace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8200d454_66bc_34df_9410_e89400195414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectedFace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapBounds) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFaceDetector(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFaceDetector {
    type Vtable = IFaceDetector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16b672dc_fe6f_3117_8d95_c3f04d51630c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, image: ::windows::runtime::RawPtr, searcharea: super::super::Graphics::Imaging::BitmapBounds, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFaceDetectorStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFaceDetectorStatics {
    type Vtable = IFaceDetectorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbc042d67_9047_33f6_881b_6746c1b218b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFaceTracker(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFaceTracker {
    type Vtable = IFaceTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6ba67d8c_a841_4420_93e6_2420a1884fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, videoframe: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Graphics::Imaging::BitmapSize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFaceTrackerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFaceTrackerStatics {
    type Vtable = IFaceTrackerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe9629198_1801_3fa5_932e_31d767af6c4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceTrackerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmappixelformat: super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
